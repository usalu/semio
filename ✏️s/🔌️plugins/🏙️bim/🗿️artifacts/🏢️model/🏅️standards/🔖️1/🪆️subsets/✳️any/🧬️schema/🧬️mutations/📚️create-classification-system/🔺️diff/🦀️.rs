//! 🔺️ Diff constructor for `CreateClassificationSystem`: one created system entry. The id must be free in every collection (and not `entry`, the tag of a diff entry, which would collide with the system id used as a key of
//! an element's classifications), the system needs a name and an entry table with unique codes whose parents are rows of the table and form a forest.

use crate::standards::v1::subsets::any::schema::mutations::create_classification_system::CreateClassificationSystem;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{entries_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateClassificationSystem, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if payload.id == "entry" {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "The id \"entry\" is reserved: it tags the entries of a diff.", ["id"]);
    }
    let system = &payload.system;
    if system.name.trim().is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A classification system needs a name.", ["system", "name"]);
    }
    if let Some(message) = entries_problem(&system.entries) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["system", "entries"]);
    }
    MutationOutcome::new(ModelDiff::classification_systems(payload.id.clone(), Entry::Created(system.clone())))
}
