//! 🔺️ `change-provided-axis-distance-mm` sparse diff construction — writes only `En1992Diff.provided_axis_distance_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_provided_axis_distance_mm::mutation::ChangeProvidedAxisDistanceMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeProvidedAxisDistanceMm, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_provided_axis_distance_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Provided axis distance mm must be a finite number.", Vec::<String>::new());
    }
    if base.provided_axis_distance_mm == payload.new_provided_axis_distance_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Provided axis distance mm already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { provided_axis_distance_mm: Some(payload.new_provided_axis_distance_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
