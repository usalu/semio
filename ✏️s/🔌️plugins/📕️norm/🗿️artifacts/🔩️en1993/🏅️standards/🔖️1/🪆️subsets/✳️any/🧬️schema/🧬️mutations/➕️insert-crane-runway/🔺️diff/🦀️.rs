//! ➕️ `insert-crane-runway` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertCraneRunway;
use crate::diff::{En1993Diff, En1993CraneRunwayDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertCraneRunway, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.crane_runways.iter().any(|existing| existing.id == payload.crane_runway.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Crane runway id {} already exists.", payload.crane_runway.id), [payload.crane_runway.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.crane_runways.len());
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::insertion(index, payload.crane_runway.clone()), ..Default::default() })
}
