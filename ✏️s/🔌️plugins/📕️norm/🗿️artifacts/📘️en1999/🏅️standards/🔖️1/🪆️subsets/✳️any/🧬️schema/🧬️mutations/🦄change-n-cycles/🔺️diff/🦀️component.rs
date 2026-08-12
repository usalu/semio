//! 🔺️ `change-n-cycles` sparse diff construction — writes only `En1999Diff.n_cycles` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_n_cycles::mutation::ChangeNCycles;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNCycles, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { n_cycles: Some(payload.new_n_cycles.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
