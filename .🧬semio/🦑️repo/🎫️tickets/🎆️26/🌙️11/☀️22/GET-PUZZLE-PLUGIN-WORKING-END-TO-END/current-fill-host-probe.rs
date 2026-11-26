#![allow(unused)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️prelude/🦀️.rs"));
mod context {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️context/🦀️.rs"));
}
use context::*;

#[semio_framework_async_macros::async_test]
async fn publish_fill_count_from_original_window() {
    let mut app=app().await;
    let _=committed_document(&app);
    dispatch(&mut app,"setFillCount",Some(&json!({"value":3})),None).await.expect("first fill count");
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app,"fill completes",|p|p.state==protocol::PresenceToolRunState::Complete).await;
    dispatch(&mut app,"setFillCount",Some(&json!({"value":5})),None).await.expect("change running count");
}
