//! 🧽️ Clear Try Values in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "clear-try-values")]
#[mutation_leaf(contract = ::protocol)]
pub struct ClearTryValues {

}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for ClearTryValues {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "clear", entity: "try-values", kind: "clear-try-values", record: "ClearTryValues" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let mut next = base.clone();
        next.try_values = FormsTryValues::default();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &FormsConfig) -> Vec<FormsConfigMutation> { vec![FormsConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Clear Try Values".into() }
    fn target(&self) -> Vec<String> { vec!["try-values".into()] }
}
