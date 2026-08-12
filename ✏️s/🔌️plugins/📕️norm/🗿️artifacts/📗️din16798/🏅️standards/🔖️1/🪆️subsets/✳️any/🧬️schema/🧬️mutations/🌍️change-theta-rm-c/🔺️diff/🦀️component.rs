//! 🔺️ `change-theta-rm-c` sparse diff construction — writes only `Din16798Diff.theta_rm_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_theta_rm_c::mutation::ChangeThetaRmC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaRmC, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { theta_rm_c: Some(payload.new_theta_rm_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
