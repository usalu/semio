//! ↩️ `change-fk-mpa` inverse — restores the pre-change `f_k_mpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_f_k_mpa::mutation::ChangeFKMpa;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFKMpa, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeFKMpa(ChangeFKMpa { new_f_k_mpa: base.f_k_mpa.clone() })]
}
//#endregion 🔖️Inverse
