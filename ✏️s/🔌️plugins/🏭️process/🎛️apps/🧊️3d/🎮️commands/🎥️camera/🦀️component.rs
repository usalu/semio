//! 🎥️ Process 3d play app commands — the 3D viewport camera (config-only, ephemeral view state).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::artifacts::process3d::{op::Process3dOperation, Process3dDocument};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
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

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigOperation::SetCamera { position: payload.position, target: payload.target, fov: payload.fov }]))
    }
}
//#endregion 🔖️SetCamera
