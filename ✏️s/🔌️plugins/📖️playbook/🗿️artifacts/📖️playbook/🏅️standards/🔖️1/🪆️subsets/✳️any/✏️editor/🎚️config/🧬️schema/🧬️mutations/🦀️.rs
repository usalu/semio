//! 🧬️ Playbook configuration mutation collection.

use super::PlaybookConfig;
#[path = "📸️replace/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "🧩️set/🦀️.rs"]
mod set_contributions;
pub use set_contributions::SetContributions;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = PlaybookConfig, diff = PlaybookConfig, schema = "playbook.config")]
pub enum PlaybookConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-contributions")]
    SetContributions(SetContributions),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
