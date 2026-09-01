//! ↩️ `change-weld-length-mm` inverse — restores the pre-change `weld_length_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_weld_length_mm::ChangeWeldLengthMm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWeldLengthMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeWeldLengthMm(ChangeWeldLengthMm { new_weld_length_mm: base.weld_length_mm.clone() })]
}
//#endregion 🔖️Inverse
