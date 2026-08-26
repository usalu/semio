//! ↩️ `change-l-cr-mm` inverse — restores the pre-change `l_cr_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_l_cr_mm::mutation::ChangeLCrMm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLCrMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeLCrMm(ChangeLCrMm { new_l_cr_mm: base.l_cr_mm.clone() })]
}
//#endregion 🔖️Inverse
