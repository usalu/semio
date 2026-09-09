//! ✅️ Commit Try Values Batch in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "commit-try-values-batch")]
#[mutation_leaf(contract = ::protocol)]
pub struct CommitTryValuesBatch {
    pub staging_id: String,
    pub entry_count: u64,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for CommitTryValuesBatch {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "commit", entity: "try-values", kind: "commit-try-values-batch", record: "CommitTryValuesBatch" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let mut next = base.clone();
        let Some(values) = commit_staged_try_values_batch(&self.staging_id, self.entry_count) else {
            return protocol::MutationOutcome::new(next).absorb_messages([FormsStageError::Missing.message()]);
        };
        next.try_values = values;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &FormsConfig) -> Vec<FormsConfigMutation> {
        vec![FormsConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Commit Try Values Batch".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["try-values".into()]
    }
}
