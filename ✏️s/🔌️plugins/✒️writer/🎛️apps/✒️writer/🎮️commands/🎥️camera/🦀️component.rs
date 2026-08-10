//! 🎥️ Writer play app commands — the editor viewport camera. Config-only: the viewport never touches
//! the document.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::{WriterCamera, WriterSnapshot};
use crate::artifacts::writer::op::WriterMutation;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: WriterCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(Emit::config(vec![WriterConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_camera;
    use crate::apps::writer::testkit::new_app;
    use crate::apps::writer::{WriterCommand, WRITER_PLAY_BODY_MAIN};
    use crate::artifacts::writer::WriterCamera;
    use semio_framework_plugin::{PluginApp, ViewModel};
    use serde_json::{json, Value};

    /// 🎥️ `SetCamera` is a config-only command — it must never emit a `WriterMutation` (no VCS edit,
    /// no undo entry) and instead write into `WriterConfig`, reflected in render.
    #[test]
    fn set_camera_command_writes_config_not_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetCamera(set_camera::SetCamera { camera: WriterCamera { x: 3.0, y: 4.0, zoom: 2.0 } }), &semio_framework_plugin::testkit::meta("local")).expect("set camera");
        assert!(result.mutations.is_empty(), "setCamera must not emit a VCS operation");
        let node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewModel::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let camera: Value = serde_json::from_str(payload["textEditor"]["cameraJson"].as_str().unwrap()).unwrap();
        assert_eq!(camera["x"], json!(3.0));
        assert_eq!(camera["zoom"], json!(2.0));
    }
}
//#endregion 🧪️Tests
