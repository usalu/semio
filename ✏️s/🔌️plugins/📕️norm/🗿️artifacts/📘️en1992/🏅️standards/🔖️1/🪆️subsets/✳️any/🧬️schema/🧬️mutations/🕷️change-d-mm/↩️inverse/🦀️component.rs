//! ↩️ `change-d-mm` inverse — restores the pre-change `d_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_d_mm::mutation::ChangeDMm;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeDMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeDMm(ChangeDMm { new_d_mm: base.d_mm.clone() })]
}
//#endregion 🔖️Inverse
