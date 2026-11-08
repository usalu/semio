//! ↩️ `change-zone-theta-i-heat-c` inverse.

use crate::change_zone_theta_i_heat_c::ChangeZoneThetaIHeatC;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneThetaIHeatC, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_theta_i_heat_c::ChangeZoneThetaIHeatC> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneThetaIHeatC { zone_id: payload.zone_id.clone(), new_theta_i_heat_c: row.theta_i_heat_c })]).unwrap_or_default())
}
