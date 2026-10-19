//! 🔺️ Diff constructor for `SetMepElement`: a sparse MEP element patch of exactly the provided fields that differ. The element that results must break none of the create rules. Whole-list ruling: the path
//! is ONE centre line, so a provided path replaces the path as one field. Providing only equal values, or no field, is a no-op.

use crate::standards::v1::subsets::any::schema::mutations::set_mep_element::SetMepElement;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetMepElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.mep_elements.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("MEP element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::component_rules::mep_fault(base, &next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("MEP element \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::mep_elements(payload.id.clone(), Entry::Patched(change)))
}
