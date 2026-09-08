//! 🔺️ `change-occupancy` sparse diff construction — writes only `Din16798Diff.occupancy` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_occupancy::ChangeOccupancy;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeOccupancy, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.occupancy == payload.new_occupancy {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Occupancy type is already \"{}\".", payload.new_occupancy));
    }
    protocol::MutationOutcome::new(Din16798Diff { occupancy: Some(payload.new_occupancy.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
