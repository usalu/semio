//! 🔺️ `change-theta-c` sparse diff construction — writes only `En1999Diff.theta_c` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_theta_c::mutation::ChangeThetaC;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaC, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { theta_c: Some(payload.new_theta_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
