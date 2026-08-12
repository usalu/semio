//! 🔺️ `change-t-ef-mm` sparse diff construction — writes only `En1996Diff.t_ef_mm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_t_ef_mm::mutation::ChangeTEfMm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTEfMm, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { t_ef_mm: Some(payload.new_t_ef_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
