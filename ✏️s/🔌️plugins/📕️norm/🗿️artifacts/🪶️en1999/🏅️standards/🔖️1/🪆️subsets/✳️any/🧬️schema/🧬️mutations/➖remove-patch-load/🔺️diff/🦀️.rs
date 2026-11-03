//! 🔺️ `remove-patch-load` diff.

use crate::remove_patch_load::RemovePatchLoad;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PatchLoadsRows};

pub fn diff(payload: &RemovePatchLoad, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.patch_loads.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown web patch load id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { patch_loads: Some(En1999PatchLoadsRows::removal(&base.patch_loads, index)), ..Default::default() })
}
