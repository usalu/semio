//! 📖️ `create-sheet-revision` payload. Adds a row to the revision table of a sheet: the mark, its date, what changed and who changed it. The table is printed in the corner of the sheet by the inferred sheet layout.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, SheetRevision};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSheetRevision {
    pub id: String,
    pub sheet_revision: SheetRevision,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateSheetRevision {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "sheet-revision", kind: "create-sheet-revision", record: "CreateSheetRevision" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add revision \"{}\" to sheet \"{}\"", self.sheet_revision.number, self.sheet_revision.sheet), &format!("Änderung \"{}\" zu Blatt \"{}\" hinzufügen", self.sheet_revision.number, self.sheet_revision.sheet))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
