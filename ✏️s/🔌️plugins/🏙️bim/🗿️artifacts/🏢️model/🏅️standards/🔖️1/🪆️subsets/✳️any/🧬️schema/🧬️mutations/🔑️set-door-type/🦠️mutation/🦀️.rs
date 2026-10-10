//! 🔑️ `set-door-type` payload. Changes exactly the provided fields of a door type; every element of the type follows by inference.

use crate::{DoorLeaves, DoorTypePatch, ModelDiff, ModelMutation, ModelSnapshot, Swing};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDoorType {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frame_depth: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub leaves: Option<DoorLeaves>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub swing: Option<Swing>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

impl SetDoorType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> DoorTypePatch {
        DoorTypePatch { name: self.name.clone(), width: self.width.clone(), height: self.height.clone(), frame_width: self.frame_width.clone(), frame_depth: self.frame_depth.clone(), leaves: self.leaves.clone(), swing: self.swing.clone(), material: self.material.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: DoorTypePatch) -> Self {
        Self { id, name: patch.name, width: patch.width, height: patch.height, frame_width: patch.frame_width, frame_depth: patch.frame_depth, leaves: patch.leaves, swing: patch.swing, material: patch.material }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetDoorType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "door-type", kind: "set-door-type", record: "SetDoorType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change door type \"{}\"", self.id), &format!("Türtyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
