//! 🔺️ Diff constructor for `SetRamp`: a sparse ramp patch of exactly the provided fields that differ from the base. The patched ramp must hold every
//! ramp invariant again (a path of two distinct finite vertices, positive width, thickness and slope limit, finite non-negative landings, a material that
//! exists, a top storey of the same building); providing only equal values is a no-op. Whole-list ruling: the path is ONE centre line, the geometry of the
//! ramp, so a provided path replaces the path as one field. The slope the ramp ends up with is inferred and checked by diagnostics, never refused here.

use super::super::placement::{ramp_issue, refuse_stair};
use super::SetRamp;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetRamp, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(ramp) = base.ramps.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ramp \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(ramp);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Ramp \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(issue) = ramp_issue(base, &patch.write(ramp)) {
        return refuse_stair(issue, &[]);
    }
    MutationOutcome::new(ModelDiff::ramps(payload.id.clone(), Entry::Patched(patch)))
}
