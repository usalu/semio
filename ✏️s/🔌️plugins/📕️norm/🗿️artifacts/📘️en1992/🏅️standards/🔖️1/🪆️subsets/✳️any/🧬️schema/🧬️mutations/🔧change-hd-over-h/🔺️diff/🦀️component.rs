//! 🔺️ `change-hd-over-h` sparse diff construction — writes only `En1992Diff.hd_over_h` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_hd_over_h::mutation::ChangeHdOverH;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHdOverH, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { hd_over_h: Some(payload.new_hd_over_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
