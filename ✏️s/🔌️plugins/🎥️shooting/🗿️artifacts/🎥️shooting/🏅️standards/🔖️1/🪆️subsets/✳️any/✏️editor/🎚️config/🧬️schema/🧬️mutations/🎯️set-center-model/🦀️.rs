//! 🧬️ Set Center Model in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-center-model")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCenterModel {
    pub value: bool,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetCenterModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "center-model", kind: "set-center-model", record: "SetCenterModel" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfigDiff> {
        match base.center_model == self.value {
            true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Center model is unchanged."),
            false => protocol::MutationOutcome::new(ShootingConfigDiff { center_model: Some(self.value), ..Default::default() }),
        }
    }
    fn inverse(&self, base: &ShootingConfig) -> Result<Vec<ShootingConfigMutation>, semio_framework_value::ValueError> {
    Ok(vec![ShootingConfigMutation::SetCenterModel(Self { value: base.center_model })])
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Center Model", "Modellzentrierung setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["center_model".into()]
    }
}
