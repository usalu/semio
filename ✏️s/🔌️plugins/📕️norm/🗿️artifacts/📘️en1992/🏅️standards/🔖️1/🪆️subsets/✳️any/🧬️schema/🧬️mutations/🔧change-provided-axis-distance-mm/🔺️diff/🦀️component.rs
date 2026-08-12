//! 🔺️ `change-provided-axis-distance-mm` sparse diff construction — writes only `En1992Diff.provided_axis_distance_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_provided_axis_distance_mm::mutation::ChangeProvidedAxisDistanceMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeProvidedAxisDistanceMm, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { provided_axis_distance_mm: Some(payload.new_provided_axis_distance_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
