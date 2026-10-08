//! 🔆️ `set-curtain-wall` payload. Adjusts the named parameters of a curtain wall (axis, base offset, top, grid spacings, mullion profile, materials, name); every field left out stays as it is.

use crate::{Axis, CurtainWallPatch, ModelDiff, ModelMutation, ModelSnapshot, Profile, TopConstraint};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCurtainWall {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub axis: Option<Axis>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub u_spacing: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub v_spacing: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mullion: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub panel_material: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mullion_material: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetCurtainWall {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> CurtainWallPatch {
        CurtainWallPatch { axis: self.axis.clone(), base_offset: self.base_offset.clone(), top: self.top.clone(), u_spacing: self.u_spacing.clone(), v_spacing: self.v_spacing.clone(), mullion: self.mullion.clone(), panel_material: self.panel_material.clone(), mullion_material: self.mullion_material.clone(), name: self.name.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: CurtainWallPatch) -> Self {
        Self { id, axis: patch.axis, base_offset: patch.base_offset, top: patch.top, u_spacing: patch.u_spacing, v_spacing: patch.v_spacing, mullion: patch.mullion, panel_material: patch.panel_material, mullion_material: patch.mullion_material, name: patch.name }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetCurtainWall {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "curtain-wall", kind: "set-curtain-wall", record: "SetCurtainWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Adjust curtain wall \"{}\"", self.id), &format!("Vorhangfassade \"{}\" anpassen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
