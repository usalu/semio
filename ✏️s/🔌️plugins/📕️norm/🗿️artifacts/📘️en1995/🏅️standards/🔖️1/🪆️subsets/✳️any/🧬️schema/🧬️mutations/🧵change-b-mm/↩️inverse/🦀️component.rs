//! ↩️ `change-b-mm` inverse — restores the pre-change `b_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_b_mm::mutation::ChangeBMm;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBMm, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeBMm(ChangeBMm { new_b_mm: base.b_mm.clone() })]
}
//#endregion 🔖️Inverse
