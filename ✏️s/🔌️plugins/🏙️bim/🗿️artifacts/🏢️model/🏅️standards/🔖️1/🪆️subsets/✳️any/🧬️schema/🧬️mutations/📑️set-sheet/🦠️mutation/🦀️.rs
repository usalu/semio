//! 📑️ `set-sheet` payload. Sparsely changes a sheet: number, name, paper, orientation and the authored fields of the title block. The title block, the frame and the revision table are inferred and follow.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::{Orientation, Paper, SheetPatch};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSheet {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub paper: Option<Paper>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub drawn_by: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub checked_by: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale_label: Option<String>,
}

impl SetSheet {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SheetPatch {
        SheetPatch { number: self.number.clone(), name: self.name.clone(), paper: self.paper.clone(), orientation: self.orientation, project: self.project.clone(), drawn_by: self.drawn_by.clone(), checked_by: self.checked_by.clone(), date: self.date.clone(), revision: self.revision.clone(), scale_label: self.scale_label.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: SheetPatch) -> Self {
        Self { id, number: patch.number, name: patch.name, paper: patch.paper, orientation: patch.orientation, project: patch.project, drawn_by: patch.drawn_by, checked_by: patch.checked_by, date: patch.date, revision: patch.revision, scale_label: patch.scale_label }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetSheet {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "sheet", kind: "set-sheet", record: "SetSheet" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change sheet \"{}\"", self.id), &format!("Blatt \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
