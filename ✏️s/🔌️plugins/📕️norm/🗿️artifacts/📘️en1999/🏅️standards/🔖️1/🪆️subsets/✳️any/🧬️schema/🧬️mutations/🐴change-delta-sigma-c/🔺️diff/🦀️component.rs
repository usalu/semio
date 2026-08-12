//! 🔺️ `change-delta-sigma-c` sparse diff construction — writes only `En1999Diff.delta_sigma_c` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_delta_sigma_c::mutation::ChangeDeltaSigmaC;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaSigmaC, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { delta_sigma_c: Some(payload.new_delta_sigma_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
