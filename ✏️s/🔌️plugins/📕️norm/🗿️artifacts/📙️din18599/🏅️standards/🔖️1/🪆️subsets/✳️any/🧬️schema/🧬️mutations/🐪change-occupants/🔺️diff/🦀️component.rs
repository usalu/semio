//! 🔺️ `change-occupants` sparse diff construction — writes only `Din18599Diff.occupants` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_occupants::mutation::ChangeOccupants;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeOccupants, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { occupants: Some(payload.new_occupants.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
