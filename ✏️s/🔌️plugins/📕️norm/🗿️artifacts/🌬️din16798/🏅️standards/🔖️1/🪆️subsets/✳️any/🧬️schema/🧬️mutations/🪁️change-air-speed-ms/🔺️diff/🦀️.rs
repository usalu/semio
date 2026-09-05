//! 🔺️ `change-air-speed-ms` sparse diff construction — writes only `Din16798Diff.air_speed_m_s` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_air_speed_m_s::ChangeAirSpeedMS;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAirSpeedMS, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_air_speed_m_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Air speed must be a finite number, got {}.", payload.new_air_speed_m_s), Vec::<String>::new());
    }
    if base.air_speed_m_s == payload.new_air_speed_m_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Air speed is already {}.", payload.new_air_speed_m_s));
    }
    protocol::MutationOutcome::new(Din16798Diff { air_speed_m_s: Some(payload.new_air_speed_m_s.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
