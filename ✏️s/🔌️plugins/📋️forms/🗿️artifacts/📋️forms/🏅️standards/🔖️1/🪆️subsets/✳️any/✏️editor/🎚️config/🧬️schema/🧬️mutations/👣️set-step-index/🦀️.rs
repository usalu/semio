//! 👣️ Set Step Index in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-step-index")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStepIndex {
    pub index: u32,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for SetStepIndex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "step-index", kind: "set-step-index", record: "SetStepIndex" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let mut next = base.clone();
        next.current_step_index = self.index;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &FormsConfig) -> Vec<FormsConfigMutation> { vec![FormsConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Step Index".into() }
    fn target(&self) -> Vec<String> { vec!["step-index".into()] }
}
