//! ↩️ `change-vent-filter-sup` inverse.
use super::ChangeVentFilterSup;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentFilterSup, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentFilterSup(ChangeVentFilterSup { vent_id: payload.vent_id.clone(), new_filter_sup_class: v.filter_sup_class.clone() })]
}
