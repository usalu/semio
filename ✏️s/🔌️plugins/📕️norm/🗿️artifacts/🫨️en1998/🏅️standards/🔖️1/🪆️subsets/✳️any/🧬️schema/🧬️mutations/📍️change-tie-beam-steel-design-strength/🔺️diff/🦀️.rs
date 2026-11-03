//! 📍️ `change-tie-beam-steel-design-strength` diff: patches `steel-design-strength` of the addressed tie_beam row.

use crate::change_tie_beam_steel_design_strength::ChangeTieBeamSteelDesignStrength;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TieBeamDelta, En1998TieBeamPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTieBeamSteelDesignStrength, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_steel_design_strength.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.tie_beams.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("TieBeam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.steel_design_strength == payload.new_steel_design_strength {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tie_beam already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { tie_beams: En1998TieBeamDelta::modification(&payload.id, En1998TieBeamPatch { steel_design_strength: Some(payload.new_steel_design_strength), ..Default::default() }), ..Default::default() })
}
