//! 🔺️ Sparse declarative set-option-group diff.
use crate::standards::v1::subsets::any::schema::mutations::set_option_group::SetOptionGroup;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff, Patch}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetOptionGroup, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.option_groups.get(&payload.id) else { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Record is missing.", [payload.id.clone()]); };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Values are unchanged.", [payload.id.clone()]); }
    let value = patch.write(record);
    if (&value).name.trim().is_empty() { return MutationOutcome::refuse(OutcomeCode::Invariant, "Name is empty.", [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::option_groups(payload.id.clone(), Entry::Patched(patch)))
}
