//! 🧬️ Replace Presence in the shooting.presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: ShootingPresence,
}

impl protocol::MutationKind<ShootingPresence, ShootingPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, base: &ShootingPresence) -> protocol::MutationOutcome<ShootingPresenceDiff> {
        let diff = ShootingPresenceDiff {
            selected_shot_ids: (base.selected_shot_ids != self.presence.selected_shot_ids).then(|| self.presence.selected_shot_ids.clone()),
            camera: (base.camera != self.presence.camera).then(|| self.presence.camera.clone()),
        };
        match protocol::DiffAlgebra::<ShootingPresence>::is_empty(&diff) {
            true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Presence is unchanged."),
            false => protocol::MutationOutcome::new(diff),
        }
    }
    fn inverse(&self, base: &ShootingPresence) -> Result<Vec<ShootingPresenceMutation>, semio_framework_value::ValueError> {
    Ok(vec![ShootingPresenceMutation::ReplacePresence(Self { presence: base.clone() })])
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Presence", "Präsenz ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}
