//! 🔺️ Diff constructor for `SetDimension`: a sparse dimension patch of exactly the provided fields that differ. The record that results must break none of the create rules: the storey and the style must exist, there are at least two anchors that name existing elements or finite points, the direction and offset are finite and a lock is a positive length.
//! Providing only equal values, or no field, is a no-op.

use crate::standards::v1::subsets::any::schema::mutations::set_dimension::SetDimension;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetDimension, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.dimensions.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Dimension \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::annotating::dimension_fault(base, &next) {
        let path = fault.path(None);
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Dimension \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::dimensions(payload.id.clone(), Entry::Patched(change)))
}
