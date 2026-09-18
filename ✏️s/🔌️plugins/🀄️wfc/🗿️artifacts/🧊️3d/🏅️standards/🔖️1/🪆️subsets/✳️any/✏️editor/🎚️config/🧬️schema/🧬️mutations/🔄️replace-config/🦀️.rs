//! 🔄️ Replace Config in the WFC 3D config facet — the whole-record swap a host restore performs.

use super::{Wfc3dConfig, Wfc3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: Wfc3dConfig,
}

impl protocol::MutationKind<Wfc3dConfig, Wfc3dConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, _base: &Wfc3dConfig) -> protocol::MutationOutcome<Wfc3dConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &Wfc3dConfig) -> Vec<Wfc3dConfigMutation> {
        vec![Wfc3dConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Replace Config".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
