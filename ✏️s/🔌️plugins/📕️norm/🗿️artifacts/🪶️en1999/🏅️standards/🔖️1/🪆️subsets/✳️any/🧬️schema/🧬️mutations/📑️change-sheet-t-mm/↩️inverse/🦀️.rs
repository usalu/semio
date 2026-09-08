//! ↩️ `change-sheet-t-mm` inverse — restores the pre-change `sheet_t_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_sheet_t_mm::ChangeSheetTMm;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSheetTMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSheetTMm(ChangeSheetTMm { new_sheet_t_mm: base.sheet_t_mm })]
}
//#endregion 🔖️Inverse
