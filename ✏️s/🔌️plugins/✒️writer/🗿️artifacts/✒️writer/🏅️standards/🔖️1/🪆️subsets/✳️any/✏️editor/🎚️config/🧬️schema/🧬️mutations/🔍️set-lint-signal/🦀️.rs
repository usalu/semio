//! 🧬️ Set Lint Signal in the Writer config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-lint-signal")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLintSignal {
    pub value: u32,
}

impl protocol::MutationKind<WriterConfig, WriterConfigMutation> for SetLintSignal {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "lint-signal", kind: "set-lint-signal", record: "SetLintSignal" };
    fn diff(&self, base: &WriterConfig) -> protocol::MutationOutcome<WriterConfig> {
        let mut next = base.clone();
        next.lint_signal = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WriterConfig) -> Vec<WriterConfigMutation> { vec![WriterConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Lint Signal".into() }
    fn target(&self) -> Vec<String> { vec!["lint_signal".into()] }
}
