//! 🧬️ Set Center Model in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-center-model")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCenterModel {
    pub value: bool,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetCenterModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "center-model", kind: "set-center-model", record: "SetCenterModel" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfig> {
        let mut next = base.clone();
        next.center_model = self.value;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &ShootingConfig) -> Vec<ShootingConfigMutation> { vec![ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Center Model".into() }
    fn target(&self) -> Vec<String> { vec!["center_model".into()] }
}
