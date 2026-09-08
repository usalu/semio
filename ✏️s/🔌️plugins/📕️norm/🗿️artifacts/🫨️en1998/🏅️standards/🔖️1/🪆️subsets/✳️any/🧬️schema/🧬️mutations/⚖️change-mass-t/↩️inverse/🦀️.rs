//! ↩️ `change-mass-t` inverse — restores the pre-change `mass_t` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_mass_t::ChangeMassT;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMassT, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeMassT(ChangeMassT { new_mass_t: base.mass_t })]
}
//#endregion 🔖️Inverse
