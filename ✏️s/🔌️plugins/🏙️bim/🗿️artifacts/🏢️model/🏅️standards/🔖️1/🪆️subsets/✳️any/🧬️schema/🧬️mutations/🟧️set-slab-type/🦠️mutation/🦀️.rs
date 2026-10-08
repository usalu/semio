//! 🟧️ `set-slab-type` payload. Patches exactly the provided fields of a slab type; the layer stack is one field and replaces the whole stack.

use crate::{Layer, ModelDiff, ModelMutation, ModelSnapshot, SlabTypePatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSlabType {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

impl SetSlabType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SlabTypePatch {
        SlabTypePatch { name: self.name.clone(), layers: self.layers.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: SlabTypePatch) -> Self {
        Self { id, name: patch.name, layers: patch.layers }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetSlabType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "slab-type", kind: "set-slab-type", record: "SetSlabType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit slab type \"{}\"", self.id), &format!("Deckentyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
