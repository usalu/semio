use super::InsertZone;
use crate::mutations::remove_zone::RemoveZone;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &InsertZone, _base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::RemoveZone(RemoveZone { zone_id: payload.zone.id.clone() })]
}
