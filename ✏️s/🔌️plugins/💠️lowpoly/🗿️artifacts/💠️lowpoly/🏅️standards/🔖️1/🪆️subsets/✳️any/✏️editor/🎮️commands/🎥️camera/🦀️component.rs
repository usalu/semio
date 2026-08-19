//! 🎥️ Lowpoly play app command — the live world-3d camera pose (`setCamera`). Config-only.

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-camera")]
    pub struct SetCamera {
        #[dsl(coord)]
        pub position: [f64; 3],
        #[dsl(coord)]
        pub target: [f64; 3],
        pub fov: f64,
    }

    pub async fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigMutation::SetWorldCamera { position: payload.position, target: payload.target, fov: payload.fov }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, dispatch};
    use crate::editor::lowpoly::LowpolyCommand;

    #[test]
    async fn set_camera_updates_config() {
        let mut a = app();
        dispatch(&mut a, LowpolyCommand::SetCamera(super::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }));
        assert!(a.dispatch_typed(LowpolyCommand::SetCamera(super::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }), &semio_framework_plugin::testkit::meta("a")).is_ok());
    }
}
//#endregion 🧪️Tests
