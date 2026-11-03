//! 🔺️ `change-patch-subject-id` diff: patches `subject_id` of the addressed web patch load row.

use crate::change_patch_subject_id::ChangePatchSubjectId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PatchLoadsPatch, En1999PatchLoadsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangePatchSubjectId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "patch-load", Vec::<String>::new());
    };
    if r0.subject_id == payload.new_subject_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The web patch load already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { patch_loads: Some(En1999PatchLoadsRows::modification(&payload.patch_load_id, En1999PatchLoadsPatch { subject_id: Some(payload.new_subject_id.clone()), ..Default::default() })), ..Default::default() })
}
