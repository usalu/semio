//! ↩️ `change-occupants` inverse — restores the pre-change `occupants` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_occupants::ChangeOccupants;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeOccupants, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeOccupants(ChangeOccupants { new_occupants: base.occupants })]
}
//#endregion 🔖️Inverse
