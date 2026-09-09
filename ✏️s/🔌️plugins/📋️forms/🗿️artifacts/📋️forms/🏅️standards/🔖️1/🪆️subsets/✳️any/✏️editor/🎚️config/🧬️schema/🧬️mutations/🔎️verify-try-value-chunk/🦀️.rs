//! 🔎️ Verify Try Value Chunk in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "verify-try-value-chunk")]
#[mutation_leaf(contract = ::protocol)]
pub struct VerifyTryValueChunk {
    pub staging_id: String,
    pub content_id: String,
    pub index: u64,
    pub chunk_count: u64,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for VerifyTryValueChunk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "verify", entity: "try-value", kind: "verify-try-value-chunk", record: "VerifyTryValueChunk" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let next = base.clone();
        let expected = base.try_values.content_chunk_by_id(&self.content_id, self.index);
        let staged = try_value_blobs().lock().expect("forms try-value blob lock").get(&self.staging_id).and_then(|blob| blob.chunks.get(&self.index)).cloned();
        if expected.as_deref() != staged.as_deref() || self.index.saturating_add(1) > self.chunk_count {
            return protocol::MutationOutcome::new(next).absorb_messages([FormsStageError::Conflict.message()]);
        }
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, _base: &FormsConfig) -> Vec<FormsConfigMutation> {
        Vec::new()
    }
    fn label(&self) -> String {
        "Verify Try Value Chunk".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["try-value".into()]
    }
}
