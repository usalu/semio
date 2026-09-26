//! ↩️ `change-vent-oda-class` inverse.
use super::ChangeVentOdaClass;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentOdaClass, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentOdaClass(ChangeVentOdaClass { vent_id: payload.vent_id.clone(), new_oda_class: v.oda_class.clone() })]
}
