//! ↩️ `change-vent-duct-leakage` inverse.
use super::ChangeVentDuctLeakage;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentDuctLeakage, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentDuctLeakage(ChangeVentDuctLeakage { vent_id: payload.vent_id.clone(), new_duct_leakage_m3_s_m2: v.duct_leakage_m3_s_m2 })]
}
