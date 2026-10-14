//! ➖️ `remove-material` diff — removes the row at the index.

use crate::remove_material::RemoveMaterial;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveMaterial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.materials.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("material index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { materials: En1993MaterialDelta::removal(&base.materials, payload.index), ..Default::default() })
}
