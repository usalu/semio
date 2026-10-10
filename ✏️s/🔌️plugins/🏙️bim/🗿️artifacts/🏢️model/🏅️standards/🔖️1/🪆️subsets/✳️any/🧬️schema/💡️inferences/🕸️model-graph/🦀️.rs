//! 🕸️ `model-graph`: every derived value of the model in one `InferredField`. A node is a typed key ([`ModelNode`]), its value a typed enum ([`ModelValue`]), and the parents of a node are the
//! real nodes it is computed from: storey → wall layout (+ the bands of the walls it touches) → host → opening frame → solid, quantity, totals; storey + layouts → room → plan; levels +
//! layouts + frames + runs + rooms → diagnostics. Each node is computed once from its parents' values by the pure functions of the leaf modules (`storey-levels`, `wall-layout`, `opening-frames`,
//! `element-solids`, `spaces`, `quantities`, `plan-linework`, `diagnostics`); no consumer recomputes a layout, host, top, validity, run or room. `ModelInference` is a projection of the node values.
//!
//! The submodules: [`plan`] builds the topological order and the parent lists (indexes built once per run: touching walls per storey, hosted openings, elements per storey), [`compute`] is the
//! node dispatch and the dependency of each node (the honesty contract of `dep_input`), [`projection`] copies node values into [`ModelInference`] (whole or incrementally) and
//! [`session`] is the stepped, cancellable entry point of every consumer and [`instance`] reaches the one session per mounted instance that the framework owns, for the editor, the viewer, the exports and the imports. See `r7-design-model-graph.md` and `r7-api-model-session.md` in the BIM-PLUGIN ticket.

use super::super::annotation_layout::StoreyAnnotations;
use super::super::curtain_layout::CurtainLayout;
use super::super::diagnostics::{Diagnostic, DiagnosticIndex};
use super::super::element_solids::{SolidEntry, SolidKey};
use super::super::components::ComponentEntry;
use super::super::families::FamilyValue;
use super::super::mep::clash::MepClashes;
use super::super::mep::MepValue;
use super::super::clash_sets::{ClashSetResult, SolidProbe};
use super::super::rule_results::RuleResult;
use super::super::opening_frames::{CutRect, HostExtent, OpeningFrame};
use super::super::phase_visibility::PhaseVisibility;
use super::super::plan_linework::PlanLinework;
use super::super::quantities::{ElementQuantity, QuantityTotals};
use super::super::schedules::ScheduleTable;
use super::super::zones::{SchemeTotals, ZoneTotals};
use super::super::spaces::StoreyRooms;
use super::super::stair_runs::StairRun;
use super::super::ramp_runs::RampRun;
use super::super::storey_levels::StoreyLevel;
use super::super::wall_layout::joins::Band;
use super::super::view_linework::ViewLinework;
use super::super::sheet_layout::SheetLayout;
use super::super::wall_layout::attach::AttachSurface;
use super::super::effective_properties::EffectiveProperties;
use super::super::energy_envelope::{EnergyScope, EnergyTotals, EnvelopeSpace};
use super::super::wall_layout::WallLayout;
use super::super::ModelInference;
use crate::ModelSnapshot;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;
use std::sync::Arc;

#[path = "🧮️compute/🦀️.rs"]
pub mod compute;
#[path = "🎯️dirty/🦀️.rs"]
pub mod dirty;
#[path = "🧭️plan/🦀️.rs"]
pub mod plan;
#[path = "🪞️projection/🦀️.rs"]
pub mod projection;
#[path = "🧳️instance/🦀️.rs"]
pub mod instance;
#[path = "📡️session/🦀️.rs"]
pub mod session;

pub use session::{ModelInferenceSession, RunProgress, SessionRun, UpdateReport};

//#region 🔖️Kinds
/// 🧩️ The kind of a node; the selections of [`kinds`] are sets of kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeKind {
    StructuralAnalysis,
    Storey,
    Band,
    Cut,
    WallLayout,
    CurtainLayout,
    Host,
    OpeningFrame,
    StairRun,
    RampRun,
    Solid,
    Room,
    Plan,
    Quantity,
    Totals,
    Diagnostics,
    DiagnosticIndex,
    Annotation,
    Schedule,
    Zone,
    Scheme,
    View,
    Sheet,
    PhaseVisibility,
    Surface,
    Properties,
    Family,
    Probe,
    ClashSet,
    Rule,
    Envelope,
    EnergyTotals,
    Component,
    Mep,
    MepClash,
    OptionScope,
}

impl NodeKind {
    /// 🔢️ Every kind, in plan order.
    pub const ALL: [NodeKind; 36] = [
        Self::StructuralAnalysis,
        Self::Storey,
        Self::Band,
        Self::Cut,
        Self::WallLayout,
        Self::CurtainLayout,
        Self::Host,
        Self::OpeningFrame,
        Self::StairRun,
        Self::RampRun,
        Self::Solid,
        Self::Room,
        Self::Plan,
        Self::Quantity,
        Self::Totals,
        Self::Diagnostics,
        Self::DiagnosticIndex,
        Self::Annotation,
        Self::Schedule,
        Self::Zone,
        Self::Scheme,
        Self::View,
        Self::Sheet,
        Self::PhaseVisibility,
        Self::Surface,
        Self::Properties,
        Self::Family,
        Self::Probe,
        Self::ClashSet,
        Self::Rule,
        Self::Envelope,
        Self::EnergyTotals,
        Self::Component,
        Self::Mep,
        Self::MepClash,
        Self::OptionScope,
    ];

    /// 🔢️ The bit of the kind in a selection mask.
    pub const fn bit(self) -> u64 {
        1u64 << self as u32
    }

    /// 🏷️ The stable name of the kind (the keys of `UpdateReport::computed_by_kind`).
    pub const fn name(self) -> &'static str {
        match self {
            Self::StructuralAnalysis => "structural-analysis",
            Self::Storey => "storey",
            Self::Band => "band",
            Self::Cut => "cut",
            Self::WallLayout => "wall-layout",
            Self::CurtainLayout => "curtain-layout",
            Self::Host => "host",
            Self::OpeningFrame => "opening-frame",
            Self::StairRun => "stair-run",
            Self::RampRun => "ramp-run",
            Self::Solid => "solid",
            Self::Room => "room",
            Self::Plan => "plan",
            Self::Quantity => "quantity",
            Self::Totals => "totals",
            Self::Diagnostics => "diagnostics",
            Self::DiagnosticIndex => "diagnostic-index",
            Self::Annotation => "annotation",
            Self::Schedule => "schedule",
            Self::Zone => "zone",
            Self::Scheme => "scheme",
            Self::View => "view",
            Self::Sheet => "sheet",
            Self::PhaseVisibility => "phase-visibility",
            Self::Surface => "surface",
            Self::Properties => "properties",
            Self::Family => "family",
            Self::Probe => "probe",
            Self::ClashSet => "clash-set",
            Self::Rule => "rule",
            Self::Envelope => "envelope",
            Self::EnergyTotals => "energy-totals",
            Self::Component => "component",
            Self::Mep => "mep",
            Self::MepClash => "mep-clash",
            Self::OptionScope => "option-scope",
        }
    }

    /// 🔗️ The kinds whose nodes a node of this kind can have as parents.
    pub const fn requires(self) -> &'static [NodeKind] {
        use NodeKind::*;
        match self {
            StructuralAnalysis => &[Storey, WallLayout, Solid],
            Storey | Band | Cut | PhaseVisibility | Family => &[],
            Surface => &[Storey],
            Properties => &[Properties],
            WallLayout => &[Storey, Band, Surface],
            CurtainLayout => &[Storey],
            Host => &[Storey, WallLayout, CurtainLayout],
            OpeningFrame => &[Host, Cut],
            StairRun => &[Storey],
            RampRun => &[Storey],
            Solid => &[Storey, WallLayout, CurtainLayout, OpeningFrame, StairRun, RampRun, Family, Component, Mep],
            Room => &[Storey, WallLayout],
            Annotation => &[WallLayout],
            Plan => &[Storey, WallLayout, CurtainLayout, OpeningFrame, StairRun, RampRun, Room, Annotation, Component, Mep],
            Quantity => &[Storey, WallLayout, CurtainLayout, OpeningFrame, StairRun, RampRun, Solid, Room, Family, Component, Mep],
            Totals => &[Quantity],
            Diagnostics => &[Storey, WallLayout, OpeningFrame, StairRun, RampRun, Room, Solid, Annotation, Properties, Family, Envelope, Component, Mep, MepClash],
            Self::DiagnosticIndex => &[Diagnostics],
            Schedule => &[Quantity, Properties],
            Zone | Scheme => &[Quantity],
            View => &[Storey, WallLayout, CurtainLayout, OpeningFrame, StairRun, RampRun, Room, Solid, Component, Mep],
            Sheet => &[View],
            Probe => &[Solid],
            ClashSet => &[Probe],
            Rule => &[Room, StairRun, RampRun, OpeningFrame, Zone],
            Envelope => &[Room, WallLayout, OpeningFrame],
            EnergyTotals => &[Envelope],
            Component => &[Storey, Family, WallLayout],
            Mep => &[Storey],
            MepClash => &[Mep],
            OptionScope => &[Quantity],
        }
    }
}

/// 🧩️ The selections of the graph: a mask of the kinds a run wants. The run plans those kinds and every kind they require.
pub mod kinds {
    use super::NodeKind;

    const fn mask(kind: NodeKind) -> u64 {
        kind.bit()
    }

    pub const STRUCTURE: u64 = mask(NodeKind::StructuralAnalysis);
    pub const LEVELS: u64 = mask(NodeKind::Storey);
    pub const LAYOUTS: u64 = mask(NodeKind::WallLayout);
    pub const CURTAINS: u64 = mask(NodeKind::CurtainLayout);
    pub const FRAMES: u64 = mask(NodeKind::OpeningFrame);
    pub const RUNS: u64 = mask(NodeKind::StairRun);
    pub const RAMP_RUNS: u64 = mask(NodeKind::RampRun);
    pub const SOLIDS: u64 = mask(NodeKind::Solid);
    pub const ROOMS: u64 = mask(NodeKind::Room);
    pub const PLANS: u64 = mask(NodeKind::Plan);
    pub const QUANTITIES: u64 = mask(NodeKind::Totals);
    pub const DIAGNOSTICS: u64 = mask(NodeKind::Diagnostics);
    pub const DIAGNOSTIC_INDEX: u64 = mask(NodeKind::DiagnosticIndex);
    pub const ANNOTATIONS: u64 = mask(NodeKind::Annotation);
    pub const SCHEDULES: u64 = mask(NodeKind::Schedule);
    pub const ZONES: u64 = mask(NodeKind::Zone) | mask(NodeKind::Scheme);
    pub const VIEWS: u64 = mask(NodeKind::View);
    pub const SHEETS: u64 = mask(NodeKind::Sheet);
    pub const OPTIONS: u64 = mask(NodeKind::OptionScope);
    pub const PHASES: u64 = mask(NodeKind::PhaseVisibility);
    pub const SURFACES: u64 = mask(NodeKind::Surface);
    pub const PROPERTIES: u64 = mask(NodeKind::Properties);
    pub const FAMILIES: u64 = mask(NodeKind::Family);
    pub const COMPONENTS: u64 = mask(NodeKind::Component);
    pub const MEPS: u64 = mask(NodeKind::Mep);
    pub const CLASHES: u64 = mask(NodeKind::ClashSet);
    pub const RULES: u64 = mask(NodeKind::Rule);
    pub const ENERGY: u64 = mask(NodeKind::EnergyTotals);
    pub const ALL: u64 = (1u64 << NodeKind::ALL.len()) - 1;

    /// 🔗️ The wanted kinds and everything they require, transitively.
    pub const fn closure(wanted: u64) -> u64 {
        let mut mask = wanted;
        loop {
            let mut next = mask;
            let mut index = 0;
            while index < NodeKind::ALL.len() {
                let kind = NodeKind::ALL[index];
                if mask & kind.bit() != 0 {
                    let required = kind.requires();
                    let mut at = 0;
                    while at < required.len() {
                        next |= required[at].bit();
                        at += 1;
                    }
                }
                index += 1;
            }
            if next == mask {
                return mask;
            }
            mask = next;
        }
    }
}
//#endregion 🔖️Kinds

//#region 🔖️Keys
/// 🧮️ The scope of a `Totals` node.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum TotalsScope {
    Storey(String),
    Building(String),
    Project,
}

/// ⚠️ The scope of a `Diagnostics` node.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum DiagnosticScope {
    Storey(String),
    Building(String),
    Model,
    Energy,
    Data,
}

/// 🔑️ A node of the model graph. Ids are the ids of the snapshot (unique across collections); a `Room`, `Plan` and storey scope is the id of the storey.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum ModelNode {
    StructuralAnalysis,
    Storey(String),
    Band(String),
    Cut(String),
    WallLayout(String),
    CurtainLayout(String),
    Host(String),
    OpeningFrame(String),
    StairRun(String),
    RampRun(String),
    Solid(SolidKey),
    Room(String),
    Plan(String),
    Quantity(String),
    Totals(TotalsScope),
    Diagnostics(DiagnosticScope),
    DiagnosticIndex,
    Annotation(String),
    Schedule(String),
    Zone(String),
    Scheme(String),
    View(String),
    Sheet(String),
    PhaseVisibility(String),
    Surface(String),
    Properties(String),
    Family(String),
    Probe(SolidKey),
    ClashSet(String),
    Rule(String),
    Envelope(String),
    EnergyTotals(EnergyScope),
    Component(String),
    Mep(String),
    MepClash(String),
    OptionScope,
}

impl ModelNode {
    /// 🧩️ The kind of the node.
    pub fn kind(&self) -> NodeKind {
        match self {
            Self::StructuralAnalysis => NodeKind::StructuralAnalysis,
            Self::Storey(_) => NodeKind::Storey,
            Self::Band(_) => NodeKind::Band,
            Self::Cut(_) => NodeKind::Cut,
            Self::WallLayout(_) => NodeKind::WallLayout,
            Self::CurtainLayout(_) => NodeKind::CurtainLayout,
            Self::Host(_) => NodeKind::Host,
            Self::OpeningFrame(_) => NodeKind::OpeningFrame,
            Self::StairRun(_) => NodeKind::StairRun,
            Self::RampRun(_) => NodeKind::RampRun,
            Self::Solid(_) => NodeKind::Solid,
            Self::Room(_) => NodeKind::Room,
            Self::Plan(_) => NodeKind::Plan,
            Self::Quantity(_) => NodeKind::Quantity,
            Self::Totals(_) => NodeKind::Totals,
            Self::Diagnostics(_) => NodeKind::Diagnostics,
            Self::DiagnosticIndex => NodeKind::DiagnosticIndex,
            Self::Annotation(_) => NodeKind::Annotation,
            Self::Schedule(_) => NodeKind::Schedule,
            Self::Zone(_) => NodeKind::Zone,
            Self::Scheme(_) => NodeKind::Scheme,
            Self::View(_) => NodeKind::View,
            Self::Sheet(_) => NodeKind::Sheet,
            Self::PhaseVisibility(_) => NodeKind::PhaseVisibility,
            Self::Surface(_) => NodeKind::Surface,
            Self::Properties(_) => NodeKind::Properties,
            Self::Family(_) => NodeKind::Family,
            Self::Component(_) => NodeKind::Component,
            Self::Mep(_) => NodeKind::Mep,
            Self::MepClash(_) => NodeKind::MepClash,
            Self::OptionScope => NodeKind::OptionScope,
            Self::Probe(_) => NodeKind::Probe,
            Self::ClashSet(_) => NodeKind::ClashSet,
            Self::Rule(_) => NodeKind::Rule,
            Self::Envelope(_) => NodeKind::Envelope,
            Self::EnergyTotals(_) => NodeKind::EnergyTotals,
        }
    }
}
//#endregion 🔖️Keys

//#region 🔖️Values
/// 📦️ What a node holds. Everything bigger than a `Copy` struct is an `Arc`, so the engine's per-parent clone and a cache hit are pointer copies.
#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Structure(Arc<super::super::analytical_members::StructuralAnalysis>),
    OptionScope(Arc<super::super::option_scope::OptionScope>),
    Level(StoreyLevel),
    Band(Option<Band>),
    Cut(CutRect),
    Layout(Arc<WallLayout>),
    Curtain(Arc<CurtainLayout>),
    Host(Option<Arc<HostExtent>>),
    Frame(Arc<OpeningFrame>),
    Run(Arc<StairRun>),
    RampRun(Arc<RampRun>),
    Solid(Arc<SolidEntry>),
    Rooms(Arc<StoreyRooms>),
    Plan(Arc<PlanLinework>),
    Quantity(Option<Arc<ElementQuantity>>),
    Totals(Arc<QuantityTotals>),
    Findings(Arc<Vec<Diagnostic>>),
    Index(Arc<DiagnosticIndex>),
    Annotations(Arc<StoreyAnnotations>),
    Schedule(Arc<ScheduleTable>),
    Zone(Arc<ZoneTotals>),
    Scheme(Arc<SchemeTotals>),
    View(Arc<ViewLinework>),
    Sheet(Arc<SheetLayout>),
    Phases(Arc<PhaseVisibility>),
    Surface(Arc<AttachSurface>),
    Properties(Arc<EffectiveProperties>),
    Family(Arc<FamilyValue>),
    Probe(Arc<SolidProbe>),
    Clashes(Arc<ClashSetResult>),
    Rule(Arc<RuleResult>),
    Envelope(Arc<EnvelopeSpace>),
    EnergyTotals(Arc<EnergyTotals>),
    Component(Arc<ComponentEntry>),
    Mep(Arc<MepValue>),
    MepClashes(Arc<MepClashes>),
}

/// 📦️ A node value: the key it was computed for and its data. The key travels with the value so a parent is found by key, never by position.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelValue {
    pub node: ModelNode,
    pub data: Data,
}

impl ModelValue {
    /// 🔁️ Whether two values are the same computation: the same `Arc` payload, or equal `Copy` payloads.
    pub fn same(&self, other: &ModelValue) -> bool {
        match (&self.data, &other.data) {
            (Data::Structure(a), Data::Structure(b)) => Arc::ptr_eq(a,b),
            (Data::Layout(a), Data::Layout(b)) => Arc::ptr_eq(a, b),
            (Data::Curtain(a), Data::Curtain(b)) => Arc::ptr_eq(a, b),
            (Data::Host(Some(a)), Data::Host(Some(b))) => Arc::ptr_eq(a, b),
            (Data::Frame(a), Data::Frame(b)) => Arc::ptr_eq(a, b),
            (Data::Run(a), Data::Run(b)) => Arc::ptr_eq(a, b),
            (Data::RampRun(a), Data::RampRun(b)) => Arc::ptr_eq(a, b),
            (Data::Solid(a), Data::Solid(b)) => Arc::ptr_eq(a, b),
            (Data::Rooms(a), Data::Rooms(b)) => Arc::ptr_eq(a, b),
            (Data::Plan(a), Data::Plan(b)) => Arc::ptr_eq(a, b),
            (Data::Quantity(Some(a)), Data::Quantity(Some(b))) => Arc::ptr_eq(a, b),
            (Data::Totals(a), Data::Totals(b)) => Arc::ptr_eq(a, b),
            (Data::Findings(a), Data::Findings(b)) => Arc::ptr_eq(a, b),
            (Data::Index(a), Data::Index(b)) => Arc::ptr_eq(a, b),
            (Data::Annotations(a), Data::Annotations(b)) => Arc::ptr_eq(a, b),
            (Data::Schedule(a), Data::Schedule(b)) => Arc::ptr_eq(a, b),
            (Data::Zone(a), Data::Zone(b)) => Arc::ptr_eq(a, b),
            (Data::Scheme(a), Data::Scheme(b)) => Arc::ptr_eq(a, b),
            (Data::View(a), Data::View(b)) => Arc::ptr_eq(a, b),
            (Data::Sheet(a), Data::Sheet(b)) => Arc::ptr_eq(a, b),
            (Data::Phases(a), Data::Phases(b)) => Arc::ptr_eq(a, b),
            (Data::Surface(a), Data::Surface(b)) => Arc::ptr_eq(a, b),
            (Data::Properties(a), Data::Properties(b)) => Arc::ptr_eq(a, b),
            (Data::Family(a), Data::Family(b)) => Arc::ptr_eq(a, b),
            (Data::Component(a), Data::Component(b)) => Arc::ptr_eq(a, b),
            (Data::Mep(a), Data::Mep(b)) => Arc::ptr_eq(a, b),
            (Data::MepClashes(a), Data::MepClashes(b)) => Arc::ptr_eq(a, b),
            (Data::OptionScope(a), Data::OptionScope(b)) => Arc::ptr_eq(a, b),
            (Data::Probe(a), Data::Probe(b)) => Arc::ptr_eq(a, b),
            (Data::Clashes(a), Data::Clashes(b)) => Arc::ptr_eq(a, b),
            (Data::Rule(a), Data::Rule(b)) => Arc::ptr_eq(a, b),
            (Data::Envelope(a), Data::Envelope(b)) => Arc::ptr_eq(a, b),
            (Data::EnergyTotals(a), Data::EnergyTotals(b)) => Arc::ptr_eq(a, b),
            (Data::Host(Some(_)), _) | (Data::Quantity(Some(_)), _) => false,
            (a, b) => a == b && self.node == other.node,
        }
    }

    /// ⚖️ The bytes a cached value is accounted with.
    pub fn bytes(&self) -> usize {
        let base = std::mem::size_of::<ModelValue>();
        base + match &self.data {
            Data::Structure(value) => value.members.values().map(|m| 256+(m.path.len()+m.boundary.len())*24).sum::<usize>()+(value.loads.len()+value.supports.len()+value.rigid_links.len())*256,
            Data::Layout(layout) => layout.layer_offsets.len() * 8 + layout.footprint.len() * std::mem::size_of::<crate::Vertex>() + layout.joins.iter().map(|join| std::mem::size_of_val(join) + join.other.len()).sum::<usize>(),
            Data::Curtain(layout) => (layout.u_edges.len() + layout.v_edges.len() + layout.ignored_u.len() + layout.ignored_v.len()) * 8 + layout.overrides.len() * 96 + (layout.stray.len() + layout.repeated.len()) * 32,
            Data::Solid(entry) => entry.solid.byte_size(),
            Data::Frame(frame) => std::mem::size_of_val(&**frame) + frame.plan.len() * 64,
            Data::Rooms(rooms) => rooms.values().map(|room| 128 + (room.outline.len() + room.holes.iter().map(Vec::len).sum::<usize>()) * std::mem::size_of::<crate::Vertex>()).sum(),
            Data::Plan(plan) => plan.regions.iter().map(|region| 96 + region.outer.len() * 24).sum::<usize>() + plan.polylines.iter().map(|line| 96 + line.vertices.len() * 24).sum::<usize>() + plan.texts.len() * 128,
            Data::View(view) => view.lines.regions.iter().map(|region| 96 + region.outer.len() * 24).sum::<usize>() + view.lines.polylines.iter().map(|line| 96 + line.vertices.len() * 24).sum::<usize>() + view.lines.texts.len() * 128,
            Data::Sheet(layout) => 512 + (layout.viewports.len() * 192) + layout.title_block.cells.iter().map(|cell| 96 + cell.value.len()).sum::<usize>() + layout.revisions.rows.iter().map(|row| 128 + row.description.len()).sum::<usize>() + layout.findings.len() * 96,
            Data::Findings(found) => found.len() * 160,
            Data::Index(index) => index.elements.values().map(|entry| 96 + entry.codes.len() * 8).sum::<usize>() + (index.categories.len() + index.codes.len() + index.storeys.len()) * 64,
            Data::Family(family) => family.solids.values().map(|solid| 160 + 8 * (solid.positions.len() + solid.normals.len()) + 4 * solid.indices.len()).sum::<usize>() + family.parameters.len() * 128 + family.issues.len() * 192 + family.outline.len() * 32,
            Data::Probe(probe) => probe.byte_size(),
            Data::Component(entry) => 256 + entry.value.parameters.len() * 128 + entry.value.issues.len() * 192 + if Arc::strong_count(&entry.family) == 1 { entry.family.solids.values().map(|solid| 160 + 8 * (solid.positions.len() + solid.normals.len()) + 4 * solid.indices.len()).sum::<usize>() } else { 0 },
            Data::Mep(mep) => 256 + (mep.path.len() + mep.segments.len()) * 64 + mep.issues.len() * 96,
            Data::MepClashes(found) => 64 + found.pairs.len() * 96,
            Data::OptionScope(scope) => 512 + scope.elements.len() * 128 + scope.options.values().map(|ids| ids.len() * 32).sum::<usize>() + scope.takeoff.len() * 256,
            Data::Clashes(found) => 128 + found.clashes.len() * 224 + found.groups.iter().map(|group| 48 + group.members.len() * 4).sum::<usize>(),
            Data::Rule(found) => 64 + found.violations.len() * 112,
            Data::Phases(phases) => phases.visible.values().map(|ids| 48 + ids.iter().map(|id| id.len() + 24).sum::<usize>()).sum(),
            Data::Annotations(set) => (set.dimensions.len() + set.tags.len() + set.notes.len() + set.leaders.len()) * 256 + set.findings.len() * 160,
            Data::Properties(properties) => properties.values.values().map(|set| 64 + set.len() * 96).sum::<usize>() + properties.findings.len() * 96,
            Data::Surface(surface) => surface.pieces.iter().map(|piece| 96 + (piece.outline.len() + piece.holes.iter().map(Vec::len).sum::<usize>()) * 16).sum(),
            Data::Envelope(envelope) => 256 + envelope.surfaces.iter().map(|surface| 192 + surface.polygon.len() * 24).sum::<usize>() + envelope.issues.len() * 96,
            Data::EnergyTotals(totals) => 256 + totals.spaces.len() * 32,
            Data::Totals(totals) => (totals.kinds.len() + totals.types.len() + totals.materials.len() + totals.finishes.len()) * 80,
            Data::Schedule(table) => table.rows.iter().map(|row| 64 + row.cells.len() * 32 + row.elements.len() * 24).sum(),
            _ => 0,
        }
    }
}
//#endregion 🔖️Values

//#region 🔖️InferredField
/// 🕸️ The one inferred field of the model, restricted to the kinds in `WANT` (see [`kinds`]) and their ancestors. All selections share the field id: a node's value depends only on its key and the snapshot, so a warm cache serves every selection.
pub struct ModelGraph<const WANT: u64>;

/// 🗺️ The snapshot collections the graph reads (the tier-1 gate of `infer_field_after_diff`): everything except the project info and material colours' absence of effect, which the dependency hashes decide.
pub const READS: &[&str] = &[
    "option_groups", "design_options", "worksets", "element_options", "element_worksets",
    "supports", "load_cases", "loads",
    "components", "component_overrides", "mep_elements", "space_conditions", "families", "clash_sets", "rules", "family_parameters", "family_solids", "property_templates", "classification_systems", "storeys", "buildings", "sites", "walls", "wall_sweeps", "wall_types", "curtain_walls", "curtain_wall_types", "curtain_panel_overrides", "openings", "window_types", "door_types", "columns", "column_types", "beams", "beam_types", "slabs", "slab_types", "ceilings", "ceiling_types", "roofs", "roof_types", "stairs", "ramps", "railings", "spaces", "zones", "area_schemes", "grids", "materials", "properties", "classifications", "schedules", "views", "sheets", "viewports", "sheet_revisions", "dimensions", "tags", "text_notes", "leaders", "annotation_styles",
];

impl<const WANT: u64> protocol::InferredField<ModelSnapshot> for ModelGraph<WANT> {
    type Dependency = DslValue;
    type Key = ModelNode;
    type Value = ModelValue;

    const FIELD_ID: &'static str = "s.bim.model.inference.model-graph";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        READS
    }

    fn plan(snapshot: &ModelSnapshot) -> Vec<protocol::InferenceStep<Self::Key>> {
        plan::build(snapshot, kinds::closure(WANT))
    }

    fn dep_input(snapshot: &ModelSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Self::Dependency {
        compute::dependency(snapshot, key)
    }

    fn touched_by(snapshot: &ModelSnapshot, key: &Self::Key, touched: &protocol::TouchedPaths) -> bool {
        dirty::touched(snapshot, key, touched)
    }

    fn compute(snapshot: &ModelSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        compute::value(snapshot, key, parents)
    }

    fn value_bytes(value: &Self::Value) -> usize {
        value.bytes()
    }
}

/// 🕸️ Runs the graph restricted to `WANT` without a cache and projects the values into a `ModelInference` (the fields of unwanted kinds stay empty): the reference every session run is tested against. A fault of the engine is a value.
pub fn try_infer_selected<const WANT: u64>(snapshot: &ModelSnapshot) -> Result<ModelInference, protocol::InferenceError> {
    protocol::try_infer_field::<ModelSnapshot, ModelGraph<WANT>>(snapshot, None).map(projection::project)
}

/// 🕸️ [`try_infer_selected`] for the test helpers that read one projection of a model.
#[cfg(test)]
pub fn infer_selected<const WANT: u64>(snapshot: &ModelSnapshot) -> ModelInference {
    try_infer_selected::<WANT>(snapshot).unwrap_or_else(|error| panic!("{error}"))
}

/// 🗂️ The node values of a whole run, by key.
pub type Values = BTreeMap<ModelNode, ModelValue>;
//#endregion 🔖️InferredField

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/📈️incremental/🦀️.rs"]
mod incremental_tests;

#[cfg(test)]
#[path = "🧪️tests/🧬️families/🦀️.rs"]
mod family_tests;
