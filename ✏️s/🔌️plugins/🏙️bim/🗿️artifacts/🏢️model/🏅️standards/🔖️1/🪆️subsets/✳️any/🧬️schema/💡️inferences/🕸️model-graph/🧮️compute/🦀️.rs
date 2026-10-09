//! 🧮️ What a node of the model graph is: its dependency (the honesty contract of `dep_input`: everything `value` reads of the snapshot that is not a parent) and its value, by dispatch to the pure
//! functions of the leaf modules. Parents are found by key in an [`Index`] built once per call.

use super::super::super::annotation_layout::{self, StoreyAnnotations};
use super::super::super::curtain_layout::{self, curtain_layout_of, CurtainLayout};
use super::super::super::diagnostics::{self, Inputs as FindingInputs};
use super::super::super::effective_properties::{self, EffectiveProperties};
use super::super::super::element_solids::roofs::RoofFallback;
use super::super::super::element_solids::{self, beams, ceilings, columns, curtain_walls, dep_object, dep_value, fillers, placement_of, rail_hosts, railings, ramps, roofs, slabs, stairs, wall_sweeps, walls, ElementSolid, SolidEntry, SolidFamily};
use super::super::super::opening_frames::{self, cut_rect, frame_of, CutRect, HostExtent, OpeningCut, OpeningFrame};
use super::super::super::families::{self, FamilyProfiles, FamilyValue};
use super::super::super::phase_visibility;
use super::super::super::plan_linework::{self, Inputs as PlanInputs};
use super::super::super::quantities::{self, totals_of, ElementQuantity};
use super::super::super::schedules::{self, ScheduleTable};
use super::super::super::spaces::{self, obstacles_of, rooms_from, StoreyRooms};
use super::super::super::zones;
use super::super::super::ramp_runs::{self, RampRun};
use super::super::super::stair_runs::{self, run_of, StairRun};
use super::super::super::storey_levels::{self, resolve, StoreyLevel};
use super::super::super::sheet_layout;
use super::super::super::view_linework::{self, ViewInputs, ViewLinework};
use super::super::super::wall_layout::attach::{self, AttachSurface};
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
    pub ramp_runs: BTreeMap<&'a str, &'a RampRun>,
    pub solids: BTreeMap<&'a str, &'a SolidEntry>,
    pub families: BTreeMap<&'a str, &'a FamilyValue>,
    pub rooms: BTreeMap<&'a str, &'a StoreyRooms>,
    pub annotations: BTreeMap<&'a str, &'a StoreyAnnotations>,
    pub properties: BTreeMap<&'a str, &'a EffectiveProperties>,
    pub drawings: BTreeMap<&'a str, &'a ViewLinework>,
    pub quantities: Vec<&'a ElementQuantity>,
    pub measured: BTreeMap<&'a str, Option<&'a ElementQuantity>>,
    pub surfaces: BTreeMap<&'a str, &'a AttachSurface>,
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
                (ModelNode::Surface(id), Data::Surface(surface)) => {
                    index.surfaces.insert(id, surface);
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
                (ModelNode::RampRun(id), Data::RampRun(run)) => {
                    index.ramp_runs.insert(id, run);
                }
                (ModelNode::Family(id), Data::Family(value)) => {
                    index.families.insert(id, value);
                }
                (ModelNode::Solid(key), Data::Solid(entry)) => {
                    index.solids.insert(&key.id, entry);
                }
                (ModelNode::Properties(id), Data::Properties(properties)) => {
                    index.properties.insert(id, properties);
                }
                (ModelNode::Annotation(id), Data::Annotations(set)) => {
                    index.annotations.insert(id, set);
                }
                (ModelNode::View(id), Data::View(drawing)) => {
                    index.drawings.insert(id, drawing);
                }
                (ModelNode::Room(id), Data::Rooms(rooms)) => {
                    index.rooms.insert(id, rooms);
                }
                (ModelNode::Quantity(id), Data::Quantity(quantity)) => {
                    index.measured.insert(id, quantity.as_deref());
                    if let Some(quantity) = quantity {
                        index.quantities.push(quantity);
                    }
                }
                _ => {}
            }
        }
        index
    }

    /// ▭️ The outlines of the profile families among the parents.
    pub fn profiles(&self) -> FamilyProfiles<'_> {
        self.families.iter().fold(FamilyProfiles::new(), |profiles, (id, family)| profiles.with(id, &family.outline))
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
        ModelNode::CurtainLayout(id) => snapshot.curtain_walls.get(id).map_or(DslValue::Null, |curtain| curtain_layout::dependency(snapshot, id, curtain)),
        ModelNode::Host(id) => match (snapshot.walls.get(id), snapshot.curtain_walls.get(id)) {
            (Some(wall), _) => dep_object([("placement", placement(snapshot, &wall.storey))]),
            (None, Some(curtain)) => dep_object([("placement", placement(snapshot, &curtain.storey)), ("axis", dep_value(&curtain.axis)), ("mullion", dep_value(&snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).map(|kind| kind.interior_mullion.clone())))]),
            (None, None) => DslValue::Null,
        },
        ModelNode::OpeningFrame(id) => snapshot.openings.get(id).map_or(DslValue::Null, |opening| opening_frames::dependency(snapshot, &opening.anonymous())),
        ModelNode::StairRun(id) => snapshot.stairs.get(id).map_or(DslValue::Null, |stair| stair_runs::dependency(&stair.anonymous())),
        ModelNode::RampRun(id) => snapshot.ramps.get(id).map_or(DslValue::Null, |ramp| ramp_runs::dependency(&ramp.anonymous())),
        ModelNode::Solid(solid) => solid_dependency(snapshot, solid),
        ModelNode::Room(storey) => spaces::dependency(snapshot, storey),
        ModelNode::Plan(storey) => plan_linework::dependency(snapshot, storey),
        ModelNode::Annotation(storey) => annotation_layout::dependency(snapshot, storey),
        ModelNode::View(id) => view_linework::dependency(snapshot, id),
        ModelNode::Sheet(id) => sheet_layout::dependency(snapshot, id),
        ModelNode::Quantity(id) => quantities::dependency(snapshot, id),
        ModelNode::Totals(_) | ModelNode::DiagnosticIndex => DslValue::Null,
        ModelNode::Diagnostics(DiagnosticScope::Storey(id)) => diagnostics::storey_dependency(snapshot, id),
        ModelNode::Diagnostics(DiagnosticScope::Building(id)) => diagnostics::building_dependency(snapshot, id),
        ModelNode::Diagnostics(DiagnosticScope::Model) => dep_object([("model", diagnostics::model_dependency(snapshot)), ("profiles", families::findings::reference_dependency(snapshot))]),
        ModelNode::Diagnostics(DiagnosticScope::Data) => effective_properties::findings_dependency(snapshot),
        ModelNode::Properties(id) => effective_properties::dependency(snapshot, id),
        ModelNode::Family(id) => families::dependency(snapshot, id),
        ModelNode::Schedule(id) => schedules::dependency(snapshot, id),
        ModelNode::Zone(id) => zones::zone_dependency(snapshot, id),
        ModelNode::Scheme(id) => zones::scheme_dependency(snapshot, id),
        ModelNode::PhaseVisibility(storey) => phase_visibility::dependency(snapshot, storey),
        ModelNode::Surface(id) => attach::dependency(snapshot, id),
    };
    keyed(key, input)
}

fn solid_dependency(snapshot: &ModelSnapshot, solid: &element_solids::SolidKey) -> DslValue {
    use element_solids::Anonymous;
    let id = &solid.id;
    let storey = element_storey(snapshot, id).unwrap_or_default();
    let family = match solid.family {
        SolidFamily::Wall => snapshot.walls.get(id).map_or(DslValue::Null, |wall| walls::dependency(snapshot, wall)),
        SolidFamily::CurtainWall => snapshot.curtain_walls.get(id).map_or(DslValue::Null, |curtain| curtain_walls::dependency(snapshot, id, &curtain.anonymous())),
        SolidFamily::Window | SolidFamily::Door => snapshot.openings.get(id).map_or(DslValue::Null, |opening| fillers::dependency(snapshot, opening)),
        SolidFamily::Column => snapshot.columns.get(id).map_or(DslValue::Null, |column| columns::dependency(snapshot, &column.anonymous())),
        SolidFamily::Beam => snapshot.beams.get(id).map_or(DslValue::Null, |beam| beams::dependency(snapshot, &beam.anonymous())),
        SolidFamily::Slab => snapshot.slabs.get(id).map_or(DslValue::Null, |slab| slabs::dependency(snapshot, &slab.anonymous())),
        SolidFamily::Ceiling => snapshot.ceilings.get(id).map_or(DslValue::Null, |ceiling| ceilings::dependency(snapshot, &ceiling.anonymous())),
        SolidFamily::WallSweep => snapshot.wall_sweeps.get(id).map_or(DslValue::Null, |sweep| wall_sweeps::dependency(snapshot, sweep)),
        SolidFamily::Roof => snapshot.roofs.get(id).map_or(DslValue::Null, |roof| roofs::dependency(snapshot, &roof.anonymous())),
        SolidFamily::Stair => snapshot.stairs.get(id).map_or(DslValue::Null, |stair| stairs::dependency(stair)),
        SolidFamily::Ramp => snapshot.ramps.get(id).map_or(DslValue::Null, |ramp| ramps::dependency(&ramp.anonymous())),
        SolidFamily::Railing => snapshot.railings.get(id).map_or(DslValue::Null, |railing| dep_object([("railing", railings::dependency(snapshot, &railing.anonymous())), ("host", rail_hosts::dependency(snapshot, railing))])),
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
        ModelNode::CurtainLayout(id) => Data::Curtain(Arc::new(snapshot.curtain_walls.get(id).map_or_else(CurtainLayout::default, |curtain| curtain_layout_of(snapshot, id, curtain, &index.level(&curtain.storey), index.target(&curtain.storey, &curtain.top).as_ref())))),
        ModelNode::Host(id) => Data::Host(host_value(snapshot, id, &index).map(Arc::new)),
        ModelNode::OpeningFrame(id) => Data::Frame(Arc::new(frame_value(snapshot, id, &index))),
        ModelNode::RampRun(id) => Data::RampRun(Arc::new(snapshot.ramps.get(id).map_or_else(RampRun::default, |ramp| ramp_runs::run_of(ramp, &index.level(&ramp.storey), index.target(&ramp.storey, &ramp.top).as_ref())))),
        ModelNode::StairRun(id) => Data::Run(Arc::new(snapshot.stairs.get(id).map_or_else(StairRun::default, |stair| run_of(stair, &index.level(&stair.storey), index.target(&stair.storey, &stair.top).as_ref())))),
        ModelNode::Solid(solid) => Data::Solid(Arc::new(solid_value(snapshot, solid, &index))),
        ModelNode::Room(storey) => Data::Rooms(Arc::new(room_value(snapshot, storey, &index))),
        ModelNode::Plan(storey) => Data::Plan(Arc::new(plan_value(snapshot, storey, &index))),
        ModelNode::Annotation(storey) => Data::Annotations(Arc::new(annotation_value(snapshot, storey, &index))),
        ModelNode::Properties(id) => Data::Properties(Arc::new(effective_properties::effective_of(snapshot, id, index.properties.values().next().copied()))),
        ModelNode::View(id) => Data::View(Arc::new(view_value(snapshot, id, &index))),
        ModelNode::Sheet(id) => Data::Sheet(Arc::new(sheet_layout::layout_of(snapshot, id, &index.drawings))),
        ModelNode::Quantity(id) => Data::Quantity(quantity_value(snapshot, id, &index).map(Arc::new)),
        ModelNode::Totals(scope) => Data::Totals(Arc::new(totals_value(scope, parents))),
        ModelNode::Schedule(id) => Data::Schedule(Arc::new(schedule_value(snapshot, id, parents))),
        ModelNode::Zone(id) => Data::Zone(Arc::new(zone_value(snapshot, id, &index))),
        ModelNode::Scheme(id) => Data::Scheme(Arc::new(scheme_value(snapshot, id, &index))),
        ModelNode::Family(id) => Data::Family(Arc::new(families::family_of(snapshot, id, &BTreeMap::new()))),
        ModelNode::PhaseVisibility(storey) => Data::Phases(Arc::new(phase_visibility::visibility_of(snapshot, storey))),
        ModelNode::Surface(id) => Data::Surface(Arc::new(attach::storey_of(snapshot, id).map_or_else(AttachSurface::default, |storey| attach::surface_of(snapshot, id, &index.level(storey))))),
        ModelNode::Diagnostics(scope) => Data::Findings(Arc::new(findings_value(snapshot, scope, &index))),
        ModelNode::DiagnosticIndex => Data::Index(Arc::new(diagnostics::DiagnosticIndex::of(&super::projection::rebuild_diagnostics(parents)))),
    };
    wrap(key, data)
}

fn wall_layout_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> WallLayout {
    let Some(wall) = snapshot.walls.get(id) else { return WallLayout::default() };
    let bands: BTreeMap<String, Band> = index.bands.iter().filter_map(|(band, row)| row.map(|row| ((*band).to_string(), row))).collect();
    attach::apply(snapshot, id, wall, layout_of(snapshot, id, wall, &index.level(&wall.storey), index.target(&wall.storey, &wall.top).as_ref(), &bands), &index.surfaces)
}

fn host_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> Option<HostExtent> {
    if let Some(wall) = snapshot.walls.get(id) {
        let layout = index.layouts.get(id)?;
        return Some(HostExtent::of_wall(wall, layout, &index.level(&wall.storey), opening_frames::building_placement(snapshot, &wall.storey)));
    }
    let curtain = snapshot.curtain_walls.get(id)?;
    let layout = index.curtains.get(id)?;
    Some(HostExtent::of_curtain(curtain, snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).map(|kind| &kind.interior_mullion), layout, &index.level(&curtain.storey), opening_frames::building_placement(snapshot, &curtain.storey)))
}

fn frame_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> OpeningFrame {
    let Some(opening) = snapshot.openings.get(id) else { return OpeningFrame::default() };
    let own = index.cuts.get(id).copied().unwrap_or_else(|| cut_rect(snapshot, opening));
    let host = index.hosts.get(opening.host.as_str()).copied().flatten();
    let siblings: Vec<(String, OpeningCut)> = index.cuts.iter().filter(|(other, _)| **other != id).map(|(other, cut)| ((*other).to_string(), cut.cut)).collect();
    frame_of(snapshot, id, &own, host, &siblings)
}

fn host_of<'i>(snapshot: &'i ModelSnapshot, railing: &crate::Railing, index: &'i Index<'_>) -> Option<rail_hosts::Host<'i>> {
    let spec = railing.host.as_ref()?;
    if let Some(run) = index.runs.get(spec.element.as_str()) {
        return Some(rail_hosts::Host::Stair(run));
    }
    if let (Some(ramp), Some(run)) = (snapshot.ramps.get(&spec.element), index.ramp_runs.get(spec.element.as_str())) {
        return Some(rail_hosts::Host::Ramp { ramp, run });
    }
    let slab = snapshot.slabs.get(&spec.element)?;
    Some(rail_hosts::Host::Slab { slab, own: index.levels.get(&slab.storey)? })
}

/// 🔗️ The columns a beam can join with their types and resolved vertical extent (from the levels the beam solid node has as parents).
fn joiners_of<'a>(snapshot: &'a ModelSnapshot, beam: &crate::Beam, index: &Index<'_>) -> Vec<columns::Joiner<'a>> {
    beams::joining(snapshot, beam)
        .into_iter()
        .filter_map(|(_, column)| {
            let kind = snapshot.column_types.get(&column.column_type)?;
            let (base_z, top_z) = storey_levels::vertical_of(column.base_offset, &column.top, &index.level(&column.storey), index.target(&column.storey, &column.top).as_ref());
            Some(columns::Joiner { column, profile: &kind.profile, base_z, top_z })
        })
        .collect()
}

fn solid_value(snapshot: &ModelSnapshot, solid: &element_solids::SolidKey, index: &Index<'_>) -> SolidEntry {
    let id = solid.id.as_str();
    let empty = |family: SolidFamily| SolidEntry { solid: element_solids::SolidBuilder::new(family).build(), fallback: None };
    let Some(storey) = element_storey(snapshot, id) else { return empty(solid.family) };
    let own = index.level(&storey);
    let target = |top: &TopConstraint| index.target(&storey, top);
    let profiles = index.profiles();
    let built: Option<SolidEntry> = match solid.family {
        SolidFamily::Wall => snapshot.walls.get(id).zip(index.layouts.get(id)).map(|(wall, layout)| SolidEntry { solid: walls::wall_solid(snapshot, wall, layout, &walls::cuts_of(index.frames.values().copied())), fallback: None }),
        SolidFamily::CurtainWall => snapshot.curtain_walls.get(id).zip(index.curtains.get(id)).map(|(curtain, layout)| {
            let cuts: Vec<opening_frames::OpeningCut> = snapshot.openings.iter().filter(|(_, opening)| opening.host == id).filter_map(|(opening, _)| index.frames.get(opening.as_str())).filter(|frame| frame.valid).map(|frame| frame.cut).collect();
            SolidEntry { solid: curtain_walls::curtain_solid_in(snapshot, curtain, layout, &cuts, &profiles), fallback: None }
        }),
        SolidFamily::Window | SolidFamily::Door => snapshot.openings.get(id).zip(index.frames.get(id)).and_then(|(opening, frame)| fillers::filler_solid(snapshot, opening, frame)).map(|solid| SolidEntry { solid, fallback: None }),
        SolidFamily::Column => snapshot.columns.get(id).map(|column| SolidEntry { solid: columns::column_solid_in(snapshot, column, &own, target(&column.top).as_ref(), &profiles), fallback: None }),
        SolidFamily::Beam => snapshot.beams.get(id).map(|beam| {
            let joiners = joiners_of(snapshot, beam, index);
            SolidEntry { solid: beams::beam_solid_in(snapshot, beam, &own, &joiners, &profiles), fallback: None }
        }),
        SolidFamily::Slab => snapshot.slabs.get(id).map(|slab| SolidEntry { solid: slabs::slab_solid(snapshot, slab, &own), fallback: None }),
        SolidFamily::Ceiling => snapshot.ceilings.get(id).map(|ceiling| SolidEntry { solid: ceilings::ceiling_solid(snapshot, ceiling, &own), fallback: None }),
        SolidFamily::WallSweep => snapshot.wall_sweeps.get(id).and_then(|sweep| snapshot.walls.get(&sweep.host).zip(index.layouts.get(sweep.host.as_str())).map(|(wall, layout)| SolidEntry { solid: wall_sweeps::sweep_solid(sweep, wall, layout, &walls::cuts_of(index.frames.values().copied())), fallback: None })),
        SolidFamily::Roof => snapshot.roofs.get(id).map(|roof| roofs::roof_solid(snapshot, roof, &own)),
        SolidFamily::Stair => snapshot.stairs.get(id).zip(index.runs.get(id)).map(|(stair, run)| SolidEntry { solid: stairs::stair_solid(stair, run), fallback: None }),
        SolidFamily::Ramp => snapshot.ramps.get(id).zip(index.ramp_runs.get(id)).map(|(ramp, run)| SolidEntry { solid: ramps::ramp_solid(ramp, run), fallback: None }),
        SolidFamily::Railing => snapshot.railings.get(id).map(|railing| profiles.resolve_railing(railing)).map(|railing| SolidEntry { solid: if railing.host.is_some() { rail_hosts::hosted_solid(&railing, host_of(snapshot, &railing, index).as_ref()) } else { railings::railing_solid(snapshot, &railing, &own) }, fallback: None }),
    };
    let mut entry = built.unwrap_or_else(|| empty(solid.family));
    entry.solid = entry.solid.placed(&storey, placement_of(snapshot, &storey, Some(&own)));
    entry
}

fn room_value(snapshot: &ModelSnapshot, storey: &str, index: &Index<'_>) -> StoreyRooms {
    let obstacles = obstacles_of(snapshot, storey, &index.layouts);
    rooms_from(snapshot, storey, &index.level(storey), &obstacles)
}

fn view_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> view_linework::ViewLinework {
    let Some(view) = snapshot.views.get(id) else { return view_linework::ViewLinework { view: id.to_string(), kind: crate::ViewKind::Plan, scale: 0, detail: crate::DetailLevel::Medium, lines: plan_linework::Sheet::default().finish("", 0.0) } };
    let storey = view.storey.as_deref().unwrap_or_default();
    let plan = PlanInputs { levels: index.levels.clone(), layouts: index.layouts.clone(), curtains: index.curtains.clone(), frames: index.frames.clone(), runs: index.runs.clone(), ramp_runs: index.ramp_runs.clone(), rooms: index.rooms.get(storey).copied(), annotations: None };
    let inputs = ViewInputs { plan, solids: index.solids.iter().map(|(element, entry)| (*element, &entry.solid)).collect() };
    view_linework::view_of(snapshot, id, view, &inputs)
}

fn annotation_value(snapshot: &ModelSnapshot, storey: &str, index: &Index<'_>) -> StoreyAnnotations {
    annotation_layout::annotations_of(snapshot, storey, &annotation_layout::Inputs { layouts: index.layouts.clone() })
}

fn plan_value(snapshot: &ModelSnapshot, storey: &str, index: &Index<'_>) -> plan_linework::PlanLinework {
    let inputs = PlanInputs { levels: index.levels.clone(), layouts: index.layouts.clone(), curtains: index.curtains.clone(), frames: index.frames.clone(), runs: index.runs.clone(), ramp_runs: index.ramp_runs.clone(), rooms: index.rooms.get(storey).copied(), annotations: index.annotations.get(storey).copied() };
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
    if let Some(sweep) = snapshot.wall_sweeps.get(id) {
        return snapshot.walls.get(&sweep.host).zip(index.layouts.get(sweep.host.as_str())).map(|(wall, layout)| quantities::sweep_quantity(snapshot, sweep, wall, layout, &hosted(&sweep.host), solid(SolidFamily::WallSweep)));
    }
    if let Some(slab) = snapshot.slabs.get(id) {
        return Some(quantities::slab_quantity(snapshot, slab));
    }
    if let Some(ceiling) = snapshot.ceilings.get(id) {
        return Some(quantities::ceiling_quantity(snapshot, ceiling));
    }
    if let Some(roof) = snapshot.roofs.get(id) {
        return Some(quantities::roof_quantity(snapshot, roof, solid(SolidFamily::Roof)));
    }
    if let Some(column) = snapshot.columns.get(id) {
        return quantities::column_quantity_in(snapshot, column, &index.level(&column.storey), index.target(&column.storey, &column.top).as_ref(), &index.profiles());
    }
    if let Some(beam) = snapshot.beams.get(id) {
        return quantities::beam_quantity_in(snapshot, beam, solid(SolidFamily::Beam), &index.profiles());
    }
    if let Some(opening) = snapshot.openings.get(id) {
        return index.frames.get(id).map(|frame| quantities::opening_quantity(snapshot, opening, frame, index.solids.get(id).map(|entry| &entry.solid)));
    }
    if let Some(stair) = snapshot.stairs.get(id) {
        return index.runs.get(id).map(|run| quantities::stair_quantity(stair, run, solid(SolidFamily::Stair)));
    }
    if let Some(ramp) = snapshot.ramps.get(id) {
        return index.ramp_runs.get(id).map(|run| quantities::ramp_quantity(snapshot, ramp, run, solid(SolidFamily::Ramp)));
    }
    if let Some(railing) = snapshot.railings.get(id) {
        return Some(match railing.host.as_ref() {
            None => quantities::railing_quantity(snapshot, railing, solid(SolidFamily::Railing)),
            Some(spec) => {
                let length = host_of(snapshot, railing, index).and_then(|host| rail_hosts::host_paths(railing, spec, &host).ok()).map_or(0.0, |paths| rail_hosts::length_of(&paths));
                quantities::hosted_railing_quantity(snapshot, railing, length, solid(SolidFamily::Railing))
            }
        });
    }
    let space = snapshot.spaces.get(id)?;
    let frames: Vec<&OpeningFrame> = index.frames.values().copied().collect();
    index.rooms.get(space.storey.as_str()).and_then(|rooms| rooms.get(id)).and_then(|room| quantities::space_quantity(snapshot, space, room, &frames))
}

fn schedule_value(snapshot: &ModelSnapshot, id: &str, parents: &[ModelValue]) -> ScheduleTable {
    let Some(schedule) = snapshot.schedules.get(id) else { return ScheduleTable::default() };
    let quantities: BTreeMap<&str, &ElementQuantity> = parents.iter().filter_map(|value| if let (ModelNode::Quantity(element), Data::Quantity(Some(quantity))) = (&value.node, &value.data) { Some((element.as_str(), &**quantity)) } else { None }).collect();
    let ids = schedules::rows::candidates(snapshot, schedule);
    let mut view = schedules::AuthoredView::of(snapshot, schedule, &ids);
    schedules::rows::with_effective(&mut view, schedule, &Index::of(parents).properties);
    schedules::schedule_of(schedule, &view, &quantities)
}

fn members_of<'a>(snapshot: &'a ModelSnapshot, index: &Index<'a>, member: impl Fn(&crate::Space) -> bool) -> Vec<zones::Member<'a>> {
    snapshot.spaces.iter().filter(|(_, space)| member(space)).map(|(id, space)| (space, index.measured.get(id.as_str()).copied().flatten())).collect()
}

fn zone_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> zones::ZoneTotals {
    snapshot.zones.get(id).map_or_else(zones::ZoneTotals::default, |zone| zones::zone_totals(zone, &members_of(snapshot, index, |space| space.zone.as_deref() == Some(id))))
}

fn scheme_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> zones::SchemeTotals {
    let densities: BTreeMap<&str, f64> = snapshot.zones.iter().map(|(zone, row)| (zone.as_str(), row.occupancy_density)).collect();
    snapshot.area_schemes.get(id).map_or_else(zones::SchemeTotals::default, |scheme| zones::scheme_totals(scheme, &densities, &members_of(snapshot, index, |space| zones::counts(scheme, space))))
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
    let inputs = FindingInputs { levels: index.levels.clone(), layouts: index.layouts.clone(), frames: index.frames.clone(), runs: index.runs.clone(), ramp_runs: index.ramp_runs.clone(), rooms, fallbacks, curtains: index.curtains.clone() };
    match scope {
        DiagnosticScope::Storey(storey) => {
            let mut found = diagnostics::storey_findings(snapshot, storey, &inputs);
            found.extend(index.annotations.get(storey.as_str()).into_iter().flat_map(|set| set.findings.iter().cloned()));
            found
        }
        DiagnosticScope::Building(building) => diagnostics::building_findings(snapshot, building, &inputs),
        DiagnosticScope::Model => {
            let mut found = diagnostics::model_findings(&diagnostics::references::ReferenceView::of(snapshot));
            found.extend(family_findings(snapshot, index));
            found
        }
        DiagnosticScope::Data => effective_properties::findings(snapshot, &index.properties, &|id| element_storey(snapshot, id)),
    }
}
/// 🧬️ The findings of the families: their issues and the dangling profile references.
fn family_findings(snapshot: &ModelSnapshot, index: &Index<'_>) -> Vec<diagnostics::Diagnostic> {
    use diagnostics::DiagnosticCode as Code;
    use families::findings::FindingCode;
    use families::FamilyIssueCode as Issue;
    let issues = families::findings::issue_findings(&index.families);
    let references = families::findings::reference_findings(snapshot, &index.families);
    issues
        .into_iter()
        .chain(references)
        .map(|finding| {
            let code = match finding.code {
                FindingCode::ProfileReference => Code::RefProfileFamily,
                FindingCode::Issue(Issue::Syntax) => Code::FamilySyntax,
                FindingCode::Issue(Issue::Kind) => Code::FamilyKind,
                FindingCode::Issue(Issue::Cycle) => Code::FamilyCycle,
                FindingCode::Issue(Issue::Unknown) => Code::FamilyUnknown,
                FindingCode::Issue(Issue::DivisionByZero) => Code::FamilyDivisionByZero,
                FindingCode::Issue(Issue::Negative) => Code::FamilyNegative,
                FindingCode::Issue(Issue::Dependency) => Code::FamilyDependency,
                FindingCode::Issue(Issue::Domain) => Code::FamilyDomain,
                FindingCode::Issue(Issue::Outline) => Code::FamilyOutline,
            };
            let elements: Vec<&str> = finding.elements.iter().map(String::as_str).collect();
            finding.missing.iter().fold(diagnostics::Diagnostic::new(code, &elements), |found, missing| found.lacking(missing))
        })
        .collect()
}
//#endregion 🔖️Value

/// 🪜️ The storey of an opening, for callers outside the graph that need it.
pub fn storey_of_opening(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    opening_storey(snapshot, id)
}
