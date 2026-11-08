//! ↩️ `change-zone-volume-m3` inverse.

use crate::change_zone_volume_m3::ChangeZoneVolumeM3;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneVolumeM3, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_volume_m3::ChangeZoneVolumeM3> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneVolumeM3 { zone_id: payload.zone_id.clone(), new_volume_m3: row.volume_m3 })]).unwrap_or_default())
}
