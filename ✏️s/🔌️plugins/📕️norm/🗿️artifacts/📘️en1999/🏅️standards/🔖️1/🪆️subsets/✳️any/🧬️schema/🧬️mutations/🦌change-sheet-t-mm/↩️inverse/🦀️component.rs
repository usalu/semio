//! ↩️ `change-sheet-t-mm` inverse — restores the pre-change `sheet_t_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_sheet_t_mm::mutation::ChangeSheetTMm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSheetTMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSheetTMm(ChangeSheetTMm { new_sheet_t_mm: base.sheet_t_mm.clone() })]
}
//#endregion 🔖️Inverse
