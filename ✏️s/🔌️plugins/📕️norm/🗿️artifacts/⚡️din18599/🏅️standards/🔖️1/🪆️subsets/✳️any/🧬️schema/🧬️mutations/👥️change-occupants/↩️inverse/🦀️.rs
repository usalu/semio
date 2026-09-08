//! ↩️ `change-occupants` inverse — restores the pre-change `occupants` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_occupants::ChangeOccupants;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeOccupants, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeOccupants(ChangeOccupants { new_occupants: base.occupants })]
}
//#endregion 🔖️Inverse
