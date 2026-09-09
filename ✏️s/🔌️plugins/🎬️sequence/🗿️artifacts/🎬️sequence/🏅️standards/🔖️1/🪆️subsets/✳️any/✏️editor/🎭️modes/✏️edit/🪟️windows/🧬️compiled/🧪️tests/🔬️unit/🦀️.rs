use super::*;
use crate::editor::sequence::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_compiled_wire_editor() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_COMPILED).await.contains("text-editor"));
}
