//! 📗️ `set-sheet-revision` payload. Sparsely changes a revision row: the mark, the date, the description and the author. The sheet of a row never changes.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::SheetRevisionPatch;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSheetRevision {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

impl SetSheetRevision {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SheetRevisionPatch {
        SheetRevisionPatch { number: self.number.clone(), date: self.date.clone(), description: self.description.clone(), author: self.author.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: SheetRevisionPatch) -> Self {
        Self { id, number: patch.number, date: patch.date, description: patch.description, author: patch.author }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetSheetRevision {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "sheet-revision", kind: "set-sheet-revision", record: "SetSheetRevision" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change revision \"{}\"", self.id), &format!("Änderung \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
