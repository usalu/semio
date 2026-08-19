//! ↩️ `change-silo-height-m` inverse — restores the pre-change `silo_height_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_silo_height_m::mutation::ChangeSiloHeightM;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSiloHeightM, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeSiloHeightM(ChangeSiloHeightM { new_silo_height_m: base.silo_height_m.clone() })]
}
//#endregion 🔖️Inverse
