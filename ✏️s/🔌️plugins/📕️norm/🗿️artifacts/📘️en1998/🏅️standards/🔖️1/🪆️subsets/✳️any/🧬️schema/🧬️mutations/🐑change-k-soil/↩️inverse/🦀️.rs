//! ↩️ `change-k-soil` inverse — restores the pre-change `k_soil` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_k_soil::ChangeKSoil;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeKSoil, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeKSoil(ChangeKSoil { new_k_soil: base.k_soil.clone() })]
}
//#endregion 🔖️Inverse
