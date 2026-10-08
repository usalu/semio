//! 🔺️ Diff constructor for `SetBeam`: a sparse beam patch naming only the fields that really change. The beam type must exist and
//! the resulting beam must keep its length. No elevation is written: it is inferred.

use super::SetBeam;
use crate::{BeamPatch, Entry, ModelDiff, ModelSnapshot, Point2};
use protocol::{MutationOutcome, OutcomeCode};

fn changed<T: Clone + PartialEq>(value: &Option<T>, current: &T) -> Option<T> {
    value.as_ref().filter(|next| *next != current).cloned()
}

fn finite(point: &Point2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}

pub fn diff(payload: &SetBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(beam) = base.beams.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(kind) = &payload.beam_type {
        if !base.beam_types.contains_key(kind) {
            return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \"{kind}\" does not exist."), ["beam_type"]);
        }
    }
    if payload.start.as_ref().is_some_and(|point| !finite(point)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam start must be finite.", ["start"]);
    }
    if payload.end.as_ref().is_some_and(|point| !finite(point)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam end must be finite.", ["end"]);
    }
    if payload.top_offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam top offset must be finite.", ["top_offset"]);
    }
    let patch = BeamPatch {
        beam_type: changed(&payload.beam_type, &beam.beam_type),
        start: changed(&payload.start, &beam.start),
        end: changed(&payload.end, &beam.end),
        top_offset: changed(&payload.top_offset, &beam.top_offset),
        name: changed(&payload.name, &beam.name),
        ..Default::default()
    };
    if patch == BeamPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Beam \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if patch.start.unwrap_or(beam.start) == patch.end.unwrap_or(beam.end) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam must have length.", [if patch.end.is_some() { "end" } else { "start" }]);
    }
    MutationOutcome::new(ModelDiff::beams(payload.id.clone(), Entry::Patched(patch)))
}
