//! 🔺️ Diff constructor for `MoveElements`: one sparse patch per moved element holding exactly its placement fields (axis, position,
//! start/end, boundary and holes, footprint, path, outline). Openings are never patched, they follow their host by inference.
//! Refused: an empty selection, a non-finite vector, unknown ids and elements without a placement; a zero vector is `mutation.no-op`.

use super::super::elements::{self, Change, Refusal};
use super::MoveElements;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

fn changes(payload: &MoveElements, base: &ModelSnapshot) -> Result<Change, Refusal> {
    let vector = payload.vector;
    if !(vector.x.is_finite() && vector.y.is_finite()) {
        return Err(Refusal::new(OutcomeCode::Invariant, "A translation must be a finite vector.", ["vector"]));
    }
    let change = elements::changes(base, &payload.ids, |placement| placement.translated(vector))?;
    if vector.x == 0.0 && vector.y == 0.0 {
        return Err(Refusal::new(OutcomeCode::NoOp, "A zero vector moves nothing.", ["vector"]));
    }
    Ok(change)
}

pub fn diff(payload: &MoveElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    elements::placed(changes(payload, base), "ids")
}
