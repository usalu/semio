//! 🧬️ Presentation presentation.config mutation collection.

use super::*;
#[path = "⌨️set-engagement-input/🦀️.rs"]
mod set_engagement_input;
pub use set_engagement_input::SetEngagementInput;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = PresentationConfig, diff = PresentationConfigDiff, schema = "presentation.config")]
pub enum PresentationConfigMutation {
    #[dsl(key = "set-engagement-input")]
    SetEngagementInput(SetEngagementInput),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
