//! ↩️ `change-vent-name` inverse.
use crate::change_vent_name::ChangeVentName;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentName, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_name::ChangeVentName> {
    Ok((|| {
    let Some(row) = base.vent_systems.iter().find(|row| row.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentName { vent_id: payload.vent_id.clone(), new_name: row.name.clone() })]

    })())
}
