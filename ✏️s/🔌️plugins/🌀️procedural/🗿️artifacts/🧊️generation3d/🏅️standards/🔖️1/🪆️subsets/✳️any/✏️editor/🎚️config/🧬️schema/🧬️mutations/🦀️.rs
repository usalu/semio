//! 🎚️ Generation3d editor config — the closed semantic mutation aggregate.
//!
//! Six authored leaf directories, one per settled view-state interaction. Every leaf is an
//! immediate child of this aggregate's own mutation root, so `dsl::Mutations`'s leaf-ownership
//! contract holds and not one descriptor is provisional.

use super::{Generation3dConfigPatch, Generation3dSelectedGenerationChange, CameraJson, Generation3dConfig, Generation3dPreviewCamera};

#[path = "🔬️set-lod-mode/🦀️.rs"]
mod set_lod_mode;
#[path = "👁️set-show-mode/🦀️.rs"]
mod set_show_mode;
#[path = "🕸️set-camera/🦀️.rs"]
mod set_camera;
#[path = "📷️set-preview-camera/🦀️.rs"]
mod set_preview_camera;
#[path = "🌞️set-sun/🦀️.rs"]
mod set_sun;
#[path = "🧬️set-selected/🦀️.rs"]
mod set_selected_generation;

pub use set_camera::SetCamera;
pub use set_lod_mode::SetLodMode;
pub use set_preview_camera::SetPreviewCamera;
pub use set_selected_generation::SetSelectedGeneration;
pub use set_show_mode::SetShowMode;
pub use set_sun::SetSun;

/// 🧮️ [`Generation3dConfig`]'s operation enum — one newtype variant per authored leaf, in binary-tag
/// order. Appending is safe, reordering is a wire-format break.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Generation3dConfig, diff = Generation3dConfigPatch, schema = "generation3dcfg")]
pub enum Generation3dConfigMutation {
    #[dsl(key = "lod-mode")]
    SetLodMode(SetLodMode),
    #[dsl(key = "show-mode")]
    SetShowMode(SetShowMode),
    #[dsl(key = "camera")]
    SetCamera(SetCamera),
    #[dsl(key = "preview-camera")]
    SetPreviewCamera(SetPreviewCamera),
    #[dsl(key = "sun")]
    SetSun(SetSun),
    #[dsl(key = "selected-generation")]
    SetSelectedGeneration(SetSelectedGeneration),
}


impl store::snapshot_clone_preparation::ConfigApplyMutation<Generation3dConfig> for Generation3dConfigMutation {
    fn exchange(self, post: &mut Generation3dConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetLodMode(SetLodMode { value }) => Self::SetLodMode(SetLodMode { value: std::mem::replace(&mut post.lod_mode, value) }),
            Self::SetShowMode(SetShowMode { value }) => Self::SetShowMode(SetShowMode { value: std::mem::replace(&mut post.show_mode, value) }),
            Self::SetCamera(SetCamera { camera }) => Self::SetCamera(SetCamera { camera: std::mem::replace(&mut post.camera, camera) }),
            Self::SetPreviewCamera(SetPreviewCamera { camera }) => Self::SetPreviewCamera(SetPreviewCamera { camera: std::mem::replace(&mut post.preview_camera, camera) }),
            Self::SetSun(SetSun { json }) => Self::SetSun(SetSun { json: std::mem::replace(&mut post.sun_json, json) }),
            Self::SetSelectedGeneration(SetSelectedGeneration { selected_generation_id }) => Self::SetSelectedGeneration(SetSelectedGeneration { selected_generation_id: std::mem::replace(&mut post.selected_generation_id, selected_generation_id) }),
        })
    }

    fn payload_bytes(&self) -> usize {
        match self {
            Self::SetLodMode(SetLodMode { value }) => value.len(),
            Self::SetShowMode(SetShowMode { value }) => value.len(),
            Self::SetCamera(SetCamera { camera }) => { let _ = camera; 0 },
            Self::SetPreviewCamera(SetPreviewCamera { camera }) => { let _ = camera; 0 },
            Self::SetSun(SetSun { json }) => json.len(),
            Self::SetSelectedGeneration(SetSelectedGeneration { selected_generation_id }) => selected_generation_id.as_ref().map_or(0, String::len),
        }
    }
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
