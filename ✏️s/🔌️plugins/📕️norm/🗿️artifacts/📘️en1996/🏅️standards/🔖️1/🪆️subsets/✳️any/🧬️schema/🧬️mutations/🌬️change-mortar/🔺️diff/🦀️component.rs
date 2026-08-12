//! 🔺️ `change-mortar` sparse diff construction — writes only `En1996Diff.mortar` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_mortar::mutation::ChangeMortar;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMortar, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { mortar: Some(payload.new_mortar.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
