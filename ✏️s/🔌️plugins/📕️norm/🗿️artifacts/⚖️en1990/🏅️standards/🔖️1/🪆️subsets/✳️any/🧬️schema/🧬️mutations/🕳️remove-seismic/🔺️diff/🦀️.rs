//! 🕳️ `remove-seismic` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveSeismic;
use crate::diff::{En1990Diff, En1990SeismicDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveSeismic, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.seismics.len() {
        return MutationOutcome::error("mutation.target-missing", "seismics index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { seismics: En1990SeismicDelta::removal(&base.seismics[payload.index].id), ..En1990Diff::default() })
}
