//! ↩️ `change-vent-heat-recovery` inverse.
use super::ChangeVentHeatRecovery;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentHeatRecovery, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentHeatRecovery(ChangeVentHeatRecovery { vent_id: payload.vent_id.clone(), new_heat_recovery_eta: v.heat_recovery_eta })]
}
