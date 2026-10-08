//! 🧱️ `insert-retaining-wall` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertRetainingWall;
use crate::diff::{En1998Diff, En1998RetainingWallDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertRetainingWall, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.retaining_walls.iter().any(|existing| existing.id == payload.wall.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Wall id {} already exists.", payload.wall.id), [payload.wall.id.clone()]);
    }
    let index = payload.index.min(base.retaining_walls.len());
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: En1998RetainingWallDelta::insertion(&base.retaining_walls, index, payload.wall.clone()), ..Default::default() })
}
