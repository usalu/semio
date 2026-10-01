//! Diff for `insert-retaining-wall`.
use super::InsertRetainingWall;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertRetainingWall, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.retaining_walls.iter().any(|existing| existing.id == payload.wall.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Wall id {} already exists.", payload.wall.id), [payload.wall.id.clone()]);
    }
    let mut items = base.retaining_walls.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.wall.clone());
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: Some(items), ..Default::default() })
}
