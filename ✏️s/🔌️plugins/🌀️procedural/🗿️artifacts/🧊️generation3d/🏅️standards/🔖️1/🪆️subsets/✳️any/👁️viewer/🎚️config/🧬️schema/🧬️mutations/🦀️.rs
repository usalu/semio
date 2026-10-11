//! 🎚️ Generation3d viewer config — the closed semantic mutation aggregate.
//!
//! Five authored leaf directories, one per settled read-only interaction. Every leaf is an
//! immediate child of this aggregate's own mutation root, so `dsl::Mutations`'s leaf-ownership
//! contract holds without a single provisional descriptor.

use super::{Generation3dViewConfigPatch, Generation3dActiveExampleChange, Generation3dViewCamera, Generation3dViewConfig};

#[path = "👁️set-show-mode/🦀️.rs"]
mod set_show_mode;
#[path = "🔬️set-lod-mode/🦀️.rs"]
mod set_lod_mode;
#[path = "📷️set-preview-camera/🦀️.rs"]
mod set_preview_camera;
#[path = "🌞️set-sun/🦀️.rs"]
mod set_sun;
#[path = "🎨️set-active-example/🦀️.rs"]
mod set_active_example;

pub use set_active_example::SetActiveExample;
pub use set_lod_mode::SetLodMode;
pub use set_preview_camera::SetPreviewCamera;
pub use set_show_mode::SetShowMode;
pub use set_sun::SetSun;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Generation3dViewConfig, diff = Generation3dViewConfigPatch, schema = "generation3dviewcfg")]
pub enum Generation3dViewConfigMutation {
    #[dsl(key = "show-mode")]
    SetShowMode(SetShowMode),
    #[dsl(key = "lod-mode")]
    SetLodMode(SetLodMode),
    #[dsl(key = "preview-camera")]
    SetPreviewCamera(SetPreviewCamera),
    #[dsl(key = "sun")]
    SetSun(SetSun),
    /// 🎨️ Which bundled example this read-only surface is looking at — a config leaf, because a
    /// viewer opens a document rather than rewriting one.
    #[dsl(key = "active-example")]
    SetActiveExample(SetActiveExample),
}


impl store::snapshot_clone_preparation::ConfigApplyMutation<Generation3dViewConfig> for Generation3dViewConfigMutation {
    fn exchange(self, post: &mut Generation3dViewConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetShowMode(SetShowMode { value }) => Self::SetShowMode(SetShowMode { value: std::mem::replace(&mut post.show_mode, value) }),
            Self::SetLodMode(SetLodMode { value }) => Self::SetLodMode(SetLodMode { value: std::mem::replace(&mut post.lod_mode, value) }),
            Self::SetPreviewCamera(SetPreviewCamera { camera }) => Self::SetPreviewCamera(SetPreviewCamera { camera: std::mem::replace(&mut post.preview_camera, camera) }),
            Self::SetSun(SetSun { json }) => Self::SetSun(SetSun { json: std::mem::replace(&mut post.sun_json, json) }),
            Self::SetActiveExample(SetActiveExample { value }) => Self::SetActiveExample(SetActiveExample { value: std::mem::replace(&mut post.active_example_id, value) }),
        })
    }

    fn payload_bytes(&self) -> usize {
        match self {
            Self::SetShowMode(SetShowMode { value }) => value.len(),
            Self::SetLodMode(SetLodMode { value }) => value.len(),
            Self::SetPreviewCamera(SetPreviewCamera { camera }) => { let _ = camera; 0 },
            Self::SetSun(SetSun { json }) => json.len(),
            Self::SetActiveExample(SetActiveExample { value }) => value.as_ref().map_or(0, String::len),
        }
    }
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
