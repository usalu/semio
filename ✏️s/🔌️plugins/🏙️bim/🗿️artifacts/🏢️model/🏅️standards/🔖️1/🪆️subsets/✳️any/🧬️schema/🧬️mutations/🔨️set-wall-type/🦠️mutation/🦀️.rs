//! 🔨️ `set-wall-type` payload. Patches exactly the provided fields of a wall type; the layer stack is one field and replaces the whole stack.

use crate::{Layer, ModelDiff, ModelMutation, ModelSnapshot, WallTypePatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallType {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

impl SetWallType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> WallTypePatch {
        WallTypePatch { name: self.name.clone(), layers: self.layers.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: WallTypePatch) -> Self {
        Self { id, name: patch.name, layers: patch.layers }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetWallType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall-type", kind: "set-wall-type", record: "SetWallType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit wall type \"{}\"", self.id), &format!("Wandtyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
