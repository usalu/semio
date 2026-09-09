//! 👁️ GIS 3D play app command — the free/live viewport camera. Config-only: it emits
//! `config_mutations`, never document operations.

use crate::editor::gis3d::config::{Gis3dConfig, Gis3dConfigMutation, SetCamera as SetCameraMutation};
use crate::op::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub camera_json: String,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis3dConfigMutation::SetCamera(SetCameraMutation { camera_json: payload.camera_json.clone() })]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
