//! 🖊️ `set-annotation-style` payload. Sets exactly the provided fields of an annotation style; every dimension, tag, note and leader that uses the style follows by inference.

use crate::{AnnotationStylePatch, DimensionUnit, ModelDiff, ModelMutation, ModelSnapshot, Terminator};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetAnnotationStyle {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text_height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub terminator: Option<Terminator>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<DimensionUnit>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mark_size: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overshoot: Option<f64>,
}

impl SetAnnotationStyle {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> AnnotationStylePatch {
        AnnotationStylePatch { name: self.name.clone(), text_height: self.text_height, terminator: self.terminator, unit: self.unit, precision: self.precision, mark_size: self.mark_size, gap: self.gap, overshoot: self.overshoot }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: AnnotationStylePatch) -> Self {
        Self { id, name: patch.name, text_height: patch.text_height, terminator: patch.terminator, unit: patch.unit, precision: patch.precision, mark_size: patch.mark_size, gap: patch.gap, overshoot: patch.overshoot }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetAnnotationStyle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "annotation-style", kind: "set-annotation-style", record: "SetAnnotationStyle" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change annotation style \"{}\"", self.id), &format!("Beschriftungsstil \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
