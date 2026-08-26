//! ↩️ `change-h-ef-mm` inverse — restores the pre-change `h_ef_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_h_ef_mm::mutation::ChangeHEfMm;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHEfMm, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeHEfMm(ChangeHEfMm { new_h_ef_mm: base.h_ef_mm.clone() })]
}
//#endregion 🔖️Inverse
