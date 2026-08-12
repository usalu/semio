//! 🔺️ `change-mu` sparse diff construction — writes only `En1996Diff.mu` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_mu::mutation::ChangeMu;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMu, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { mu: Some(payload.new_mu.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
