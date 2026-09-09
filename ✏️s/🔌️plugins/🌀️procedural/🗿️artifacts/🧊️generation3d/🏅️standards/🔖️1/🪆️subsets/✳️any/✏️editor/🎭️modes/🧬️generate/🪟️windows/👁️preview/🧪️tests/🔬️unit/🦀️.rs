use super::*;
use crate::editor::generation3d::testkit::{app, render as render_body};
use semio_framework_plugin::testkit::decode_fixture_scene;

/// 📝️ With no generation selected the window falls back to a TEXT-EDITOR surface carrying the hint,
/// and a surface's text rides inside its packed `doc`, never as JSON in the projected tree — asserting
/// on the projection string alone read the pack bytes, not the words
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn generate_preview_hints_without_evaluated_output() {
    let mut app = app().await;
    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW).await;
    let scene: TextEditorScene = decode_fixture_scene(&body).expect("the unevaluated generate preview is a text-editor surface");
    assert!(scene.buffer.contains("evaluate a generation"), "the generate-mode preview must hint at picking a generation: {}", scene.buffer);
}
