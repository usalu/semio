//! 🔺️ `change-exposure` sparse diff construction — writes only `En1996Diff.exposure` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_exposure::mutation::ChangeExposure;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeExposure, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { exposure: Some(payload.new_exposure.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
