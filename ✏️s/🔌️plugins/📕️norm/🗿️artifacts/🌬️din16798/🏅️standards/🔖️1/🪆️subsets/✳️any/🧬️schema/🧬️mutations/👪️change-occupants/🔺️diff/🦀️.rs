//! 🔺️ `change-occupants` sparse diff construction — writes only `Din16798Diff.occupants` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_occupants::ChangeOccupants;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeOccupants, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.occupants == payload.new_occupants {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Number of occupants is already {}.", payload.new_occupants));
    }
    protocol::MutationOutcome::new(Din16798Diff { occupants: Some(payload.new_occupants), ..Default::default() })
}
//#endregion 🔖️Diff
