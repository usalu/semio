//! 🔺️ `change-fire-duration-min` sparse diff construction — writes only `En1995Diff.fire_duration_min` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_fire_duration_min::mutation::ChangeFireDurationMin;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireDurationMin, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { fire_duration_min: Some(payload.new_fire_duration_min.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
