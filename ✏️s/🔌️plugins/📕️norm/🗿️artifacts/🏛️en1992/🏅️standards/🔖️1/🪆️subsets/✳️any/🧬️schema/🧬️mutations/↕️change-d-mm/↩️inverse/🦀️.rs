//! ↩️ `change-d-mm` inverse — restores the pre-change `d_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_d_mm::ChangeDMm;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeDMm(ChangeDMm { new_d_mm: base.d_mm })]
}
//#endregion 🔖️Inverse
