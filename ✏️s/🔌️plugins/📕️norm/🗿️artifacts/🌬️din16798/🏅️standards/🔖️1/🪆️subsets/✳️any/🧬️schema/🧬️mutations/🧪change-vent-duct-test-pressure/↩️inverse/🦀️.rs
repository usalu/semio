//! ↩️ `change-vent-duct-test-pressure` inverse.
use crate::change_vent_duct_test_pressure::ChangeVentDuctTestPressure;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentDuctTestPressure, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_duct_test_pressure::ChangeVentDuctTestPressure> {
    Ok((|| {
    let Some(row) = base.vent_systems.iter().find(|row| row.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentDuctTestPressure { vent_id: payload.vent_id.clone(), new_duct_test_pressure_pa: row.duct_test_pressure_pa })]

    })())
}
