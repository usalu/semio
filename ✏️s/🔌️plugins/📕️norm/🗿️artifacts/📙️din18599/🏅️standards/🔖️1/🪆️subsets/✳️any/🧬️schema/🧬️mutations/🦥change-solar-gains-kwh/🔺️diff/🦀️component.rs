//! 🔺️ `change-solar-gains-kwh` sparse diff construction — writes only `Din18599Diff.solar_gains_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_solar_gains_kwh::mutation::ChangeSolarGainsKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSolarGainsKwh, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { solar_gains_kwh: Some(payload.new_solar_gains_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
