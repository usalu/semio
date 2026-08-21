//! 🎥️ Process 3d play app commands — the 3D viewport camera (config-only, ephemeral view state).

use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(coord)]
        pub position: [f64; 3],
        #[dsl(coord)]
        pub target: [f64; 3],
        pub fov: f64,
    }

    pub async fn handle(
        payload: &SetCamera,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetCamera { position: payload.position, target: payload.target, fov: payload.fov }]))
    }
}
//#endregion 🔖️SetCamera
