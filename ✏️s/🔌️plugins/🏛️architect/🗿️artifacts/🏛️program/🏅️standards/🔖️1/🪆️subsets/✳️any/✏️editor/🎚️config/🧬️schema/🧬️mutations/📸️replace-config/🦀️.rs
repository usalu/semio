//! 🧬️ Replace Config in the architect.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: ArchitectConfig,
}

impl protocol::MutationKind<ArchitectConfig, ArchitectConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &ArchitectConfig) -> protocol::MutationOutcome<ArchitectConfig> {
        if &self.config == base {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Requested config already matches.");
        }
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &ArchitectConfig) -> Vec<ArchitectConfigMutation> {
        vec![ArchitectConfigMutation::ReplaceConfig(Self { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Replace Config".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
