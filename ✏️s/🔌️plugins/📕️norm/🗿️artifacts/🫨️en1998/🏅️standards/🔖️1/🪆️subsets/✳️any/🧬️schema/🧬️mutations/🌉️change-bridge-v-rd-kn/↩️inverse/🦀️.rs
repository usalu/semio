//! ↩️ `change-bridge-v-rd-kn` inverse — restores the pre-change `bridge_v_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_bridge_v_rd_kn::ChangeBridgeVRdKn;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBridgeVRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeBridgeVRdKn(ChangeBridgeVRdKn { new_bridge_v_rd_kn: base.bridge_v_rd_kn })]
}
//#endregion 🔖️Inverse
