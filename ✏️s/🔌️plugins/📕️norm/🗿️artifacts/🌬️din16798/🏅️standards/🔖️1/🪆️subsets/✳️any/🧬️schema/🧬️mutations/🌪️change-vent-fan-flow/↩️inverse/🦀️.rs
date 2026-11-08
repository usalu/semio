//! ↩️ `change-vent-fan-flow` inverse.
use crate::change_vent_fan_flow::ChangeVentFanFlow;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentFanFlow, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_fan_flow::ChangeVentFanFlow> {
    Ok((|| {
    let Some(row) = base.vent_systems.iter().find(|row| row.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentFanFlow { vent_id: payload.vent_id.clone(), new_fan_q_v_m3_s: row.fan_q_v_m3_s })]

    })())
}
