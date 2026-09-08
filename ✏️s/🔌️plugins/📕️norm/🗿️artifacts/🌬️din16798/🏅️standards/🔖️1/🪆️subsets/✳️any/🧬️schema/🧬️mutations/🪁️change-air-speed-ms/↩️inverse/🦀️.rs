//! ↩️ `change-air-speed-ms` inverse — restores the pre-change `air_speed_m_s` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_air_speed_m_s::ChangeAirSpeedMS;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAirSpeedMS, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeAirSpeedMS(ChangeAirSpeedMS { new_air_speed_m_s: base.air_speed_m_s })]
}
//#endregion 🔖️Inverse
