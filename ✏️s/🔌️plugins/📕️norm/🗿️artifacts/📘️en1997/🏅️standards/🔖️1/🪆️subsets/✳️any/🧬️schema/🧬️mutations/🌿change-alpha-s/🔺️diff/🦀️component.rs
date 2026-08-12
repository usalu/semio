//! 🔺️ `change-alpha-s` sparse diff construction — writes only `En1997Diff.alpha_s` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_alpha_s::mutation::ChangeAlphaS;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAlphaS, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { alpha_s: Some(payload.new_alpha_s.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
