//! ↩️ `change-vent-sfp-class` inverse.
use super::ChangeVentSfpClass;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentSfpClass, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentSfpClass(ChangeVentSfpClass { vent_id: payload.vent_id.clone(), new_sfp_required_class: v.sfp_required_class })]
}
