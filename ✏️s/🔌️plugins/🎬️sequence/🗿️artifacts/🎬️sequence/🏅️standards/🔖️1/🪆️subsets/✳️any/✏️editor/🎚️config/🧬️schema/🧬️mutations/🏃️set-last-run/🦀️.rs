//! 🧬️ Set Last Run in the sequence.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-last-run")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLastRun {
    pub json: String,
}

impl protocol::MutationKind<SequenceConfig, SequenceConfigMutation> for SetLastRun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "last-run", kind: "set-last-run", record: "SetLastRun" };
    fn diff(&self, base: &SequenceConfig) -> protocol::MutationOutcome<SequenceConfig> {
        let mut next = base.clone();
        next.last_run_json = self.json.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &SequenceConfig) -> Vec<SequenceConfigMutation> {
        vec![SequenceConfigMutation::SetLastRun(Self { json: base.last_run_json.clone() })]
    }
    fn label(&self) -> String {
        "Set Last Run".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["last_run_json".into()]
    }
}
