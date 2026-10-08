//! Diff for `change-beam-construction`.
use super::ChangeBeamConstruction;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamConstruction, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.construction == payload.new_construction {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows { modified: vec![En1994BeamsPatch { index: payload.index, construction: Some(payload.new_construction.clone()), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
