//! 🔺️ Sparse declarative create-support diff; no derived state is evaluated.
use super::CreateSupport;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &CreateSupport, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
if let Some(noun) = super::super::elements::taken(base, &payload.id) { return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} already exists."), [payload.id.clone()]); }
if let Some(problem) = crate::support_problem(base, &payload.support) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::supports(payload.id.clone(), Entry::Created(payload.support.clone())))
}
