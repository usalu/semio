//! 🔺️ `change-occupants` sparse diff construction — writes only `Din16798Diff.occupants` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_occupants::mutation::ChangeOccupants;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeOccupants, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { occupants: Some(payload.new_occupants.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
