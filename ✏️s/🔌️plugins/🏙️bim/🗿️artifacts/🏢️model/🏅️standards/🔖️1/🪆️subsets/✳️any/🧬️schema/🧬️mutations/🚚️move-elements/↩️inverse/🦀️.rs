//! ↩️ Inverse of `MoveElements`: one `PlaceElements` carrying the exact base placement of every element the move changes, read from the
//! base through the shared element vocabulary, so no floating-point rounding of a back translation can leave a residue. Empty when the
//! vector is not a finite non-zero translation or no placement changes.

use super::super::elements;
use super::super::place_elements::PlaceElements;
use super::MoveElements;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &MoveElements, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let vector = payload.vector;
    if !(vector.x.is_finite() && vector.y.is_finite()) || (vector.x == 0.0 && vector.y == 0.0) {
        return Vec::new();
    }
    match elements::changes(base, &payload.ids, |placement| placement.translated(vector)) {
        Ok(change) if !change.before.is_empty() => vec![ModelMutation::PlaceElements(PlaceElements { placements: change.before })],
        _ => Vec::new(),
    }
}
