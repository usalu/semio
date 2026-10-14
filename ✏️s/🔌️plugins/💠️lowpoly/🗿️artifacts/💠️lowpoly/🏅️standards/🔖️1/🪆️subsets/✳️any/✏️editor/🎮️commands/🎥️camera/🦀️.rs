//! 🎥️ Lowpoly play app command — the live world-3d camera pose (`setCamera`). Config-only.

use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation, SetWorldCameraEdit};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::LowpolyScratch;
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-camera")]
    pub struct SetCamera {
        #[dsl(coord)]
        pub position: [f64; 3],
        #[dsl(coord)]
        pub target: [f64; 3],
        pub fov: f64,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigMutation::SetWorldCamera(SetWorldCameraEdit { position: payload.position, target: payload.target, fov: payload.fov })]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
//#endregion 🧪️Tests
