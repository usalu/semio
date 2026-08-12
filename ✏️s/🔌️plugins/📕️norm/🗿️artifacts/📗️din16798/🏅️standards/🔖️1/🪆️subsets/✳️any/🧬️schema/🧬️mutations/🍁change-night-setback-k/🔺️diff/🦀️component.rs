//! 🔺️ `change-night-setback-k` sparse diff construction — writes only `Din16798Diff.night_setback_k` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_night_setback_k::mutation::ChangeNightSetbackK;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNightSetbackK, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { night_setback_k: Some(payload.new_night_setback_k.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
