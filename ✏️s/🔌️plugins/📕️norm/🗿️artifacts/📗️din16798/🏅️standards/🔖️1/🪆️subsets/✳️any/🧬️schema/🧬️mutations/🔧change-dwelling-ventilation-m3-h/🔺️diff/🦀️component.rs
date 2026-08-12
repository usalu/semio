//! 🔺️ `change-dwelling-ventilation-m3-h` sparse diff construction — writes only `Din16798Diff.dwelling_ventilation_m3_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_dwelling_ventilation_m3_h::mutation::ChangeDwellingVentilationM3H;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDwellingVentilationM3H, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { dwelling_ventilation_m3_h: Some(payload.new_dwelling_ventilation_m3_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
