use super::RemoveZone;
use crate::mutations::insert_zone::InsertZone;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &RemoveZone, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some((index, zone)) = base.zones.iter().enumerate().find(|(_, z)| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::InsertZone(InsertZone { index, zone: zone.clone() })]
}
