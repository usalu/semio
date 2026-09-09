use super::*;
use crate::editor::sequence::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_script_editor() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_SCRIPT).await.contains("text-editor"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_text_editor_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, SEQUENCE_PLAY_BODY_SCRIPT);
    assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
}
