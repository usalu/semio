//! ↩️ `change-silo-q-nominal` inverse — restores the pre-change `silo_q_nominal` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_silo_q_nominal::ChangeSiloQNominal;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSiloQNominal, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeSiloQNominal(ChangeSiloQNominal { new_silo_q_nominal: base.silo_q_nominal })]
}
//#endregion 🔖️Inverse
