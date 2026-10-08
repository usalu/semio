//! 🕸️ `model-graph`: every derived value of the model in one `InferredField`. A node is a typed key ([`ModelNode`]), its value a typed enum ([`ModelValue`]), and the parents of a node are the
//! real nodes it is computed from: storey → wall layout (+ the bands of the walls it touches) → host → opening frame → solid, quantity, totals; storey + layouts → room → plan; levels +
//! layouts + frames + runs + rooms → diagnostics. Each node is computed once from its parents' values by the pure functions of the leaf modules (`storey-levels`, `wall-layout`, `opening-frames`,
//! `element-solids`, `spaces`, `quantities`, `plan-linework`, `diagnostics`); no consumer recomputes a layout, host, top, validity, run or room. `ModelInference` is a projection of the node values.
//!
//! The submodules: [`plan`] builds the topological order and the parent lists (indexes built once per run: touching walls per storey, hosted openings, elements per storey), [`compute`] is the
//! node dispatch and the dependency of each node (the honesty contract of `dep_input`), [`projection`] copies node values into [`ModelInference`] (whole or incrementally) and
//! [`session`] is the incremental entry point of editor, viewer and export. See `r7-design-model-graph.md` and `r7-api-model-session.md` in the BIM-PLUGIN ticket.

use super::super::curtain_layout::CurtainLayout;
use super::super::diagnostics::Diagnostic;
use super::super::element_solids::{SolidEntry, SolidKey};
use super::super::opening_frames::{CutRect, HostExtent, OpeningFrame};
use super::super::plan_linework::PlanLinework;
use super::super::quantities::{ElementQuantity, QuantityTotals};
use super::super::spaces::StoreyRooms;
use super::super::stair_runs::StairRun;
use super::super::storey_levels::StoreyLevel;
use super::super::wall_layout::joins::Band;
use super::super::wall_layout::WallLayout;
use super::super::ModelInference;
use crate::ModelSnapshot;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;
use std::sync::Arc;

#[path = "🧮️compute/🦀️.rs"]
pub mod compute;
#[path = "🧭️plan/🦀️.rs"]
pub mod plan;
#[path = "🪞️projection/🦀️.rs"]
pub mod projection;
#[path = "📡️session/🦀️.rs"]
pub mod session;

pub use session::{ModelInferenceSession, UpdateReport};

//#region 🔖️Kinds
/// 🧩️ The kind of a node; the selections of [`kinds`] are sets of kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeKind {
    Storey,
    Band,
    Cut,
    WallLayout,
    CurtainLayout,
    Host,
    OpeningFrame,
    StairRun,
    Solid,
    Room,
    Plan,
    Quantity,
    Totals,
    Diagnostics,
}

impl NodeKind {
    /// 🔢️ Every kind, in plan order.
    pub const ALL: [NodeKind; 14] = [
        Self::Storey,
        Self::Band,
        Self::Cut,
        Self::WallLayout,
        Self::CurtainLayout,
        Self::Host,
        Self::OpeningFrame,
        Self::StairRun,
        Self::Solid,
        Self::Room,
        Self::Plan,
        Self::Quantity,
        Self::Totals,
        Self::Diagnostics,
    ];

    /// 🔢️ The bit of the kind in a selection mask.
    pub const fn bit(self) -> u32 {
        1 << self as u32
    }

    /// 🏷️ The stable name of the kind (the keys of `UpdateReport::computed_by_kind`).
    pub const fn name(self) -> &'static str {
        match self {
            Self::Storey => "storey",
            Self::Band => "band",
            Self::Cut => "cut",
            Self::WallLayout => "wall-layout",
            Self::CurtainLayout => "curtain-layout",
            Self::Host => "host",
            Self::OpeningFrame => "opening-frame",
            Self::StairRun => "stair-run",
            Self::Solid => "solid",
            Self::Room => "room",
            Self::Plan => "plan",
            Self::Quantity => "quantity",
            Self::Totals => "totals",
            Self::Diagnostics => "diagnostics",
        }
    }

    /// 🔗️ The kinds whose nodes a node of this kind can have as parents.
    pub const fn requires(self) -> &'static [NodeKind] {
        use NodeKind::*;
        match self {
            Storey | Band | Cut => &[],
            WallLayout => &[Storey, Band],
            CurtainLayout => &[Storey],
            Host => &[Storey, WallLayout, CurtainLayout],
            OpeningFrame => &[Host, Cut],
            StairRun => &[Storey],
            Solid => &[Storey, WallLayout, CurtainLayout, OpeningFrame, StairRun],
            Room => &[Storey, WallLayout],
            Plan => &[Storey, WallLayout, CurtainLayout, OpeningFrame, StairRun, Room],
            Quantity => &[Storey, WallLayout, CurtainLayout, OpeningFrame, StairRun, Solid, Room],
            Totals => &[Quantity],
            Diagnostics => &[Storey, WallLayout, OpeningFrame, StairRun, Room, Solid],
        }
    }
}

/// 🧩️ The selections of the graph: a mask of the kinds a run wants. The run plans those kinds and every kind they require.
pub mod kinds {
    use super::NodeKind;

    const fn mask(kind: NodeKind) -> u32 {
        kind.bit()
    }

    pub const LEVELS: u32 = mask(NodeKind::Storey);
    pub const LAYOUTS: u32 = mask(NodeKind::WallLayout);
    pub const CURTAINS: u32 = mask(NodeKind::CurtainLayout);
    pub const FRAMES: u32 = mask(NodeKind::OpeningFrame);
    pub const RUNS: u32 = mask(NodeKind::StairRun);
    pub const SOLIDS: u32 = mask(NodeKind::Solid);
    pub const ROOMS: u32 = mask(NodeKind::Room);
    pub const PLANS: u32 = mask(NodeKind::Plan);
    pub const QUANTITIES: u32 = mask(NodeKind::Totals);
    pub const DIAGNOSTICS: u32 = mask(NodeKind::Diagnostics);
    pub const ALL: u32 = (1 << NodeKind::ALL.len()) - 1;

    /// 🔗️ The wanted kinds and everything they require, transitively.
    pub const fn closure(wanted: u32) -> u32 {
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum TotalsScope {
    Storey(String),
    Building(String),
    Project,
}

/// ⚠️ The scope of a `Diagnostics` node.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum DiagnosticScope {
    Storey(String),
    Building(String),
    Model,
}

/// 🔑️ A node of the model graph. Ids are the ids of the snapshot (unique across collections); a `Room`, `Plan` and storey scope is the id of the storey.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum ModelNode {
    Storey(String),
    Band(String),
    Cut(String),
    WallLayout(String),
    CurtainLayout(String),
    Host(String),
    OpeningFrame(String),
    StairRun(String),
    Solid(SolidKey),
    Room(String),
    Plan(String),
    Quantity(String),
    Totals(TotalsScope),
    Diagnostics(DiagnosticScope),
}

impl ModelNode {
    /// 🧩️ The kind of the node.
    pub fn kind(&self) -> NodeKind {
        match self {
            Self::Storey(_) => NodeKind::Storey,
            Self::Band(_) => NodeKind::Band,
            Self::Cut(_) => NodeKind::Cut,
            Self::WallLayout(_) => NodeKind::WallLayout,
            Self::CurtainLayout(_) => NodeKind::CurtainLayout,
            Self::Host(_) => NodeKind::Host,
            Self::OpeningFrame(_) => NodeKind::OpeningFrame,
            Self::StairRun(_) => NodeKind::StairRun,
            Self::Solid(_) => NodeKind::Solid,
            Self::Room(_) => NodeKind::Room,
            Self::Plan(_) => NodeKind::Plan,
            Self::Quantity(_) => NodeKind::Quantity,
            Self::Totals(_) => NodeKind::Totals,
            Self::Diagnostics(_) => NodeKind::Diagnostics,
        }
    }
}
//#endregion 🔖️Keys

//#region 🔖️Values
/// 📦️ What a node holds. Everything bigger than a `Copy` struct is an `Arc`, so the engine's per-parent clone and a cache hit are pointer copies.
#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Level(StoreyLevel),
    Band(Option<Band>),
    Cut(CutRect),
    Layout(Arc<WallLayout>),
    Curtain(CurtainLayout),
    Host(Option<Arc<HostExtent>>),
    Frame(Arc<OpeningFrame>),
    Run(Arc<StairRun>),
    Solid(Arc<SolidEntry>),
    Rooms(Arc<StoreyRooms>),
    Plan(Arc<PlanLinework>),
    Quantity(Option<Arc<ElementQuantity>>),
    Totals(Arc<QuantityTotals>),
    Findings(Arc<Vec<Diagnostic>>),
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
            (Data::Layout(a), Data::Layout(b)) => Arc::ptr_eq(a, b),
            (Data::Host(Some(a)), Data::Host(Some(b))) => Arc::ptr_eq(a, b),
            (Data::Frame(a), Data::Frame(b)) => Arc::ptr_eq(a, b),
            (Data::Run(a), Data::Run(b)) => Arc::ptr_eq(a, b),
            (Data::Solid(a), Data::Solid(b)) => Arc::ptr_eq(a, b),
            (Data::Rooms(a), Data::Rooms(b)) => Arc::ptr_eq(a, b),
            (Data::Plan(a), Data::Plan(b)) => Arc::ptr_eq(a, b),
            (Data::Quantity(Some(a)), Data::Quantity(Some(b))) => Arc::ptr_eq(a, b),
            (Data::Totals(a), Data::Totals(b)) => Arc::ptr_eq(a, b),
            (Data::Findings(a), Data::Findings(b)) => Arc::ptr_eq(a, b),
            (Data::Host(Some(_)), _) | (Data::Quantity(Some(_)), _) => false,
            (a, b) => a == b && self.node == other.node,
        }
    }

    /// ⚖️ The bytes a cached value is accounted with.
    pub fn bytes(&self) -> usize {
        let base = std::mem::size_of::<ModelValue>();
        base + match &self.data {
            Data::Layout(layout) => layout.layer_offsets.len() * 8 + layout.footprint.len() * std::mem::size_of::<crate::Vertex>() + layout.joins.iter().map(|join| std::mem::size_of_val(join) + join.other.len()).sum::<usize>(),
            Data::Solid(entry) => entry.solid.byte_size(),
            Data::Frame(frame) => std::mem::size_of_val(&**frame) + frame.plan.len() * 64,
            Data::Rooms(rooms) => rooms.values().map(|room| 128 + (room.outline.len() + room.holes.iter().map(Vec::len).sum::<usize>()) * std::mem::size_of::<crate::Vertex>()).sum(),
            Data::Plan(plan) => plan.regions.iter().map(|region| 96 + region.outer.len() * 24).sum::<usize>() + plan.polylines.iter().map(|line| 96 + line.vertices.len() * 24).sum::<usize>() + plan.texts.len() * 128,
            Data::Findings(found) => found.len() * 160,
            Data::Totals(totals) => (totals.kinds.len() + totals.types.len() + totals.materials.len()) * 80,
            _ => 0,
        }
    }
}
//#endregion 🔖️Values

//#region 🔖️InferredField
/// 🕸️ The one inferred field of the model, restricted to the kinds in `WANT` (see [`kinds`]) and their ancestors. All selections share the field id: a node's value depends only on its key and the snapshot, so a warm cache serves every selection.
pub struct ModelGraph<const WANT: u32>;

/// 🗺️ The snapshot collections the graph reads (the tier-1 gate of `infer_field_after_diff`): everything except the project info and material colours' absence of effect, which the dependency hashes decide.
pub const READS: &[&str] = &[
    "storeys", "buildings", "sites", "walls", "wall_types", "curtain_walls", "openings", "window_types", "door_types", "columns", "column_types", "beams", "beam_types", "slabs", "slab_types", "roofs", "roof_types", "stairs", "railings", "spaces", "grids", "materials", "properties", "classifications",
];

impl<const WANT: u32> protocol::InferredField<ModelSnapshot> for ModelGraph<WANT> {
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

    fn compute(snapshot: &ModelSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        compute::value(snapshot, key, parents)
    }

    fn value_bytes(value: &Self::Value) -> usize {
        value.bytes()
    }
}

/// 🕸️ Runs the graph restricted to `WANT` without a cache and projects the values into a `ModelInference` (the fields of unwanted kinds stay empty).
pub fn infer_selected<const WANT: u32>(snapshot: &ModelSnapshot) -> ModelInference {
    projection::project(protocol::infer_field::<ModelSnapshot, ModelGraph<WANT>>(snapshot, None))
}

/// 🗂️ The node values of a whole run, by key.
pub type Values = BTreeMap<ModelNode, ModelValue>;
//#endregion 🔖️InferredField

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
