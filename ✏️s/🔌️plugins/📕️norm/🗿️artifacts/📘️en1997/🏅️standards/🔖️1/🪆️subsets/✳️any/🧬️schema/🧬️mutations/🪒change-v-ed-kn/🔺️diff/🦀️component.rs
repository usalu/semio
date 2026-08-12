//! 🔺️ `change-v-ed-kn` sparse diff construction — writes only `En1997Diff.v_ed_kn` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_v_ed_kn::mutation::ChangeVEdKn;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeVEdKn, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { v_ed_kn: Some(payload.new_v_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
