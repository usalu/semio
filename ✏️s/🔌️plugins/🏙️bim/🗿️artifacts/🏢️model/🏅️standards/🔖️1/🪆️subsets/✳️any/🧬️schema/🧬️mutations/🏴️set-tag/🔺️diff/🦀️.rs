//! 🔺️ Diff constructor for `SetTag`: a sparse tag patch of exactly the provided fields that differ. The record that results must break none of the create rules: the storey and the style must exist, the element is one that can be tagged and the offset is finite.
//! Providing only equal values, or no field, is a no-op.

use crate::standards::v1::subsets::any::schema::mutations::set_tag::SetTag;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetTag, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.tags.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Tag \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::annotating::tag_fault(base, &next) {
        let path = fault.path(None);
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Tag \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::tags(payload.id.clone(), Entry::Patched(change)))
}
