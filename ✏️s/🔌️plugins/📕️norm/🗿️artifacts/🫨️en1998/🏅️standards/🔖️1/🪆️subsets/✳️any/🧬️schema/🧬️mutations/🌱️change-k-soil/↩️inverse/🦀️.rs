//! ↩️ `change-k-soil` inverse — restores the pre-change `k_soil` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_k_soil::ChangeKSoil;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeKSoil, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeKSoil(ChangeKSoil { new_k_soil: base.k_soil })]
}
//#endregion 🔖️Inverse
