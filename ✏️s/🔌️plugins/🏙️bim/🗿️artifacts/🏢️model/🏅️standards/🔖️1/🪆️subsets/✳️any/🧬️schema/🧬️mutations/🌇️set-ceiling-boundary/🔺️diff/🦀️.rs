//! 🔺️ Diff constructor for `SetCeilingBoundary`: the boundary and the holes are validated together as one outline and patched as far
//! as they differ. Both loops follow the create rules (counter-clockwise, no self-intersection, holes strictly inside); an outline
//! equal to the current one is a no-op.
//! Whole-list ruling: the boundary and the holes are ONE planar region that only validates together (every hole strictly inside
//! the boundary); holes have no identity, so the region replaces as the boundary and holes it names.

use super::super::horizontal_rules::{holes_fault, loop_fault};
use super::SetCeilingBoundary;
use crate::{CeilingPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCeilingBoundary, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(ceiling) = base.ceilings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(fault) = loop_fault(&payload.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["boundary"]);
    }
    if let Some(fault) = holes_fault(&payload.boundary, &payload.holes) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["holes"]);
    }
    let patch = CeilingPatch {
        boundary: (payload.boundary != ceiling.boundary).then(|| payload.boundary.clone()),
        holes: (payload.holes != ceiling.holes).then(|| payload.holes.clone()),
        ..Default::default()
    };
    if patch == CeilingPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Ceiling \"{}\" already has this outline.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::ceilings(payload.id.clone(), Entry::Patched(patch)))
}
