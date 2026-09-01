//! ↩️ `change-f-c-0-k` inverse — restores the pre-change `f_c_0_k` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_f_c_0_k::ChangeFC0K;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFC0K, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeFC0K(ChangeFC0K { new_f_c_0_k: base.f_c_0_k.clone() })]
}
//#endregion 🔖️Inverse
