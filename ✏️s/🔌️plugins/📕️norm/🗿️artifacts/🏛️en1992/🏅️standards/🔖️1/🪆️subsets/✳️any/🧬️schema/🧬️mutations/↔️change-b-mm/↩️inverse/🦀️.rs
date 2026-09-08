//! ↩️ `change-b-mm` inverse — restores the pre-change `b_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_b_mm::ChangeBMm;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeBMm(ChangeBMm { new_b_mm: base.b_mm })]
}
//#endregion 🔖️Inverse
