//! ↩️ Inverse of `AlignElements`: one `PlaceElements` carrying the exact base placement of every element the alignment moves, read from
//! the base, so no floating-point back translation can leave a residue. Empty when the alignment is refused or moves nothing.

use crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements;
use crate::standards::v1::subsets::any::schema::mutations::align_elements::diff::changes;
use crate::standards::v1::subsets::any::schema::mutations::align_elements::AlignElements;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &AlignElements, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    match changes(payload, base) {
        Ok(change) if !change.before.is_empty() => vec![Op::from(PlaceElements { placements: change.before })],
        _ => Vec::new(),
    }
}
