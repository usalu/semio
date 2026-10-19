//! 🔺️ Diff constructor for `DeleteWall`: the wall and the openings it hosts leave in one sparse diff together with everything that depends on them
//! (see the shared cascade), including the properties and classifications of every removed element. A storey that a
//! surviving element's top constraint still points at cannot cascade and is refused as `mutation.target-referenced`.

use crate::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.walls.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Wall", Some(&payload.id))
}
