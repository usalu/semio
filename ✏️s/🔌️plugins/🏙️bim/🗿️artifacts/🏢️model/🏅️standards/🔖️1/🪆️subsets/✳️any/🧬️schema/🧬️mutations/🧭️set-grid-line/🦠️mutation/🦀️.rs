//! 🧭️ `set-grid-line` payload. Patches a grid line (label, start, end): exactly the provided fields change; the building it belongs to never changes here.

use crate::{GridLinePatch, ModelDiff, ModelMutation, ModelSnapshot, Point2};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetGridLine {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<Point2>,
}

impl SetGridLine {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> GridLinePatch {
        GridLinePatch { label: self.label.clone(), start: self.start.clone(), end: self.end.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: GridLinePatch) -> Self {
        Self { id, label: patch.label, start: patch.start, end: patch.end }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetGridLine {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "grid-line", kind: "set-grid-line", record: "SetGridLine" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update grid line \"{}\"", self.id), &format!("Rasterlinie \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
