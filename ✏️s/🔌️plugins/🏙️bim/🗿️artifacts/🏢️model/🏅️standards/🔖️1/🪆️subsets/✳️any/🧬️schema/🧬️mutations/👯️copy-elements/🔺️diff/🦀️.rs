//! 🔺️ Diff constructor for `CopyElements`: one created record per copied element, the image of its source translated by the vector, with
//! the openings of a copied wall or curtain wall hosted on its copy and the property and classification entries of every source. The ids
//! are minted from the prefix; a space takes the next free number of its storey and a grid line the next free label of its building.
//! Refused: an empty selection, unknown ids, elements without a placement, an opening without its host, a blank prefix, a minted id that
//! is taken and a vector that is not finite.

use super::super::modify::{self, Map};
use super::CopyElements;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CopyElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let map = Map::Translate(payload.vector);
    if !map.finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A translation must be a finite vector.", ["vector"]);
    }
    match modify::duplicate(base, &payload.ids, &payload.prefix, &[map]) {
        Err(refusal) => refusal.outcome(),
        Ok(built) => {
            let carried = built.carried;
            let outcome = MutationOutcome::new(built.diff);
            if carried == 0 {
                outcome
            } else {
                outcome.info(OutcomeCode::Cascade, format!("{carried} opening(s) were copied with their hosts."))
            }
        }
    }
}
