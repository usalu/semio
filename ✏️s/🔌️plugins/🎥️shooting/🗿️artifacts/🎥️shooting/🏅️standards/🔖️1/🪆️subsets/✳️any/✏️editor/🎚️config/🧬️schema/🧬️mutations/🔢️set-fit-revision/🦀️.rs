//! 🧬️ Set Fit Revision in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-fit-revision")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFitRevision {
    pub value: u32,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetFitRevision {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "fit-revision", kind: "set-fit-revision", record: "SetFitRevision" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfigDiff> {
        match base.fit_revision == self.value {
            true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Fit revision is unchanged."),
            false => protocol::MutationOutcome::new(ShootingConfigDiff { fit_revision: Some(self.value), ..Default::default() }),
        }
    }
    fn inverse(&self, base: &ShootingConfig) -> Result<Vec<ShootingConfigMutation>, semio_framework_value::ValueError> {
    Ok(vec![ShootingConfigMutation::SetFitRevision(Self { value: base.fit_revision })])
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Fit Revision", "Einpassungsrevision setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["fit_revision".into()]
    }
}
