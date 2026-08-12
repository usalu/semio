//! 🔺️ `change-a-ef-mm2` sparse diff construction — writes only `En1995Diff.a_ef_mm2` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_a_ef_mm2::mutation::ChangeAEfMm2;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAEfMm2, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { a_ef_mm2: Some(payload.new_a_ef_mm2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
