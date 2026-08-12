//! 🔺️ `change-span-m` sparse diff construction — writes only `En1992Diff.span_m` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_span_m::mutation::ChangeSpanM;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSpanM, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { span_m: Some(payload.new_span_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
