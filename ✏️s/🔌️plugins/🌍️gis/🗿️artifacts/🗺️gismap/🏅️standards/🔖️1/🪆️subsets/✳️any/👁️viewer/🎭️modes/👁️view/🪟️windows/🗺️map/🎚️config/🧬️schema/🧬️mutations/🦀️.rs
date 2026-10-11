//! 🧬️ GIS map viewer window-config mutations — one verb, `set-camera`.

use super::{GisMapViewerCamera, GisMapViewerWindowConfig, GisMapViewerWindowConfigDiff};
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = GisMapViewerWindowConfig, diff = GisMapViewerWindowConfigDiff, schema = "gis.mapviewerwindowcfg")]
pub enum GisMapViewerWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge

impl store::snapshot_clone_preparation::ConfigApplyMutation<GisMapViewerWindowConfig> for GisMapViewerWindowConfigMutation {
    fn exchange(self, post: &mut GisMapViewerWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetCamera(SetCamera { camera }) => Self::SetCamera(SetCamera { camera: std::mem::replace(&mut post.camera, camera) }),
        })
    }
    fn payload_bytes(&self) -> usize {
        0
    }
}

