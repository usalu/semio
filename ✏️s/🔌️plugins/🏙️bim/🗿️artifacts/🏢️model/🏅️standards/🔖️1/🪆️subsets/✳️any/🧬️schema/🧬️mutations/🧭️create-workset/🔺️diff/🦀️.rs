//! 🔺️ Sparse declarative create-workset diff.
use super::CreateWorkset;
use crate::*;
use super::super::{elements, option_rules};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &CreateWorkset, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if payload.id.trim().is_empty() { return MutationOutcome::refuse(OutcomeCode::Invariant, "Id is empty.", [payload.id.clone()]); }
    if elements::taken(base, &payload.id).is_some() { return MutationOutcome::refuse(OutcomeCode::DuplicateId, "Id already exists.", [payload.id.clone()]); }
    if (&payload.workset).name.trim().is_empty() { return MutationOutcome::refuse(OutcomeCode::Invariant, "Name is empty.", [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::worksets(payload.id.clone(), Entry::Created(payload.workset.clone())))
}
