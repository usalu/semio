//! 🦏 `change-sheet-w-el-mm3` payload — changes the En1999 document's `sheet_w_el_mm3` (sheet elastic section modulus [mm3]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
//#region 🔖️ChangeSheetWElMm3
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSheetWElMm3 {
    pub new_sheet_w_el_mm3: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeSheetWElMm3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sheet-w-el-mm3", kind: "change-sheet-w-el-mm3", record: "ChangedSheetWElMm3" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change sheet elastic section modulus [mm3] to {}", self.new_sheet_w_el_mm3)
    }
}
//#endregion 🔖️ChangeSheetWElMm3
