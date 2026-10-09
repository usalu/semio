//! 👯️ The copies and the in-place images of the modify kernel. A copy is a set of created records: the image of every source element under
//! a map, the openings of a copied wall or curtain wall hosted on its copy, the panel overrides of a copied curtain wall, the sweeps of a copied wall hosted on its copy, and the property and classification entries of every source.
//! The id of each created record is minted from the prefix of the mutation, the number of the copy and the position of the source in id
//! order, so no clock and no counter is involved. An in-place image is the sparse patch of what a map changes of its sources and the
//! absolute base states an exact inverse restores.

use crate::mutations::elements::{self, placement_diff, Placement, Refusal};
use crate::mutations::placement::host_length;
use super::map::{self, Image, Map};
use crate::{Assigned, CurtainPanelOverride, EndJoin, Entry, KeyedDelta, ModelDiff, ModelSnapshot, Opening, Patch, WallPatch, WallSweep};
use protocol::OutcomeCode;
use std::collections::{BTreeMap, BTreeSet};

/// 🔢️ The most records and data rows one mutation may create: what one delete restores (the inverse of every copy), so that undoing a copy never meets the bound of a removal.
pub const MAX_CREATED: usize = super::super::cascade::INVERSE_ROWS;

//#region 🔖️Sources
/// 🧲️ What a modify mutation acts on: the placed elements it names, in id order, the openings hosted by them and the curtain panel overrides of the curtain walls among them and the wall sweeps of the walls among them, all in id order.
#[derive(Clone, Debug, PartialEq)]
pub struct Sources {
    pub elements: Vec<String>,
    pub openings: Vec<String>,
    pub overrides: Vec<String>,
    pub sweeps: Vec<String>,
}

/// 🧲️ The sources named by `ids`: an empty selection is `Invariant`, an unknown id `TargetMissing`, an element without a placement
/// (a site, a building, a storey) `Invariant`. An opening is acted on with its host, never alone, because it follows its host by inference.
pub fn sources(base: &ModelSnapshot, ids: &[String]) -> Result<Sources, Refusal> {
    if ids.is_empty() {
        return Err(Refusal::new(OutcomeCode::Invariant, "No element is selected.", ["ids"]));
    }
    let wanted: BTreeSet<&String> = ids.iter().collect();
    let (mut elements, mut loose) = (Vec::new(), Vec::new());
    for id in wanted {
        if elements::placement(base, id).is_some() {
            elements.push(id.clone());
        } else if base.openings.contains_key(id) {
            loose.push(id.clone());
        } else if elements::exists(base, id) {
            return Err(Refusal::new(OutcomeCode::Invariant, format!("Element \"{id}\" has no placement of its own."), [id.clone()]));
        } else {
            return Err(Refusal::new(OutcomeCode::TargetMissing, format!("Element \"{id}\" does not exist."), [id.clone()]));
        }
    }
    let openings: Vec<String> = base.openings.iter().filter(|(_, opening)| elements.contains(&opening.host)).map(|(id, _)| id.clone()).collect();
    if let Some(orphan) = loose.iter().find(|id| !openings.contains(id)) {
        return Err(Refusal::new(OutcomeCode::Invariant, format!("Opening \"{orphan}\" is acted on together with its host, never alone."), [orphan.clone()]));
    }
    let overrides: Vec<String> = base.curtain_panel_overrides.iter().filter(|(_, row)| elements.contains(&row.curtain)).map(|(id, _)| id.clone()).collect();
    let sweeps: Vec<String> = base.wall_sweeps.iter().filter(|(_, row)| elements.contains(&row.host)).map(|(id, _)| id.clone()).collect();
    Ok(Sources { elements, openings, overrides, sweeps })
}
//#endregion 🔖️Sources

//#region 🔖️Minting
/// 🪪️ The id of the record created for the source at position `ordinal` (elements first, then their openings, both in id order) by copy
/// number `copy` (the first copy is 1) of the mutation with id prefix `prefix`.
pub fn mint(prefix: &str, copy: u32, ordinal: usize) -> String {
    format!("{prefix}-{copy}-{ordinal}")
}

fn free_name(name: &str, used: &BTreeSet<String>) -> String {
    let number = name.parse::<u64>().ok();
    (1u64..).map(|step| number.map_or_else(|| format!("{name}-{step}"), |number| number.saturating_add(step).to_string())).find(|candidate| !used.contains(candidate)).unwrap_or_default()
}
//#endregion 🔖️Minting

//#region 🔖️Copies
/// 👯️ The created records of a set of copies: the sparse diff, the minted ids of the copied elements in creation order and how many
/// openings were carried along.
#[derive(Clone, Debug, PartialEq)]
pub struct Built {
    pub diff: ModelDiff,
    pub roots: Vec<String>,
    pub carried: usize,
}

fn created<T: Clone + PartialEq, P: Patch<T>>(slot: &mut Option<KeyedDelta<T, P>>, id: &str, record: T) {
    slot.get_or_insert_with(KeyedDelta::default).0.insert(id.to_string(), Entry::Created(record));
}

fn insert(diff: &mut ModelDiff, id: &str, image: Image) {
    match image {
        Image::Wall(row) => created(&mut diff.walls, id, row),
        Image::CurtainWall(row) => created(&mut diff.curtain_walls, id, row),
        Image::Column(row) => created(&mut diff.columns, id, row),
        Image::Beam(row) => created(&mut diff.beams, id, row),
        Image::Slab(row) => created(&mut diff.slabs, id, row),
        Image::Ceiling(row) => created(&mut diff.ceilings, id, row),
        Image::Roof(row) => created(&mut diff.roofs, id, row),
        Image::Stair(row) => created(&mut diff.stairs, id, row),
        Image::Railing(row) => created(&mut diff.railings, id, row),
        Image::Ramp(row) => created(&mut diff.ramps, id, row),
        Image::Space(row) => created(&mut diff.spaces, id, row),
        Image::Grid(row) => created(&mut diff.grids, id, row),
    }
}

fn data(base: &ModelSnapshot, diff: &mut ModelDiff, from: &str, to: &str) -> usize {
    let mut rows = 0;
    if let Some(set) = base.properties.get(from) {
        rows += set.values().map(|properties| properties.len()).sum::<usize>();
        created(&mut diff.properties, to, set.clone());
    }
    if let Some(classification) = base.classifications.get(from) {
        rows += classification.len();
        created(&mut diff.classifications, to, classification.clone());
    }
    rows
}

fn free(base: &ModelSnapshot, id: &str) -> Result<(), Refusal> {
    match elements::taken(base, id) {
        Some(noun) => Err(Refusal::new(OutcomeCode::DuplicateId, format!("{noun} \"{id}\" already exists."), ["prefix".to_string()])),
        None => Ok(()),
    }
}

/// 👯️ One copy of the sources per map: the image of every source element, the openings of a copied wall or curtain wall hosted on its
/// copy and the data of every source. A copied space takes the next free number of its storey and a copied grid line the next free label
/// of its building, because both are unique. A copy whose minted id is taken, or a stair with a fixed hand under a reflection, is refused.
pub fn duplicate(base: &ModelSnapshot, ids: &[String], prefix: &str, maps: &[Map]) -> Result<Built, Refusal> {
    if prefix.trim().is_empty() {
        return Err(Refusal::new(OutcomeCode::Invariant, "A copy needs a non-blank id prefix.", ["prefix"]));
    }
    let found = sources(base, ids)?;
    if (found.elements.len() + found.openings.len() + found.overrides.len() + found.sweeps.len()) * maps.len() > MAX_CREATED {
        return Err(Refusal::new(OutcomeCode::Invariant, format!("A mutation creates at most {MAX_CREATED} records."), ["ids"]));
    }
    let mut diff = ModelDiff::default();
    let mut roots = Vec::new();
    let mut rows = 0usize;
    let mut numbers: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut labels: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (index, map) in maps.iter().enumerate() {
        let copy = index as u32 + 1;
        let mut hosts: BTreeMap<String, String> = BTreeMap::new();
        for (ordinal, id) in found.elements.iter().enumerate() {
            let minted = mint(prefix, copy, ordinal);
            free(base, &minted)?;
            let mut image = map::image(base, id, map)?;
            match &mut image {
                Image::Space(space) => {
                    let used = numbers.entry(space.storey.clone()).or_insert_with(|| base.spaces.values().filter(|row| row.storey == space.storey).map(|row| row.number.clone()).collect());
                    space.number = free_name(&space.number, used);
                    used.insert(space.number.clone());
                }
                Image::Grid(grid) => {
                    let used = labels.entry(grid.building.clone()).or_insert_with(|| base.grids.values().filter(|row| row.building == grid.building).map(|row| row.label.clone()).collect());
                    grid.label = free_name(&grid.label, used);
                    used.insert(grid.label.clone());
                }
                _ => {}
            }
            insert(&mut diff, &minted, image);
            rows += 1 + data(base, &mut diff, id, &minted);
            hosts.insert(id.clone(), minted.clone());
            roots.push(minted);
        }
        for (position, id) in found.openings.iter().enumerate() {
            let minted = mint(prefix, copy, found.elements.len() + position);
            free(base, &minted)?;
            let opening = &base.openings[id];
            let length = host_length(base, &opening.host).ok_or_else(|| Refusal::new(OutcomeCode::TargetMissing, format!("Host \"{}\" of opening \"{id}\" does not exist.", opening.host), [id.clone()]))?;
            let moved = map::opening(opening, length, base.curtain_walls.contains_key(&opening.host), map);
            created(&mut diff.openings, &minted, Opening { host: hosts[&opening.host].clone(), ..moved });
            rows += 1 + data(base, &mut diff, id, &minted);
        }
        for (position, id) in found.overrides.iter().enumerate() {
            let minted = mint(prefix, copy, found.elements.len() + found.openings.len() + position);
            free(base, &minted)?;
            let record = &base.curtain_panel_overrides[id];
            created(&mut diff.curtain_panel_overrides, &minted, CurtainPanelOverride { curtain: hosts[&record.curtain].clone(), ..record.clone() });
            rows += 1;
        }
        for (position, id) in found.sweeps.iter().enumerate() {
            let minted = mint(prefix, copy, found.elements.len() + found.openings.len() + found.overrides.len() + position);
            free(base, &minted)?;
            let record = &base.wall_sweeps[id];
            created(&mut diff.wall_sweeps, &minted, WallSweep { host: hosts[&record.host].clone(), ..record.clone() });
            rows += 1 + data(base, &mut diff, id, &minted);
        }
    }
    if rows > MAX_CREATED {
        return Err(Refusal::new(OutcomeCode::Invariant, format!("A mutation creates at most {MAX_CREATED} records and data rows, so that one delete restores them."), ["ids"]));
    }
    Ok(Built { diff, roots, carried: found.openings.len() * maps.len() })
}
//#endregion 🔖️Copies

//#region 🔖️InPlace
/// 🪞️ The image of the sources written onto them: the sparse diff, and the absolute base states an exact inverse restores (the
/// placements of the changed elements and openings, with the flight of a stair, and the join preferences of the changed walls).
#[derive(Clone, Debug, PartialEq)]
pub struct Placed {
    pub diff: ModelDiff,
    pub before: BTreeMap<String, Placement>,
    pub joins: BTreeMap<String, (Option<EndJoin>, Option<EndJoin>)>,
    pub carried: usize,
}

impl Placed {
    /// 🕳️ Whether the map changes none of the sources.
    pub fn is_empty(&self) -> bool {
        self.before.is_empty() && self.joins.is_empty()
    }
}

fn patch_wall(diff: &mut ModelDiff, id: &str, edit: impl FnOnce(&mut WallPatch)) {
    if let Entry::Patched(patch) = diff.walls.get_or_insert_with(KeyedDelta::default).0.entry(id.to_string()).or_insert_with(|| Entry::Patched(WallPatch::default())) {
        edit(patch);
    }
}

/// 🪞️ The sources moved by `map` where they stand: one sparse patch per element holding exactly the fields the map changes (placement,
/// the flight of a mirrored stair, the join preferences of a mirrored wall) and one per opening holding its offset and flips.
pub fn in_place(base: &ModelSnapshot, ids: &[String], map: &Map) -> Result<Placed, Refusal> {
    let found = sources(base, ids)?;
    let (mut before, mut after) = (BTreeMap::new(), BTreeMap::new());
    let mut joins: BTreeMap<String, (Option<EndJoin>, Option<EndJoin>)> = BTreeMap::new();
    let mut next_joins: BTreeMap<String, (Option<EndJoin>, Option<EndJoin>)> = BTreeMap::new();
    for id in &found.elements {
        let image = map::image(base, id, map)?;
        let (was, now) = (elements::placement(base, id).unwrap_or_else(|| image.placement()), image.placement());
        if was != now {
            before.insert(id.clone(), was);
            after.insert(id.clone(), now);
        }
        if let (Image::Wall(row), Some(old)) = (&image, base.walls.get(id)) {
            if (old.start_join, old.end_join) != (row.start_join, row.end_join) {
                joins.insert(id.clone(), (old.start_join, old.end_join));
                next_joins.insert(id.clone(), (row.start_join, row.end_join));
            }
        }
    }
    for id in &found.openings {
        let opening = &base.openings[id];
        let length = host_length(base, &opening.host).ok_or_else(|| Refusal::new(OutcomeCode::TargetMissing, format!("Host \"{}\" of opening \"{id}\" does not exist.", opening.host), [id.clone()]))?;
        let moved = map::opening(opening, length, base.curtain_walls.contains_key(&opening.host), map);
        let (was, now) = (Placement::Opening { offset: opening.offset, flip_hand: opening.flip_hand, flip_facing: opening.flip_facing }, Placement::Opening { offset: moved.offset, flip_hand: moved.flip_hand, flip_facing: moved.flip_facing });
        if was != now {
            before.insert(id.clone(), was);
            after.insert(id.clone(), now);
        }
    }
    let mut diff = placement_diff(&before, &after);
    for (id, (start, end)) in &next_joins {
        let old = joins[id.as_str()];
        patch_wall(&mut diff, id, |patch| {
            patch.start_join = (old.0 != *start).then(|| Assigned::new(*start));
            patch.end_join = (old.1 != *end).then(|| Assigned::new(*end));
        });
    }
    Ok(Placed { diff, before, joins, carried: found.openings.len() })
}
//#endregion 🔖️InPlace
