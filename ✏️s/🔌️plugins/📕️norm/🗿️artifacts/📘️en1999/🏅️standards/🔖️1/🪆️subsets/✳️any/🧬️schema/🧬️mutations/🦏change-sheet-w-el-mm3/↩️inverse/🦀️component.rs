//! ↩️ `change-sheet-w-el-mm3` inverse — restores the pre-change `sheet_w_el_mm3` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_sheet_w_el_mm3::mutation::ChangeSheetWElMm3;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSheetWElMm3, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSheetWElMm3(ChangeSheetWElMm3 { new_sheet_w_el_mm3: base.sheet_w_el_mm3.clone() })]
}
//#endregion 🔖️Inverse
