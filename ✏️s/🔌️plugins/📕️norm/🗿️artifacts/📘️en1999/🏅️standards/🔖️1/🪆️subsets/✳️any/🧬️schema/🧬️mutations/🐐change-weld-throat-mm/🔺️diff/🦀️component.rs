//! 🔺️ `change-weld-throat-mm` sparse diff construction — writes only `En1999Diff.weld_throat_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_weld_throat_mm::mutation::ChangeWeldThroatMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWeldThroatMm, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { weld_throat_mm: Some(payload.new_weld_throat_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
