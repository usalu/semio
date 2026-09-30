//! Diff for `remove-retaining-wall`.
use super::RemoveRetainingWall;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &RemoveRetainingWall, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.retaining_walls.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("retaining wall #{}", payload.index), [payload.index.to_string()]);
    }
    let mut items = base.retaining_walls.clone();
    items.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: Some(items), ..Default::default() })
}
