//! 🧬️ Forms app configuration mutations.

use super::*;
#[path = "📸️replace/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "🧩️set/🦀️.rs"]
mod set_contributions;
pub use set_contributions::SetContributions;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = FormsConfig, diff = FormsConfigDiff, schema = "forms.config")]
pub enum FormsConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-contributions")]
    SetContributions(SetContributions),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
