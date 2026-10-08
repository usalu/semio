//! ➖️ `remove-retaining-wall` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveRetainingWall;
use crate::diff::En1998RowEdit as _;
use crate::diff::{En1998Diff, En1998RetainingWallEdit};
use crate::En1998Snapshot;

pub fn diff(payload: &RemoveRetainingWall, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.retaining_walls.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("retaining wall #{}", payload.index), [payload.index.to_string()]);
    };
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: vec![En1998RetainingWallEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
