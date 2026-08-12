//! 🔺️ `change-fk-mpa` sparse diff construction — writes only `En1996Diff.f_k_mpa` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_f_k_mpa::mutation::ChangeFKMpa;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFKMpa, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { f_k_mpa: Some(payload.new_f_k_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
