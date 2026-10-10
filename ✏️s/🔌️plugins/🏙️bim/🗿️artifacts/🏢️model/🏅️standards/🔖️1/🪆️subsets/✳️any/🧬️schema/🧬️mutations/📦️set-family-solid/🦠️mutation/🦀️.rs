//! 📦️ `set-family-solid` payload. Sets exactly the provided fields of a family solid: name, shape, material, visibility and offset; absent fields stay untouched.

use crate::{ExprPoint3, FamilySolidPatch, ModelDiff, ModelMutation, ModelSnapshot, SolidShape};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFamilySolid {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<SolidShape>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<ExprPoint3>,
}

impl SetFamilySolid {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> FamilySolidPatch {
        FamilySolidPatch { name: self.name.clone(), shape: self.shape.clone(), material: self.material.clone(), visible: self.visible.clone(), offset: self.offset.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: FamilySolidPatch) -> Self {
        Self { id, name: patch.name, shape: patch.shape, material: patch.material, visible: patch.visible, offset: patch.offset }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetFamilySolid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "family-solid", kind: "set-family-solid", record: "SetFamilySolid" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change family solid \"{}\"", self.id), &format!("Familienkörper \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
