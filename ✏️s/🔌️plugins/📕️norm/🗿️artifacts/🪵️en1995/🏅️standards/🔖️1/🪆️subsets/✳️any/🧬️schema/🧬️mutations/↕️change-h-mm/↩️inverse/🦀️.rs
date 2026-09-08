//! ↩️ `change-h-mm` inverse — restores the pre-change `h_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_h_mm::ChangeHMm;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHMm, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeHMm(ChangeHMm { new_h_mm: base.h_mm })]
}
//#endregion 🔖️Inverse
