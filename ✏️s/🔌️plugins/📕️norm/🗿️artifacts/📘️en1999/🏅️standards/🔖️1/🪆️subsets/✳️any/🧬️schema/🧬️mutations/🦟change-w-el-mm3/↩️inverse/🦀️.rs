//! ↩️ `change-w-el-mm3` inverse — restores the pre-change `w_el_mm3` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_w_el_mm3::ChangeWElMm3;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWElMm3, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeWElMm3(ChangeWElMm3 { new_w_el_mm3: base.w_el_mm3.clone() })]
}
//#endregion 🔖️Inverse
