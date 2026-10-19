//! 🔺️ Diff constructor for `RotateElements`: every placed element turns counter-clockwise about the pivot, one sparse patch per element
//! with exactly its placement fields; rotation fields (column rotation, stair direction, slab fall, roof ridge) turn by the same angle.
//! Openings follow their host by inference. Refused: an empty selection, a non-finite pivot or angle, unknown ids and elements
//! without a placement; a zero angle is `mutation.no-op`.

use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::{Change, Refusal};
use crate::standards::v1::subsets::any::schema::mutations::rotate_elements::RotateElements;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

fn changes(payload: &RotateElements, base: &ModelSnapshot) -> Result<Change, Refusal> {
    let (pivot, angle) = (payload.pivot, payload.angle);
    if !(pivot.x.is_finite() && pivot.y.is_finite()) {
        return Err(Refusal::new(OutcomeCode::Invariant, "A rotation pivot must be a finite point.", ["pivot"]));
    }
    if !angle.is_finite() {
        return Err(Refusal::new(OutcomeCode::Invariant, "A rotation angle must be finite.", ["angle"]));
    }
    let change = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::changes(base, &payload.ids, |placement| placement.rotated(pivot, angle))?;
    if angle == 0.0 {
        return Err(Refusal::new(OutcomeCode::NoOp, "A zero angle turns nothing.", ["angle"]));
    }
    Ok(change)
}

pub fn diff(payload: &RotateElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::placed(changes(payload, base), "ids")
}
