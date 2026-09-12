use super::SetCamera;
use crate::editor::writer::unit_tests::context::{dispatch, main_window_view, new_app};
use crate::editor::writer::{WriterCommand, WRITER_PLAY_BODY_MAIN};
use crate::WriterCamera;
use semio_framework_plugin::PluginApp;
use serde_json::{json, Value};

/// 🎥️ `SetCamera` publishes only to the addressed main-window configuration and renders there.
#[semio_framework_async_macros::async_test]
async fn set_camera_command_writes_config_not_operations() {
    let mut app = new_app().await;
    let result = dispatch(&mut app, WriterCommand::SetCamera(SetCamera { camera: WriterCamera { x: 3.0, y: 4.0, zoom: 2.0 } })).await;
    assert!(result.mutations.is_empty(), "setCamera must not emit a VCS operation");
    let node = app.render(WRITER_PLAY_BODY_MAIN, None, &main_window_view()).await.expect("render");
    let payload = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("render JSON");
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::TextEditorScene>(&payload).expect("Writer scene");
    let camera: Value = serde_json::from_str(scene.camera_json.as_deref().expect("camera JSON")).unwrap();
    assert_eq!(camera["x"], json!(3.0));
    assert_eq!(camera["zoom"], json!(2.0));
}
