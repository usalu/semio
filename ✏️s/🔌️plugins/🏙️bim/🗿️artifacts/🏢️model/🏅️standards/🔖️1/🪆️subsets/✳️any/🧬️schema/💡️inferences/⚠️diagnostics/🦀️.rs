//! ⚠️ `diagnostics`: every problem of the model as typed findings, each with a severity, a stable code, the ids of the elements involved and
//! an English and German message (key `bim.diagnostic.<slug>`, texts in [`messages`]).
//!
//! What is checked: clashes between element bodies (a bounding-box sweep, then the exact overlap of the footprints over the shared heights),
//! referential integrity (missing types, hosts, storeys, buildings, sites, materials, orphaned data, duplicate ids), invalid openings (from
//! `opening-frames`), degenerate elements (zero length, thickness, height, size, area, self-crossing or non-finite outlines), storey levels
//! that are not contiguous or overlap per building, stair compliance (from `stair-runs`) and spaces that are not enclosed or share a number.
//!
//! The DAG: storey levels are the roots; the findings of a storey depend on the levels it is resolved by and on its elements, the findings of a
//! building (clashes, levels, space numbers) on the levels and elements of all its storeys, the model findings (references) on the whole snapshot.

pub use messages::{render, row_of, Row, LOCALES};

use super::super::element_solids::roofs::RoofFallback;
use super::super::element_solids::{dep_object, dep_records, dep_types, dep_value};
use super::super::opening_frames::OpeningFrame;
use super::super::spaces::StoreyRooms;
use super::super::stair_runs::StairRun;
use super::super::storey_levels::StoreyLevel;
use super::super::wall_layout::WallLayout;
use crate::ModelSnapshot;
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

#[path = "💥️clashes/🦀️.rs"]
pub mod clashes;
#[path = "🪜️levels/🦀️.rs"]
pub mod levels;
#[path = "💬️messages/🦀️.rs"]
pub mod messages;
#[path = "🔗️references/🦀️.rs"]
pub mod references;
#[path = "📐️validity/🦀️.rs"]
pub mod validity;

//#region 🔖️Values
/// 🚦️ How serious a finding is: `Error` breaks the model, `Warning` is probably unintended, `Info` is worth knowing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// 🏷️ What a finding is about; the slug and the texts of a code are in [`messages::row_of`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum DiagnosticCode {
    ClashWallWall,
    ClashWallColumn,
    ClashColumnColumn,
    ClashWallBeam,
    ClashBeamColumn,
    ClashBeamBeam,
    ClashBeamSlab,
    ClashStairWall,
    ClashStairColumn,
    ClashStairBeam,
    ClashStairStair,
    ClashSlabSlab,
    RefWallType,
    RefColumnType,
    RefBeamType,
    RefSlabType,
    RefRoofType,
    RefWindowType,
    RefDoorType,
    RefTopStorey,
    RefOpeningHost,
    RefElementStorey,
    RefStoreyBuilding,
    RefBuildingSite,
    RefGridBuilding,
    RefLayerMaterial,
    RefTypeMaterial,
    RefPropertyElement,
    DuplicateId,
    OpeningOutsideHost,
    OpeningBelowBase,
    OpeningAboveTop,
    OpeningOverlap,
    OpeningSize,
    OpeningOutsideTrimmed,
    DegenerateAxis,
    DegenerateThickness,
    DegenerateHeight,
    DegenerateProfile,
    DegenerateLoop,
    SelfIntersectingLoop,
    DegeneratePath,
    NonFinite,
    DegenerateSpacing,
    DegenerateStorey,
    StoreyLevelGap,
    StoreyLevelDuplicate,
    StoreyNoDatum,
    StairNoRise,
    StairRiserHeight,
    StairTreadDepth,
    StairComfort,
    SpaceNotEnclosed,
    SpaceSeedInWall,
    SpaceDuplicateNumber,
    RoofFlatCurved,
    RoofFlatNonConvex,
    RoofFlatDegenerate,
    RoofFlatPitch,
    RoofOverhangCollapsed,
}

impl DiagnosticCode {
    /// 📖️ Every code in declaration order.
    pub const ALL: &'static [DiagnosticCode] = &[
        Self::ClashWallWall, Self::ClashWallColumn, Self::ClashColumnColumn, Self::ClashWallBeam, Self::ClashBeamColumn, Self::ClashBeamBeam, Self::ClashBeamSlab, Self::ClashStairWall, Self::ClashStairColumn, Self::ClashStairBeam, Self::ClashStairStair, Self::ClashSlabSlab,
        Self::RefWallType, Self::RefColumnType, Self::RefBeamType, Self::RefSlabType, Self::RefRoofType, Self::RefWindowType, Self::RefDoorType, Self::RefTopStorey, Self::RefOpeningHost, Self::RefElementStorey, Self::RefStoreyBuilding, Self::RefBuildingSite, Self::RefGridBuilding, Self::RefLayerMaterial, Self::RefTypeMaterial, Self::RefPropertyElement, Self::DuplicateId,
        Self::OpeningOutsideHost, Self::OpeningBelowBase, Self::OpeningAboveTop, Self::OpeningOverlap, Self::OpeningSize, Self::OpeningOutsideTrimmed,
        Self::DegenerateAxis, Self::DegenerateThickness, Self::DegenerateHeight, Self::DegenerateProfile, Self::DegenerateLoop, Self::SelfIntersectingLoop, Self::DegeneratePath, Self::NonFinite, Self::DegenerateSpacing, Self::DegenerateStorey,
        Self::StoreyLevelGap, Self::StoreyLevelDuplicate, Self::StoreyNoDatum,
        Self::StairNoRise, Self::StairRiserHeight, Self::StairTreadDepth, Self::StairComfort,
        Self::SpaceNotEnclosed, Self::SpaceSeedInWall, Self::SpaceDuplicateNumber,
        Self::RoofFlatCurved, Self::RoofFlatNonConvex, Self::RoofFlatDegenerate, Self::RoofFlatPitch, Self::RoofOverhangCollapsed,
    ];

    /// 🏷️ The stable slug, for example `clash.wall-wall`.
    pub fn slug(self) -> &'static str {
        row_of(self).slug
    }

    /// 🔑️ The message key, `bim.diagnostic.<slug>`.
    pub fn message_key(self) -> String {
        format!("bim.diagnostic.{}", self.slug())
    }

    /// 🚦️ The severity of the code.
    pub fn severity(self) -> Severity {
        row_of(self).severity
    }
}

/// ⚠️ One finding. `elements` are the ids involved (ordered), `missing` the referenced ids that do not exist, `storey` the storey it is on when it has one,
/// `values` the numbers of the message (areas, volumes, lengths, levels).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub message_key: String,
    pub elements: Vec<String>,
    #[value(default)]
    pub missing: Vec<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(default)]
    pub values: BTreeMap<String, f64>,
}

impl Diagnostic {
    /// ⚠️ A finding about `elements`.
    pub fn new(code: DiagnosticCode, elements: &[&str]) -> Self {
        Self { code, severity: code.severity(), message_key: code.message_key(), elements: elements.iter().map(|id| id.to_string()).collect(), missing: Vec::new(), storey: None, values: BTreeMap::new() }
    }

    /// 🪜️ Names the storey the finding is on.
    pub fn on(mut self, storey: &str) -> Self {
        self.storey = Some(storey.to_string());
        self
    }

    /// 🔗️ Names the referenced id that does not exist.
    pub fn lacking(mut self, id: &str) -> Self {
        self.missing.push(id.to_string());
        self
    }

    /// 🔢️ Adds a number of the message.
    pub fn with(mut self, name: &str, value: f64) -> Self {
        self.values.insert(name.to_string(), value);
        self
    }

    /// 🌐️ The text in a locale (`en` or `de`).
    pub fn text(&self, locale: &str) -> Option<String> {
        render(self, locale)
    }
}
//#endregion 🔖️Values

//#region 🔖️Scopes
/// 🧾️ The inferred values the findings of a scope are computed from (the parents of its `Diagnostics` node): the levels of the storeys the elements are resolved by, the wall layouts, opening frames and stair runs by element id,
/// the rooms of the storey and the reason a roof fell back to a flat roof.
#[derive(Default)]
pub struct Inputs<'a> {
    pub levels: BTreeMap<String, StoreyLevel>,
    pub layouts: BTreeMap<&'a str, &'a WallLayout>,
    pub frames: BTreeMap<&'a str, &'a OpeningFrame>,
    pub runs: BTreeMap<&'a str, &'a StairRun>,
    pub rooms: Option<&'a StoreyRooms>,
    pub fallbacks: BTreeMap<&'a str, RoofFallback>,
}

/// 🪜️ The storeys of a building.
pub fn building_storeys(snapshot: &ModelSnapshot, building: &str) -> Vec<String> {
    snapshot.storeys.iter().filter(|(_, storey)| storey.building == building).map(|(id, _)| id.clone()).collect()
}

/// 🪜️ The findings of one storey from the values its parents inferred.
pub fn storey_findings(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>) -> Vec<Diagnostic> {
    let mut found = references::storey(snapshot, storey);
    found.extend(validity::storey(snapshot, storey, inputs));
    found
}

/// 🏢️ The findings of one building from the values of its storeys.
pub fn building_findings(snapshot: &ModelSnapshot, building: &str, inputs: &Inputs<'_>) -> Vec<Diagnostic> {
    let mut found = levels::building(snapshot, building);
    found.extend(clashes::building(snapshot, building, inputs));
    found
}

/// 🌍️ The findings that belong to no storey or building: dangling references of the whole model.
pub fn model_findings(view: &references::ReferenceView) -> Vec<Diagnostic> {
    references::model(view)
}

//#region 🔖️Dependency
/// 📖️ The snapshot collections the diagnostics read: all of them.
pub const READS: &[&str] = &[
    "project", "materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "sites", "buildings", "storeys", "grids", "walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "properties", "classifications",
];

fn present<'a, T>(ids: impl IntoIterator<Item = &'a String>, library: &BTreeMap<String, T>) -> DslValue {
    let ids: BTreeSet<&String> = ids.into_iter().collect();
    DslValue::object(ids.into_iter().map(|id| (id.clone(), semio_framework_value::ToValue::to_value(&library.contains_key(id)))))
}

/// 🔑️ What the findings of `storey` read of the snapshot besides the values of their parents (levels, wall layouts, opening frames, stair runs, rooms, roof solids; wall and opening records are inside those):
/// the storey record, the curtain walls, columns, beams, slabs, roofs, railings and spaces of the storey, the profiles of the column and beam types they name and whether their other types exist.
pub fn storey_dependency(snapshot: &ModelSnapshot, storey: &str) -> DslValue {
    let columns = snapshot.columns.iter().filter(|(_, row)| row.storey == storey);
    let beams = snapshot.beams.iter().filter(|(_, row)| row.storey == storey);
    let slabs = snapshot.slabs.iter().filter(|(_, row)| row.storey == storey);
    let roofs = snapshot.roofs.iter().filter(|(_, row)| row.storey == storey);
    let spaces = snapshot.spaces.iter().filter(|(_, row)| row.storey == storey).map(|(id, space)| (id.clone(), dep_value(&space.boundary)));
    let profiles = |rows: Vec<(&String, Option<crate::Profile>)>| DslValue::object(rows.into_iter().map(|(id, profile)| (id.clone(), dep_value(&profile))));
    dep_object([
        ("storey", dep_value(&snapshot.storeys.get(storey).cloned())),
        ("curtain_walls", dep_records(snapshot.curtain_walls.iter().filter(|(_, row)| row.storey == storey))),
        ("columns", dep_records(columns.clone())),
        ("column_profiles", profiles(columns.map(|(_, row)| (&row.column_type, snapshot.column_types.get(&row.column_type).map(|kind| kind.profile.clone()))).collect())),
        ("beams", dep_records(beams.clone())),
        ("beam_profiles", profiles(beams.map(|(_, row)| (&row.beam_type, snapshot.beam_types.get(&row.beam_type).map(|kind| kind.profile.clone()))).collect())),
        ("slabs", dep_records(slabs.clone())),
        ("slab_types", present(slabs.map(|(_, row)| &row.slab_type), &snapshot.slab_types)),
        ("roofs", dep_records(roofs.clone())),
        ("roof_types", present(roofs.map(|(_, row)| &row.roof_type), &snapshot.roof_types)),
        ("railings", dep_records(snapshot.railings.iter().filter(|(_, row)| row.storey == storey))),
        ("spaces", DslValue::object(spaces)),
    ])
}

/// 🔑️ What the findings of `building` read besides the values of their parents (the levels of all its storeys, their wall layouts and stair runs): the levels and numbers it checks, and the curtain walls, columns, beams and slabs that become bodies.
pub fn building_dependency(snapshot: &ModelSnapshot, building: &str) -> DslValue {
    let storeys: BTreeSet<&String> = snapshot.storeys.iter().filter(|(_, row)| row.building == building).map(|(id, _)| id).collect();
    let of = |storey: &String| storeys.contains(storey);
    let columns = snapshot.columns.iter().filter(|(_, row)| of(&row.storey));
    let beams = snapshot.beams.iter().filter(|(_, row)| of(&row.storey));
    let slabs = snapshot.slabs.iter().filter(|(_, row)| of(&row.storey));
    let levels = snapshot.storeys.iter().filter(|(_, row)| row.building == building).map(|(id, row)| (id.clone(), dep_value(&row.level)));
    let numbers = snapshot.spaces.iter().filter(|(_, row)| of(&row.storey)).map(|(id, row)| (id.clone(), dep_value(&row.number)));
    dep_object([
        ("levels", DslValue::object(levels)),
        ("numbers", DslValue::object(numbers)),
        ("curtain_walls", dep_records(snapshot.curtain_walls.iter().filter(|(_, row)| of(&row.storey)))),
        ("columns", dep_records(columns.clone())),
        ("column_types", dep_types(columns.map(|(_, row)| &row.column_type), &snapshot.column_types)),
        ("beams", dep_records(beams.clone())),
        ("beam_types", dep_types(beams.map(|(_, row)| &row.beam_type), &snapshot.beam_types)),
        ("slabs", dep_records(slabs.clone())),
        ("slab_types", dep_types(slabs.map(|(_, row)| &row.slab_type), &snapshot.slab_types)),
    ])
}

/// 🔑️ What the model findings read: the reference structure of the model, no geometry.
pub fn model_dependency(snapshot: &ModelSnapshot) -> DslValue {
    dep_value(&references::ReferenceView::of(snapshot))
}
//#endregion 🔖️Dependency

/// 🧮️ Sorted, de-duplicated findings: most severe first, then by code, storey and elements.
pub fn ordered(mut found: Vec<Diagnostic>) -> Vec<Diagnostic> {
    found.sort_by(|a, b| b.severity.cmp(&a.severity).then(a.code.cmp(&b.code)).then_with(|| a.storey.cmp(&b.storey)).then_with(|| a.elements.cmp(&b.elements)).then_with(|| a.missing.cmp(&b.missing)));
    found.dedup();
    found
}

/// ⚠️ The findings of the whole model (the `Diagnostics` nodes of the model graph, ordered).
pub fn compute_diagnostics(snapshot: &ModelSnapshot) -> Vec<Diagnostic> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::DIAGNOSTICS }>(snapshot).diagnostics)
}

/// 📖️ The codes a third-party library can adjudicate on its own: pairs of prismatic bodies, missing ids, extents and level arithmetic.
pub const ADJUDICATED: &[DiagnosticCode] = &[
    DiagnosticCode::ClashWallWall,
    DiagnosticCode::ClashWallColumn,
    DiagnosticCode::ClashColumnColumn,
    DiagnosticCode::ClashWallBeam,
    DiagnosticCode::ClashBeamColumn,
    DiagnosticCode::ClashBeamBeam,
    DiagnosticCode::RefWallType,
    DiagnosticCode::RefOpeningHost,
    DiagnosticCode::OpeningOutsideHost,
    DiagnosticCode::DegenerateAxis,
    DiagnosticCode::StoreyLevelDuplicate,
    DiagnosticCode::StoreyLevelGap,
    DiagnosticCode::SpaceDuplicateNumber,
];

/// ⚖️ The measure of a finding the oracle compares: the overlap area of a clash, the number of levels a gap skips, else zero.
pub fn measure_of(finding: &Diagnostic) -> f64 {
    match finding.code {
        DiagnosticCode::StoreyLevelGap => finding.values.get("to").zip(finding.values.get("from")).map_or(0.0, |(to, from)| to - from),
        _ => finding.values.get("overlap_area").copied().unwrap_or(0.0),
    }
}


//#endregion 🔖️Scopes


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
