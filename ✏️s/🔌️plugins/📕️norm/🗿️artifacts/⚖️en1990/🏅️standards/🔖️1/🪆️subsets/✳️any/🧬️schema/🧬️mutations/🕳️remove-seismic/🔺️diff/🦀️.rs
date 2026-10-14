//! 🕳️ `remove-seismic` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_seismic::RemoveSeismic;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990SeismicDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveSeismic, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.seismics.len() {
        return MutationOutcome::error("mutation.target-missing", "seismics index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { seismics: En1990SeismicDelta::removal(&base.seismics, payload.index), ..En1990Diff::default() })
}
