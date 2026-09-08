//! ↩️ `change-b-mm` inverse — restores the pre-change `b_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_b_mm::ChangeBMm;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBMm, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeBMm(ChangeBMm { new_b_mm: base.b_mm })]
}
//#endregion 🔖️Inverse
