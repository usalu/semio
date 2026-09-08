//! 🧹️ Discard Try Values Batch in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "discard-try-values-batch")]
#[mutation_leaf(contract = ::protocol)]
pub struct DiscardTryValuesBatch {
    pub staging_id: String,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for DiscardTryValuesBatch {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "discard", entity: "try-values", kind: "discard-try-values-batch", record: "DiscardTryValuesBatch" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let next = base.clone();
        discard_staged_try_values_batch(&self.staging_id);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, _base: &FormsConfig) -> Vec<FormsConfigMutation> { Vec::new() }
    fn label(&self) -> String { "Discard Try Values Batch".into() }
    fn target(&self) -> Vec<String> { vec!["try-values".into()] }
}
