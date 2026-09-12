use super::*;
use crate::editor::flow::unit_tests::context::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_compiled_wire_editor() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_COMPILED).await.contains("text-editor"));
}
