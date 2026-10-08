//! ➖️ `remove-material` diff — removes the row at the index.

use super::RemoveMaterial;
use crate::diff::{En1993Diff, En1993MaterialDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveMaterial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.materials.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("material index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { materials: En1993MaterialDelta::removal(&base.materials, payload.index), ..Default::default() })
}
