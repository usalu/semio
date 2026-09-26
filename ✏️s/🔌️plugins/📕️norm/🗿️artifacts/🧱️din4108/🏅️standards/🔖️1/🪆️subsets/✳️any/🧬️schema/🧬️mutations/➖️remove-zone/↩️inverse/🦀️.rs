//! ↩️ `remove-zone` inverse.

use super::RemoveZone;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &RemoveZone, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
