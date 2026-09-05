//! 🎥️ 🎥️ Fem2d play app commands command — `set-camera`.

use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::FemCamera;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️SetCamera
//#endregion 🔖️SetCamera

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Fem2dConfigMutation::SetCamera { camera: FemCamera { x: payload.x, y: payload.y, zoom: payload.zoom } }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{dispatch, fem2d_app};
    use crate::editor::fem2d::Fem2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_camera_action_writes_config_not_artifact_mutations() {
        let mut app = fem2d_app();
        let before = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let result = dispatch(&mut app, Fem2dCommand::SetCamera(SetCamera { x: 1.0, y: 2.0, zoom: 1.5 })).await;
        assert!(result.mutations.is_empty(), "setCamera must not emit a document VCS operation");
        assert_eq!(semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot"), before, "the document must be unchanged by a config-only command");
    }
}
//#endregion 🧪️Tests
