//! 🧬️ Set Shot Selection in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-shot-selection")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetShotSelection {
    pub shot_ids: Vec<String>,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetShotSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shot-selection", kind: "set-shot-selection", record: "SetShotSelection" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfig> {
        let mut next = base.clone();
        next.selected_shot_ids = self.shot_ids.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &ShootingConfig) -> Vec<ShootingConfigMutation> { vec![ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Shot Selection".into() }
    fn target(&self) -> Vec<String> { vec!["selected_shot_ids".into()] }
}
