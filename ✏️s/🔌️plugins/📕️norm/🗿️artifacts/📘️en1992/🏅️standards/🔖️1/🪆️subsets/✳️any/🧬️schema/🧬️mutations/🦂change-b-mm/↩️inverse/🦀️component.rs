//! ↩️ `change-b-mm` inverse — restores the pre-change `b_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_b_mm::mutation::ChangeBMm;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeBMm(ChangeBMm { new_b_mm: base.b_mm.clone() })]
}
//#endregion 🔖️Inverse
