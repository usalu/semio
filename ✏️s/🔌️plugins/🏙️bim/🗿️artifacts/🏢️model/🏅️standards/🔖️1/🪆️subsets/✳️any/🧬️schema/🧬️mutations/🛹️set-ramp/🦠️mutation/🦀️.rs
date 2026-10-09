//! 📈️ `set-ramp` payload. Sets any of a ramp's authored path, width, landing lengths, slope limit, thickness, material, base offset, top constraint, side railings and name; absent fields stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, RampPatch, TopConstraint, Vertex};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRamp {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Vertex>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub landing_start: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub landing_end: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub landing_turn: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_slope: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub railing_left: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub railing_right: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetRamp {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> RampPatch {
        RampPatch {
            path: self.path.clone(),
            width: self.width,
            landing_start: self.landing_start,
            landing_end: self.landing_end,
            landing_turn: self.landing_turn,
            max_slope: self.max_slope,
            thickness: self.thickness,
            material: self.material.clone(),
            base_offset: self.base_offset,
            top: self.top.clone(),
            railing_left: self.railing_left,
            railing_right: self.railing_right,
            name: self.name.clone(),
            ..Default::default()
        }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: RampPatch) -> Self {
        Self {
            id,
            path: patch.path,
            width: patch.width,
            landing_start: patch.landing_start,
            landing_end: patch.landing_end,
            landing_turn: patch.landing_turn,
            max_slope: patch.max_slope,
            thickness: patch.thickness,
            material: patch.material,
            base_offset: patch.base_offset,
            top: patch.top,
            railing_left: patch.railing_left,
            railing_right: patch.railing_right,
            name: patch.name,
        }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetRamp {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "ramp", kind: "set-ramp", record: "SetRamp" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit ramp \"{}\"", self.id), &format!("Rampe \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
