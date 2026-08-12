//! 🔺️ `change-occupancy` sparse diff construction — writes only `Din16798Diff.occupancy` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_occupancy::mutation::ChangeOccupancy;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeOccupancy, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { occupancy: Some(payload.new_occupancy.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
