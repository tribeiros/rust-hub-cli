#![allow(unused)]

use color_eyre::{Result};
use structopt::StructOpt;
use serde::{Serialize, Deserialize};

#[derive(StructOpt)]
struct Kind {
  kind: String,
  #[structopt(subcommand)]
  command: Opt
}

#[derive(StructOpt)]
enum Opt {
  Alarms
}

#[derive(Serialize, Deserialize, Debug)]
struct Alarme {
    description: String,
    plugin_output: String,
    nokFromNow: String
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = reqwest::Client::new();
  let args = Kind::from_args();
  match args.command {
    Opt::Alarms => {
      let response = client
        .get(format!("<uri>"))
        .query(&[("key", args.kind)])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
      
      let resposta = response.to_string();
      let datas: Vec<Alarme> = serde_json::from_str(&resposta)?;

      for data in datas.iter() {
        println!("{:#?}", data);
    }

    }
  }
  Ok(())
}
