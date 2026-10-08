//! 🔺️ Diff constructor for `CreateRailing`: one created railing entry. The storey and the material must exist, the path needs at
//! least two finite points that are not all the same, height and post spacing are positive lengths, the rail and post sections, the baluster row and the infill are sound. Posts, rails and extent are inferred.

use super::super::elements;
use super::CreateRailing;
use crate::{railing_construction_problem, Entry, ModelDiff, ModelSnapshot, Point2};
use protocol::{MutationOutcome, OutcomeCode};

fn traceable(path: &[Point2]) -> bool {
    path.len() >= 2 && path.iter().all(|point| point.x.is_finite() && point.y.is_finite()) && path.windows(2).any(|pair| pair[0] != pair[1])
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

pub fn diff(payload: &CreateRailing, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let railing = &payload.railing;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&railing.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", railing.storey), ["railing", "storey"]);
    }
    if !base.materials.contains_key(&railing.material) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", railing.material), ["railing", "material"]);
    }
    if !traceable(&railing.path) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing path needs at least two distinct finite points.", ["railing", "path"]);
    }
    if !positive(railing.height) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing height must be a positive length.", ["railing", "height"]);
    }
    if !positive(railing.post_spacing) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing post spacing must be a positive length.", ["railing", "post_spacing"]);
    }
    if !railing.base_offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing base offset must be a finite length.", ["railing", "base_offset"]);
    }
    if let Some((field, message)) = railing_construction_problem(railing) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["railing", field]);
    }
    MutationOutcome::new(ModelDiff::railings(payload.id.clone(), Entry::Created(railing.clone())))
}
