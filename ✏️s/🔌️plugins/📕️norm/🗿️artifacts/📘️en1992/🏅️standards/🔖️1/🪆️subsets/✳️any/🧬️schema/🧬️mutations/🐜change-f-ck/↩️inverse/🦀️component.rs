//! ↩️ `change-f-ck` inverse — restores the pre-change `f_ck` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_f_ck::mutation::ChangeFCk;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFCk, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeFCk(ChangeFCk { new_f_ck: base.f_ck.clone() })]
}
//#endregion 🔖️Inverse
