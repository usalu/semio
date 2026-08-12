//! 🔺️ `change-f-ed-kn` sparse diff construction — writes only `En1995Diff.f_ed_kn` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_f_ed_kn::mutation::ChangeFEdKn;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFEdKn, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { f_ed_kn: Some(payload.new_f_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
