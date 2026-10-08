//! 🕳️ `remove-seismic` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveSeismic;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990SeismicEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveSeismic, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.seismics.len() {
        return MutationOutcome::error("mutation.target-missing", "seismics index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { seismics: vec![En1990SeismicEdit::remove(payload.index, base.seismics[payload.index].id.clone())], ..En1990Diff::default() })
}
