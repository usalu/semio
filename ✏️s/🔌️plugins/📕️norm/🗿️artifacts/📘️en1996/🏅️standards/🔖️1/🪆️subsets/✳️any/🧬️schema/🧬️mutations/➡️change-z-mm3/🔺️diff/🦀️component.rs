//! 🔺️ `change-z-mm3` sparse diff construction — writes only `En1996Diff.z_mm3` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_z_mm3::mutation::ChangeZMm3;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeZMm3, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { z_mm3: Some(payload.new_z_mm3.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
