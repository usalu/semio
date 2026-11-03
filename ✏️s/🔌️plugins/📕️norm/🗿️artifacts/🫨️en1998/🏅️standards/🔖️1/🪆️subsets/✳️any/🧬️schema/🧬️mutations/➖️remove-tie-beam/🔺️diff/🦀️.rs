//! ➖️ `remove-tie-beam` diff: removes the addressed tie_beam row.

use crate::remove_tie_beam::RemoveTieBeam;
use semio_s_artifact_norm_en1998_diff::{En1998TieBeamDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveTieBeam, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.tie_beams.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tie_beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { tie_beams: En1998TieBeamDelta::removal(&base.tie_beams, index), ..Default::default() })
}
