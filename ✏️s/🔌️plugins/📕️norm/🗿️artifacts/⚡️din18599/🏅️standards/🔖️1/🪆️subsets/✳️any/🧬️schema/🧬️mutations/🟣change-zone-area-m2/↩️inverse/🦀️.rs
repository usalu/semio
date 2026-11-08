//! ↩️ `change-zone-area-m2` inverse.

use crate::change_zone_area_m2::ChangeZoneAreaM2;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneAreaM2, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_area_m2::ChangeZoneAreaM2> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneAreaM2 { zone_id: payload.zone_id.clone(), new_area_m2: row.area_m2 })]).unwrap_or_default())
}
