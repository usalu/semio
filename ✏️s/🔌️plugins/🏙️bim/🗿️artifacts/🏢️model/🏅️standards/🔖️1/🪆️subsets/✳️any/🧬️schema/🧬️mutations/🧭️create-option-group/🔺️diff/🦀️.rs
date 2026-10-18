//! 🔺️ Sparse declarative create-option-group diff.
use crate::standards::v1::subsets::any::schema::mutations::create_option_group::CreateOptionGroup;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff}, snapshot::ModelSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::elements;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &CreateOptionGroup, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if payload.id.trim().is_empty() { return MutationOutcome::refuse(OutcomeCode::Invariant, "Id is empty.", [payload.id.clone()]); }
    if elements::taken(base, &payload.id).is_some() { return MutationOutcome::refuse(OutcomeCode::DuplicateId, "Id already exists.", [payload.id.clone()]); }
    if (&payload.option_group).name.trim().is_empty() { return MutationOutcome::refuse(OutcomeCode::Invariant, "Name is empty.", [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::option_groups(payload.id.clone(), Entry::Created(payload.option_group.clone())))
}
