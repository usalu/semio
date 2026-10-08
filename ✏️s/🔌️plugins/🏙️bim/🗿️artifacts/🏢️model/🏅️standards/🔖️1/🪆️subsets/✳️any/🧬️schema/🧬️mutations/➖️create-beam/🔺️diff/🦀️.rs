//! 🔺️ Diff constructor for `CreateBeam`: one created beam entry. The storey and the beam type must exist and the beam must have
//! length. The beam top lies the signed, finite `top_offset` above (positive) or below (negative) the storey top: no elevation is stored, it is inferred.

use super::super::elements;
use super::CreateBeam;
use crate::{Entry, ModelDiff, ModelSnapshot, Point2};
use protocol::{MutationOutcome, OutcomeCode};

fn finite(point: &Point2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}

pub fn diff(payload: &CreateBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let beam = &payload.beam;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&beam.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", beam.storey), ["beam", "storey"]);
    }
    if !base.beam_types.contains_key(&beam.beam_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \"{}\" does not exist.", beam.beam_type), ["beam", "beam_type"]);
    }
    if !finite(&beam.start) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam start must be finite.", ["beam", "start"]);
    }
    if !finite(&beam.end) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam end must be finite.", ["beam", "end"]);
    }
    if !beam.top_offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam top offset must be finite.", ["beam", "top_offset"]);
    }
    if beam.start == beam.end {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam must have length.", ["beam", "end"]);
    }
    MutationOutcome::new(ModelDiff::beams(payload.id.clone(), Entry::Created(beam.clone())))
}
