//! Diff for `insert-retaining-wall`.
use super::InsertRetainingWall;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertRetainingWall, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut items = base.retaining_walls.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.wall.clone());
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: Some(items), ..Default::default() })
}
