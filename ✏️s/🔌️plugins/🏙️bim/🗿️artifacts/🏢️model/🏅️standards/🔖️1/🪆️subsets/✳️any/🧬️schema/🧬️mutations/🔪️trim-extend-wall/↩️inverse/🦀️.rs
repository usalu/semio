//! ↩️ Inverse of `TrimExtendWall`: one `PlaceElements` carrying the exact base axis of the wall and the exact base offset and flips of
//! every opening whose offset the call moved, read from the base. One diff restores the wall and its openings together, so no restored
//! opening can collide with one that still has its moved offset. Empty when the call is refused.

use super::super::elements::{self, Placement};
use super::super::place_elements::PlaceElements;
use super::diff::plan;
use super::TrimExtendWall;
use crate::{ModelMutation, ModelSnapshot};
use std::collections::BTreeMap;

pub fn inverse(payload: &TrimExtendWall, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Ok((_, offsets)) = plan(payload, base) else {
        return Vec::new();
    };
    let Some(wall) = elements::placement(base, &payload.id) else {
        return Vec::new();
    };
    let openings = offsets.keys().filter_map(|id| elements::state(base, id).map(|state| (id.clone(), state)));
    let placements: BTreeMap<String, Placement> = std::iter::once((payload.id.clone(), wall)).chain(openings).collect();
    vec![ModelMutation::PlaceElements(PlaceElements { placements })]
}
