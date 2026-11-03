//! ✏️ `change-tie-beam-foundation-b-id` diff: patches `foundation_b_id` of the addressed tie_beam row.

use crate::change_tie_beam_foundation_b_id::ChangeTieBeamFoundationBId;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TieBeamDelta, En1998TieBeamPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTieBeamFoundationBId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.tie_beams.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tie_beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.foundation_b_id == payload.new_foundation_b_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tie_beam already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { tie_beams: En1998TieBeamDelta::modification(&payload.id, En1998TieBeamPatch { foundation_b_id: Some(payload.new_foundation_b_id.clone()), ..Default::default() }), ..Default::default() })
}
