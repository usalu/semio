//! 🔺️ `change-delta-sigma-ed` sparse diff construction — writes only `En1999Diff.delta_sigma_ed` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_delta_sigma_ed::mutation::ChangeDeltaSigmaEd;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaSigmaEd, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { delta_sigma_ed: Some(payload.new_delta_sigma_ed.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
