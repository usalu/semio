//! ↩️ `change-t1-s` inverse — restores the pre-change `t1_s` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_t1_s::ChangeT1S;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeT1S, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeT1S(ChangeT1S { new_t1_s: base.t1_s })]
}
//#endregion 🔖️Inverse
