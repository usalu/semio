//! 🧬️ Architect architect.config mutation collection.

use super::*;
#[path = "📸️set-config/🦀️.rs"]
mod set_config;
pub use set_config::SetConfig;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = ArchitectConfig, diff = ArchitectConfigDiff, schema = "architect.config")]
pub enum ArchitectConfigMutation {
    #[dsl(key = "set-config")]
    SetConfig(SetConfig),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
