use crate::editor::sequence::modes::edit::windows::compiled::SEQUENCE_PLAY_BODY_COMPILED;
use crate::editor_unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_compiled_wire_editor() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_COMPILED).await.contains("text-editor"));
}
