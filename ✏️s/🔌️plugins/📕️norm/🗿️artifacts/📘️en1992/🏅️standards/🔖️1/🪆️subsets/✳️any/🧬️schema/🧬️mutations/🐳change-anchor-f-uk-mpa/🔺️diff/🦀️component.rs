//! 🔺️ `change-anchor-f-uk-mpa` sparse diff construction — writes only `En1992Diff.anchor_f_uk_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_f_uk_mpa::mutation::ChangeAnchorFUkMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorFUkMpa, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { anchor_f_uk_mpa: Some(payload.new_anchor_f_uk_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
