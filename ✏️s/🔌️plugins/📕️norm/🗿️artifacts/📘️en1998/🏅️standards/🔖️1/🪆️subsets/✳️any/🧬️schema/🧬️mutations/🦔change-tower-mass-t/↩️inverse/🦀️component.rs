//! ↩️ `change-tower-mass-t` inverse — restores the pre-change `tower_mass_t` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_tower_mass_t::mutation::ChangeTowerMassT;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTowerMassT, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTowerMassT(ChangeTowerMassT { new_tower_mass_t: base.tower_mass_t.clone() })]
}
//#endregion 🔖️Inverse
