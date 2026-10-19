//! 🔺️ Diff for `insert-beam`.
use crate::insert_beam::InsertBeam;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsInserted};

pub fn diff(payload: &InsertBeam, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index > base.beams.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamsRows { inserted: vec![En1994BeamsInserted { index: payload.index, row: payload.beam.clone() }], ..Default::default() }), ..Default::default() })
}
