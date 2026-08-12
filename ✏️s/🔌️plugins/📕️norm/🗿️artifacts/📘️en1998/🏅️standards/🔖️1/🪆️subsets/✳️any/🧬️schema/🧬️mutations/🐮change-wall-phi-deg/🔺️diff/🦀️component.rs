//! 🔺️ `change-wall-phi-deg` sparse diff construction — writes only `En1998Diff.wall_phi_deg` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_wall_phi_deg::mutation::ChangeWallPhiDeg;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallPhiDeg, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { wall_phi_deg: Some(payload.new_wall_phi_deg.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
