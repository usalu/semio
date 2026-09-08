//! 📸️ Replace Config in the Playbook configuration channel.

use super::{PlaybookConfig, PlaybookConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: PlaybookConfig,
}

impl protocol::MutationKind<PlaybookConfig, PlaybookConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, _base: &PlaybookConfig) -> protocol::MutationOutcome<PlaybookConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &PlaybookConfig) -> Vec<PlaybookConfigMutation> { vec![PlaybookConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Replace Config".into() }
    fn target(&self) -> Vec<String> { vec!["config".into()] }
}
