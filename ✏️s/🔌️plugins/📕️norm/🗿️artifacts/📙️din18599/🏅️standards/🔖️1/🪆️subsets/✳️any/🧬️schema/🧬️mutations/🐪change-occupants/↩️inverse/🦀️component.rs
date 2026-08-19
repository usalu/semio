//! ↩️ `change-occupants` inverse — restores the pre-change `occupants` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_occupants::mutation::ChangeOccupants;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeOccupants, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeOccupants(ChangeOccupants { new_occupants: base.occupants.clone() })]
}
//#endregion 🔖️Inverse
