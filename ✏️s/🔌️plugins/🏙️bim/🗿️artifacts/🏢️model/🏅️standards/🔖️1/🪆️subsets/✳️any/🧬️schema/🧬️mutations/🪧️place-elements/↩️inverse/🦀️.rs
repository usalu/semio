//! ↩️ Inverse of `PlaceElements`: another `PlaceElements` carrying the exact base placement of every element whose placement the call
//! changes, read from the base. Empty when any listed element has no placement of the given kind, any placement is not finite, or no
//! placement changes.

use super::super::elements::{self, Placement};
use super::PlaceElements;
use crate::{ModelMutation, ModelSnapshot};
use std::collections::BTreeMap;

pub fn inverse(payload: &PlaceElements, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let rows: Option<Vec<(String, Placement, bool)>> = payload
        .placements
        .iter()
        .map(|(id, target)| {
            let current = elements::state(base, id).filter(|current| current.same_kind(target) && target.numbers().iter().all(|number| number.is_finite()))?;
            let changes = current != *target;
            Some((id.clone(), current, changes))
        })
        .collect();
    let before: BTreeMap<String, Placement> = rows.into_iter().flatten().filter(|(_, _, changes)| *changes).map(|(id, current, _)| (id, current)).collect();
    if before.is_empty() {
        Vec::new()
    } else {
        vec![ModelMutation::PlaceElements(PlaceElements { placements: before })]
    }
}
