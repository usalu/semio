//! 🔺️ Sparse declarative create-load-case diff; no derived state is evaluated.
use super::CreateLoadCase;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &CreateLoadCase, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
if let Some(noun) = super::super::elements::taken(base, &payload.id) { return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} already exists."), [payload.id.clone()]); }
if let Some(problem) = crate::load_case_problem(base, &payload.load_case) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::load_cases(payload.id.clone(), Entry::Created(payload.load_case.clone())))
}
