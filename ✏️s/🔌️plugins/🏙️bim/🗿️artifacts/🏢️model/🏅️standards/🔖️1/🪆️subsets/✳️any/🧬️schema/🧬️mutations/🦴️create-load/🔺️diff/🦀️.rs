//! 🔺️ Sparse declarative create-load diff; no derived state is evaluated.
use super::CreateLoad;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &CreateLoad, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
if let Some(noun) = super::super::elements::taken(base, &payload.id) { return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} already exists."), [payload.id.clone()]); }
if let Some(problem) = crate::load_problem(base, &payload.load) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::loads(payload.id.clone(), Entry::Created(payload.load.clone())))
}
