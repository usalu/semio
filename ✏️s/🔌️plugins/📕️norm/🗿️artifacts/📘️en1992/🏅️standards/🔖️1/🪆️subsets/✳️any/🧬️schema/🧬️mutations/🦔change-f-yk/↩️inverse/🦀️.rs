//! ↩️ `change-f-yk` inverse — restores the pre-change `f_yk` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_f_yk::ChangeFYk;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFYk, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeFYk(ChangeFYk { new_f_yk: base.f_yk.clone() })]
}
//#endregion 🔖️Inverse
