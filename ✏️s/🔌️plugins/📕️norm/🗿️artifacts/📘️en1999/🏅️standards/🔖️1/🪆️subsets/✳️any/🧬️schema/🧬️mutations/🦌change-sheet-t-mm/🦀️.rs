//! 🦌 `change-sheet-t-mm` payload — changes the En1999 document's `sheet_t_mm` (sheet thickness t [mm]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_sheet_t_mm::ChangeSheetTMm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSheetTMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSheetTMm {
    pub new_sheet_t_mm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeSheetTMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sheet-t-mm", kind: "change-sheet-t-mm", record: "ChangedSheetTMm" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change sheet thickness t [mm] to {}", self.new_sheet_t_mm)
    }
}
//#endregion 🔖️ChangeSheetTMm
