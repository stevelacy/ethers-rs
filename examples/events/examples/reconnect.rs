use ethers::prelude::*;
use eyre::Result;
use std::{sync::Arc, time::Duration};
use tokio::time;
use tracing::info;
// use tracing_subscriber;

const WS_URL: &str = "ws://localhost:8545";

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();
    let client = Provider::<Ws>::connect_with_reconnects(WS_URL, 10).await?;

    let _ = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            let res = client.get_block(0).await;
            println!("Block number: {:?}", res);
        }
    })
    .await;

    Ok(())
}
