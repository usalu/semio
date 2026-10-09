//! 🔺️ Diff constructor for `SplitWall`: the original wall keeps the first part of its axis, a created wall (same storey, type, location,
//! base, top, phase and name) takes the second, and every opening hosted at or beyond the split point is re-hosted by a sparse opening
//! patch with its offset re-based onto the new wall, and every sweep of the wall is repeated on the new wall (as `<sweep id>-<new wall id>`), because a sweep runs along the whole face of its host. Both parts of an arc keep the sweep share of their arc length
//! (`bulge = tan(sweep / 4)`); an opening belongs to the part that contains its offset.

use super::super::elements;
use super::super::wall_geometry::{snap, split, Split};
use super::SplitWall;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot, Opening, OpeningPatch, Wall, WallPatch, WallSweep};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

fn plan(payload: &SplitWall, base: &ModelSnapshot) -> Option<Split> {
    base.walls.get(&payload.id).and_then(|wall| split(&wall.axis, payload.t))
}

fn carried<'a>(base: &'a ModelSnapshot, wall: &'a str, at: f64) -> impl Iterator<Item = (&'a String, &'a Opening)> {
    base.openings.iter().filter(move |(_, opening)| opening.host == wall && opening.offset >= at)
}

pub fn diff(payload: &SplitWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(noun) = elements::taken(base, &payload.new_id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.new_id), [payload.new_id.clone()]);
    }
    let Some(parts) = plan(payload, base) else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A wall splits strictly between its ends into two parts that both have length.", ["t"]);
    };
    let moved: BTreeMap<String, Entry<Opening, OpeningPatch>> = carried(base, &payload.id, parts.at)
        .map(|(id, opening)| (id.clone(), Entry::Patched(OpeningPatch { host: Some(payload.new_id.clone()), offset: Some(snap(opening.offset - parts.at)), ..Default::default() })))
        .collect();
    let swept: BTreeMap<String, Entry<WallSweep, crate::WallSweepPatch>> = base.wall_sweeps.iter().filter(|(_, sweep)| sweep.host == payload.id).map(|(id, sweep)| (format!("{id}-{}", payload.new_id), Entry::Created(WallSweep { host: payload.new_id.clone(), ..sweep.clone() }))).collect();
    if let Some((noun, id)) = swept.keys().find_map(|id| elements::taken(base, id).map(|noun| (noun, id))) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{id}\" already exists."), [id.clone()]);
    }
    let count = moved.len() + swept.len();
    let walls = KeyedDelta(BTreeMap::from([
        (payload.id.clone(), Entry::Patched(WallPatch { axis: Some(parts.first), ..Default::default() })),
        (payload.new_id.clone(), Entry::Created(Wall { axis: parts.second, ..wall.clone() })),
    ]));
    let outcome = MutationOutcome::new(ModelDiff { walls: Some(walls), openings: (!moved.is_empty()).then_some(KeyedDelta(moved)), wall_sweeps: (!swept.is_empty()).then_some(KeyedDelta(swept)), ..ModelDiff::default() });
    if count == 0 {
        outcome
    } else {
        outcome.info(OutcomeCode::Cascade, format!("Wall \"{}\" handed {count} opening(s) and sweep(s) over to \"{}\".", payload.id, payload.new_id))
    }
}
