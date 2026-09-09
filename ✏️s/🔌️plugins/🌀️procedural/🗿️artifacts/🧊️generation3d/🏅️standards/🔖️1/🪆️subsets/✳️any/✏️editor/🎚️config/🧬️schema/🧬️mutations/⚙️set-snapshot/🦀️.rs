//! ⚙️ Replaces the whole editor config in one settled step — the inverse every non-invertible bulk
//! change (example load, config restore) hands back.

use super::{Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "snapshot")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub config: Generation3dConfig,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, _base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }

    fn inverse(&self, base: &Generation3dConfig) -> Vec<Generation3dConfigMutation> {
        vec![Self { config: base.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Snapshot".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
