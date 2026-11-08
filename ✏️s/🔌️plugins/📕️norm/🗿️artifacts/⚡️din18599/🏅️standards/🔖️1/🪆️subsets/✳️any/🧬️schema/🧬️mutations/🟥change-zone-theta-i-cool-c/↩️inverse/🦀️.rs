//! ↩️ `change-zone-theta-i-cool-c` inverse.

use crate::change_zone_theta_i_cool_c::ChangeZoneThetaICoolC;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneThetaICoolC, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_theta_i_cool_c::ChangeZoneThetaICoolC> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneThetaICoolC { zone_id: payload.zone_id.clone(), new_theta_i_cool_c: row.theta_i_cool_c })]).unwrap_or_default())
}
