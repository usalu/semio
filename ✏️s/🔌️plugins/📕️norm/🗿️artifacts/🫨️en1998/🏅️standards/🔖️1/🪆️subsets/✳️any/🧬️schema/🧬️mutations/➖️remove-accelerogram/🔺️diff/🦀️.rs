//! ➖️ `remove-accelerogram` diff: removes the addressed accelerogram row.

use crate::remove_accelerogram::RemoveAccelerogram;
use semio_s_artifact_norm_en1998_diff::{En1998AccelerogramDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveAccelerogram, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.accelerograms.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("accelerogram {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { accelerograms: En1998AccelerogramDelta::removal(&base.accelerograms, index), ..Default::default() })
}
