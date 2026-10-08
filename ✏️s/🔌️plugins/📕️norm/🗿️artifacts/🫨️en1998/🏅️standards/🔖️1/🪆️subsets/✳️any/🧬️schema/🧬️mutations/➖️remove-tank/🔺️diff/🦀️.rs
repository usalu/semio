//! ➖️ `remove-tank` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveTank;
use crate::diff::{En1998Diff, En1998TankDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &RemoveTank, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.tanks.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tank #{}", payload.index), [payload.index.to_string()]);
    };
    protocol::MutationOutcome::new(En1998Diff { tanks: En1998TankDelta::removal(&row.id), ..Default::default() })
}
