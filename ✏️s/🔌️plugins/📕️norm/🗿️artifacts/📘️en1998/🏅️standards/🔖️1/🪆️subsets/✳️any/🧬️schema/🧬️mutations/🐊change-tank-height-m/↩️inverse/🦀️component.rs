//! ↩️ `change-tank-height-m` inverse — restores the pre-change `tank_height_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_tank_height_m::mutation::ChangeTankHeightM;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeTankHeightM, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTankHeightM(ChangeTankHeightM { new_tank_height_m: base.tank_height_m.clone() })]
}
//#endregion 🔖️Inverse
