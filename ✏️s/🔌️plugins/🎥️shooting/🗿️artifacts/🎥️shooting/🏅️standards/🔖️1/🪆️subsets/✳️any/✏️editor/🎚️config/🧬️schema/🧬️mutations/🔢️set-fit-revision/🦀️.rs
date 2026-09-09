//! 🧬️ Set Fit Revision in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-fit-revision")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFitRevision {
    pub value: u32,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetFitRevision {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "fit-revision", kind: "set-fit-revision", record: "SetFitRevision" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfig> {
        let mut next = base.clone();
        next.fit_revision = self.value;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &ShootingConfig) -> Vec<ShootingConfigMutation> {
        vec![ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Set Fit Revision".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["fit_revision".into()]
    }
}
