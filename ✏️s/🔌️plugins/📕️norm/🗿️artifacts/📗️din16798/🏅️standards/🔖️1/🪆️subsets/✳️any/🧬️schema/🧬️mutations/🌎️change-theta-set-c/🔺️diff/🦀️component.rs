//! 🔺️ `change-theta-set-c` sparse diff construction — writes only `Din16798Diff.theta_set_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_theta_set_c::mutation::ChangeThetaSetC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaSetC, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { theta_set_c: Some(payload.new_theta_set_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
