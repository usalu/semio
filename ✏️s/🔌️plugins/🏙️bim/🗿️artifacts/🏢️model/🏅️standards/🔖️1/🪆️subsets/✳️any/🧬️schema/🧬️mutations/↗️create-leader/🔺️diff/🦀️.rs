//! 🔺️ Diff constructor for `CreateLeader`: one created leader entry. The id is free in every collection. The storey and the style must exist, the anchor names an existing element or a finite point, the offset is finite and the text is not blank.
//! Everything the leader shows is inferred from the current geometry of what it names, never stored.

use crate::standards::v1::subsets::any::schema::mutations::create_leader::CreateLeader;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateLeader, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::annotating::leader_fault(base, &payload.leader) {
        let path = fault.path(Some("leader"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::leaders(payload.id.clone(), Entry::Created(payload.leader.clone())))
}
