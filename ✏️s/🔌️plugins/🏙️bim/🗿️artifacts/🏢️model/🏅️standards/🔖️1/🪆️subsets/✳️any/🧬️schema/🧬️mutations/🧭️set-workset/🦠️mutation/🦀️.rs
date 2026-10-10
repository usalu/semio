//! 🧭️ set-workset authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWorkset {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_visible: Option<bool>,
}
impl SetWorkset {
    pub fn patch(&self) -> WorksetPatch { WorksetPatch { name: self.name.clone(), default_visible: self.default_visible.clone() } }
    pub fn from_patch(id: String, patch: WorksetPatch) -> Self { Self { id, name: patch.name, default_visible: patch.default_visible } }
}
impl MutationKind<ModelSnapshot, ModelMutation> for SetWorkset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "workset", kind: "set-workset", record: "SetWorkset" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set workset", "Ändern") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
