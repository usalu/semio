//! 🔺️ `change-es-mpa` sparse diff construction — writes only `En1997Diff.e_s_mpa` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_e_s_mpa::mutation::ChangeESMpa;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeESMpa, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { e_s_mpa: Some(payload.new_e_s_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
