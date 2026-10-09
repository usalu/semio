//! 🔺️ Diff constructor for `SetWallSweep`: a sparse wall sweep patch of exactly the provided fields that differ. The sweep that results must follow the create rules (host and material exist, profile with positive
//! dimensions, height not negative, inset leaving part of the profile showing); providing only equal values, or no field, is a no-op.

use super::super::wall_depth::sweep_flaw;
use super::SetWallSweep;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallSweep, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(sweep) = base.wall_sweeps.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall sweep \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let change = payload.patch().minimal(sweep);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall sweep \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = sweep_flaw(base, &change.write(sweep)) {
        return flaw.refuse();
    }
    MutationOutcome::new(ModelDiff::wall_sweeps(payload.id.clone(), Entry::Patched(change)))
}
