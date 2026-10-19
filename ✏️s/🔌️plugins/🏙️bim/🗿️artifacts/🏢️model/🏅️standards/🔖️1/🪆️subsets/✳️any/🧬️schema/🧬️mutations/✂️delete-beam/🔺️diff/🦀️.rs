//! 🔺️ Diff constructor for `DeleteBeam`: the beam leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the beam leaves.

use crate::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.beams.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Beam", Some(&payload.id))
}
