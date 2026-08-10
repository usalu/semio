//! 🎥️ Fem2d play app commands — the canvas camera (pan/zoom). Config-only: never touches the document.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::FemCamera;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub x: f64,
        pub y: f64,
        pub zoom: f64,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Fem2dConfigMutation::SetCamera { camera: FemCamera { x: payload.x, y: payload.y, zoom: payload.zoom } }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app};
    use crate::apps::fem2d::Fem2dCommand;

    #[test]
    fn set_camera_action_writes_config_not_artifact_mutations() {
        let mut app = fem2d_app();
        let before = app.snapshot().expect("snapshot");
        let result = dispatch(&mut app, Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 }));
        assert!(result.mutations.is_empty(), "setCamera must not emit a document VCS operation");
        assert_eq!(app.snapshot().expect("snapshot"), before, "the document must be unchanged by a config-only command");
    }
}
//#endregion 🧪️Tests
