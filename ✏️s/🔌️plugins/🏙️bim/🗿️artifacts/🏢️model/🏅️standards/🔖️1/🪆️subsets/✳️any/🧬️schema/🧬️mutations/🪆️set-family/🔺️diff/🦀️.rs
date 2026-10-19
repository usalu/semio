//! 🔺️ Diff constructor for `SetFamily`: a sparse family patch of exactly the provided fields that differ. The name stays non-blank; a profile family cannot leave its category while a type uses it as a profile.
//! Providing only equal values, or no field, is a no-op.

use crate::standards::v1::subsets::any::schema::mutations::set_family::SetFamily;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::FamilyCategory;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetFamily, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.families.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::family_record_fault(&next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Family \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if record.category == FamilyCategory::Profile && next.category != FamilyCategory::Profile {
        if let Some(noun) = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::profile_user(base, &payload.id) {
            return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Family \"{}\" is still the profile of {noun} and must stay a profile.", payload.id), [payload.id.clone()]);
        }
    }
    if record.category != FamilyCategory::Profile && next.category == FamilyCategory::Profile {
        if let Some(noun) = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::component_user(base, &payload.id) {
            return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Family \"{}\" is still placed by {noun} and cannot become a profile.", payload.id), [payload.id.clone()]);
        }
    }
    MutationOutcome::new(ModelDiff::families(payload.id.clone(), Entry::Patched(change)))
}
