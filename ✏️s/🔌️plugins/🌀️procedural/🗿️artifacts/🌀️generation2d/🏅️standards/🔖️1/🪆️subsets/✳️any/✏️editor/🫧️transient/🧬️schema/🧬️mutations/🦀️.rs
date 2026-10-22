//! 🫧️ Generation2d app-transient mutation aggregate.

use super::{Generation2dTransient, Generation2dTransientPatch};

#[path = "👁️set-generation-preview/🦀️.rs"]
mod set_generation_preview;
pub use set_generation_preview::SetGenerationPreview;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Generation2dTransient, diff = Generation2dTransientPatch, schema = "procedural.generation2dtransient", reference = Generation2dTransient, reference_diff = Generation2dTransientPatch)]
pub enum Generation2dTransientMutation {
    #[dsl(key = "set-generation-preview")]
    SetGenerationPreview(SetGenerationPreview),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
