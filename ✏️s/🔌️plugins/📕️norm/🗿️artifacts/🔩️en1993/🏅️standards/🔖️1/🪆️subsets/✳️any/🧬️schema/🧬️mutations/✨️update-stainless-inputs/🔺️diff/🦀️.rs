//! ✨️ `update-stainless-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateStainlessInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993MaterialEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateStainlessInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.materials.iter().position(|row| row.id == payload.material.id) {
        Some(index) if base.materials[index] == payload.material => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993MaterialEdit::replace(index, payload.material.id.clone(), payload.material.clone()),
        None => En1993MaterialEdit::insert(base.materials.len(), payload.material.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { materials: vec![edit], ..Default::default() })
}
