//! ↩️ `change-mu` inverse — restores the pre-change `mu` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_mu::mutation::ChangeMu;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeMu, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeMu(ChangeMu { new_mu: base.mu.clone() })]
}
//#endregion 🔖️Inverse
