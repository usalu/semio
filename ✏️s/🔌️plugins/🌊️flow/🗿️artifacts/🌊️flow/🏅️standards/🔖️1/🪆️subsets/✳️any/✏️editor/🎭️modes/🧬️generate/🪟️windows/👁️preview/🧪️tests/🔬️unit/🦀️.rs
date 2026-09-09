use super::*;
use crate::editor::flow::testkit::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_preview_renders_a_text_editor_surface() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW).await.contains("text-editor"));
}
