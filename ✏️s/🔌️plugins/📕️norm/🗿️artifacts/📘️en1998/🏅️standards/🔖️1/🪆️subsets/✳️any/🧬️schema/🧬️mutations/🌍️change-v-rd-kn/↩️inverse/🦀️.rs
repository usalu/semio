//! ↩️ `change-v-rd-kn` inverse — restores the pre-change `v_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_v_rd_kn::ChangeVRdKn;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeVRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeVRdKn(ChangeVRdKn { new_v_rd_kn: base.v_rd_kn.clone() })]
}
//#endregion 🔖️Inverse
