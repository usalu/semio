//! 🔺️ `change-patch-x` diff: patches `x` of the addressed web patch load row.

use crate::change_patch_x::ChangePatchX;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PatchLoadsPatch, En1999PatchLoadsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangePatchX, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "patch-load", Vec::<String>::new());
    };
    if r0.x == payload.new_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The web patch load already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { patch_loads: Some(En1999PatchLoadsRows::modification(&payload.patch_load_id, En1999PatchLoadsPatch { x: Some(payload.new_x), ..Default::default() })), ..Default::default() })
}
