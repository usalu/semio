//! 🔺️ `change-air-speed-ms` sparse diff construction — writes only `Din16798Diff.air_speed_m_s` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_air_speed_m_s::mutation::ChangeAirSpeedMS;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAirSpeedMS, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { air_speed_m_s: Some(payload.new_air_speed_m_s.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
