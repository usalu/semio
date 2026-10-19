//! 🔺️ Diff constructor for `DeleteSheetRevision`: the revision leaves in one sparse diff together with its properties and classifications (see the shared cascade). The outcome carries a cascade note when more than the
//! revision leaves.

use crate::standards::v1::subsets::any::schema::mutations::delete_sheet_revision::DeleteSheetRevision;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSheetRevision, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.sheet_revisions.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Revision \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Revision", Some(&payload.id))
}
