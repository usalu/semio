//! 🧬️ Set Defaults in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-defaults")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDefaults {
    pub shot_format: String,
    pub shot_shape: String,
    pub asset_format: String,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetDefaults {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "defaults", kind: "set-defaults", record: "SetDefaults" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfig> {
        let mut next = base.clone();
        next.default_shot_format = self.shot_format.clone();
        next.default_shot_shape = self.shot_shape.clone();
        next.default_asset_format = self.asset_format.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &ShootingConfig) -> Vec<ShootingConfigMutation> {
        vec![ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Set Defaults".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["default_shot_format".into(), "default_shot_shape".into(), "default_asset_format".into()]
    }
}
