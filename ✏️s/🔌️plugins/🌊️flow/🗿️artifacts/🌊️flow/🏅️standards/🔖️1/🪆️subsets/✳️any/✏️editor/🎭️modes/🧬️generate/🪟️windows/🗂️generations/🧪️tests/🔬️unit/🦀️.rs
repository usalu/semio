
use super::*;
use crate::editor::flow::testkit::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_empty_generation_list_still_offers_the_add_action() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATIONS).await.contains("addGeneration"));
}
