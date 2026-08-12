//! 🔺️ `change-v-rd-kn` sparse diff construction — writes only `En1998Diff.v_rd_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_v_rd_kn::mutation::ChangeVRdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeVRdKn, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { v_rd_kn: Some(payload.new_v_rd_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
