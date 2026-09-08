//! ↩️ `change-sheet-b-mm` inverse — restores the pre-change `sheet_b_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_sheet_b_mm::ChangeSheetBMm;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSheetBMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSheetBMm(ChangeSheetBMm { new_sheet_b_mm: base.sheet_b_mm })]
}
//#endregion 🔖️Inverse
