//! 🧮️ What a node of the model graph is: its dependency (the honesty contract of `dep_input`: everything `value` reads of the snapshot that is not a parent) and its value, by dispatch to the pure
//! functions of the leaf modules. Parents are found by key in an [`Index`] built once per call.

use super::super::super::curtain_layout::{self, curtain_layout_of, CurtainLayout};
use super::super::super::diagnostics::{self, Inputs as FindingInputs};
use super::super::super::element_solids::roofs::RoofFallback;
use super::super::super::element_solids::{self, beams, columns, curtain_walls, dep_object, dep_value, fillers, placement_of, railings, roofs, slabs, stairs, walls, ElementSolid, SolidEntry, SolidFamily};
use super::super::super::opening_frames::{self, cut_rect, frame_of, CutRect, HostExtent, OpeningCut, OpeningFrame};
use super::super::super::plan_linework::{self, Inputs as PlanInputs};
use super::super::super::quantities::{self, totals_of, ElementQuantity};
use super::super::super::spaces::{self, obstacles_of, rooms_from, StoreyRooms};
use super::super::super::stair_runs::{self, run_of, StairRun};
use super::super::super::storey_levels::{self, resolve, StoreyLevel};
use super::super::super::wall_layout::joins::Band;
use super::super::super::wall_layout::{self, band_of, layout_of, WallLayout};
use super::plan::{element_storey, opening_storey};
use super::{Data, DiagnosticScope, ModelNode, ModelValue, TotalsScope};
use crate::{ModelSnapshot, TopConstraint};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;
use std::sync::Arc;

//#region 🔖️Counter
thread_local! {
    static COMPUTED: std::cell::RefCell<BTreeMap<&'static str, usize>> = const { std::cell::RefCell::new(BTreeMap::new()) };
}

/// 🔢️ The engine call counter: how many nodes of each kind `compute` ran on this thread since the counter was last taken. The session reports the difference around an update.
pub fn take_computed() -> BTreeMap<&'static str, usize> {
    COMPUTED.with(|counter| std::mem::take(&mut *counter.borrow_mut()))
}

fn count(node: &ModelNode) {
    COMPUTED.with(|counter| *counter.borrow_mut().entry(node.kind().name()).or_insert(0) += 1);
}

thread_local! {
    static HASHED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 🔢️ How many dependencies the engine asked for on this thread since the counter was last taken: with no cache it asks for none.
pub fn take_hashed() -> usize {
    HASHED.with(|counter| counter.replace(0))
}
//#endregion 🔖️Counter

//#region 🔖️Index
/// 🔎️ The parent values of one node by id, each kind in its own map (ids are unique across collections, a storey scope is the storey id).
#[derive(Default)]
pub struct Index<'a> {
    pub levels: BTreeMap<String, StoreyLevel>,
    pub bands: BTreeMap<&'a str, Option<Band>>,
    pub cuts: BTreeMap<&'a str, CutRect>,
    pub layouts: BTreeMap<&'a str, &'a WallLayout>,
    pub curtains: BTreeMap<&'a str, &'a CurtainLayout>,
    pub hosts: BTreeMap<&'a str, Option<&'a HostExtent>>,
    pub frames: BTreeMap<&'a str, &'a OpeningFrame>,
    pub runs: BTreeMap<&'a str, &'a StairRun>,
    pub solids: BTreeMap<&'a str, &'a SolidEntry>,
    pub rooms: BTreeMap<&'a str, &'a StoreyRooms>,
    pub quantities: Vec<&'a ElementQuantity>,
}

impl<'a> Index<'a> {
    /// 🔎️ Sorts the values of the parents of a node by kind.
    pub fn of(parents: &'a [ModelValue]) -> Self {
        let mut index = Self::default();
        for value in parents {
            match (&value.node, &value.data) {
                (ModelNode::Storey(id), Data::Level(level)) => {
                    index.levels.insert(id.clone(), *level);
                }
                (ModelNode::Band(id), Data::Band(band)) => {
                    index.bands.insert(id, *band);
                }
                (ModelNode::Cut(id), Data::Cut(cut)) => {
                    index.cuts.insert(id, *cut);
                }
                (ModelNode::WallLayout(id), Data::Layout(layout)) => {
                    index.layouts.insert(id, layout);
                }
                (ModelNode::CurtainLayout(id), Data::Curtain(layout)) => {
                    index.curtains.insert(id, layout);
                }
                (ModelNode::Host(id), Data::Host(host)) => {
                    index.hosts.insert(id, host.as_deref());
                }
                (ModelNode::OpeningFrame(id), Data::Frame(frame)) => {
                    index.frames.insert(id, frame);
                }
                (ModelNode::StairRun(id), Data::Run(run)) => {
                    index.runs.insert(id, run);
                }
                (ModelNode::Solid(key), Data::Solid(entry)) => {
                    index.solids.insert(&key.id, entry);
                }
                (ModelNode::Room(id), Data::Rooms(rooms)) => {
                    index.rooms.insert(id, rooms);
                }
                (ModelNode::Quantity(_), Data::Quantity(Some(quantity))) => index.quantities.push(quantity),
                _ => {}
            }
        }
        index
    }

    fn level(&self, storey: &str) -> StoreyLevel {
        self.levels.get(storey).copied().unwrap_or_default()
    }

    fn target(&self, storey: &str, top: &TopConstraint) -> Option<StoreyLevel> {
        match top {
            TopConstraint::Storey { storey: target, .. } if target != storey => self.levels.get(target).copied(),
            _ => None,
        }
    }
}
//#endregion 🔖️Index

//#region 🔖️Dependency
fn keyed(key: &ModelNode, input: DslValue) -> DslValue {
    dep_object([("key", dep_value(key)), ("input", input)])
}

fn placement(snapshot: &ModelSnapshot, storey: &str) -> DslValue {
    dep_value(&opening_frames::building_placement(snapshot, storey))
}

/// 🔑️ Everything `value` reads of the snapshot for `key` that is not a parent, together with the key itself (a cached value is never handed to another node).
pub fn dependency(snapshot: &ModelSnapshot, key: &ModelNode) -> DslValue {
    use element_solids::Anonymous;
    HASHED.with(|counter| counter.set(counter.get() + 1));
    let input = match key {
        ModelNode::Storey(id) => storey_levels::dependency(snapshot, id),
        ModelNode::Band(id) => snapshot.walls.get(id).map_or(DslValue::Null, |wall| wall_layout::band_dependency(snapshot, wall)),
        ModelNode::Cut(id) => snapshot.openings.get(id).map_or(DslValue::Null, |opening| opening_frames::cut_dependency(snapshot, opening)),
        ModelNode::WallLayout(id) => snapshot.walls.get(id).map_or(DslValue::Null, |wall| wall_layout::dependency(snapshot, &wall.anonymous())),
        ModelNode::CurtainLayout(id) => snapshot.curtain_walls.get(id).map_or(DslValue::Null, curtain_layout::dependency),
        ModelNode::Host(id) => match (snapshot.walls.get(id), snapshot.curtain_walls.get(id)) {
            (Some(wall), _) => dep_object([("placement", placement(snapshot, &wall.storey))]),
            (None, Some(curtain)) => dep_object([("placement", placement(snapshot, &curtain.storey)), ("axis", dep_value(&curtain.axis)), ("mullion", dep_value(&curtain.mullion))]),
            (None, None) => DslValue::Null,
        },
        ModelNode::OpeningFrame(id) => snapshot.openings.get(id).map_or(DslValue::Null, |opening| opening_frames::dependency(snapshot, &opening.anonymous())),
        ModelNode::StairRun(id) => snapshot.stairs.get(id).map_or(DslValue::Null, |stair| stair_runs::dependency(&stair.anonymous())),
        ModelNode::Solid(solid) => solid_dependency(snapshot, solid),
        ModelNode::Room(storey) => spaces::dependency(snapshot, storey),
        ModelNode::Plan(storey) => plan_linework::dependency(snapshot, storey),
        ModelNode::Quantity(id) => quantities::dependency(snapshot, id),
        ModelNode::Totals(_) => DslValue::Null,
        ModelNode::Diagnostics(DiagnosticScope::Storey(id)) => diagnostics::storey_dependency(snapshot, id),
        ModelNode::Diagnostics(DiagnosticScope::Building(id)) => diagnostics::building_dependency(snapshot, id),
        ModelNode::Diagnostics(DiagnosticScope::Model) => diagnostics::model_dependency(snapshot),
    };
    keyed(key, input)
}

fn solid_dependency(snapshot: &ModelSnapshot, solid: &element_solids::SolidKey) -> DslValue {
    use element_solids::Anonymous;
    let id = &solid.id;
    let storey = element_storey(snapshot, id).unwrap_or_default();
    let family = match solid.family {
        SolidFamily::Wall => snapshot.walls.get(id).map_or(DslValue::Null, |wall| walls::dependency(snapshot, wall)),
        SolidFamily::CurtainWall => snapshot.curtain_walls.get(id).map_or(DslValue::Null, |curtain| curtain_walls::dependency(&curtain.anonymous())),
        SolidFamily::Window | SolidFamily::Door => snapshot.openings.get(id).map_or(DslValue::Null, |opening| fillers::dependency(snapshot, opening)),
        SolidFamily::Column => snapshot.columns.get(id).map_or(DslValue::Null, |column| columns::dependency(snapshot, &column.anonymous())),
        SolidFamily::Beam => snapshot.beams.get(id).map_or(DslValue::Null, |beam| beams::dependency(snapshot, &beam.anonymous())),
        SolidFamily::Slab => snapshot.slabs.get(id).map_or(DslValue::Null, |slab| slabs::dependency(snapshot, &slab.anonymous())),
        SolidFamily::Roof => snapshot.roofs.get(id).map_or(DslValue::Null, |roof| roofs::dependency(snapshot, &roof.anonymous())),
        SolidFamily::Stair => snapshot.stairs.get(id).map_or(DslValue::Null, |stair| stairs::dependency(stair)),
        SolidFamily::Railing => snapshot.railings.get(id).map_or(DslValue::Null, |railing| railings::dependency(&railing.anonymous())),
    };
    dep_object([("family", family), ("placement", placement(snapshot, &storey))])
}
//#endregion 🔖️Dependency

//#region 🔖️Value
fn wrap(key: &ModelNode, data: Data) -> ModelValue {
    ModelValue { node: key.clone(), data }
}

/// 🧮️ The value of `key` from the values of its parents (in plan order).
pub fn value(snapshot: &ModelSnapshot, key: &ModelNode, parents: &[ModelValue]) -> ModelValue {
    count(key);
    let index = Index::of(parents);
    let data = match key {
        ModelNode::Storey(id) => Data::Level(resolve(snapshot, id, index.levels.values().next())),
        ModelNode::Band(id) => Data::Band(snapshot.walls.get(id).and_then(|wall| band_of(snapshot, wall))),
        ModelNode::Cut(id) => Data::Cut(snapshot.openings.get(id).map(|opening| cut_rect(snapshot, opening)).unwrap_or_else(|| CutRect { size: opening_frames::Resolved { width: 0.0, height: 0.0, sill: 0.0, type_found: false }, cut: OpeningCut::default() })),
        ModelNode::WallLayout(id) => Data::Layout(Arc::new(wall_layout_value(snapshot, id, &index))),
        ModelNode::CurtainLayout(id) => Data::Curtain(snapshot.curtain_walls.get(id).map_or_else(CurtainLayout::default, |curtain| curtain_layout_of(curtain, &index.level(&curtain.storey), index.target(&curtain.storey, &curtain.top).as_ref()))),
        ModelNode::Host(id) => Data::Host(host_value(snapshot, id, &index).map(Arc::new)),
        ModelNode::OpeningFrame(id) => Data::Frame(Arc::new(frame_value(snapshot, id, &index))),
        ModelNode::StairRun(id) => Data::Run(Arc::new(snapshot.stairs.get(id).map_or_else(StairRun::default, |stair| run_of(stair, &index.level(&stair.storey), index.target(&stair.storey, &stair.top).as_ref())))),
        ModelNode::Solid(solid) => Data::Solid(Arc::new(solid_value(snapshot, solid, &index))),
        ModelNode::Room(storey) => Data::Rooms(Arc::new(room_value(snapshot, storey, &index))),
        ModelNode::Plan(storey) => Data::Plan(Arc::new(plan_value(snapshot, storey, &index))),
        ModelNode::Quantity(id) => Data::Quantity(quantity_value(snapshot, id, &index).map(Arc::new)),
        ModelNode::Totals(scope) => Data::Totals(Arc::new(totals_value(scope, parents))),
        ModelNode::Diagnostics(scope) => Data::Findings(Arc::new(findings_value(snapshot, scope, &index))),
    };
    wrap(key, data)
}

fn wall_layout_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> WallLayout {
    let Some(wall) = snapshot.walls.get(id) else { return WallLayout::default() };
    let bands: BTreeMap<String, Band> = index.bands.iter().filter_map(|(band, row)| row.map(|row| ((*band).to_string(), row))).collect();
    layout_of(snapshot, id, wall, &index.level(&wall.storey), index.target(&wall.storey, &wall.top).as_ref(), &bands)
}

fn host_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> Option<HostExtent> {
    if let Some(wall) = snapshot.walls.get(id) {
        let layout = index.layouts.get(id)?;
        return Some(HostExtent::of_wall(wall, layout, &index.level(&wall.storey), opening_frames::building_placement(snapshot, &wall.storey)));
    }
    let curtain = snapshot.curtain_walls.get(id)?;
    let layout = index.curtains.get(id)?;
    Some(HostExtent::of_curtain(curtain, layout, &index.level(&curtain.storey), opening_frames::building_placement(snapshot, &curtain.storey)))
}

fn frame_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> OpeningFrame {
    let Some(opening) = snapshot.openings.get(id) else { return OpeningFrame::default() };
    let own = index.cuts.get(id).copied().unwrap_or_else(|| cut_rect(snapshot, opening));
    let host = index.hosts.get(opening.host.as_str()).copied().flatten();
    let siblings: Vec<(String, OpeningCut)> = index.cuts.iter().filter(|(other, _)| **other != id).map(|(other, cut)| ((*other).to_string(), cut.cut)).collect();
    frame_of(snapshot, id, &own, host, &siblings)
}

fn solid_value(snapshot: &ModelSnapshot, solid: &element_solids::SolidKey, index: &Index<'_>) -> SolidEntry {
    let id = solid.id.as_str();
    let empty = |family: SolidFamily| SolidEntry { solid: element_solids::SolidBuilder::new(family).build(), fallback: None };
    let Some(storey) = element_storey(snapshot, id) else { return empty(solid.family) };
    let own = index.level(&storey);
    let target = |top: &TopConstraint| index.target(&storey, top);
    let built: Option<SolidEntry> = match solid.family {
        SolidFamily::Wall => snapshot.walls.get(id).zip(index.layouts.get(id)).map(|(wall, layout)| SolidEntry { solid: walls::wall_solid(snapshot, wall, layout, &walls::cuts_of(index.frames.values().copied())), fallback: None }),
        SolidFamily::CurtainWall => snapshot.curtain_walls.get(id).zip(index.curtains.get(id)).map(|(curtain, layout)| SolidEntry { solid: curtain_walls::curtain_solid(curtain, layout), fallback: None }),
        SolidFamily::Window | SolidFamily::Door => snapshot.openings.get(id).zip(index.frames.get(id)).and_then(|(opening, frame)| fillers::filler_solid(snapshot, opening, frame)).map(|solid| SolidEntry { solid, fallback: None }),
        SolidFamily::Column => snapshot.columns.get(id).map(|column| SolidEntry { solid: columns::column_solid(snapshot, column, &own, target(&column.top).as_ref()), fallback: None }),
        SolidFamily::Beam => snapshot.beams.get(id).map(|beam| SolidEntry { solid: beams::beam_solid(snapshot, beam, &own), fallback: None }),
        SolidFamily::Slab => snapshot.slabs.get(id).map(|slab| SolidEntry { solid: slabs::slab_solid(snapshot, slab, &own), fallback: None }),
        SolidFamily::Roof => snapshot.roofs.get(id).map(|roof| roofs::roof_solid(snapshot, roof, &own)),
        SolidFamily::Stair => index.runs.get(id).map(|run| SolidEntry { solid: stairs::stair_solid(run), fallback: None }),
        SolidFamily::Railing => snapshot.railings.get(id).map(|railing| SolidEntry { solid: railings::railing_solid(railing, &own), fallback: None }),
    };
    let mut entry = built.unwrap_or_else(|| empty(solid.family));
    entry.solid = entry.solid.placed(&storey, placement_of(snapshot, &storey, Some(&own)));
    entry
}

fn room_value(snapshot: &ModelSnapshot, storey: &str, index: &Index<'_>) -> StoreyRooms {
    let obstacles = obstacles_of(snapshot, storey, &index.layouts);
    rooms_from(snapshot, storey, &index.level(storey), &obstacles)
}

fn plan_value(snapshot: &ModelSnapshot, storey: &str, index: &Index<'_>) -> plan_linework::PlanLinework {
    let inputs = PlanInputs { levels: index.levels.clone(), layouts: index.layouts.clone(), curtains: index.curtains.clone(), frames: index.frames.clone(), runs: index.runs.clone(), rooms: index.rooms.get(storey).copied() };
    plan_linework::plan_of(snapshot, storey, &inputs)
}

fn quantity_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> Option<ElementQuantity> {
    let hosted = |host: &str| -> Vec<&OpeningFrame> { snapshot.openings.iter().filter(|(_, opening)| opening.host == host).filter_map(|(opening, _)| index.frames.get(opening.as_str()).copied()).collect() };
    let solid = |_family: SolidFamily| index.solids.get(id).map(|entry| &entry.solid);
    if let Some(wall) = snapshot.walls.get(id) {
        return index.layouts.get(id).map(|layout| quantities::wall_quantity(snapshot, wall, layout, &hosted(id)));
    }
    if let Some(curtain) = snapshot.curtain_walls.get(id) {
        return index.curtains.get(id).map(|layout| quantities::curtain_quantity(snapshot, curtain, layout, &hosted(id), solid(SolidFamily::CurtainWall)));
    }
    if let Some(slab) = snapshot.slabs.get(id) {
        return Some(quantities::slab_quantity(snapshot, slab));
    }
    if let Some(roof) = snapshot.roofs.get(id) {
        return Some(quantities::roof_quantity(snapshot, roof, solid(SolidFamily::Roof)));
    }
    if let Some(column) = snapshot.columns.get(id) {
        return quantities::column_quantity(snapshot, column, &index.level(&column.storey), index.target(&column.storey, &column.top).as_ref());
    }
    if let Some(beam) = snapshot.beams.get(id) {
        return quantities::beam_quantity(snapshot, beam);
    }
    if let Some(opening) = snapshot.openings.get(id) {
        return index.frames.get(id).map(|frame| quantities::opening_quantity(snapshot, opening, frame, index.solids.get(id).map(|entry| &entry.solid)));
    }
    if let Some(stair) = snapshot.stairs.get(id) {
        return index.runs.get(id).map(|run| quantities::stair_quantity(stair, run, solid(SolidFamily::Stair)));
    }
    if let Some(railing) = snapshot.railings.get(id) {
        return Some(quantities::railing_quantity(snapshot, railing, solid(SolidFamily::Railing)));
    }
    let space = snapshot.spaces.get(id)?;
    index.rooms.get(space.storey.as_str()).and_then(|rooms| rooms.get(id)).and_then(|room| quantities::space_quantity(space, room))
}

fn totals_value(_scope: &TotalsScope, parents: &[ModelValue]) -> quantities::QuantityTotals {
    totals_of(Index::of(parents).quantities)
}

fn findings_value(snapshot: &ModelSnapshot, scope: &DiagnosticScope, index: &Index<'_>) -> Vec<diagnostics::Diagnostic> {
    let fallbacks: BTreeMap<&str, RoofFallback> = index.solids.iter().filter_map(|(id, entry)| entry.fallback.map(|fallback| (*id, fallback))).collect();
    let rooms = match scope {
        DiagnosticScope::Storey(storey) => index.rooms.get(storey.as_str()).copied(),
        _ => None,
    };
    let inputs = FindingInputs { levels: index.levels.clone(), layouts: index.layouts.clone(), frames: index.frames.clone(), runs: index.runs.clone(), rooms, fallbacks };
    match scope {
        DiagnosticScope::Storey(storey) => diagnostics::storey_findings(snapshot, storey, &inputs),
        DiagnosticScope::Building(building) => diagnostics::building_findings(snapshot, building, &inputs),
        DiagnosticScope::Model => diagnostics::model_findings(&diagnostics::references::ReferenceView::of(snapshot)),
    }
}
//#endregion 🔖️Value

/// 🪜️ The storey of an opening, for callers outside the graph that need it.
pub fn storey_of_opening(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    opening_storey(snapshot, id)
}
