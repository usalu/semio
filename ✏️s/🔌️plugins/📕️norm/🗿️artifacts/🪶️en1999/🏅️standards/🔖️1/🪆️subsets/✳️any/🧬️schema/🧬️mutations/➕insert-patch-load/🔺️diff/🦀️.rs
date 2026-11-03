//! 🔺️ `insert-patch-load` diff.

use crate::insert_patch_load::InsertPatchLoad;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PatchLoadsRows};

pub fn diff(payload: &InsertPatchLoad, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.patch_loads.iter().any(|existing| existing.id == payload.patch_load.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Web patch load id {} already exists.", payload.patch_load.id), [payload.patch_load.id.clone()]);
    }
    if (payload.index as usize) > base.patch_loads.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.patch_loads.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { patch_loads: Some(En1999PatchLoadsRows::insertion(payload.index as usize, payload.patch_load.clone())), ..Default::default() })
}
