//! 🔺️ `change-anchor-f-yk-mpa` sparse diff construction — writes only `En1992Diff.anchor_f_yk_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_f_yk_mpa::mutation::ChangeAnchorFYkMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorFYkMpa, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { anchor_f_yk_mpa: Some(payload.new_anchor_f_yk_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
