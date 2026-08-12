//! 🔺️ `change-h-ed-kn` sparse diff construction — writes only `En1996Diff.h_ed_kn` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_h_ed_kn::mutation::ChangeHEdKn;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHEdKn, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { h_ed_kn: Some(payload.new_h_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
