//! 🎚️ Generation3d viewer config — the closed semantic mutation aggregate.
//!
//! Five authored leaf directories, one per settled read-only interaction. Every leaf is an
//! immediate child of this aggregate's own mutation root, so `dsl::Mutations`'s leaf-ownership
//! contract holds without a single provisional descriptor.

use super::{Generation3dViewConfigPatch, Generation3dViewCamera, Generation3dViewConfig};

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

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
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





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
