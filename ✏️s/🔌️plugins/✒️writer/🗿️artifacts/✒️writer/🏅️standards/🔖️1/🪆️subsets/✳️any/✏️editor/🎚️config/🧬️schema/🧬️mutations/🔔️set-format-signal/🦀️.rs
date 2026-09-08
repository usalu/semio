//! 🧬️ Set Format Signal in the Writer config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-format-signal")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFormatSignal {
    pub value: u32,
}

impl protocol::MutationKind<WriterConfig, WriterConfigMutation> for SetFormatSignal {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "format-signal", kind: "set-format-signal", record: "SetFormatSignal" };
    fn diff(&self, base: &WriterConfig) -> protocol::MutationOutcome<WriterConfig> {
        let mut next = base.clone();
        next.format_signal = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WriterConfig) -> Vec<WriterConfigMutation> { vec![WriterConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Format Signal".into() }
    fn target(&self) -> Vec<String> { vec!["format_signal".into()] }
}
