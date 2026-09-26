//! ↩️ `change-vent-system-type` inverse.
use super::ChangeVentSystemType;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentSystemType, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentSystemType(ChangeVentSystemType { vent_id: payload.vent_id.clone(), new_system_type: v.system_type.clone() })]
}
