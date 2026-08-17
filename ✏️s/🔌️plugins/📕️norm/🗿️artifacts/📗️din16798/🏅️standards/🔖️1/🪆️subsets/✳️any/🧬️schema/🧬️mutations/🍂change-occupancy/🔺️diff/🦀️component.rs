//! 🔺️ `change-occupancy` sparse diff construction — writes only `Din16798Diff.occupancy` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_occupancy::mutation::ChangeOccupancy;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeOccupancy, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.occupancy == payload.new_occupancy {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Occupancy type is already \"{}\".", payload.new_occupancy));
    }
    protocol::MutationOutcome::new(Din16798Diff { occupancy: Some(payload.new_occupancy.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
