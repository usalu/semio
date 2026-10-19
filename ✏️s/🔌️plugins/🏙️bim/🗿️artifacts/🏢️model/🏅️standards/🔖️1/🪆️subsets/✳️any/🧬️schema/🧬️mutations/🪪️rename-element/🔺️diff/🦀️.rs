//! 🔺️ Diff constructor for `RenameElement`: a one-field patch of the name (a grid line's label) in whichever collection holds the id.

use crate::standards::v1::subsets::any::schema::mutations::rename_element::RenameElement;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RenameElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some((current, diff)) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::rename(base, &payload.id, &payload.name) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if current == payload.name {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Element \"{}\" is already named \"{}\".", payload.id, payload.name), [payload.id.clone()]);
    }
    MutationOutcome::new(diff)
}
