//! ➖️ `remove-material` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveMaterial;
use crate::diff::{En1993Diff, En1993MaterialDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveMaterial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.materials.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("material index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { materials: En1993MaterialDelta::removal(&row.id), ..Default::default() })
}
