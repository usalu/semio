//! 🔺️ `change-occupants` sparse diff construction — writes only `Din16798Diff.occupants` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_occupants::ChangeOccupants;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeOccupants, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.occupants == payload.new_occupants {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Number of occupants is already {}.", payload.new_occupants));
    }
    protocol::MutationOutcome::new(Din16798Diff { occupants: Some(payload.new_occupants.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
