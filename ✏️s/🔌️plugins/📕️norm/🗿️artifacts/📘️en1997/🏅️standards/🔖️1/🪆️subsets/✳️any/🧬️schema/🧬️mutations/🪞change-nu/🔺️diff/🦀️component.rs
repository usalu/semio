//! 🔺️ `change-nu` sparse diff construction — writes only `En1997Diff.nu` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_nu::mutation::ChangeNu;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNu, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { nu: Some(payload.new_nu.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
