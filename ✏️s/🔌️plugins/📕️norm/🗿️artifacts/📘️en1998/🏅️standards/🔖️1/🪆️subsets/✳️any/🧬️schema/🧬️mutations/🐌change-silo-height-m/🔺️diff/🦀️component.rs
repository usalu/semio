//! 🔺️ `change-silo-height-m` sparse diff construction — writes only `En1998Diff.silo_height_m` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_silo_height_m::mutation::ChangeSiloHeightM;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloHeightM, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { silo_height_m: Some(payload.new_silo_height_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
