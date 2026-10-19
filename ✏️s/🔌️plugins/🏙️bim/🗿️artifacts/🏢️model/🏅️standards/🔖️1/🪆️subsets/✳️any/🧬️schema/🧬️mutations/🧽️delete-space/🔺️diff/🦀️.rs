//! 🔺️ Diff constructor for `DeleteSpace`: the space leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the space leaves.

use crate::standards::v1::subsets::any::schema::mutations::delete_space::DeleteSpace;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSpace, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.spaces.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Space \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Space", Some(&payload.id))
}
