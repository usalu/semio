//! ↩️ `change-w-mm3` inverse — restores the pre-change `w_mm3` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_w_mm3::ChangeWMm3;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWMm3, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeWMm3(ChangeWMm3 { new_w_mm3: base.w_mm3.clone() })]
}
//#endregion 🔖️Inverse
