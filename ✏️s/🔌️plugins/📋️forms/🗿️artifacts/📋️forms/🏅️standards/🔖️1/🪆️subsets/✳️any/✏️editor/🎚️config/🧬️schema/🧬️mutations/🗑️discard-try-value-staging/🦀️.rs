//! 🗑️ Discard Try Value Staging in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "discard-try-value-staging")]
#[mutation_leaf(contract = ::protocol)]
pub struct DiscardTryValueStaging {
    pub staging_id: String,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for DiscardTryValueStaging {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "discard", entity: "try-value", kind: "discard-try-value-staging", record: "DiscardTryValueStaging" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let next = base.clone();
        discard_staged_try_value(&self.staging_id);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, _base: &FormsConfig) -> Vec<FormsConfigMutation> {
        Vec::new()
    }
    fn label(&self) -> String {
        "Discard Try Value Staging".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["try-value".into()]
    }
}
