//! 🔺️ `change-sfp-wm3-s` sparse diff construction — writes only `Din16798Diff.sfp_w_m3_s` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_sfp_w_m3_s::mutation::ChangeSfpWM3S;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSfpWM3S, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { sfp_w_m3_s: Some(payload.new_sfp_w_m3_s.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
