//! ↩️ `change-silo-v-rd-kn` inverse — restores the pre-change `silo_v_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_silo_v_rd_kn::ChangeSiloVRdKn;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSiloVRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeSiloVRdKn(ChangeSiloVRdKn { new_silo_v_rd_kn: base.silo_v_rd_kn.clone() })]
}
//#endregion 🔖️Inverse
