//! ↩️ `insert-zone` inverse — removes the inserted zone at its landing position, computed from BASE state; a missing target yields no step.

use super::InsertZone;
use crate::mutations::remove_zone::RemoveZone;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &InsertZone, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din4108Mutation::RemoveZone(RemoveZone { index: payload.index.unwrap_or(usize::MAX).min(base.zones.len()) })]

    })())
}
