//! 🔺️ `change-fan-energy-reference-kwh` sparse diff construction — writes only `Din16798Diff.fan_energy_reference_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_fan_energy_reference_kwh::mutation::ChangeFanEnergyReferenceKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFanEnergyReferenceKwh, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { fan_energy_reference_kwh: Some(payload.new_fan_energy_reference_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
