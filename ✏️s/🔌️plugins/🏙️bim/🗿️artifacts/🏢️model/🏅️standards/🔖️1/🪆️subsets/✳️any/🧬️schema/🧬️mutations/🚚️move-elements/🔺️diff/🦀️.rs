//! 🔺️ Diff constructor for `MoveElements`: one sparse patch per moved element holding exactly its placement fields (axis, position,
//! start/end, boundary and holes, footprint, path, outline). Openings are never patched, they follow their host by inference.
//! Refused: an empty selection, a non-finite vector, unknown ids and elements without a placement; a zero vector is `mutation.no-op`.

use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::{Change, Refusal};
use crate::standards::v1::subsets::any::schema::mutations::move_elements::MoveElements;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

fn changes(payload: &MoveElements, base: &ModelSnapshot) -> Result<Change, Refusal> {
    let vector = payload.vector;
    if !(vector.x.is_finite() && vector.y.is_finite()) {
        return Err(Refusal::new(OutcomeCode::Invariant, "A translation must be a finite vector.", ["vector"]));
    }
    let change = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::changes(base, &payload.ids, |placement| placement.translated(vector))?;
    if vector.x == 0.0 && vector.y == 0.0 {
        return Err(Refusal::new(OutcomeCode::NoOp, "A zero vector moves nothing.", ["vector"]));
    }
    Ok(change)
}

pub fn diff(payload: &MoveElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::placed(changes(payload, base), "ids")
}
