//! 🖊️ `set-annotation-style` payload. Sets exactly the provided fields of an annotation style; every dimension, tag, note and leader that uses the style follows by inference.

use crate::standards::v1::subsets::any::schema::{diff::{patches::AnnotationStylePatch, ModelDiff}, snapshot::{DimensionUnit, ModelSnapshot, Terminator}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

impl<Op> MutationKind<ModelSnapshot, Op> for SetAnnotationStyle
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetAnnotationStyle>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "annotation-style", kind: "set-annotation-style", record: "SetAnnotationStyle" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_annotation_style::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_annotation_style::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change annotation style \"{}\"", self.id), &format!("Beschriftungsstil \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
