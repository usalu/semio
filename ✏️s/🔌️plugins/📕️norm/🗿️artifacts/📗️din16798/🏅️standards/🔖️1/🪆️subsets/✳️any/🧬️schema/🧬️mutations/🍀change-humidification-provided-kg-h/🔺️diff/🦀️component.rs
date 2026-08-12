//! 🔺️ `change-humidification-provided-kg-h` sparse diff construction — writes only `Din16798Diff.humidification_provided_kg_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_humidification_provided_kg_h::mutation::ChangeHumidificationProvidedKgH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHumidificationProvidedKgH, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { humidification_provided_kg_h: Some(payload.new_humidification_provided_kg_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
