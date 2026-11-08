//! ↩️ `change-vent-id` inverse.
use crate::change_vent_id::ChangeVentId;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentId, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_id::ChangeVentId> {
    Ok((|| {
    if !base.vent_systems.iter().any(|row| row.id == payload.vent_id) { return Vec::new(); }
    vec![Op::from(ChangeVentId { vent_id: payload.new_id.clone(), new_id: payload.vent_id.clone() })]

    })())
}
