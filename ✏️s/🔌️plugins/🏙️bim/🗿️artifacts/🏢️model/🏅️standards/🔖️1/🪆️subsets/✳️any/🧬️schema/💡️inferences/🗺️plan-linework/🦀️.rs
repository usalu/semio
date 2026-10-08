//! 🗺️ `plan-linework`: the architectural floor plan of every storey as typed primitives, cut at a fixed height above the storey elevation.
//!
//! Output is a [`PlanLinework`] per storey in building coordinates (metres): filled [`PlanRegion`]s with holes (cut walls, cut columns and
//! mullions as poché), [`PlanPolyline`]s whose vertices carry a bulge (layer lines, openings, outlines, stairs, railings, grid lines, bubbles)
//! and [`PlanText`] anchors (space tags, grid labels). Every primitive has a [`PlanStyle`] (`Cut`, `Projection`, `Hidden`, `Annotation`), a
//! [`PlanKind`] that tells what it depicts, the id of the snapshot element it belongs to (picking) and a unique id. A region or polyline maps
//! one to one onto a `Canvas2d` layer record: `outer`/`vertices` become `Move`/`Line`/`Arc` path segments (the bulge `tan(sweep / 4)` is the
//! arc), the style picks stroke weight and dash.
//!
//! Plan convention: the cut plane is [`CUT_HEIGHT`] above the storey elevation. An element whose vertical span `[low, high)` contains the cut is
//! cut (poché or cut outline), one entirely below is drawn in projection, one entirely above is drawn hidden (dashed): beams, roofs, demolished
//! walls. Walls lose the part of the footprint that an opening cutting the plane removes; openings add their symbols (door leaf and swing, window
//! frame, glazing and sill); stairs show risers, the cut line and the up arrow; spaces show outline and tag; grid lines show bubbles and labels.
//!
//! The graph is `Storey`, `WallLayout`, `CurtainLayout`, `OpeningFrame`, `StairRun`, `Room` → `Plan(storey)`: the plan of a storey is drawn from the values of
//! those parents ([`Inputs`]), never from a second derivation of them, and depends besides on the elements it draws from the snapshot (see [`dependency`]).
//!
//! Related: <https://en.wikipedia.org/wiki/Floor_plan>.

use super::super::curtain_layout::CurtainLayout;
use super::super::opening_frames::OpeningFrame;
use super::super::spaces::StoreyRooms;
use super::super::stair_runs::StairRun;
use super::super::storey_levels::StoreyLevel;
use super::super::wall_layout::WallLayout;
use super::super::element_solids::{dep_object, dep_records, dep_types, dep_value};
use crate::ModelSnapshot;
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::loops::Vertex as Corner;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

#[path = "🏛️members/🦀️.rs"]
pub mod members;
#[path = "📍️annotations/🦀️.rs"]
pub mod annotations;
#[path = "🪜️stairs/🦀️.rs"]
pub mod stairs;
#[path = "🧱️walls/🦀️.rs"]
pub mod walls;

//#region 🔖️Values
/// ✂️ Height of the cut plane above the storey elevation, in metres: the plan convention for every storey.
pub const CUT_HEIGHT: f64 = 1.2;

/// 🖊️ The line class of a primitive: how the plan window strokes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum PlanStyle {
    Cut,
    Projection,
    Hidden,
    Annotation,
}

/// 🏷️ What a primitive depicts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum PlanKind {
    WallCut,
    WallLayer,
    WallOutline,
    CurtainAxis,
    CurtainMullion,
    WindowFrame,
    WindowGlazing,
    WindowSill,
    DoorLeaf,
    DoorSwing,
    ColumnCut,
    ColumnOutline,
    BeamOutline,
    SlabEdge,
    SlabHole,
    RoofOutline,
    StairOutline,
    StairRiser,
    StairCutLine,
    StairArrow,
    StairLanding,
    RailingPath,
    SpaceOutline,
    SpaceTag,
    GridLine,
    GridBubble,
    GridLabel,
}

/// 📍️ A path vertex: a point and the bulge `tan(sweep / 4)` of the segment leaving it (zero = straight).
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PlanVertex {
    pub x: f64,
    pub y: f64,
    pub bulge: f64,
}

/// 🟫️ A filled region: a closed outer loop and closed hole loops.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PlanRegion {
    pub id: String,
    pub element: String,
    pub kind: PlanKind,
    pub style: PlanStyle,
    pub outer: Vec<PlanVertex>,
    pub holes: Vec<Vec<PlanVertex>>,
}

/// 〰️ A stroked path, open or closed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PlanPolyline {
    pub id: String,
    pub element: String,
    pub kind: PlanKind,
    pub style: PlanStyle,
    pub closed: bool,
    pub vertices: Vec<PlanVertex>,
}

/// 🔤️ A text anchor: `label` is the primary text (space number, grid label), `detail` the secondary (space name), `measure` a quantity (space area in square metres).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PlanText {
    pub id: String,
    pub element: String,
    pub kind: PlanKind,
    pub style: PlanStyle,
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub label: String,
    pub detail: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub measure: Option<f64>,
}

/// ▭️ The rectangle that holds every primitive of a plan.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PlanBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

/// 🗺️ The plan of one storey. `cut_elevation` is the height of the cut plane above the building datum, `cut_height` above the storey elevation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PlanLinework {
    pub storey: String,
    pub cut_height: f64,
    pub cut_elevation: f64,
    pub regions: Vec<PlanRegion>,
    pub polylines: Vec<PlanPolyline>,
    pub texts: Vec<PlanText>,
    pub bounds: PlanBounds,
}
//#endregion 🔖️Values

//#region 🔖️Sheet
/// 🧭️ Where an element sits relative to the cut plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cutting {
    Below,
    Cut,
    Above,
}

/// ✂️ The cutting of the half-open span `[low, high)` by the plane at `cut`.
pub fn classify(low: f64, high: f64, cut: f64) -> Cutting {
    if high <= cut {
        Cutting::Below
    } else if low > cut {
        Cutting::Above
    } else {
        Cutting::Cut
    }
}

/// 🖊️ The style of an uncut element: projection below the cut, hidden above it.
pub fn outline_style(cutting: Cutting) -> PlanStyle {
    match cutting {
        Cutting::Cut => PlanStyle::Cut,
        Cutting::Below => PlanStyle::Projection,
        Cutting::Above => PlanStyle::Hidden,
    }
}

/// 🧾️ The inferred values a plan is drawn from: the levels of the storeys the elements are resolved by, and the wall layouts, curtain layouts, opening frames, stair runs and rooms of the storey (the parents of its `Plan` node), by element id.
#[derive(Default)]
pub struct Inputs<'a> {
    pub levels: BTreeMap<String, StoreyLevel>,
    pub layouts: BTreeMap<&'a str, &'a WallLayout>,
    pub curtains: BTreeMap<&'a str, &'a CurtainLayout>,
    pub frames: BTreeMap<&'a str, &'a OpeningFrame>,
    pub runs: BTreeMap<&'a str, &'a StairRun>,
    pub rooms: Option<&'a StoreyRooms>,
}

/// 🧾️ What every drawing routine needs: the model, the storey, its levels, the inferred values and the cut.
pub struct Context<'a> {
    pub snapshot: &'a ModelSnapshot,
    pub storey: &'a str,
    pub own: StoreyLevel,
    pub levels: &'a BTreeMap<String, StoreyLevel>,
    pub inputs: &'a Inputs<'a>,
    pub cut: f64,
}

/// 🖍️ The primitives collected for one storey, with ids assigned per (element, kind).
#[derive(Default)]
pub struct Sheet {
    pub regions: Vec<PlanRegion>,
    pub polylines: Vec<PlanPolyline>,
    pub texts: Vec<PlanText>,
    counts: BTreeMap<(String, PlanKind), u32>,
}

impl Sheet {
    fn id(&mut self, element: &str, kind: PlanKind) -> String {
        let count = self.counts.entry((element.to_string(), kind)).or_insert(0);
        *count += 1;
        format!("{element}/{kind:?}/{}", *count - 1)
    }

    /// 🟫️ Adds a filled region.
    pub fn region(&mut self, element: &str, kind: PlanKind, style: PlanStyle, outer: Vec<PlanVertex>, holes: Vec<Vec<PlanVertex>>) {
        if outer.len() >= 2 {
            let id = self.id(element, kind);
            self.regions.push(PlanRegion { id, element: element.to_string(), kind, style, outer, holes });
        }
    }

    /// 〰️ Adds a stroked path; paths with fewer than two vertices are dropped.
    pub fn polyline(&mut self, element: &str, kind: PlanKind, style: PlanStyle, closed: bool, vertices: Vec<PlanVertex>) {
        if vertices.len() >= 2 {
            let id = self.id(element, kind);
            self.polylines.push(PlanPolyline { id, element: element.to_string(), kind, style, closed, vertices });
        }
    }

    /// 🔤️ Adds a text anchor.
    #[allow(clippy::too_many_arguments)]
    pub fn text(&mut self, element: &str, kind: PlanKind, style: PlanStyle, at: Point, rotation: f64, label: &str, detail: &str, measure: Option<f64>) {
        let id = self.id(element, kind);
        self.texts.push(PlanText { id, element: element.to_string(), kind, style, x: at.x, y: at.y, rotation, label: label.to_string(), detail: detail.to_string(), measure });
    }

    /// 🟫️ Adds a region from geometry loops.
    pub fn loop_region(&mut self, element: &str, kind: PlanKind, style: PlanStyle, outer: &[Corner], holes: &[Vec<Corner>]) {
        self.region(element, kind, style, vertices_of(outer), holes.iter().map(|hole| vertices_of(hole)).collect());
    }

    /// 〰️ Adds a closed path from a geometry loop.
    pub fn loop_polyline(&mut self, element: &str, kind: PlanKind, style: PlanStyle, outline: &[Corner]) {
        self.polyline(element, kind, style, true, vertices_of(outline));
    }

    /// 〰️ Adds an open path along a segment.
    pub fn segment(&mut self, element: &str, kind: PlanKind, style: PlanStyle, line: &BulgeSeg) {
        self.polyline(element, kind, style, false, vec![PlanVertex { x: line.start.x, y: line.start.y, bulge: line.bulge }, PlanVertex { x: line.end.x, y: line.end.y, bulge: 0.0 }]);
    }

    /// 〰️ Adds an open path through straight points.
    pub fn path(&mut self, element: &str, kind: PlanKind, style: PlanStyle, points: &[Point]) {
        self.polyline(element, kind, style, false, points.iter().map(|p| PlanVertex { x: p.x, y: p.y, bulge: 0.0 }).collect());
    }
}

/// 📍️ Path vertices of a geometry loop.
pub fn vertices_of(corners: &[Corner]) -> Vec<PlanVertex> {
    corners.iter().map(|c| PlanVertex { x: c.point.x, y: c.point.y, bulge: c.bulge }).collect()
}

/// 📍️ Path vertices of a polygon ring.
pub fn ring_vertices(ring: &[[f64; 2]]) -> Vec<PlanVertex> {
    ring.iter().map(|p| PlanVertex { x: p[0], y: p[1], bulge: 0.0 }).collect()
}

fn extend(bounds: &mut Option<[f64; 4]>, point: Point) {
    let [x0, y0, x1, y1] = bounds.unwrap_or([point.x, point.y, point.x, point.y]);
    *bounds = Some([x0.min(point.x), y0.min(point.y), x1.max(point.x), y1.max(point.y)]);
}

fn measure(bounds: &mut Option<[f64; 4]>, vertices: &[PlanVertex], closed: bool) {
    let count = vertices.len();
    let spans = if closed { count } else { count.saturating_sub(1) };
    for index in 0..spans {
        let (a, b) = (vertices[index], vertices[(index + 1) % count]);
        let rect = BulgeSeg::new(Point::new(a.x, a.y), Point::new(b.x, b.y), a.bulge).bounds();
        extend(bounds, Point::new(rect.x0(), rect.y0()));
        extend(bounds, Point::new(rect.x1(), rect.y1()));
    }
}

fn loop_area(vertices: &[PlanVertex]) -> f64 {
    let ring: Vec<Corner> = vertices.iter().map(|v| Corner::new(Point::new(v.x, v.y), v.bulge)).collect();
    semio_framework_geometry::loops::area(&ring)
}

impl PlanLinework {
    /// 📐️ Total area (square metres) of the regions of a kind, holes subtracted.
    pub fn area_of(&self, kind: PlanKind) -> f64 {
        self.regions.iter().filter(|region| region.kind == kind).map(|region| loop_area(&region.outer) - region.holes.iter().map(|hole| loop_area(hole)).sum::<f64>()).sum()
    }

    /// 📏️ Total length (metres) of the polylines of a kind; a closed polyline counts its closing segment.
    pub fn length_of(&self, kind: PlanKind) -> f64 {
        self.polylines
            .iter()
            .filter(|line| line.kind == kind)
            .map(|line| {
                let count = line.vertices.len();
                let spans = if line.closed { count } else { count.saturating_sub(1) };
                (0..spans).map(|i| BulgeSeg::new(Point::new(line.vertices[i].x, line.vertices[i].y), Point::new(line.vertices[(i + 1) % count].x, line.vertices[(i + 1) % count].y), line.vertices[i].bulge).length()).sum::<f64>()
            })
            .sum()
    }

    /// 🔢️ Number of regions, polylines and texts of a kind.
    pub fn count_of(&self, kind: PlanKind) -> usize {
        self.regions.iter().filter(|row| row.kind == kind).count() + self.polylines.iter().filter(|row| row.kind == kind).count() + self.texts.iter().filter(|row| row.kind == kind).count()
    }
}

impl Sheet {
    /// 📦️ The finished plan of `storey`.
    pub fn finish(self, storey: &str, cut_elevation: f64) -> PlanLinework {
        let mut bounds = None;
        self.regions.iter().for_each(|region| measure(&mut bounds, &region.outer, true));
        self.polylines.iter().for_each(|line| measure(&mut bounds, &line.vertices, line.closed));
        self.texts.iter().for_each(|text| extend(&mut bounds, Point::new(text.x, text.y)));
        let [min_x, min_y, max_x, max_y] = bounds.unwrap_or([0.0; 4]);
        PlanLinework { storey: storey.to_string(), cut_height: CUT_HEIGHT, cut_elevation, regions: self.regions, polylines: self.polylines, texts: self.texts, bounds: PlanBounds { min_x, min_y, max_x, max_y } }
    }
}
//#endregion 🔖️Sheet

//#region 🔖️Plan
/// 🗺️ The plan of one storey from the values its parents inferred.
pub fn plan_of(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>) -> PlanLinework {
    let own = inputs.levels.get(storey).copied().unwrap_or_default();
    let cx = Context { snapshot, storey, own, levels: &inputs.levels, inputs, cut: own.elevation + CUT_HEIGHT };
    let mut sheet = Sheet::default();
    annotations::draw_grids(&mut sheet, &cx);
    walls::draw(&mut sheet, &cx);
    members::draw(&mut sheet, &cx);
    stairs::draw(&mut sheet, &cx);
    annotations::draw_spaces(&mut sheet, &cx);
    sheet.finish(storey, cx.cut)
}

/// 🗺️ The plan of every storey (the `Plan` nodes of the model graph).
pub fn compute_plan_linework(snapshot: &ModelSnapshot) -> BTreeMap<String, PlanLinework> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::PLANS }>(snapshot).plan_linework)
}
/// 📏️ The measures of a plan that third-party geometry libraries can adjudicate: areas of cut regions and lengths of outlines.
pub const METRICS: &[(&str, PlanKind, bool)] = &[
    ("WallCut.area", PlanKind::WallCut, true),
    ("ColumnCut.area", PlanKind::ColumnCut, true),
    ("CurtainMullion.area", PlanKind::CurtainMullion, true),
    ("SlabEdge.length", PlanKind::SlabEdge, false),
    ("BeamOutline.length", PlanKind::BeamOutline, false),
    ("RailingPath.length", PlanKind::RailingPath, false),
    ("GridLine.length", PlanKind::GridLine, false),
    ("SpaceOutline.length", PlanKind::SpaceOutline, false),
];

/// 📏️ The canonical JSON table `storey → measure → value` the plan oracle compares.
pub fn metrics_json(plans: &BTreeMap<String, PlanLinework>) -> String {
    let table: BTreeMap<String, BTreeMap<String, f64>> = plans.iter().map(|(storey, plan)| (storey.clone(), METRICS.iter().map(|(name, kind, area)| (name.to_string(), if *area { plan.area_of(*kind) } else { plan.length_of(*kind) })).collect())).collect();
    semio_framework_pack_json::to_json_string(&table)
}
//#endregion 🔖️Plan

//#region 🔖️Dependency
/// 🔑️ Everything the plan of `storey` reads of the snapshot besides the values of its parents (levels, wall and curtain layouts, opening frames, stair runs, rooms): the storey record, the
/// curtain walls, columns, beams, slabs, roofs and railings of the storey with the types they name, the tags of its spaces and the grid lines of its building.
pub fn dependency(snapshot: &ModelSnapshot, storey: &str) -> DslValue {
    let building = snapshot.storeys.get(storey).map(|row| row.building.clone());
    let columns = snapshot.columns.iter().filter(|(_, row)| row.storey == storey);
    let beams = snapshot.beams.iter().filter(|(_, row)| row.storey == storey);
    let slabs = snapshot.slabs.iter().filter(|(_, row)| row.storey == storey);
    let spaces = snapshot.spaces.iter().filter(|(_, row)| row.storey == storey).map(|(id, space)| (id.clone(), dep_object([("number", dep_value(&space.number)), ("name", dep_value(&space.name)), ("boundary", dep_value(&space.boundary))])));
    let grids = snapshot.grids.iter().filter(|(_, row)| Some(&row.building) == building.as_ref());
    dep_object([
        ("storey", dep_value(&snapshot.storeys.get(storey).cloned())),
        ("curtain_walls", dep_records(snapshot.curtain_walls.iter().filter(|(_, row)| row.storey == storey))),
        ("columns", dep_records(columns.clone())),
        ("column_types", dep_types(columns.map(|(_, row)| &row.column_type), &snapshot.column_types)),
        ("beams", dep_records(beams.clone())),
        ("beam_types", dep_types(beams.map(|(_, row)| &row.beam_type), &snapshot.beam_types)),
        ("slabs", dep_records(slabs.clone())),
        ("slab_types", dep_types(slabs.map(|(_, row)| &row.slab_type), &snapshot.slab_types)),
        ("roofs", dep_records(snapshot.roofs.iter().filter(|(_, row)| row.storey == storey))),
        ("railings", dep_records(snapshot.railings.iter().filter(|(_, row)| row.storey == storey))),
        ("spaces", DslValue::object(spaces)),
        ("grids", DslValue::object(grids.map(|(id, grid)| (id.clone(), dep_value(grid))))),
    ])
}

/// 📖️ The snapshot collections the plan reads.
pub const READS: &[&str] = &[
    "storeys", "buildings", "sites", "walls", "wall_types", "curtain_walls", "openings", "window_types", "door_types", "columns", "column_types", "beams", "beam_types", "slabs", "slab_types", "roofs", "stairs", "railings", "spaces", "grids", "materials",
];
//#endregion 🔖️Dependency

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
