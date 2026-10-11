//! 🧬️ Architect architect.config mutation collection.

use super::*;
#[path = "📸️set-config/🦀️.rs"]
mod set_config;
pub use set_config::SetConfig;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = ArchitectConfig, diff = ArchitectConfigDiff, schema = "architect.config")]
pub enum ArchitectConfigMutation {
    #[dsl(key = "set-config")]
    SetConfig(SetConfig),
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<ArchitectConfig> for ArchitectConfigMutation {
    fn exchange(self, post: &mut ArchitectConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        let swap = |slot: &mut String, requested: Option<String>| requested.map(|value| std::mem::replace(slot, value));
        Ok(match self {
            Self::SetConfig(SetConfig { search_query, search_history_json, last_result_json, last_analysis_json }) => Self::SetConfig(SetConfig {
                search_query: swap(&mut post.search_query, search_query),
                search_history_json: swap(&mut post.search_history_json, search_history_json),
                last_result_json: swap(&mut post.last_result_json, last_result_json),
                last_analysis_json: swap(&mut post.last_analysis_json, last_analysis_json),
            }),
        })
    }

    fn payload_bytes(&self) -> usize {
        match self {
            Self::SetConfig(SetConfig { search_query, search_history_json, last_result_json, last_analysis_json }) => [search_query, search_history_json, last_result_json, last_analysis_json].into_iter().flatten().map(String::len).sum(),
        }
    }
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
