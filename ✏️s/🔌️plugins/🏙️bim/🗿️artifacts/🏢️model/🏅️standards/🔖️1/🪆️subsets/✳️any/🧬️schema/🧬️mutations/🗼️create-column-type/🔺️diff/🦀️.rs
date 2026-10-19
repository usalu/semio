//! 🔺️ Diff constructor for `CreateColumnType`: one created column type entry. The id must be free, the material must exist and the profile must be a valid cross-section.

use crate::standards::v1::subsets::any::schema::mutations::create_column_type::CreateColumnType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{profile_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateColumnType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let record = &payload.column_type;
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.materials.contains_key(&record.material) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", record.material), ["column_type", "material"]);
    }
    if let Some(problem) = profile_problem(&record.profile) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, problem, ["column_type", "profile"]);
    }
    MutationOutcome::new(ModelDiff::column_types(payload.id.clone(), Entry::Created(record.clone())))
}
