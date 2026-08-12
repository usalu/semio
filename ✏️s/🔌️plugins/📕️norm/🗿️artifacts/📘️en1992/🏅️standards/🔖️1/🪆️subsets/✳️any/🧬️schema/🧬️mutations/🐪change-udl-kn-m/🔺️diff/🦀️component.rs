//! 🔺️ `change-udl-kn-m` sparse diff construction — writes only `En1992Diff.udl_kn_m` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_udl_kn_m::mutation::ChangeUdlKnM;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeUdlKnM, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { udl_kn_m: Some(payload.new_udl_kn_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
