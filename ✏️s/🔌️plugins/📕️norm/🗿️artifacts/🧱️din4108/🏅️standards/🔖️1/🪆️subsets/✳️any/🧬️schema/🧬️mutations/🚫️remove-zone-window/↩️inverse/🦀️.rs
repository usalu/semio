//! ↩️ `remove-zone-window` inverse.

use super::RemoveZoneWindow;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &RemoveZoneWindow, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
