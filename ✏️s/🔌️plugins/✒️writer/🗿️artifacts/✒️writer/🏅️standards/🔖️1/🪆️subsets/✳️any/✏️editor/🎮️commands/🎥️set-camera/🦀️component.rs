//! 🎥️ 🎥️ Writer play app commands command — `set-camera`.

use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{WriterCamera, WriterSnapshot};
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: WriterCamera,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    Ok(Emit::config(vec![WriterConfigMutation::SetCamera { camera: payload.camera.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::SetCamera;
    use crate::artifacts::writer::WriterCamera;
    use crate::editor::writer::testkit::new_app;
    use crate::editor::writer::{WriterCommand, WRITER_PLAY_BODY_MAIN};
    use semio_framework_plugin::{PluginApp, ViewModel};
    use serde_json::{json, Value};

    /// 🎥️ `SetCamera` is a config-only command — it must never emit a `WriterMutation` (no VCS edit,
    /// no undo entry) and instead write into `WriterConfig`, reflected in render.
    #[semio_framework_async_macros::async_test]
    async fn set_camera_command_writes_config_not_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetCamera(SetCamera { camera: WriterCamera { x: 3.0, y: 4.0, zoom: 2.0 } }), &semio_framework_plugin::testkit::meta("local")).expect("set camera");
        assert!(result.mutations.is_empty(), "setCamera must not emit a VCS operation");
        let node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewModel::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let camera: Value = serde_json::from_str(payload["textEditor"]["cameraJson"].as_str().unwrap()).unwrap();
        assert_eq!(camera["x"], json!(3.0));
        assert_eq!(camera["zoom"], json!(2.0));
    }
}
//#endregion 🧪️Tests
