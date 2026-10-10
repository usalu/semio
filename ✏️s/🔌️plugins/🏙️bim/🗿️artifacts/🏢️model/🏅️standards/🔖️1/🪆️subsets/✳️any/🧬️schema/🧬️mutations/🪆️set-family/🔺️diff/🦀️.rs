//! 🔺️ Diff constructor for `SetFamily`: a sparse family patch of exactly the provided fields that differ. The name stays non-blank; a profile family cannot leave its category while a type uses it as a profile.
//! Providing only equal values, or no field, is a no-op.

use super::super::family_rules;
use super::SetFamily;
use crate::{Entry, FamilyCategory, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetFamily, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.families.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = family_rules::family_record_fault(&next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Family \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if record.category == FamilyCategory::Profile && next.category != FamilyCategory::Profile {
        if let Some(noun) = family_rules::profile_user(base, &payload.id) {
            return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Family \"{}\" is still the profile of {noun} and must stay a profile.", payload.id), [payload.id.clone()]);
        }
    }
    if record.category != FamilyCategory::Profile && next.category == FamilyCategory::Profile {
        if let Some(noun) = family_rules::component_user(base, &payload.id) {
            return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Family \"{}\" is still placed by {noun} and cannot become a profile.", payload.id), [payload.id.clone()]);
        }
    }
    MutationOutcome::new(ModelDiff::families(payload.id.clone(), Entry::Patched(change)))
}
