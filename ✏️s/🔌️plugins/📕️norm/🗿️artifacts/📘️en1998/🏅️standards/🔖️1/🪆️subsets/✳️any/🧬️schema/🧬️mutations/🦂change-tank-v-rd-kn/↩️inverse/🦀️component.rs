//! ↩️ `change-tank-v-rd-kn` inverse — restores the pre-change `tank_v_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_tank_v_rd_kn::mutation::ChangeTankVRdKn;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeTankVRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTankVRdKn(ChangeTankVRdKn { new_tank_v_rd_kn: base.tank_v_rd_kn.clone() })]
}
//#endregion 🔖️Inverse
