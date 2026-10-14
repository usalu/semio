//! ↩️ `change-vent-design-airflow` inverse.
use crate::change_vent_design_airflow::ChangeVentDesignAirflow;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentDesignAirflow, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_design_airflow::ChangeVentDesignAirflow> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentDesignAirflow { vent_id: payload.vent_id.clone(), new_design_airflow_m3_h: v.design_airflow_m3_h })]

    })())
}
