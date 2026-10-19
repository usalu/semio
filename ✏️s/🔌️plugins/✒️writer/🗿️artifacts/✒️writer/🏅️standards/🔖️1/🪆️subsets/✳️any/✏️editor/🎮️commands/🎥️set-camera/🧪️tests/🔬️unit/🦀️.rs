use crate::editor::writer::commands::set_camera::SetCamera;
use crate::editor_unit_tests::context::{dispatch, main_window_view, new_app};
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
    let payload = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, WRITER_PLAY_BODY_MAIN, &main_window_view(), app.1.cx()).await.expect("render JSON");
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::TextEditorScene>(&payload).expect("Writer scene");
    let camera: Value = serde_json::from_str(scene.camera_json.as_deref().expect("camera JSON")).unwrap();
    assert_eq!(camera["x"], json!(3.0));
    assert_eq!(camera["zoom"], json!(2.0));
}
