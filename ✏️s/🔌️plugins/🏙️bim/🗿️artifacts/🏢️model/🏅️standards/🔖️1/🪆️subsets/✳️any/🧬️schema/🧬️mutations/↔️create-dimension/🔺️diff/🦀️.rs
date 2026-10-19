//! 🔺️ Diff constructor for `CreateDimension`: one created dimension entry. The id is free in every collection. The storey and the style must exist, there are at least two anchors that name existing elements or finite points, the direction and offset are finite and a lock is a positive length.
//! Everything the dimension shows is inferred from the current geometry of what it names, never stored.

use crate::standards::v1::subsets::any::schema::mutations::create_dimension::CreateDimension;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateDimension, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::annotating::dimension_fault(base, &payload.dimension) {
        let path = fault.path(Some("dimension"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::dimensions(payload.id.clone(), Entry::Created(payload.dimension.clone())))
}
