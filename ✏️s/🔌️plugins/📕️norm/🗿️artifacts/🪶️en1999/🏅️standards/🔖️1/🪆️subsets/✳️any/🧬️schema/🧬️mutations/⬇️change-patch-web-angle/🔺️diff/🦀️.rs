//! 🔺️ `change-patch-web-angle` diff: patches `web_angle` of the addressed web patch load row.

use crate::change_patch_web_angle::ChangePatchWebAngle;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PatchLoadsPatch, En1999PatchLoadsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangePatchWebAngle, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_web_angle.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "patch-load", Vec::<String>::new());
    };
    if r0.web_angle == payload.new_web_angle {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The web patch load already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { patch_loads: Some(En1999PatchLoadsRows::modification(&payload.patch_load_id, En1999PatchLoadsPatch { web_angle: Some(payload.new_web_angle), ..Default::default() })), ..Default::default() })
}
