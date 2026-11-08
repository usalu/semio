//! ↩️ `change-zone-id` inverse.
use crate::change_zone_id::ChangeZoneId;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneId, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_id::ChangeZoneId> {
    Ok((|| {
    if !base.zones.iter().any(|row| row.id == payload.zone_id) { return Vec::new(); }
    vec![Op::from(ChangeZoneId { zone_id: payload.new_id.clone(), new_id: payload.zone_id.clone() })]

    })())
}
