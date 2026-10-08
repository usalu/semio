//! 🧬️ Architect architect.config mutation collection.

use super::*;
#[path = "📸️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = ArchitectConfig, diff = ArchitectConfigDiff, schema = "architect.config")]
pub enum ArchitectConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
