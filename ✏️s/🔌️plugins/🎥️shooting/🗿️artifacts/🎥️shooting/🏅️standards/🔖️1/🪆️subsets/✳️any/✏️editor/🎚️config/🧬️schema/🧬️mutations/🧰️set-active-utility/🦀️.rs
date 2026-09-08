//! 🧬️ Set Active Utility in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-active-utility")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetActiveUtility {
    pub utility_id: String,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetActiveUtility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "active-utility", kind: "set-active-utility", record: "SetActiveUtility" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfig> {
        let mut next = base.clone();
        next.active_utility_id = self.utility_id.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &ShootingConfig) -> Vec<ShootingConfigMutation> { vec![ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Active Utility".into() }
    fn target(&self) -> Vec<String> { vec!["active_utility_id".into()] }
}
