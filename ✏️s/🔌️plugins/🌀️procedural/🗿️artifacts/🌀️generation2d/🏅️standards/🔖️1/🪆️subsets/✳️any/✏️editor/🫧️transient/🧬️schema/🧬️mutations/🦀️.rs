//! 🫧️ Generation2d app-transient mutation aggregate.

use super::{Generation2dTransient, Generation2dTransientPatch};

#[path = "👁️set-generation/🦀️.rs"]
mod set_generation_preview;
pub use set_generation_preview::SetGenerationPreview;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = Generation2dTransient, diff = Generation2dTransientPatch, schema = "procedural.generation2dtransient")]
pub enum Generation2dTransientMutation {
    #[dsl(key = "set-generation-preview")]
    SetGenerationPreview(SetGenerationPreview),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
