//! 🧬️ Set Defaults in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfigDiff> {
        let diff = ShootingConfigDiff {
            default_shot_format: (base.default_shot_format != self.shot_format).then(|| self.shot_format.clone()),
            default_shot_shape: (base.default_shot_shape != self.shot_shape).then(|| self.shot_shape.clone()),
            default_asset_format: (base.default_asset_format != self.asset_format).then(|| self.asset_format.clone()),
            ..Default::default()
        };
        match protocol::DiffAlgebra::<ShootingConfig>::is_empty(&diff) {
            true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Defaults are unchanged."),
            false => protocol::MutationOutcome::new(diff),
        }
    }
    fn inverse(&self, base: &ShootingConfig) -> Result<Vec<ShootingConfigMutation>, semio_framework_value::ValueError> {
    Ok(vec![ShootingConfigMutation::SetDefaults(Self { shot_format: base.default_shot_format.clone(), shot_shape: base.default_shot_shape.clone(), asset_format: base.default_asset_format.clone() })])
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Defaults", "Standardwerte setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["default_shot_format".into(), "default_shot_shape".into(), "default_asset_format".into()]
    }
}
