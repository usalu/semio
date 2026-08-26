//! 🦏 `change-sheet-w-el-mm3` payload — changes the En1999 document's `sheet_w_el_mm3` (sheet elastic section modulus [mm3]).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSheetWElMm3
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSheetWElMm3 {
    pub new_sheet_w_el_mm3: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeSheetWElMm3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sheet-w-el-mm3", kind: "change-sheet-w-el-mm3", record: "ChangedSheetWElMm3" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::artifacts::en1999::mutations::change_sheet_w_el_mm3::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_sheet_w_el_mm3::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change sheet elastic section modulus [mm3] to {}", self.new_sheet_w_el_mm3)
    }
}
//#endregion 🔖️ChangeSheetWElMm3
