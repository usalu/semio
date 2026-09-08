
use super::SetCamera;
use crate::editor::writer::testkit::new_app;
use crate::editor::writer::{WriterCommand, WRITER_PLAY_BODY_MAIN};
use crate::WriterCamera;
use semio_framework_plugin::{PluginApp, ViewModel};
use serde_json::{json, Value};

/// 🎥️ `SetCamera` is a config-only command — it must never emit a `WriterMutation` (no VCS edit,
/// no undo entry) and instead write into `WriterConfig`, reflected in render.
#[semio_framework_async_macros::async_test]
async fn set_camera_command_writes_config_not_operations() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(WriterCommand::SetCamera(SetCamera { camera: WriterCamera { x: 3.0, y: 4.0, zoom: 2.0 } }), &semio_framework_plugin::testkit::meta("local")).await.expect("set camera");
    assert!(result.mutations.is_empty(), "setCamera must not emit a VCS operation");
    let node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewModel::default()).await.expect("render");
    let payload: Value = serde_json::from_str(&semio_framework_plugin::testkit::project_and_retire_fixture_tree(node).expect("render JSON")).expect("JSON oracle");
    let camera: Value = serde_json::from_str(payload["textEditor"]["cameraJson"].as_str().unwrap()).unwrap();
    assert_eq!(camera["x"], json!(3.0));
    assert_eq!(camera["zoom"], json!(2.0));
}
