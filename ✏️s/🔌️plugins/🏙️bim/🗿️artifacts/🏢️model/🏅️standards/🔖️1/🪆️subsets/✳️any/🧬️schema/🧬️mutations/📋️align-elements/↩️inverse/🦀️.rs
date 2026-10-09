//! ↩️ Inverse of `AlignElements`: one `PlaceElements` carrying the exact base placement of every element the alignment moves, read from
//! the base, so no floating-point back translation can leave a residue. Empty when the alignment is refused or moves nothing.

use super::super::place_elements::PlaceElements;
use super::diff::changes;
use super::AlignElements;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &AlignElements, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match changes(payload, base) {
        Ok(change) if !change.before.is_empty() => vec![ModelMutation::PlaceElements(PlaceElements { placements: change.before })],
        _ => Vec::new(),
    }
}
