//! 🧬️ Set Orientation in the sequence.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-orientation")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetOrientation {
    pub value: String,
}

impl protocol::MutationKind<SequenceConfig, SequenceConfigMutation> for SetOrientation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "orientation", kind: "set-orientation", record: "SetOrientation" };
    fn diff(&self, base: &SequenceConfig) -> protocol::MutationOutcome<SequenceConfig> {
        let mut next = base.clone();
        next.orientation = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &SequenceConfig) -> Vec<SequenceConfigMutation> { vec![SequenceConfigMutation::SetOrientation(Self { value: base.orientation.clone() })] }
    fn label(&self) -> String { "Set Orientation".into() }
    fn target(&self) -> Vec<String> { vec!["orientation".into()] }
}
