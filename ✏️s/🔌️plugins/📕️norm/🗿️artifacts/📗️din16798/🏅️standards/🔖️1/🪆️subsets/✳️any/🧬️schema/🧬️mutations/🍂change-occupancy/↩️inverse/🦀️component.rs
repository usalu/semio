//! ↩️ `change-occupancy` inverse — restores the pre-change `occupancy` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_occupancy::mutation::ChangeOccupancy;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeOccupancy, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeOccupancy(ChangeOccupancy { new_occupancy: base.occupancy.clone() })]
}
//#endregion 🔖️Inverse
