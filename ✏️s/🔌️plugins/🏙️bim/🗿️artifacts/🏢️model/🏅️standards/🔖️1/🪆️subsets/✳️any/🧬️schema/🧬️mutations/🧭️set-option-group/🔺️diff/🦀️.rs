//! 🔺️ Sparse declarative set-option-group diff.
use super::SetOptionGroup;
use crate::*;
use super::super::{elements, option_rules};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetOptionGroup, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.option_groups.get(&payload.id) else { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Record is missing.", [payload.id.clone()]); };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Values are unchanged.", [payload.id.clone()]); }
    let value = patch.write(record);
    if (&value).name.trim().is_empty() { return MutationOutcome::refuse(OutcomeCode::Invariant, "Name is empty.", [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::option_groups(payload.id.clone(), Entry::Patched(patch)))
}
