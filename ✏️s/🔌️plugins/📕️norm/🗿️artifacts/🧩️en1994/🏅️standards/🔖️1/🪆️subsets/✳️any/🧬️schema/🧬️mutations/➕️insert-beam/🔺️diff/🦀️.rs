//! Diff for `insert-beam`.
use super::InsertBeam;
use crate::diff::En1994BeamList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &InsertBeam, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index > base.beams.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    let mut beams = base.beams.clone();
    beams.insert(payload.index, payload.beam.clone());
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamList { values: beams }), ..Default::default() })
}
