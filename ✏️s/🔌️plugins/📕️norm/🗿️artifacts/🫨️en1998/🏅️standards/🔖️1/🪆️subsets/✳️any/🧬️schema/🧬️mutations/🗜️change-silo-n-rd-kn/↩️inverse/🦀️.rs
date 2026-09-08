//! ↩️ `change-silo-n-rd-kn` inverse — restores the pre-change `silo_n_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_silo_n_rd_kn::ChangeSiloNRdKn;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSiloNRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeSiloNRdKn(ChangeSiloNRdKn { new_silo_n_rd_kn: base.silo_n_rd_kn })]
}
//#endregion 🔖️Inverse
