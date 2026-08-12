//! 🔺️ `change-theta-ec` sparse diff construction — writes only `Din16798Diff.theta_e_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_theta_e_c::mutation::ChangeThetaEC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaEC, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { theta_e_c: Some(payload.new_theta_e_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
