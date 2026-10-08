//! ✨️ `update-stainless-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateStainlessInputs;
use crate::diff::{En1993Diff, En1993MaterialDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateStainlessInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.materials.iter().position(|row| row.id == payload.material.id) {
        Some(index) if base.materials[index] == payload.material => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993MaterialDelta::removal(&payload.material.id);
            replacement.absorb(En1993MaterialDelta::insertion(&base.materials, index, payload.material.clone()));
            replacement
        }
        None => En1993MaterialDelta::insertion(&base.materials, base.materials.len(), payload.material.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { materials: delta, ..Default::default() })
}
