//! ↩️ `remove-zone` inverse — re-inserts the removed zone at its position, computed from BASE state; a missing target yields no step.

use super::RemoveZone;
use crate::mutations::insert_zone::InsertZone;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &RemoveZone, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.zones.get(payload.index).map(|zone| vec![Din4108Mutation::InsertZone(InsertZone { index: payload.index, zone: zone.clone() })]).unwrap_or_default()

    })())
}
