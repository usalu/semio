//! ➖️ `remove-retaining-wall` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveRetainingWall;
use crate::diff::{En1998Diff, En1998RetainingWallDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &RemoveRetainingWall, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.retaining_walls.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("retaining wall #{}", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: En1998RetainingWallDelta::removal(&base.retaining_walls, payload.index), ..Default::default() })
}
