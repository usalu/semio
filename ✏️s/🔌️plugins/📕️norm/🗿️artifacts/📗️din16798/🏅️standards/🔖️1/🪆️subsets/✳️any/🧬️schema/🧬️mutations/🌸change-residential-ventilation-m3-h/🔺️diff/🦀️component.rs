//! 🔺️ `change-residential-ventilation-m3-h` sparse diff construction — writes only `Din16798Diff.residential_ventilation_m3_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeResidentialVentilationM3H, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { residential_ventilation_m3_h: Some(payload.new_residential_ventilation_m3_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
