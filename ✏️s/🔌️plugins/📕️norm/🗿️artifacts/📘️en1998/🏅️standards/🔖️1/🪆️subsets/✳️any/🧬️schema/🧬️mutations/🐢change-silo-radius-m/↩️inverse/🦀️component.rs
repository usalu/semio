//! ↩️ `change-silo-radius-m` inverse — restores the pre-change `silo_radius_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_silo_radius_m::mutation::ChangeSiloRadiusM;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSiloRadiusM, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeSiloRadiusM(ChangeSiloRadiusM { new_silo_radius_m: base.silo_radius_m.clone() })]
}
//#endregion 🔖️Inverse
