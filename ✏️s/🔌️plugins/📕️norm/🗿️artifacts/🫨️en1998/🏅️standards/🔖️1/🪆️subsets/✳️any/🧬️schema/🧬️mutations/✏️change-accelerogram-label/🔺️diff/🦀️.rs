//! ✏️ `change-accelerogram-label` diff: patches `label` of the addressed accelerogram row.

use crate::change_accelerogram_label::ChangeAccelerogramLabel;
use semio_s_artifact_norm_en1998_diff::{En1998AccelerogramDelta, En1998AccelerogramPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAccelerogramLabel, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.accelerograms.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("accelerogram {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The accelerogram already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { accelerograms: En1998AccelerogramDelta::modification(&payload.id, En1998AccelerogramPatch { label: Some(payload.new_label.clone()), ..Default::default() }), ..Default::default() })
}
