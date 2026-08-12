//! 🔺️ `change-qs-kpa` sparse diff construction — writes only `En1997Diff.q_s_kpa` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_q_s_kpa::mutation::ChangeQSKpa;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeQSKpa, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { q_s_kpa: Some(payload.new_q_s_kpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
