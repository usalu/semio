//! 🔺️ `change-humidification-required-kg-h` sparse diff construction — writes only `Din16798Diff.humidification_required_kg_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_humidification_required_kg_h::mutation::ChangeHumidificationRequiredKgH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHumidificationRequiredKgH, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { humidification_required_kg_h: Some(payload.new_humidification_required_kg_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
