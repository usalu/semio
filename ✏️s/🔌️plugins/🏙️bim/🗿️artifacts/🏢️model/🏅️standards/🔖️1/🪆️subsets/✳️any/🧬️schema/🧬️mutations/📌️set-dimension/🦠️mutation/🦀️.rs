//! 📌️ `set-dimension` payload. Sets exactly the provided fields of a dimension: anchors, measuring direction, offset, style, lock (an assigned null removes it) and name; absent fields stay untouched.

use crate::{AnnotationAnchor, Assigned, DimensionPatch, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDimension {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchors: Option<Vec<AnnotationAnchor>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lock: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetDimension {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> DimensionPatch {
        DimensionPatch { anchors: self.anchors.clone(), angle: self.angle, offset: self.offset, style: self.style.clone(), lock: self.lock.clone(), name: self.name.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: DimensionPatch) -> Self {
        Self { id, anchors: patch.anchors, angle: patch.angle, offset: patch.offset, style: patch.style, lock: patch.lock, name: patch.name }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetDimension {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dimension", kind: "set-dimension", record: "SetDimension" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change dimension \"{}\"", self.id), &format!("Bemaßung \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
