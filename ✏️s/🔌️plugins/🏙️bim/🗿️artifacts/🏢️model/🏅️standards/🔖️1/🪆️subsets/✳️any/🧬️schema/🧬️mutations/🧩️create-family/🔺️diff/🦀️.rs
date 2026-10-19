//! 🔺️ Diff constructor for `CreateFamily`: one created family entry. The id is free in every collection and the name is not blank. Parameters and solids come afterwards.

use crate::standards::v1::subsets::any::schema::mutations::create_family::CreateFamily;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateFamily, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::family_record_fault(&payload.family) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("family")));
    }
    MutationOutcome::new(ModelDiff::families(payload.id.clone(), Entry::Created(payload.family.clone())))
}
