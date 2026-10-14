//! ↩️ `change-vent-inspection` inverse.
use crate::change_vent_inspection::ChangeVentInspection;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentInspection, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_inspection::ChangeVentInspection> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentInspection { vent_id: payload.vent_id.clone(), new_years_since_inspection: v.years_since_inspection })]

    })())
}
