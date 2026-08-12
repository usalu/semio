//! 🔺️ `change-n-ed-kn` sparse diff construction — writes only `En1996Diff.n_ed_kn` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_n_ed_kn::mutation::ChangeNEdKn;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNEdKn, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { n_ed_kn: Some(payload.new_n_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
