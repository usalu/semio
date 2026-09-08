//! 🧬️ Replace Config in the Writer config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: WriterConfig,
}

impl protocol::MutationKind<WriterConfig, WriterConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplacedConfig" };
    fn diff(&self, _base: &WriterConfig) -> protocol::MutationOutcome<WriterConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &WriterConfig) -> Vec<WriterConfigMutation> { vec![WriterConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Replace Config".into() }
    fn target(&self) -> Vec<String> { vec!["config".into()] }
}
