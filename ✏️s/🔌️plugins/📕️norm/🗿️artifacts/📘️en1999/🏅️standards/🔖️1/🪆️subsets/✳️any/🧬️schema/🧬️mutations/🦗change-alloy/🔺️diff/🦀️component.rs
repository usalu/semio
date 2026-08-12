//! 🔺️ `change-alloy` sparse diff construction — writes only `En1999Diff.alloy` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_alloy::mutation::ChangeAlloy;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAlloy, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { alloy: Some(payload.new_alloy.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
