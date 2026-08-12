//! 🔺️ `change-storeys` sparse diff construction — writes only `En1996Diff.storeys` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_storeys::mutation::ChangeStoreys;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStoreys, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { storeys: Some(payload.new_storeys.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
