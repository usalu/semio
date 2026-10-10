//! 🧭️ set-design-option authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDesignOption {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
impl SetDesignOption {
    pub fn patch(&self) -> DesignOptionPatch { DesignOptionPatch { group: self.group.clone(), name: self.name.clone(), primary: self.primary.clone() } }
    pub fn from_patch(id: String, patch: DesignOptionPatch) -> Self { Self { id, group: patch.group, name: patch.name, primary: patch.primary } }
}
impl MutationKind<ModelSnapshot, ModelMutation> for SetDesignOption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "design-option", kind: "set-design-option", record: "SetDesignOption" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set design option", "Ändern") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
