//! ↩️ `change-vent-duct-class` inverse.
use super::ChangeVentDuctClass;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentDuctClass, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentDuctClass(ChangeVentDuctClass { vent_id: payload.vent_id.clone(), new_duct_class: v.duct_class.clone() })]
}
