//! 🔺️ `change-fatigue-m` sparse diff construction — writes only `En1999Diff.fatigue_m` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_fatigue_m::mutation::ChangeFatigueM;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFatigueM, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { fatigue_m: Some(payload.new_fatigue_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
