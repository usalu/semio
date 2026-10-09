//! 🔺️ Diff constructor for `SetRailing`: a sparse railing patch of exactly the provided fields. A free railing needs two distinct finite points as its path, a hosted
//! one (a stair, ramp or slab edge of the same building, an assigned null releases it) has no path of its own, so hosting a railing needs the empty path in the same payload and releasing it a new path;
//! height and post spacing are positive, the rail and post sections, the baluster row and the infill must be sound, the material must exist; providing only equal values is a no-op.
//! Whole-list ruling: the path is ONE polyline, the geometry of the railing, so a provided path replaces the path as one field.

use super::super::placement::{host_issue, refuse_stair};
use super::SetRailing;
use crate::{railing_construction_problem, Entry, ModelDiff, ModelSnapshot, Patch, Point2, RailingPatch};
use protocol::{MutationOutcome, OutcomeCode};

fn traceable(path: &[Point2]) -> bool {
    path.len() >= 2 && path.iter().all(|point| point.x.is_finite() && point.y.is_finite()) && path.windows(2).any(|pair| pair[0] != pair[1])
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn construction_issue(patch: &RailingPatch, railing: &crate::Railing) -> Option<(&'static str, &'static str)> {
    railing_construction_problem(&patch.write(railing))
}

pub fn diff(payload: &SetRailing, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(railing) = base.railings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Railing \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.height.is_some_and(|height| !positive(height)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing height must be a positive length.", ["height"]);
    }
    if payload.post_spacing.is_some_and(|spacing| !positive(spacing)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing post spacing must be a positive length.", ["post_spacing"]);
    }
    if payload.base_offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing base offset must be a finite length.", ["base_offset"]);
    }
    if let Some(material) = payload.material.as_ref().filter(|material| !base.materials.contains_key(*material)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{material}\" does not exist."), ["material"]);
    }
    let patch = payload.patch().minimal(railing);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Railing \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    let next = patch.write(railing);
    if next.host.is_none() && !traceable(&next.path) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing path needs at least two distinct finite points.", ["path"]);
    }
    if let Some((field, message)) = construction_issue(&patch, railing) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
    }
    if let Some(issue) = host_issue(base, &next) {
        return refuse_stair(issue, &[]);
    }
    MutationOutcome::new(ModelDiff::railings(payload.id.clone(), Entry::Patched(patch)))
}
