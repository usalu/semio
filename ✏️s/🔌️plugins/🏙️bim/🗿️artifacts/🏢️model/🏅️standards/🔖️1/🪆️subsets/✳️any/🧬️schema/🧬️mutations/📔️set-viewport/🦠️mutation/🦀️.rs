//! 📔️ `set-viewport` payload. Sparsely changes a viewport: its sheet, its view, the position of its window on the paper, its scale (an assigned null never clears it), its crop (an assigned null clears it) and its label (an assigned null returns to the name of the view).

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::{Assigned, Point2, ViewCrop, ViewportPatch};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetViewport {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop: Option<Assigned<Option<ViewCrop>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Assigned<Option<String>>>,
}

impl SetViewport {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ViewportPatch {
        ViewportPatch { sheet: self.sheet.clone(), view: self.view.clone(), position: self.position, scale: self.scale, crop: self.crop.clone(), label: self.label.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ViewportPatch) -> Self {
        Self { id, sheet: patch.sheet, view: patch.view, position: patch.position, scale: patch.scale, crop: patch.crop, label: patch.label }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetViewport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "viewport", kind: "set-viewport", record: "SetViewport" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change viewport \"{}\"", self.id), &format!("Ansichtsfenster \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
