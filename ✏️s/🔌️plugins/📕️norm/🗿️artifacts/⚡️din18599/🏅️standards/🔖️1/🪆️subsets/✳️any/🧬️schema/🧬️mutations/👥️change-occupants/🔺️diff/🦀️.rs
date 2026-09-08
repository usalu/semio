//! 🔺️ `change-occupants` sparse diff construction — writes only `Din18599Diff.occupants` from the payload.

use crate::diff::Din18599Diff;
use crate::mutations::change_occupants::ChangeOccupants;
use crate::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeOccupants, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if base.occupants == payload.new_occupants {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Occupants already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { occupants: Some(payload.new_occupants), ..Default::default() })
}
//#endregion 🔖️Diff
