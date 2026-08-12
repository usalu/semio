//! 🔺️ `change-p-kn` sparse diff construction — writes only `En1992Diff.p_kn` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_p_kn::mutation::ChangePKn;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePKn, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { p_kn: Some(payload.new_p_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
