//! ↩️ `change-vent-heat-recovery` inverse.
use crate::change_vent_heat_recovery::ChangeVentHeatRecovery;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentHeatRecovery, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_heat_recovery::ChangeVentHeatRecovery> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentHeatRecovery { vent_id: payload.vent_id.clone(), new_heat_recovery_eta: v.heat_recovery_eta })]

    })())
}
