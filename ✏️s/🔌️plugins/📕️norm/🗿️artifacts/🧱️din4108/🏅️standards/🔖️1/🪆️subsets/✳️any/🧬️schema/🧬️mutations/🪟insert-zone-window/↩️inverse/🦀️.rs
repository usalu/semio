//! ↩️ `insert-zone-window` inverse via snapshot restore of list fields.

use super::InsertZoneWindow;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &InsertZoneWindow, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    // Whole-list restore is expressed by re-inserting base lists through set-like rebuilds in from_snapshot.
    let _ = base;
    Vec::new()
}
