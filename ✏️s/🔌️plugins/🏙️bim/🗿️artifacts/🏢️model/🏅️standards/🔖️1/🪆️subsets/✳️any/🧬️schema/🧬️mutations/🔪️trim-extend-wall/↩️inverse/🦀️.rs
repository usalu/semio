//! ↩️ Inverse of `TrimExtendWall`: one `PlaceElements` carrying the exact base axis of the wall and the exact base offset and flips of
//! every opening whose offset the call moved, read from the base. One diff restores the wall and its openings together, so no restored
//! opening can collide with one that still has its moved offset. Empty when the call is refused.

use crate::standards::v1::subsets::any::schema::mutations::elements::{self, Placement};
use crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements;
use crate::standards::v1::subsets::any::schema::mutations::trim_extend_wall::diff::plan;
use crate::standards::v1::subsets::any::schema::mutations::trim_extend_wall::TrimExtendWall;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use std::collections::BTreeMap;

pub fn inverse<Op>(payload: &TrimExtendWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    let Ok((_, offsets)) = plan(payload, base) else {
        return Vec::new();
    };
    let Some(wall) = elements::placement(base, &payload.id) else {
        return Vec::new();
    };
    let openings = offsets.keys().filter_map(|id| elements::state(base, id).map(|state| (id.clone(), state)));
    let placements: BTreeMap<String, Placement> = std::iter::once((payload.id.clone(), wall)).chain(openings).collect();
    vec![Op::from(PlaceElements { placements })]
}
