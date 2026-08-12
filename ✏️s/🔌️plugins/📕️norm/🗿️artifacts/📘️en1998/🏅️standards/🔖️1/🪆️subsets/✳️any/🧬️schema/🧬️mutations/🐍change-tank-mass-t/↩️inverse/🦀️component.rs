//! ↩️ `change-tank-mass-t` inverse — restores the pre-change `tank_mass_t` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_tank_mass_t::mutation::ChangeTankMassT;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTankMassT, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTankMassT(ChangeTankMassT { new_tank_mass_t: base.tank_mass_t.clone() })]
}
//#endregion 🔖️Inverse
