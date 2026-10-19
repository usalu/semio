//! 🔺️ Diff constructor for `DeleteGridLine`: the grid line leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the grid line leaves.

use crate::standards::v1::subsets::any::schema::mutations::delete_grid_line::DeleteGridLine;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteGridLine, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.grids.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Grid line \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Grid line", Some(&payload.id))
}
