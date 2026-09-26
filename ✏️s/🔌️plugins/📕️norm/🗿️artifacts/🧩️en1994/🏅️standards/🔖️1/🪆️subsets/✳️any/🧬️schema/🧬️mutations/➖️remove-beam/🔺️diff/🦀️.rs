//! Diff for `remove-beam`.
use super::RemoveBeam;
use crate::diff::En1994BeamList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &RemoveBeam, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index >= base.beams.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    let mut beams = base.beams.clone();
    beams.remove(payload.index);
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamList { values: beams }), ..Default::default() })
}
