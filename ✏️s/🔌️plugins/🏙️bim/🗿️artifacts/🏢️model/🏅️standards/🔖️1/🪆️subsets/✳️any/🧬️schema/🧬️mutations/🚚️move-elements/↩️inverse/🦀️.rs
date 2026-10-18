//! ↩️ Inverse of `MoveElements`: one `PlaceElements` carrying the exact base placement of every element the move changes, read from the
//! base through the shared element vocabulary, so no floating-point rounding of a back translation can leave a residue. Empty when the
//! vector is not a finite non-zero translation or no placement changes.

use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements;
use crate::standards::v1::subsets::any::schema::mutations::move_elements::MoveElements;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &MoveElements, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    let vector = payload.vector;
    if !(vector.x.is_finite() && vector.y.is_finite()) || (vector.x == 0.0 && vector.y == 0.0) {
        return Vec::new();
    }
    match elements::changes(base, &payload.ids, |placement| placement.translated(vector)) {
        Ok(change) if !change.before.is_empty() => vec![Op::from(PlaceElements { placements: change.before })],
        _ => Vec::new(),
    }
}
