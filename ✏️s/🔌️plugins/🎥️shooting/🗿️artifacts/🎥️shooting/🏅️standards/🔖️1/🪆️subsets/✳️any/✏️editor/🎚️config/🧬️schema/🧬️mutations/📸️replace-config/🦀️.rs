//! 🧬️ Replace Config in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: ShootingConfig,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfigDiff> {
        let diff = ShootingConfigDiff {
            default_shot_format: (base.default_shot_format != self.config.default_shot_format).then(|| self.config.default_shot_format.clone()),
            default_shot_shape: (base.default_shot_shape != self.config.default_shot_shape).then(|| self.config.default_shot_shape.clone()),
            default_asset_format: (base.default_asset_format != self.config.default_asset_format).then(|| self.config.default_asset_format.clone()),
            selected_shot_ids: (base.selected_shot_ids != self.config.selected_shot_ids).then(|| self.config.selected_shot_ids.clone()),
            center_model: (base.center_model != self.config.center_model).then_some(self.config.center_model),
            fit_revision: (base.fit_revision != self.config.fit_revision).then_some(self.config.fit_revision),
            camera: (base.camera != self.config.camera).then(|| self.config.camera.clone()),
        };
        match protocol::DiffAlgebra::<ShootingConfig>::is_empty(&diff) {
            true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Config is unchanged."),
            false => protocol::MutationOutcome::new(diff),
        }
    }
    fn inverse(&self, base: &ShootingConfig) -> Result<Vec<ShootingConfigMutation>, semio_framework_value::ValueError> {
    let mut undo = Vec::new();
    if base.selected_shot_ids != self.config.selected_shot_ids {
        undo.push(ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: base.selected_shot_ids.clone() }));
    }
    if base.center_model != self.config.center_model {
        undo.push(ShootingConfigMutation::SetCenterModel(SetCenterModel { value: base.center_model }));
    }
    if base.fit_revision != self.config.fit_revision {
        undo.push(ShootingConfigMutation::SetFitRevision(SetFitRevision { value: base.fit_revision }));
    }
    if base.camera != self.config.camera {
        undo.push(ShootingConfigMutation::SetCamera(SetCamera { camera: base.camera.clone() }));
    }
    if (&base.default_shot_format, &base.default_shot_shape, &base.default_asset_format) != (&self.config.default_shot_format, &self.config.default_shot_shape, &self.config.default_asset_format) {
        undo.push(ShootingConfigMutation::SetDefaults(SetDefaults { shot_format: base.default_shot_format.clone(), shot_shape: base.default_shot_shape.clone(), asset_format: base.default_asset_format.clone() }));
    }
    Ok(undo)
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
