//! ➖️ `remove-tower` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveTower;
use crate::diff::{En1998Diff, En1998TowerDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &RemoveTower, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.towers.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower #{}", payload.index), [payload.index.to_string()]);
    };
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::removal(&row.id), ..Default::default() })
}
