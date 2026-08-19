//! ↩️ `change-t-ef-mm` inverse — restores the pre-change `t_ef_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_t_ef_mm::mutation::ChangeTEfMm;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeTEfMm, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeTEfMm(ChangeTEfMm { new_t_ef_mm: base.t_ef_mm.clone() })]
}
//#endregion 🔖️Inverse
