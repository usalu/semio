//! 🔺️ Diff constructor for `RemoveFamilyParameter`: the parameter record leaves. Refused while the formula of another parameter or a solid of the family refers to it (computed from the authored
//! text of the formulas only).

use super::super::family_rules;
use super::RemoveFamilyParameter;
use crate::standards::v1::subsets::any::schema::inferences::families::formula;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveFamilyParameter, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.families.contains_key(&payload.family) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \"{}\" does not exist.", payload.family), ["family"]);
    }
    let key = formula::parameter_id(&payload.family, &payload.name);
    if !base.family_parameters.contains_key(&key) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Parameter \"{}\" of family \"{}\" does not exist.", payload.name, payload.family), [key]);
    }
    if let Some(user) = family_rules::parameter_user(base, &payload.family, &payload.name) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Parameter \"{}\" is still used by {user}.", payload.name), [key]);
    }
    MutationOutcome::new(ModelDiff::family_parameters(key, Entry::Deleted))
}
