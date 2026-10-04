//! 🧬️ Replace Presence in the shooting.presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: ShootingPresence,
}

impl protocol::MutationKind<ShootingPresence, ShootingPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, _base: &ShootingPresence) -> protocol::MutationOutcome<ShootingPresence> {
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &ShootingPresence) -> Result<Vec<ShootingPresenceMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ShootingPresenceMutation::ReplacePresence(Self { presence: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Presence", "Präsenz ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}
