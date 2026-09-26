//! ↩️ `change-zone-window-shading-fc` inverse via snapshot restore of list fields.

use super::ChangeZoneWindowShadingFc;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeZoneWindowShadingFc, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    // Whole-list restore is expressed by re-inserting base lists through set-like rebuilds in from_snapshot.
    let _ = base;
    Vec::new()
}
