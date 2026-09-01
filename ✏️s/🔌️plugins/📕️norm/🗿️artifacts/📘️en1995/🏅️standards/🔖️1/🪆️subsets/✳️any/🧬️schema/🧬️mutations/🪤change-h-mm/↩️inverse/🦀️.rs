//! ↩️ `change-h-mm` inverse — restores the pre-change `h_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_h_mm::ChangeHMm;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHMm, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeHMm(ChangeHMm { new_h_mm: base.h_mm.clone() })]
}
//#endregion 🔖️Inverse
