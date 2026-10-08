//! 🎚️ `set-column-type` payload. Changes exactly the provided fields of a column type; every element of the type follows by inference.

use crate::{ColumnTypePatch, ModelDiff, ModelMutation, ModelSnapshot, Profile};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetColumnType {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

impl SetColumnType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ColumnTypePatch {
        ColumnTypePatch { name: self.name.clone(), profile: self.profile.clone(), material: self.material.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ColumnTypePatch) -> Self {
        Self { id, name: patch.name, profile: patch.profile, material: patch.material }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetColumnType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "column-type", kind: "set-column-type", record: "SetColumnType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change column type \"{}\"", self.id), &format!("Stützentyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
