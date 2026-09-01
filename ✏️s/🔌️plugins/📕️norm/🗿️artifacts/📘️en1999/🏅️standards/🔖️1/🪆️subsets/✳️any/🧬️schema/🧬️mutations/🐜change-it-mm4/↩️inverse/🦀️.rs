//! ↩️ `change-it-mm4` inverse — restores the pre-change `i_t_mm4` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_i_t_mm4::ChangeITMm4;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeITMm4, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeITMm4(ChangeITMm4 { new_i_t_mm4: base.i_t_mm4.clone() })]
}
//#endregion 🔖️Inverse
