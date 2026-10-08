//! ➖️ `remove-material` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveMaterial;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993MaterialEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveMaterial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.materials.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("material index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { materials: vec![En1993MaterialEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
