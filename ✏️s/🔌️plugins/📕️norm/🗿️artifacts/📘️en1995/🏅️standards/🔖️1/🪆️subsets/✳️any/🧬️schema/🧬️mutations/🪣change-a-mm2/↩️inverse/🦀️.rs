//! ↩️ `change-a-mm2` inverse — restores the pre-change `a_mm2` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_a_mm2::ChangeAMm2;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAMm2, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeAMm2(ChangeAMm2 { new_a_mm2: base.a_mm2.clone() })]
}
//#endregion 🔖️Inverse
