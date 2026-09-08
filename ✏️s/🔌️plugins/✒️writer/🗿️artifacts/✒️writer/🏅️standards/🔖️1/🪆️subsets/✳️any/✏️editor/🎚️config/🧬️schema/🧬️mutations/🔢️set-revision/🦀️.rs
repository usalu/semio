//! 🧬️ Set Revision in the Writer config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-revision")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRevision {
    pub value: u32,
}

impl protocol::MutationKind<WriterConfig, WriterConfigMutation> for SetRevision {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "revision", kind: "set-revision", record: "SetRevision" };
    fn diff(&self, base: &WriterConfig) -> protocol::MutationOutcome<WriterConfig> {
        let mut next = base.clone();
        next.revision = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WriterConfig) -> Vec<WriterConfigMutation> { vec![WriterConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Revision".into() }
    fn target(&self) -> Vec<String> { vec!["revision".into()] }
}
