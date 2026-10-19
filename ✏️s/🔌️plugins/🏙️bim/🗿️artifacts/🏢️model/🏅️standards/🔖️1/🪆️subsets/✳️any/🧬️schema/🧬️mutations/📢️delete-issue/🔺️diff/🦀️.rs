//! 🔺️ Diff constructor for `DeleteIssue`: the issue leaves in one sparse diff together with its comments, its properties and classifications (see the shared cascade).

use crate::standards::v1::subsets::any::schema::mutations::delete_issue::DeleteIssue;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteIssue, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.issues.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Issue \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Issue", Some(&payload.id))
}
