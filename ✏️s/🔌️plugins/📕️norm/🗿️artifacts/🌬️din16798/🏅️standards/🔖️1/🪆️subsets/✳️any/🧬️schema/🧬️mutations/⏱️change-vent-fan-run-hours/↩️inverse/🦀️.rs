//! ↩️ `change-vent-fan-run-hours` inverse.
use crate::change_vent_fan_run_hours::ChangeVentFanRunHours;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentFanRunHours, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_fan_run_hours::ChangeVentFanRunHours> {
    Ok((|| {
    let Some(row) = base.vent_systems.iter().find(|row| row.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentFanRunHours { vent_id: payload.vent_id.clone(), new_fan_t_run_h: row.fan_t_run_h })]

    })())
}
