//! ↩️ `change-bridge-v-rd-kn` inverse — restores the pre-change `bridge_v_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_bridge_v_rd_kn::ChangeBridgeVRdKn;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBridgeVRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeBridgeVRdKn(ChangeBridgeVRdKn { new_bridge_v_rd_kn: base.bridge_v_rd_kn.clone() })]
}
//#endregion 🔖️Inverse
