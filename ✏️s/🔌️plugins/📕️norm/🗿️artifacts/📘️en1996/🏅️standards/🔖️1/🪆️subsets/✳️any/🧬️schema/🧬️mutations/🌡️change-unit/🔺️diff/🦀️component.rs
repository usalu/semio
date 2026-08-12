//! 🔺️ `change-unit` sparse diff construction — writes only `En1996Diff.unit` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_unit::mutation::ChangeUnit;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeUnit, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { unit: Some(payload.new_unit.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
