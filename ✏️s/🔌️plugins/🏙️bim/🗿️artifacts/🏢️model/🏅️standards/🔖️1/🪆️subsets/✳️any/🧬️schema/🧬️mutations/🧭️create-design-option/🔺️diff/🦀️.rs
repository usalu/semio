//! 🔺️ Sparse declarative create-design-option diff.
use super::CreateDesignOption;
use crate::*;
use super::super::{elements, option_rules};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &CreateDesignOption, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if payload.id.trim().is_empty() { return MutationOutcome::refuse(OutcomeCode::Invariant, "Id is empty.", [payload.id.clone()]); }
    if elements::taken(base, &payload.id).is_some() { return MutationOutcome::refuse(OutcomeCode::DuplicateId, "Id already exists.", [payload.id.clone()]); }
    if let Some(problem) = option_rules::option_problem(base, &payload.id, &payload.design_option) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::design_options(payload.id.clone(), Entry::Created(payload.design_option.clone())))
}
