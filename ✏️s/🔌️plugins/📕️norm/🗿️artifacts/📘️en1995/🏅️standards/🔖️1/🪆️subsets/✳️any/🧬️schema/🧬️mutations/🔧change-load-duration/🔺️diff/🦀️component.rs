//! 🔺️ `change-load-duration` sparse diff construction — writes only `En1995Diff.load_duration` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_load_duration::mutation::ChangeLoadDuration;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadDuration, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { load_duration: Some(payload.new_load_duration.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
