//! 🫧️ Generation3d app-transient mutation aggregate.

use super::{Generation3dTransient, Generation3dTransientPatch};

#[path = "👁️set-generation/🦀️.rs"]
mod set_generation_preview;
pub use set_generation_preview::SetGenerationPreview;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = Generation3dTransient, diff = Generation3dTransientPatch, schema = "generation.3dtransient")]
pub enum Generation3dTransientMutation {
    #[dsl(key = "set-generation-preview")]
    SetGenerationPreview(SetGenerationPreview),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
