//! 🔺️ `change-drift-mm` sparse diff construction — writes only `En1998Diff.drift_mm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_drift_mm::mutation::ChangeDriftMm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDriftMm, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { drift_mm: Some(payload.new_drift_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
