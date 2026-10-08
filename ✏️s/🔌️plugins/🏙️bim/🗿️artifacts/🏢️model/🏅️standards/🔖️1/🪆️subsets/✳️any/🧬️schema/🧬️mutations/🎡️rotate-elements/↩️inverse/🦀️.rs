//! ↩️ Inverse of `RotateElements`: one `PlaceElements` carrying the exact base placement of every element the rotation changes (rotation
//! fields included), read from the base through the shared element vocabulary, so no floating-point residue of a back rotation remains.
//! Empty when the pivot or angle is not finite, the angle is zero or no placement changes.

use super::super::elements;
use super::super::place_elements::PlaceElements;
use super::RotateElements;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RotateElements, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let (pivot, angle) = (payload.pivot, payload.angle);
    if !(pivot.x.is_finite() && pivot.y.is_finite() && angle.is_finite()) || angle == 0.0 {
        return Vec::new();
    }
    match elements::changes(base, &payload.ids, |placement| placement.rotated(pivot, angle)) {
        Ok(change) if !change.before.is_empty() => vec![ModelMutation::PlaceElements(PlaceElements { placements: change.before })],
        _ => Vec::new(),
    }
}
