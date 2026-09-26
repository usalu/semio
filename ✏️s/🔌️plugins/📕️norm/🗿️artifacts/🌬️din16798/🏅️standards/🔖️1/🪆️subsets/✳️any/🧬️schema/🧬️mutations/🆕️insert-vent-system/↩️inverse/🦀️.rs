use super::InsertVentSystem;
use crate::mutations::remove_vent_system::RemoveVentSystem;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &InsertVentSystem, _base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::RemoveVentSystem(RemoveVentSystem { vent_id: payload.vent.id.clone() })]
}
