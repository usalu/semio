//! 📍️ `change-tie-beam-relative-displacement` diff: patches `relative-displacement` of the addressed tie_beam row.

use crate::change_tie_beam_relative_displacement::ChangeTieBeamRelativeDisplacement;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TieBeamDelta, En1998TieBeamPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTieBeamRelativeDisplacement, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_relative_displacement.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.tie_beams.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("TieBeam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.relative_displacement == payload.new_relative_displacement {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tie_beam already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { tie_beams: En1998TieBeamDelta::modification(&payload.id, En1998TieBeamPatch { relative_displacement: Some(payload.new_relative_displacement), ..Default::default() }), ..Default::default() })
}
