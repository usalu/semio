//! 🔺️ `change-z-investigated-m` sparse diff construction — writes only `En1997Diff.z_investigated_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_z_investigated_m::mutation::ChangeZInvestigatedM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeZInvestigatedM, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { z_investigated_m: Some(payload.new_z_investigated_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
