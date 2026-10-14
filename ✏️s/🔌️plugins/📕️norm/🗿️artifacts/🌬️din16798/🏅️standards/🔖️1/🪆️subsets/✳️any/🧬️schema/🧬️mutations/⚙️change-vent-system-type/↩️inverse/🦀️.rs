//! ↩️ `change-vent-system-type` inverse.
use crate::change_vent_system_type::ChangeVentSystemType;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentSystemType, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_system_type::ChangeVentSystemType> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentSystemType { vent_id: payload.vent_id.clone(), new_system_type: v.system_type.clone() })]

    })())
}
