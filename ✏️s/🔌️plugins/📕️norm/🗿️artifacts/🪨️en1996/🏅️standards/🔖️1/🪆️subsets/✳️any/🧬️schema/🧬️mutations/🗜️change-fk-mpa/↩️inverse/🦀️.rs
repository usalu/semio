//! ↩️ `change-fk-mpa` inverse — restores the pre-change `f_k_mpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_f_k_mpa::ChangeFKMpa;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFKMpa, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeFKMpa(ChangeFKMpa { new_f_k_mpa: base.f_k_mpa })]
}
//#endregion 🔖️Inverse
