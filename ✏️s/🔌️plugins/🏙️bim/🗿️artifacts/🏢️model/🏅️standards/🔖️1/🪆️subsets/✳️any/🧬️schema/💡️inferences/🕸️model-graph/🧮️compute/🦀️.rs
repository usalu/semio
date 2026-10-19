//! 🧮️ What a node of the model graph is: its dependency (the honesty contract of `dep_input`: everything `value` reads of the snapshot that is not a parent) and its value, by dispatch to the pure
//! functions of the leaf modules. Parents are found by key in an [`Index`] built once per call.

use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::StoreyAnnotations;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::{ClashSetResult, SolidProbe};
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results::RuleResult;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::ZoneTotals;
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout::{curtain_layout_of, CurtainLayout};
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Inputs as FindingInputs;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::EffectiveProperties;
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::EnvelopeSpace;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs::RoofFallback;
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::{beams, railings, walls};
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::{ceilings, slabs, stairs};
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::{columns, ramps};
use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::{curtain_walls, wall_sweeps};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, placement_of, roofs, SolidEntry, SolidFamily};
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::{cut_rect, frame_of, CutRect, HostExtent, OpeningCut, OpeningFrame};
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components;
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::{ComponentEntry, ComponentValue};
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::{FamilyProfiles, FamilyValue};
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::clash::MepClashes;
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::MepValue;
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::Inputs as PlanInputs;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::{totals_of, ElementQuantity};
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::ScheduleTable;
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces;
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::{obstacles_of, rooms_from, StoreyRooms};
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs::RampRun;
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs::{run_of, StairRun};
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::{resolve, StoreyLevel};
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework::{ViewInputs, ViewLinework};
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::attach::AttachSurface;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references;
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::joins::Band;
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::{band_of, layout_of, WallLayout};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::plan::{element_storey, opening_storey};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{Data, DiagnosticScope, ModelNode, ModelValue, TotalsScope};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::TopConstraint;
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
    pub family_arcs: BTreeMap<&'a str, &'a Arc<FamilyValue>>,
    pub components: BTreeMap<&'a str, &'a ComponentEntry>,
    pub meps: BTreeMap<&'a str, &'a MepValue>,
    pub mep_clashes: BTreeMap<&'a str, &'a MepClashes>,
    pub probes: BTreeMap<&'a str, &'a SolidProbe>,
    pub zones: BTreeMap<&'a str, &'a ZoneTotals>,
    pub rooms: BTreeMap<&'a str, &'a StoreyRooms>,
    pub annotations: BTreeMap<&'a str, &'a StoreyAnnotations>,
    pub properties: BTreeMap<&'a str, &'a EffectiveProperties>,
    pub drawings: BTreeMap<&'a str, &'a ViewLinework>,
    pub quantities: Vec<&'a ElementQuantity>,
    pub measured: BTreeMap<&'a str, Option<&'a ElementQuantity>>,
    pub surfaces: BTreeMap<&'a str, &'a AttachSurface>,
    pub envelopes: BTreeMap<&'a str, &'a EnvelopeSpace>,
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
                (ModelNode::Envelope(id), Data::Envelope(envelope)) => {
                    index.envelopes.insert(id, envelope);
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
                    index.family_arcs.insert(id, value);
                }
                (ModelNode::Component(id), Data::Component(entry)) => {
                    index.components.insert(id, entry);
                }
                (ModelNode::Mep(id), Data::Mep(value)) => {
                    index.meps.insert(id, value);
                }
                (ModelNode::MepClash(id), Data::MepClashes(found)) => {
                    index.mep_clashes.insert(id, found);
                }
                (ModelNode::Solid(key), Data::Solid(entry)) => {
                    index.solids.insert(&key.id, entry);
                }
                (ModelNode::Probe(key), Data::Probe(probe)) => {
                    index.probes.insert(&key.id, probe);
                }
                (ModelNode::Zone(id), Data::Zone(totals)) => {
                    index.zones.insert(id, totals);
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

    /// 🪑️ The values of the components among the parents.
    pub fn component_values(&self) -> BTreeMap<&'a str, &'a ComponentValue> {
        self.components.iter().map(|(id, entry)| (*id, &entry.value)).collect()
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
    dep_value(&semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::building_placement(snapshot, storey))
}

/// 🔑️ Everything `value` reads of the snapshot for `key` that is not a parent, together with the key itself (a cached value is never handed to another node).
pub fn dependency(snapshot: &ModelSnapshot, key: &ModelNode) -> DslValue {
    use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::Anonymous;
    HASHED.with(|counter| counter.set(counter.get() + 1));
    let input = match key {
        ModelNode::Storey(id) => semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::dependency(snapshot, id),
        ModelNode::Band(id) => snapshot.walls.get(id).map_or(DslValue::Null, |wall| semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::band_dependency(snapshot, wall)),
        ModelNode::Cut(id) => snapshot.openings.get(id).map_or(DslValue::Null, |opening| semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::cut_dependency(snapshot, opening)),
        ModelNode::WallLayout(id) => snapshot.walls.get(id).map_or(DslValue::Null, |wall| semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::dependency(snapshot, &wall.anonymous())),
        ModelNode::CurtainLayout(id) => snapshot.curtain_walls.get(id).map_or(DslValue::Null, |curtain| semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout::dependency(snapshot, id, curtain)),
        ModelNode::Host(id) => match (snapshot.walls.get(id), snapshot.curtain_walls.get(id)) {
            (Some(wall), _) => dep_object([("placement", placement(snapshot, &wall.storey))]),
            (None, Some(curtain)) => dep_object([("placement", placement(snapshot, &curtain.storey)), ("axis", dep_value(&curtain.axis)), ("mullion", dep_value(&snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).map(|kind| kind.interior_mullion.clone())))]),
            (None, None) => DslValue::Null,
        },
        ModelNode::OpeningFrame(id) => snapshot.openings.get(id).map_or(DslValue::Null, |opening| semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::dependency(snapshot, &opening.anonymous())),
        ModelNode::StairRun(id) => snapshot.stairs.get(id).map_or(DslValue::Null, |stair| semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs::dependency(&stair.anonymous())),
        ModelNode::RampRun(id) => snapshot.ramps.get(id).map_or(DslValue::Null, |ramp| semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs::dependency(&ramp.anonymous())),
        ModelNode::Solid(solid) => solid_dependency(snapshot, solid),
        ModelNode::Room(storey) => semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::dependency(snapshot, storey),
        ModelNode::Plan(storey) => semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::dependency(snapshot, storey),
        ModelNode::Annotation(storey) => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::dependency(snapshot, storey),
        ModelNode::View(id) => semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework::dependency(snapshot, id),
        ModelNode::Sheet(id) => semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::dependency(snapshot, id),
        ModelNode::Probe(solid) => solid_dependency(snapshot, solid),
        ModelNode::ClashSet(id) => semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::dependency(snapshot, id),
        ModelNode::Rule(id) => semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results::dependency(snapshot, id),
        ModelNode::Quantity(id) => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::dependency(snapshot, id),
        ModelNode::Totals(_) | ModelNode::DiagnosticIndex => DslValue::Null,
        ModelNode::Diagnostics(DiagnosticScope::Storey(id)) => semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::storey_dependency(snapshot, id),
        ModelNode::Diagnostics(DiagnosticScope::Building(id)) => semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::building_dependency(snapshot, id),
        ModelNode::Diagnostics(DiagnosticScope::Model) => dep_object([("model", semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::model_dependency(snapshot)), ("profiles", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::findings::reference_dependency(snapshot))]),
        ModelNode::Diagnostics(DiagnosticScope::Data) => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::findings_dependency(snapshot),
        ModelNode::Diagnostics(DiagnosticScope::Energy) => DslValue::Null,
        ModelNode::Properties(id) => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::dependency(snapshot, id),
        ModelNode::Family(id) => semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::dependency(snapshot, id),
        ModelNode::Component(id) => semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::dependency(snapshot, id),
        ModelNode::StructuralAnalysis => semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::analytical_members::dependency(snapshot),
        ModelNode::Mep(id) => semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::dependency(snapshot, id),
        ModelNode::MepClash(_) => DslValue::Null,
        ModelNode::OptionScope => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::option_scope::dependency(snapshot),
        ModelNode::Schedule(id) => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::dependency(snapshot, id),
        ModelNode::Zone(id) => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::zone_dependency(snapshot, id),
        ModelNode::Scheme(id) => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::scheme_dependency(snapshot, id),
        ModelNode::PhaseVisibility(storey) => semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility::dependency(snapshot, storey),
        ModelNode::Surface(id) => semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::attach::dependency(snapshot, id),
        ModelNode::Envelope(id) => semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::dependency(snapshot, id),
        ModelNode::EnergyTotals(scope) => semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::totals_dependency(snapshot, scope),
    };
    keyed(key, input)
}

fn solid_dependency(snapshot: &ModelSnapshot, solid: &semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey) -> DslValue {
    use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::Anonymous;
    let id = &solid.id;
    let storey = element_storey(snapshot, id).unwrap_or_default();
    let family = match solid.family {
        SolidFamily::Wall => snapshot.walls.get(id).map_or(DslValue::Null, |wall| semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls::dependency(snapshot, wall)),
        SolidFamily::CurtainWall => snapshot.curtain_walls.get(id).map_or(DslValue::Null, |curtain| semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls::dependency(snapshot, id, &curtain.anonymous())),
        SolidFamily::Window | SolidFamily::Door => snapshot.openings.get(id).map_or(DslValue::Null, |opening| semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::fillers::dependency(snapshot, opening)),
        SolidFamily::Column => snapshot.columns.get(id).map_or(DslValue::Null, |column| semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns::dependency(snapshot, &column.anonymous())),
        SolidFamily::Beam => snapshot.beams.get(id).map_or(DslValue::Null, |beam| semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::beams::dependency(snapshot, &beam.anonymous())),
        SolidFamily::Slab => snapshot.slabs.get(id).map_or(DslValue::Null, |slab| semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs::dependency(snapshot, &slab.anonymous())),
        SolidFamily::Ceiling => snapshot.ceilings.get(id).map_or(DslValue::Null, |ceiling| semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings::dependency(snapshot, &ceiling.anonymous())),
        SolidFamily::WallSweep => snapshot.wall_sweeps.get(id).map_or(DslValue::Null, |sweep| semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps::dependency(snapshot, sweep)),
        SolidFamily::Roof => snapshot.roofs.get(id).map_or(DslValue::Null, |roof| semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs::dependency(snapshot, &roof.anonymous())),
        SolidFamily::Stair => snapshot.stairs.get(id).map_or(DslValue::Null, |stair| semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::stairs::dependency(stair)),
        SolidFamily::Ramp => snapshot.ramps.get(id).map_or(DslValue::Null, |ramp| semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::ramps::dependency(&ramp.anonymous())),
        SolidFamily::Component | SolidFamily::Mep => DslValue::Null,
        SolidFamily::Railing => snapshot.railings.get(id).map_or(DslValue::Null, |railing| dep_object([("railing", semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::railings::dependency(snapshot, &railing.anonymous())), ("host", semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::dependency(snapshot, railing))])),
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
        ModelNode::OptionScope => Data::OptionScope(Arc::new(semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::option_scope::scope_of(snapshot, parents.iter().filter_map(|parent| match (&parent.node, &parent.data) { (ModelNode::Quantity(id), Data::Quantity(Some(quantity))) => Some((id.as_str(), quantity.as_ref())), _ => None })))),
        ModelNode::Storey(id) => Data::Level(resolve(snapshot, id, index.levels.values().next())),
        ModelNode::Band(id) => Data::Band(snapshot.walls.get(id).and_then(|wall| band_of(snapshot, wall))),
        ModelNode::Cut(id) => Data::Cut(snapshot.openings.get(id).map(|opening| cut_rect(snapshot, opening)).unwrap_or_else(|| CutRect { size: semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::sizes::Resolved { width: 0.0, height: 0.0, sill: 0.0, type_found: false }, cut: OpeningCut::default() })),
        ModelNode::WallLayout(id) => Data::Layout(Arc::new(wall_layout_value(snapshot, id, &index))),
        ModelNode::CurtainLayout(id) => Data::Curtain(Arc::new(snapshot.curtain_walls.get(id).map_or_else(CurtainLayout::default, |curtain| curtain_layout_of(snapshot, id, curtain, &index.level(&curtain.storey), index.target(&curtain.storey, &curtain.top).as_ref())))),
        ModelNode::Host(id) => Data::Host(host_value(snapshot, id, &index).map(Arc::new)),
        ModelNode::OpeningFrame(id) => Data::Frame(Arc::new(frame_value(snapshot, id, &index))),
        ModelNode::RampRun(id) => Data::RampRun(Arc::new(snapshot.ramps.get(id).map_or_else(RampRun::default, |ramp| semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs::run_of(ramp, &index.level(&ramp.storey), index.target(&ramp.storey, &ramp.top).as_ref())))),
        ModelNode::StairRun(id) => Data::Run(Arc::new(snapshot.stairs.get(id).map_or_else(StairRun::default, |stair| run_of(stair, &index.level(&stair.storey), index.target(&stair.storey, &stair.top).as_ref())))),
        ModelNode::Solid(solid) => Data::Solid(Arc::new(solid_value(snapshot, solid, &index))),
        ModelNode::Room(storey) => Data::Rooms(Arc::new(room_value(snapshot, storey, &index))),
        ModelNode::Plan(storey) => Data::Plan(Arc::new(plan_value(snapshot, storey, &index))),
        ModelNode::Annotation(storey) => Data::Annotations(Arc::new(annotation_value(snapshot, storey, &index))),
        ModelNode::Properties(id) => Data::Properties(Arc::new(semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::effective_of(snapshot, id, index.properties.values().next().copied()))),
        ModelNode::View(id) => Data::View(Arc::new(view_value(snapshot, id, &index))),
        ModelNode::Sheet(id) => Data::Sheet(Arc::new(semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::layout_of(snapshot, id, &index.drawings))),
        ModelNode::Probe(key) => Data::Probe(Arc::new(probe_value(snapshot, key, &index))),
        ModelNode::ClashSet(id) => Data::Clashes(Arc::new(clash_set_value(snapshot, id, &index))),
        ModelNode::Rule(id) => Data::Rule(Arc::new(rule_value(snapshot, id, &index))),
        ModelNode::Quantity(id) => Data::Quantity(quantity_value(snapshot, id, &index).map(Arc::new)),
        ModelNode::Totals(scope) => Data::Totals(Arc::new(totals_value(scope, parents))),
        ModelNode::Schedule(id) => Data::Schedule(Arc::new(schedule_value(snapshot, id, parents))),
        ModelNode::Zone(id) => Data::Zone(Arc::new(zone_value(snapshot, id, &index))),
        ModelNode::Scheme(id) => Data::Scheme(Arc::new(scheme_value(snapshot, id, &index))),
        ModelNode::Family(id) => Data::Family(Arc::new(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::family_of(snapshot, id, &BTreeMap::new()))),
        ModelNode::Component(id) => Data::Component(Arc::new(component_value(snapshot, id, &index))),
        ModelNode::Mep(id) => Data::Mep(Arc::new(snapshot.mep_elements.get(id).map_or_else(MepValue::default, |element| semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::mep_of(snapshot, id, &index.level(&element.storey))))),
        ModelNode::StructuralAnalysis => Data::Structure(Arc::new(semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::analytical_members::analysis_of(snapshot, &index.levels, &index.layouts, &index.solids))),
        ModelNode::MepClash(_) => Data::MepClashes(Arc::new(semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::clash::clashes_of(index.meps.iter().map(|(id, value)| (*id, *value))))),
        ModelNode::PhaseVisibility(storey) => Data::Phases(Arc::new(semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility::visibility_of(snapshot, storey))),
        ModelNode::Surface(id) => Data::Surface(Arc::new(semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references::storey_of(snapshot, id).map_or_else(AttachSurface::default, |storey| semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::attach::surface_of(snapshot, id, &index.level(storey))))),
        ModelNode::Envelope(id) => Data::Envelope(Arc::new(semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::envelope_of(snapshot, id, &semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::Inputs { rooms: &index.rooms, layouts: &index.layouts, frames: &index.frames }))),
        ModelNode::EnergyTotals(scope) => Data::EnergyTotals(Arc::new(semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::totals_of(scope, &index.envelopes.values().copied().collect::<Vec<_>>(), &semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::Climate::of(snapshot)))),
        ModelNode::Diagnostics(scope) => Data::Findings(Arc::new(findings_value(snapshot, scope, &index))),
        ModelNode::DiagnosticIndex => Data::Index(Arc::new(semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticIndex::of(&crate::standards::v1::subsets::any::schema::inferences::model_graph::projection::rebuild_diagnostics(parents)))),
    };
    wrap(key, data)
}

/// 📦️ The spatial index of the solid of a `Probe` node: the world mesh, its box and its hierarchy; an element without a solid gets an empty probe.
fn probe_value(snapshot: &ModelSnapshot, key: &semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey, index: &Index<'_>) -> SolidProbe {
    index.solids.get(key.id.as_str()).map_or_else(
        || SolidProbe { class: None, storey: String::new(), bounds: None, mesh: Default::default(), bvh: Default::default() },
        |entry| semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::probe_of(snapshot, &key.id, &entry.solid),
    )
}

/// 🔎️ The probes of the parents among the elements `ids`, in id order.
fn probes_of<'i>(index: &Index<'i>, ids: &'i [String]) -> Vec<(&'i str, &'i SolidProbe)> {
    ids.iter().filter_map(|element| index.probes.get(element.as_str()).map(|probe| (element.as_str(), *probe))).collect()
}

/// 🧨️ The clashes of a clash set among the probes of its parents.
fn clash_set_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> ClashSetResult {
    let Some(set) = snapshot.clash_sets.get(id) else { return ClashSetResult::default() };
    let (a, b) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::picks(snapshot, set);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::clashes_of(set, &probes_of(index, &a), &probes_of(index, &b), &semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::hosting(snapshot), &|element| semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::storey_of(snapshot, element).cloned().unwrap_or_default(), &|| false)
}

/// ⚖️ The findings of a rule over the measures of its parents.
fn rule_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> RuleResult {
    let Some(rule) = snapshot.rules.get(id) else { return RuleResult::default() };
    let inputs = semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results::Inputs { rooms: index.rooms.clone(), runs: index.runs.clone(), ramp_runs: index.ramp_runs.clone(), frames: index.frames.clone(), zones: index.zones.clone() };
    semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results::result_of(rule, &semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results::members(snapshot, rule), &inputs)
}

fn component_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> ComponentEntry {
    let Some(component) = snapshot.components.get(id) else { return ComponentEntry::default() };
    let shared = index.family_arcs.get(component.family.as_str()).map(|family| Arc::clone(family));
    let host = component.host.as_deref().and_then(|wall| index.layouts.get(wall).copied());
    semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::component_of(snapshot, id, &index.level(&component.storey), shared.as_ref(), host)
}

fn wall_layout_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> WallLayout {
    let Some(wall) = snapshot.walls.get(id) else { return WallLayout::default() };
    let bands: BTreeMap<String, Band> = index.bands.iter().filter_map(|(band, row)| row.map(|row| ((*band).to_string(), row))).collect();
    semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::attach::apply(snapshot, id, wall, layout_of(snapshot, id, wall, &index.level(&wall.storey), index.target(&wall.storey, &wall.top).as_ref(), &bands), &index.surfaces)
}

fn host_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> Option<HostExtent> {
    if let Some(wall) = snapshot.walls.get(id) {
        let layout = index.layouts.get(id)?;
        return Some(HostExtent::of_wall(wall, layout, &index.level(&wall.storey), semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::building_placement(snapshot, &wall.storey)));
    }
    let curtain = snapshot.curtain_walls.get(id)?;
    let layout = index.curtains.get(id)?;
    Some(HostExtent::of_curtain(curtain, snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).map(|kind| &kind.interior_mullion), layout, &index.level(&curtain.storey), semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::building_placement(snapshot, &curtain.storey)))
}

fn frame_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> OpeningFrame {
    let Some(opening) = snapshot.openings.get(id) else { return OpeningFrame::default() };
    let own = index.cuts.get(id).copied().unwrap_or_else(|| cut_rect(snapshot, opening));
    let host = index.hosts.get(opening.host.as_str()).copied().flatten();
    let siblings: Vec<(String, OpeningCut)> = index.cuts.iter().filter(|(other, _)| **other != id).map(|(other, cut)| ((*other).to_string(), cut.cut)).collect();
    frame_of(snapshot, id, &own, host, &siblings)
}

fn host_of<'i>(snapshot: &'i ModelSnapshot, railing: &semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Railing, index: &'i Index<'_>) -> Option<semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::Host<'i>> {
    let spec = railing.host.as_ref()?;
    if let Some(run) = index.runs.get(spec.element.as_str()) {
        return Some(semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::Host::Stair(run));
    }
    if let (Some(ramp), Some(run)) = (snapshot.ramps.get(&spec.element), index.ramp_runs.get(spec.element.as_str())) {
        return Some(semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::Host::Ramp { ramp, run });
    }
    let slab = snapshot.slabs.get(&spec.element)?;
    Some(semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::Host::Slab { slab, own: index.levels.get(&slab.storey)? })
}

/// 🔗️ The columns a beam can join with their types and resolved vertical extent (from the levels the beam solid node has as parents).
fn joiners_of<'a>(snapshot: &'a ModelSnapshot, beam: &semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Beam, index: &Index<'_>) -> Vec<semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns::Joiner<'a>> {
    semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::beams::joining(snapshot, beam)
        .into_iter()
        .filter_map(|(_, column)| {
            let kind = snapshot.column_types.get(&column.column_type)?;
            let (base_z, top_z) = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::vertical_of(column.base_offset, &column.top, &index.level(&column.storey), index.target(&column.storey, &column.top).as_ref());
            Some(semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns::Joiner { column, profile: &kind.profile, base_z, top_z })
        })
        .collect()
}

fn solid_value(snapshot: &ModelSnapshot, solid: &semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey, index: &Index<'_>) -> SolidEntry {
    let id = solid.id.as_str();
    let empty = |family: SolidFamily| SolidEntry { solid: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::SolidBuilder::new(family).build(), fallback: None };
    let Some(storey) = element_storey(snapshot, id) else { return empty(solid.family) };
    let own = index.level(&storey);
    let target = |top: &TopConstraint| index.target(&storey, top);
    let profiles = index.profiles();
    let built: Option<SolidEntry> = match solid.family {
        SolidFamily::Wall => snapshot.walls.get(id).zip(index.layouts.get(id)).map(|(wall, layout)| SolidEntry { solid: semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls::wall_solid(snapshot, wall, layout, &semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls::cuts_of(index.frames.values().copied())), fallback: None }),
        SolidFamily::CurtainWall => snapshot.curtain_walls.get(id).zip(index.curtains.get(id)).map(|(curtain, layout)| {
            let cuts: Vec<OpeningCut> = snapshot.openings.iter().filter(|(_, opening)| opening.host == id).filter_map(|(opening, _)| index.frames.get(opening.as_str())).filter(|frame| frame.valid).map(|frame| frame.cut).collect();
            SolidEntry { solid: semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls::curtain_solid_in(snapshot, curtain, layout, &cuts, &profiles), fallback: None }
        }),
        SolidFamily::Window | SolidFamily::Door => snapshot.openings.get(id).zip(index.frames.get(id)).and_then(|(opening, frame)| semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::fillers::filler_solid(snapshot, opening, frame)).map(|solid| SolidEntry { solid, fallback: None }),
        SolidFamily::Column => snapshot.columns.get(id).map(|column| SolidEntry { solid: semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns::column_solid_in(snapshot, column, &own, target(&column.top).as_ref(), &profiles), fallback: None }),
        SolidFamily::Beam => snapshot.beams.get(id).map(|beam| {
            let joiners = joiners_of(snapshot, beam, index);
            SolidEntry { solid: semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::beams::beam_solid_in(snapshot, beam, &own, &joiners, &profiles), fallback: None }
        }),
        SolidFamily::Slab => snapshot.slabs.get(id).map(|slab| SolidEntry { solid: semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs::slab_solid(snapshot, slab, &own), fallback: None }),
        SolidFamily::Ceiling => snapshot.ceilings.get(id).map(|ceiling| SolidEntry { solid: semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings::ceiling_solid(snapshot, ceiling, &own), fallback: None }),
        SolidFamily::WallSweep => snapshot.wall_sweeps.get(id).and_then(|sweep| snapshot.walls.get(&sweep.host).zip(index.layouts.get(sweep.host.as_str())).map(|(wall, layout)| SolidEntry { solid: semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps::sweep_solid(sweep, wall, layout, &semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls::cuts_of(index.frames.values().copied())), fallback: None })),
        SolidFamily::Roof => snapshot.roofs.get(id).map(|roof| semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs::roof_solid(snapshot, roof, &own)),
        SolidFamily::Stair => snapshot.stairs.get(id).zip(index.runs.get(id)).map(|(stair, run)| SolidEntry { solid: semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::stairs::stair_solid(stair, run), fallback: None }),
        SolidFamily::Component => index.components.get(id).map(|entry| SolidEntry { solid: semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::element_solids::components::component_solid(entry), fallback: None }),
        SolidFamily::Mep => index.meps.get(id).map(|value| SolidEntry { solid: semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::element_solids::mep::mep_solid(value), fallback: None }),
        SolidFamily::Ramp => snapshot.ramps.get(id).zip(index.ramp_runs.get(id)).map(|(ramp, run)| SolidEntry { solid: semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::ramps::ramp_solid(ramp, run), fallback: None }),
        SolidFamily::Railing => snapshot.railings.get(id).map(|railing| profiles.resolve_railing(railing)).map(|railing| SolidEntry { solid: if railing.host.is_some() { semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::hosted_solid(&railing, host_of(snapshot, &railing, index).as_ref()) } else { semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::railings::railing_solid(snapshot, &railing, &own) }, fallback: None }),
    };
    let mut entry = built.unwrap_or_else(|| empty(solid.family));
    entry.solid = entry.solid.placed(&storey, placement_of(snapshot, &storey, Some(&own)));
    entry
}

fn room_value(snapshot: &ModelSnapshot, storey: &str, index: &Index<'_>) -> StoreyRooms {
    let obstacles = obstacles_of(snapshot, storey, &index.layouts);
    rooms_from(snapshot, storey, &index.level(storey), &obstacles)
}

fn view_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> ViewLinework {
    let Some(view) = snapshot.views.get(id) else { return ViewLinework { view: id.to_string(), kind: semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::ViewKind::Plan, scale: 0, detail: semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::DetailLevel::Medium, lines: semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::Sheet::default().finish("", 0.0) } };
    let storey = view.storey.as_deref().unwrap_or_default();
    let plan = PlanInputs { levels: index.levels.clone(), layouts: index.layouts.clone(), curtains: index.curtains.clone(), frames: index.frames.clone(), runs: index.runs.clone(), ramp_runs: index.ramp_runs.clone(), components: index.component_values(), meps: index.meps.clone(), rooms: index.rooms.get(storey).copied(), annotations: index.annotations.get(storey).copied() };
    let inputs = ViewInputs { plan, solids: index.solids.iter().map(|(element, entry)| (*element, &entry.solid)).collect() };
    semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework::view_of(snapshot, id, view, &inputs)
}

fn annotation_value(snapshot: &ModelSnapshot, storey: &str, index: &Index<'_>) -> StoreyAnnotations {
    semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::annotations_of(snapshot, storey, &semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::Inputs { layouts: index.layouts.clone() })
}

fn plan_value(snapshot: &ModelSnapshot, storey: &str, index: &Index<'_>) -> semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::PlanLinework {
    let inputs = PlanInputs { levels: index.levels.clone(), layouts: index.layouts.clone(), curtains: index.curtains.clone(), frames: index.frames.clone(), runs: index.runs.clone(), ramp_runs: index.ramp_runs.clone(), components: index.component_values(), meps: index.meps.clone(), rooms: index.rooms.get(storey).copied(), annotations: index.annotations.get(storey).copied() };
    semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::plan_of(snapshot, storey, &inputs)
}

fn quantity_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> Option<ElementQuantity> {
    let hosted = |host: &str| -> Vec<&OpeningFrame> { snapshot.openings.iter().filter(|(_, opening)| opening.host == host).filter_map(|(opening, _)| index.frames.get(opening.as_str()).copied()).collect() };
    let solid = |_family: SolidFamily| index.solids.get(id).map(|entry| &entry.solid);
    if let Some(wall) = snapshot.walls.get(id) {
        return index.layouts.get(id).map(|layout| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::wall_quantity(snapshot, wall, layout, &hosted(id)));
    }
    if let Some(curtain) = snapshot.curtain_walls.get(id) {
        return index.curtains.get(id).map(|layout| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::curtain_quantity(snapshot, curtain, layout, &hosted(id), solid(SolidFamily::CurtainWall), &index.profiles()));
    }
    if let Some(sweep) = snapshot.wall_sweeps.get(id) {
        return snapshot.walls.get(&sweep.host).zip(index.layouts.get(sweep.host.as_str())).map(|(wall, layout)| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::sweep_quantity(snapshot, sweep, wall, layout, &hosted(&sweep.host), solid(SolidFamily::WallSweep)));
    }
    if let Some(slab) = snapshot.slabs.get(id) {
        return Some(semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::slab_quantity(snapshot, slab));
    }
    if let Some(ceiling) = snapshot.ceilings.get(id) {
        return Some(semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::ceiling_quantity(snapshot, ceiling));
    }
    if let Some(roof) = snapshot.roofs.get(id) {
        return Some(semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::roof_quantity(snapshot, roof, solid(SolidFamily::Roof)));
    }
    if let Some(column) = snapshot.columns.get(id) {
        return semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::column_quantity_in(snapshot, column, &index.level(&column.storey), index.target(&column.storey, &column.top).as_ref(), &index.profiles());
    }
    if let Some(beam) = snapshot.beams.get(id) {
        return semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::beam_quantity_in(snapshot, beam, solid(SolidFamily::Beam), &index.profiles());
    }
    if let Some(opening) = snapshot.openings.get(id) {
        return index.frames.get(id).map(|frame| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::opening_quantity(snapshot, opening, frame, index.solids.get(id).map(|entry| &entry.solid)));
    }
    if let Some(stair) = snapshot.stairs.get(id) {
        return index.runs.get(id).map(|run| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::stair_quantity(stair, run, solid(SolidFamily::Stair)));
    }
    if let Some(ramp) = snapshot.ramps.get(id) {
        return index.ramp_runs.get(id).map(|run| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::ramp_quantity(snapshot, ramp, run, solid(SolidFamily::Ramp)));
    }
    if let Some(railing) = snapshot.railings.get(id) {
        return Some(match railing.host.as_ref() {
            None => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::railing_quantity(snapshot, railing, solid(SolidFamily::Railing)),
            Some(spec) => {
                let length = host_of(snapshot, railing, index).and_then(|host| semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::host_paths(railing, spec, &host).ok()).map_or(0.0, |paths| semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::length_of(&paths));
                semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::hosted_railing_quantity(snapshot, railing, length, solid(SolidFamily::Railing))
            }
        });
    }
    if let Some(component) = snapshot.components.get(id) {
        return index.components.get(id).map(|entry| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::component_quantity(snapshot, component, &entry.value, solid(SolidFamily::Component)));
    }
    if snapshot.mep_elements.contains_key(id) {
        return index.meps.get(id).map(|value| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::mep_quantity(snapshot, id, value, solid(SolidFamily::Mep)));
    }
    let space = snapshot.spaces.get(id)?;
    let frames: Vec<&OpeningFrame> = index.frames.values().copied().collect();
    index.rooms.get(space.storey.as_str()).and_then(|rooms| rooms.get(id)).and_then(|room| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::space_quantity(snapshot, space, room, &frames))
}

fn schedule_value(snapshot: &ModelSnapshot, id: &str, parents: &[ModelValue]) -> ScheduleTable {
    let Some(schedule) = snapshot.schedules.get(id) else { return ScheduleTable::default() };
    let quantities: BTreeMap<&str, &ElementQuantity> = parents.iter().filter_map(|value| if let (ModelNode::Quantity(element), Data::Quantity(Some(quantity))) = (&value.node, &value.data) { Some((element.as_str(), &**quantity)) } else { None }).collect();
    let ids = semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::rows::candidates(snapshot, schedule);
    let mut view = semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::AuthoredView::of(snapshot, schedule, &ids);
    semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::rows::with_effective(&mut view, schedule, &Index::of(parents).properties);
    semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::schedule_of(schedule, &view, &quantities)
}

fn members_of<'a>(snapshot: &'a ModelSnapshot, index: &Index<'a>, member: impl Fn(&semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Space) -> bool) -> Vec<semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::Member<'a>> {
    snapshot.spaces.iter().filter(|(_, space)| member(space)).map(|(id, space)| (space, index.measured.get(id.as_str()).copied().flatten())).collect()
}

fn zone_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> ZoneTotals {
    snapshot.zones.get(id).map_or_else(ZoneTotals::default, |zone| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::zone_totals(zone, &members_of(snapshot, index, |space| space.zone.as_deref() == Some(id))))
}

fn scheme_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::SchemeTotals {
    let densities: BTreeMap<&str, f64> = snapshot.zones.iter().map(|(zone, row)| (zone.as_str(), row.occupancy_density)).collect();
    snapshot.area_schemes.get(id).map_or_else(semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::SchemeTotals::default, |scheme| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::scheme_totals(scheme, &densities, &members_of(snapshot, index, |space| semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::counts(scheme, space))))
}

fn totals_value(_scope: &TotalsScope, parents: &[ModelValue]) -> semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::QuantityTotals {
    totals_of(Index::of(parents).quantities)
}

fn findings_value(snapshot: &ModelSnapshot, scope: &DiagnosticScope, index: &Index<'_>) -> Vec<semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic> {
    let fallbacks: BTreeMap<&str, RoofFallback> = index.solids.iter().filter_map(|(id, entry)| entry.fallback.map(|fallback| (*id, fallback))).collect();
    let rooms = match scope {
        DiagnosticScope::Storey(storey) => index.rooms.get(storey.as_str()).copied(),
        _ => None,
    };
    let inputs = FindingInputs { levels: index.levels.clone(), layouts: index.layouts.clone(), frames: index.frames.clone(), runs: index.runs.clone(), ramp_runs: index.ramp_runs.clone(), rooms, fallbacks, curtains: index.curtains.clone(), components: index.component_values(), meps: index.meps.clone(), clashes: match scope { DiagnosticScope::Storey(storey) => index.mep_clashes.get(storey.as_str()).copied(), _ => None } };
    match scope {
        DiagnosticScope::Storey(storey) => {
            let mut found = semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::storey_findings(snapshot, storey, &inputs);
            found.extend(index.annotations.get(storey.as_str()).into_iter().flat_map(|set| set.findings.iter().cloned()));
            found
        }
        DiagnosticScope::Building(building) => semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::building_findings(snapshot, building, &inputs),
        DiagnosticScope::Model => {
            let mut found = semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::model_findings(&semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::references::ReferenceView::of(snapshot));
            found.extend(family_findings(snapshot, index));
            found
        }
        DiagnosticScope::Data => semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::findings(snapshot, &index.properties, &|id| element_storey(snapshot, id)),
        DiagnosticScope::Energy => energy_findings(&index),
    }
}

/// 🌡️ The findings of the envelopes: surfaces without thermal data, outlines on no wall, and the spaces that state no conditions where others do (one finding for all of them).
fn energy_findings(index: &Index<'_>) -> Vec<semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic> {
    use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode as Code;
    use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::EnergyCode;
    let mut found: Vec<semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic> = Vec::new();
    let mut unconditioned: Vec<&str> = Vec::new();
    for envelope in index.envelopes.values() {
        for issue in &envelope.issues {
            match issue.code {
                EnergyCode::ThermalDataMissing => found.push(semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic::new(Code::EnergyThermalMissing, &[&issue.element]).lacking(&issue.detail).on(&envelope.storey)),
                EnergyCode::OpenBoundary => found.push(semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic::new(Code::EnergyOpenBoundary, &[&issue.element]).lacking(&issue.detail).on(&envelope.storey)),
                EnergyCode::ConditionsMissing => unconditioned.push(&issue.element),
            }
        }
    }
    if !unconditioned.is_empty() {
        found.push(semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic::new(Code::EnergyConditionsMissing, &unconditioned));
    }
    found
}
/// 🧬️ The findings of the families: their issues and the dangling profile references.
fn family_findings(snapshot: &ModelSnapshot, index: &Index<'_>) -> Vec<semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic> {
    use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode as Code;
    use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::findings::FindingCode;
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues::FamilyIssueCode as Issue;
    let issues = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::findings::issue_findings(&index.families);
    let references = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::findings::reference_findings(snapshot, &index.families);
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
            finding.missing.iter().fold(semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic::new(code, &elements), |found, missing| found.lacking(missing))
        })
        .collect()
}
//#endregion 🔖️Value

/// 🪜️ The storey of an opening, for callers outside the graph that need it.
pub fn storey_of_opening(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    opening_storey(snapshot, id)
}
