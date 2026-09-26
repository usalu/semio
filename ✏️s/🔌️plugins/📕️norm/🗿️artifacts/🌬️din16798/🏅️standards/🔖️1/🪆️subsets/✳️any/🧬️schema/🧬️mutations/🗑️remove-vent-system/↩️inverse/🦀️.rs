use super::RemoveVentSystem;
use crate::mutations::insert_vent_system::InsertVentSystem;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &RemoveVentSystem, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some((index, vent)) = base.vent_systems.iter().enumerate().find(|(_, v)| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::InsertVentSystem(InsertVentSystem { index, vent: vent.clone() })]
}
