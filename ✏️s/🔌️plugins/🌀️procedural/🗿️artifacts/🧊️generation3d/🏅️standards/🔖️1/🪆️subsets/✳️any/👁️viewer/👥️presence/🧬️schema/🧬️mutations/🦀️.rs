//! 👥️ Generation3d viewer presence — the closed semantic mutation aggregate.
//!
//! Two authored leaf directories, one per shareable live facet. Both are immediate children of this
//! aggregate's own mutation root, so `dsl::Mutations`'s leaf-ownership contract holds.

use super::{Generation3dViewPresence, Generation3dViewPresencePatch};
use crate::viewer::generation3d::config::Generation3dViewCamera;

#[path = "📷️set-preview/🦀️.rs"]
mod set_preview_camera;
#[path = "👁️set-show-mode/🦀️.rs"]
mod set_show_mode;

pub use set_preview_camera::SetPreviewCamera;
pub use set_show_mode::SetShowMode;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = Generation3dViewPresence, diff = Generation3dViewPresencePatch, schema = "generation3dview.presence")]
pub enum Generation3dViewPresenceMutation {
    #[dsl(key = "preview-camera")]
    SetPreviewCamera(SetPreviewCamera),
    #[dsl(key = "show-mode")]
    SetShowMode(SetShowMode),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
