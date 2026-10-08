//! 🔺️ Diff constructor for `SetRoofFootprint`: a one-field roof patch. The footprint follows the create rules (counter-clockwise, no
//! self-intersection, at least three vertices); a footprint equal to the current one is a no-op. Shape and solid follow by inference.
//! Whole-list ruling: the footprint is ONE closed loop, the geometry of the roof, so a provided footprint replaces the footprint
//! as one field.

use super::super::horizontal_rules::loop_fault;
use super::SetRoofFootprint;
use crate::{Entry, ModelDiff, ModelSnapshot, RoofPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetRoofFootprint, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(roof) = base.roofs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(fault) = loop_fault(&payload.footprint) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["footprint"]);
    }
    if roof.footprint == payload.footprint {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Roof \"{}\" already has this footprint.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::roofs(payload.id.clone(), Entry::Patched(RoofPatch { footprint: Some(payload.footprint.clone()), ..Default::default() })))
}
