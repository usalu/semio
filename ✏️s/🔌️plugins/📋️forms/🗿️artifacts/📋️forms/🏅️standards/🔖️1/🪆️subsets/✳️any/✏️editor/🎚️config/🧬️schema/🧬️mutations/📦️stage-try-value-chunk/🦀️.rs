//! 📦️ Stage Try Value Chunk in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "stage-try-value-chunk")]
#[mutation_leaf(contract = ::protocol)]
pub struct StageTryValueChunk {
    pub staging_id: String,
    pub index: u64,
    pub chunk: String,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for StageTryValueChunk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "stage", entity: "try-value", kind: "stage-try-value-chunk", record: "StageTryValueChunk" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let next = base.clone();
        if let Err(error) = stage_try_value_chunk(&self.staging_id, self.index, &self.chunk) { return protocol::MutationOutcome::new(next).absorb_messages([error.message()]); }
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, _base: &FormsConfig) -> Vec<FormsConfigMutation> { vec![FormsConfigMutation::DiscardTryValueStaging(DiscardTryValueStaging { staging_id: self.staging_id.clone() })] }
    fn label(&self) -> String { "Stage Try Value Chunk".into() }
    fn target(&self) -> Vec<String> { vec!["try-value".into()] }
}
