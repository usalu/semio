//! 🎯️ Commit Try Value in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "commit-try-value")]
#[mutation_leaf(contract = ::protocol)]
pub struct CommitTryValue {
    pub key: String,
    pub staging_id: String,
    pub content_id: String,
    pub chunk_count: u64,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for CommitTryValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "commit", entity: "try-value", kind: "commit-try-value", record: "CommitTryValue" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let mut next = base.clone();
        if base.try_values.get_json(&self.key).is_none() && base.try_values.len() >= MAX_STAGED_TRY_VALUE_BLOBS {
            discard_staged_try_value(&self.staging_id);
            return protocol::MutationOutcome::new(next).absorb_messages([FormsStageError::Busy.message()]);
        }
        let chunks = match commit_staged_try_value(&self.staging_id, &self.content_id, self.chunk_count) {
            Ok(chunks) => chunks,
            Err(error) => return protocol::MutationOutcome::new(next).absorb_messages([error.message()]),
        };
        next.try_values = next.try_values.with_chunks(&self.key, self.content_id.clone(), chunks);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &FormsConfig) -> Vec<FormsConfigMutation> { vec![FormsConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Commit Try Value".into() }
    fn target(&self) -> Vec<String> { vec!["try-value".into()] }
}
