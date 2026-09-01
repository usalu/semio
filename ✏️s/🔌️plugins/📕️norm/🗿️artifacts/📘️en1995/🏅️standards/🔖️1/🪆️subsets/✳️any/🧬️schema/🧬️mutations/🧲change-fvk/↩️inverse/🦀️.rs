//! ↩️ `change-f-v-k` inverse — restores the pre-change `f_v_k` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_f_v_k::ChangeFVK;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFVK, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeFVK(ChangeFVK { new_f_v_k: base.f_v_k.clone() })]
}
//#endregion 🔖️Inverse
