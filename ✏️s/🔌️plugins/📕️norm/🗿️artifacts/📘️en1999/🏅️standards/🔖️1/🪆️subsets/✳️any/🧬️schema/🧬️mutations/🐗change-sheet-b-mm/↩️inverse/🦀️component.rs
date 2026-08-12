//! ↩️ `change-sheet-b-mm` inverse — restores the pre-change `sheet_b_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_sheet_b_mm::mutation::ChangeSheetBMm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSheetBMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSheetBMm(ChangeSheetBMm { new_sheet_b_mm: base.sheet_b_mm.clone() })]
}
//#endregion 🔖️Inverse
