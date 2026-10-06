//! 🧬️ Wires canvas configuration mutations.

use super::{WiresCanvasCamera, WiresCanvasWindowConfig};
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = WiresCanvasWindowConfig, diff = WiresCanvasWindowConfig, schema = "reasoning.wirescanvaswindowconfig")]
pub enum WiresCanvasWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
