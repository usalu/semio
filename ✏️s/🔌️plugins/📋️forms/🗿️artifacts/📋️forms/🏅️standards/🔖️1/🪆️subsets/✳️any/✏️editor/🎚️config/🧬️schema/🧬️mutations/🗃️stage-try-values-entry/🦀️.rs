//! 🗃️ Stage Try Values Entry in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "stage-try-values-entry")]
#[mutation_leaf(contract = ::protocol)]
pub struct StageTryValuesEntry {
    pub staging_id: String,
    pub key: String,
    pub value_staging_id: String,
    pub content_id: String,
    pub chunk_count: u64,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for StageTryValuesEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "stage", entity: "try-values", kind: "stage-try-values-entry", record: "StageTryValuesEntry" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let next = base.clone();
        if let Err(error) = stage_try_values_batch_entry(&self.staging_id, &self.key, &self.value_staging_id, &self.content_id, self.chunk_count, &base.try_values) { return protocol::MutationOutcome::new(next).absorb_messages([error.message()]); }
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, _base: &FormsConfig) -> Vec<FormsConfigMutation> { vec![FormsConfigMutation::DiscardTryValuesBatch(DiscardTryValuesBatch { staging_id: self.staging_id.clone() })] }
    fn label(&self) -> String { "Stage Try Values Entry".into() }
    fn target(&self) -> Vec<String> { vec!["try-values".into()] }
}
