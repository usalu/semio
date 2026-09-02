//! 🐗 `change-sheet-b-mm` payload — changes the En1999 document's `sheet_b_mm` (sheet width b [mm]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_sheet_b_mm::ChangeSheetBMm;

//#region 🔖️ChangeSheetBMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSheetBMm {
    pub new_sheet_b_mm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeSheetBMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sheet-b-mm", kind: "change-sheet-b-mm", record: "ChangedSheetBMm" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change sheet width b [mm] to {}", self.new_sheet_b_mm)
    }
}
//#endregion 🔖️ChangeSheetBMm
