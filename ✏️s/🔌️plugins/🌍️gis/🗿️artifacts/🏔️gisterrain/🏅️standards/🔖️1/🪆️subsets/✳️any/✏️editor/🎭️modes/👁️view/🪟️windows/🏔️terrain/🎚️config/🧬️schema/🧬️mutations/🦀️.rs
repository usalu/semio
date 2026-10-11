//! 🧬️ Transparent GIS 3D configuration mutation roster.
use super::{GisTerrainWindowConfig, GisTerrainWindowConfigDiff};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🧬️Leaves
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
//#endregion 🧬️Leaves
//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = GisTerrainWindowConfig, diff = GisTerrainWindowConfigDiff, schema = "gis.gisterrainwindowcfg")]
pub enum GisTerrainWindowConfigMutation {
    SetCamera(SetCamera),
}
impl store::snapshot_clone_preparation::ConfigApplyMutation<GisTerrainWindowConfig> for GisTerrainWindowConfigMutation {
    fn exchange(self, post: &mut GisTerrainWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetCamera(SetCamera { camera_json }) => Self::SetCamera(SetCamera { camera_json: std::mem::replace(&mut post.camera_json, camera_json) }),
        })
    }
    fn payload_bytes(&self) -> usize {
        match self {
            Self::SetCamera(SetCamera { camera_json }) => camera_json.len(),
        }
    }
}

//#endregion 🧬️Aggregate
