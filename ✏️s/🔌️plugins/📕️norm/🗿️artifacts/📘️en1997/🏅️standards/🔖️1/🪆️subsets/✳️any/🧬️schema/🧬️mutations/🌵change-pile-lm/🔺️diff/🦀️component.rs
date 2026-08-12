//! 🔺️ `change-pile-lm` sparse diff construction — writes only `En1997Diff.pile_l_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_pile_l_m::mutation::ChangePileLM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePileLM, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { pile_l_m: Some(payload.new_pile_l_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
