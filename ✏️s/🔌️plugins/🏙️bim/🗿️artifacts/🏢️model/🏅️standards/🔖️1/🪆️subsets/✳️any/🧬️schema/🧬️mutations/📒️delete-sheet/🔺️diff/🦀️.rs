//! 🔺️ Diff constructor for `DeleteSheet`: the sheet leaves in one sparse diff together with its viewports, its revision rows, its properties and classifications (see the shared cascade). The outcome carries a cascade note when more than the
//! sheet leaves.

use crate::standards::v1::subsets::any::schema::mutations::delete_sheet::DeleteSheet;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSheet, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.sheets.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Sheet \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Sheet", Some(&payload.id))
}
