//! ↩️ `change-vent-design-airflow` inverse.
use super::ChangeVentDesignAirflow;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentDesignAirflow, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentDesignAirflow(ChangeVentDesignAirflow { vent_id: payload.vent_id.clone(), new_design_airflow_m3_h: v.design_airflow_m3_h })]
}
