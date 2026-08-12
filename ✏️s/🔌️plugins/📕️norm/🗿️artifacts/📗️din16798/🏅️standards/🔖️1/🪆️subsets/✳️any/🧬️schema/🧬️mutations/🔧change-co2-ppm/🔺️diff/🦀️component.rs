//! 🔺️ `change-co2-ppm` sparse diff construction — writes only `Din16798Diff.co2_ppm` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_co2_ppm::mutation::ChangeCo2Ppm;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCo2Ppm, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { co2_ppm: Some(payload.new_co2_ppm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
