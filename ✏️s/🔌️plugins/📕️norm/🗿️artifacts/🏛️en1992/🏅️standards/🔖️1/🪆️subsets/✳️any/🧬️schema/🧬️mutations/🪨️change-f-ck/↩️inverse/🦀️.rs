//! ↩️ `change-f-ck` inverse — restores the pre-change `f_ck` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_f_ck::ChangeFCk;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFCk, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeFCk(ChangeFCk { new_f_ck: base.f_ck })]
}
//#endregion 🔖️Inverse
