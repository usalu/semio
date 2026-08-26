//! ↩️ `change-t1-s` inverse — restores the pre-change `t1_s` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_t1_s::mutation::ChangeT1S;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeT1S, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeT1S(ChangeT1S { new_t1_s: base.t1_s.clone() })]
}
//#endregion 🔖️Inverse
