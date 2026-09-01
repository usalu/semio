//! ↩️ `change-tank-radius-m` inverse — restores the pre-change `tank_radius_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_tank_radius_m::ChangeTankRadiusM;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTankRadiusM, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTankRadiusM(ChangeTankRadiusM { new_tank_radius_m: base.tank_radius_m.clone() })]
}
//#endregion 🔖️Inverse
