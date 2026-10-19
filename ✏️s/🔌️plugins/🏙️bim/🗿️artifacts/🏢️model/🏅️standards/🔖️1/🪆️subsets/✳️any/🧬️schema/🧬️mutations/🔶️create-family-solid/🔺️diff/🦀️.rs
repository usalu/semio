//! 🔺️ Diff constructor for `CreateFamilySolid`: one created solid entry. The id is free in every collection, the family exists, the name is not blank, every formula slot parses and uses only
//! existing parameters of the family, and a polygon has three points and a sweep path two. The mesh, the volume and the issues of the solid are inferred, never stored.

use crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateFamilySolid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::solid_fault(base, &payload.solid) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("solid")));
    }
    MutationOutcome::new(ModelDiff::family_solids(payload.id.clone(), Entry::Created(payload.solid.clone())))
}
