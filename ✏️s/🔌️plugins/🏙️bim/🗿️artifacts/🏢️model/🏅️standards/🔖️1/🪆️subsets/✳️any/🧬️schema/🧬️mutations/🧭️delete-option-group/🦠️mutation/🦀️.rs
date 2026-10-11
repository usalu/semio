//! 🧭️ delete-option-group authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteOptionGroup {
    pub id: String,
}
impl MutationKind<ModelSnapshot, ModelMutation> for DeleteOptionGroup {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "option-group", kind: "delete-option-group", record: "DeleteOptionGroup" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Delete option group", "Löschen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
