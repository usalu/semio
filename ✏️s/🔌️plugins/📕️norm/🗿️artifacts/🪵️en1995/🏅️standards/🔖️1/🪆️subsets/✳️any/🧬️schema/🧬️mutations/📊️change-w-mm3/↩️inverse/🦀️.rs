//! ↩️ `change-w-mm3` inverse — restores the pre-change `w_mm3` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_w_mm3::ChangeWMm3;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWMm3, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeWMm3(ChangeWMm3 { new_w_mm3: base.w_mm3 })]
}
//#endregion 🔖️Inverse
