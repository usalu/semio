//! 🔺️ Sparse declarative delete-option-group diff.
use super::DeleteOptionGroup;
use crate::*;
use super::super::{elements, option_rules};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteOptionGroup, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.option_groups.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Record is missing.", [payload.id.clone()]); }
    if base.design_options.values().any(|option| option.group == payload.id) { return MutationOutcome::refuse(OutcomeCode::InUse, "Option group is in use.", [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::option_groups(payload.id.clone(), Entry::Deleted))
}
