//! 🔺️ `change-span-m` — sparse diff construction.

use super::mutation::ChangeSpanM;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSpanM, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { span_m: Some(payload.new_span_m), ..Default::default() }
}
//#endregion 🔖️Diff
