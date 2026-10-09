//! 🔺️ Diff constructor for `SetBeam`: a sparse beam patch naming only the fields that really change. The beam type must exist and both top offsets must be finite
//! (the end offset is assigned: none makes the beam level again). The axis is the business of `set-beam-axis`; no elevation is written, it is inferred.

use super::super::wall_geometry::offsets_flaw;
use super::SetBeam;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(beam) = base.beams.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(kind) = &payload.beam_type {
        if !base.beam_types.contains_key(kind) {
            return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \"{kind}\" does not exist."), ["beam_type"]);
        }
    }
    let end = payload.end_top_offset.as_ref().and_then(|assigned| assigned.value);
    if let Some(flaw) = offsets_flaw(payload.top_offset.unwrap_or(beam.top_offset), end) {
        return flaw.refuse();
    }
    let patch = payload.patch().minimal(beam);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Beam \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::beams(payload.id.clone(), Entry::Patched(patch)))
}
