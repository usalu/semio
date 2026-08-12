//! 🔺️ `change-cooling-delta-th` sparse diff construction — writes only `Din16798Diff.cooling_delta_t_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_cooling_delta_t_h::mutation::ChangeCoolingDeltaTH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCoolingDeltaTH, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { cooling_delta_t_h: Some(payload.new_cooling_delta_t_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
