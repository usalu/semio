//! 🔺️ `change-phi-deg` sparse diff construction — writes only `En1997Diff.phi_deg` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_phi_deg::mutation::ChangePhiDeg;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePhiDeg, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { phi_deg: Some(payload.new_phi_deg.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
