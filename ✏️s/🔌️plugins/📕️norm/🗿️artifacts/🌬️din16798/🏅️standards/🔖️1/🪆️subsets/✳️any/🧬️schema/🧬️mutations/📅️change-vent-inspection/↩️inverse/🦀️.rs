//! ↩️ `change-vent-inspection` inverse.
use super::ChangeVentInspection;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentInspection, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentInspection(ChangeVentInspection { vent_id: payload.vent_id.clone(), new_years_since_inspection: v.years_since_inspection })]
}
