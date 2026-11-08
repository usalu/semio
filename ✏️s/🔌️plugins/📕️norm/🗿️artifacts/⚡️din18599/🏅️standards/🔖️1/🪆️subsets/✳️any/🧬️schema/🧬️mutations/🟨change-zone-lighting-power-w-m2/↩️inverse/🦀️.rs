//! ↩️ `change-zone-lighting-power-w-m2` inverse.

use crate::change_zone_lighting_power_w_m2::ChangeZoneLightingPowerWM2;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneLightingPowerWM2, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_lighting_power_w_m2::ChangeZoneLightingPowerWM2> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneLightingPowerWM2 { zone_id: payload.zone_id.clone(), new_lighting_power_w_m2: row.lighting_power_w_m2 })]).unwrap_or_default())
}
