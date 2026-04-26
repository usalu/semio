//! `semio-store`: HTTP GraphQL (stderr = logs). See `jsonrpc.rs` (name kept) for routes.

mod jsonrpc;

use std::io;

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .or_else(|_| std::env::var("RUST_TRACING"))
                .unwrap_or_else(|_| "error,semio_store=info,semio_store_event=off".to_string()),
        )
        .with_target(false)
        .with_writer(io::stderr)
        .try_init();

    jsonrpc::run().await
}
