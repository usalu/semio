//! 🔺️ `change-fan-t-run-h` sparse diff construction — writes only `Din16798Diff.fan_t_run_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_fan_t_run_h::mutation::ChangeFanTRunH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFanTRunH, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { fan_t_run_h: Some(payload.new_fan_t_run_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
