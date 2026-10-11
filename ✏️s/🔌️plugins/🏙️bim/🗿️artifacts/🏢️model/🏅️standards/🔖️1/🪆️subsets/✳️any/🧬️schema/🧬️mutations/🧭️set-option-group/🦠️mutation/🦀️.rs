//! 🧭️ set-option-group authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetOptionGroup {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SetOptionGroup {
    pub fn patch(&self) -> OptionGroupPatch { OptionGroupPatch { name: self.name.clone() } }
    pub fn from_patch(id: String, patch: OptionGroupPatch) -> Self { Self { id, name: patch.name } }
}
impl MutationKind<ModelSnapshot, ModelMutation> for SetOptionGroup {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "option-group", kind: "set-option-group", record: "SetOptionGroup" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set option group", "Ändern") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
