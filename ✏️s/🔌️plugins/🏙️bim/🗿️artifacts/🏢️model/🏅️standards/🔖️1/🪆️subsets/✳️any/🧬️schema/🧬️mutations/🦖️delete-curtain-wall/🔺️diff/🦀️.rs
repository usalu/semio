//! 🔺️ Diff constructor for `DeleteCurtainWall`: the curtain wall and the openings it hosts leave in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the curtain wall leaves.

use crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall::DeleteCurtainWall;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCurtainWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.curtain_walls.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Curtain wall", Some(&payload.id))
}
