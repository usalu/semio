//! Diff for `remove-beam`.
use crate::remove_beam::RemoveBeam;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows};

pub fn diff(payload: &RemoveBeam, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index >= base.beams.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamsRows { removed: vec![payload.index], ..Default::default() }), ..Default::default() })
}
