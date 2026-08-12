//! 🔺️ `change-n-ed-kn` sparse diff construction — writes only `En1999Diff.n_ed_kn` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_n_ed_kn::mutation::ChangeNEdKn;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNEdKn, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { n_ed_kn: Some(payload.new_n_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
