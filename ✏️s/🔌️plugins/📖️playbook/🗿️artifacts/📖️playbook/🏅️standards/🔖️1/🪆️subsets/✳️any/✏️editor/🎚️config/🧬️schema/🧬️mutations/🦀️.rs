//! 🧬️ Playbook configuration mutation collection.

use super::{PlaybookConfig, PlaybookConfigDiff};
#[path = "📸️replace/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "🧩️set/🦀️.rs"]
mod set_contributions;
pub use set_contributions::SetContributions;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = PlaybookConfig, diff = PlaybookConfigDiff, schema = "playbook.config")]
pub enum PlaybookConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-contributions")]
    SetContributions(SetContributions),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
