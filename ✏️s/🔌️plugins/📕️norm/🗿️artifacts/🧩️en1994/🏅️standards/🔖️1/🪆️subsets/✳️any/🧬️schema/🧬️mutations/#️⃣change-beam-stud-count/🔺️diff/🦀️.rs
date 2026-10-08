//! Diff for `change-beam-stud-count`.
use super::ChangeBeamStudCount;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamStudCount, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.studs.total_count == payload.new_total_count {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows { modified: vec![En1994BeamsPatch { index: payload.index, studs_total_count: Some(payload.new_total_count), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
