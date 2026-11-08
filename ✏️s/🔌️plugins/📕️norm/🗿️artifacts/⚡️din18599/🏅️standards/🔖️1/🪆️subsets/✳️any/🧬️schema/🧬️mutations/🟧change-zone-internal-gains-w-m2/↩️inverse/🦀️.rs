//! ↩️ `change-zone-internal-gains-w-m2` inverse.

use crate::change_zone_internal_gains_w_m2::ChangeZoneInternalGainsWM2;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneInternalGainsWM2, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_internal_gains_w_m2::ChangeZoneInternalGainsWM2> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneInternalGainsWM2 { zone_id: payload.zone_id.clone(), new_internal_gains_w_m2: row.internal_gains_w_m2 })]).unwrap_or_default())
}
