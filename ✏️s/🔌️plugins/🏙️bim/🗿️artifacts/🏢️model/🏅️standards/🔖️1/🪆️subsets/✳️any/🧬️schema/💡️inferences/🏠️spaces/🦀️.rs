//! 🏠️ `spaces`: the outline, areas, clear height and volume of every space. A space stores its storey, number, name, usage and a boundary
//! that is either an explicit loop or a `Bounded { seed }`; the room itself is derived here. A bounded room is the face that contains
//! the seed in the arrangement of the storey: the extent of the storey's wall footprints (join-trimmed, exactly as `wall-layout` trims
//! them, plus curtain-wall bands) grown by [`OPEN_MARGIN`] minus the union of those footprints. A face that reaches the extent is open
//! to the outside and reported as [`SpaceStatus::NotEnclosed`]; a seed inside a wall as [`SpaceStatus::SeedInsideWall`].
//!
//! The graph has one `Room` node per storey that has spaces; its parents are that storey's level and the `WallLayout` of every wall of the storey (the footprints are the
//! obstacles): moving a wall of a storey re-infers the rooms of that storey only, editing the height of a storey re-infers its clear heights.
//!
//! Related: shapely `polygonize` is the third-party oracle, <https://shapely.readthedocs.io/en/stable/manual.html#shapely.ops.polygonize>.

use super::super::element_solids::columns::profile_loop;
use super::super::element_solids::plan_kit::{depth_of, seg};
use super::super::element_solids::{dep_object, dep_value};
use super::super::storey_levels::StoreyLevel;
use super::super::wall_layout::WallLayout;
use crate::{ModelSnapshot, Point2, SpaceBoundary, Vertex};
use semio_framework_2d::booleans::{BooleanOperation, BooleanProgress};
use semio_framework_2d::regions::{offset_regions, region_boolean, OffsetJoin, Region};
use semio_framework_2d::Vec2;
use semio_framework_geometry::bulge::band_loop;
use semio_framework_geometry::loops;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

/// 📏️ Chord tolerance (sagitta, metres) used to flatten curved wall faces and explicit arcs before the region booleans.
pub const FLATTEN_TOLERANCE: f64 = 1e-6;
/// 📏️ How far the arrangement extent extends beyond the footprints and seeds, in metres; a face that reaches it is open.
pub const OPEN_MARGIN: f64 = 1.0;
/// 📏️ Growth of a room before it is intersected with the footprints to find its bounding walls, in metres.
pub const CONTACT_GROWTH: f64 = 1e-4;
const CONTACT_AREA: f64 = CONTACT_GROWTH * 1e-3;

//#region 🔖️Values
/// 🩺️ How a room was resolved: `Inferred` from the walls around the seed, `Explicit` as authored; the rest are diagnostics (no outline, zero areas).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum SpaceStatus {
    #[default]
    Inferred,
    Explicit,
    NotEnclosed,
    SeedInsideWall,
    InvalidOutline,
}

/// 🏠️ Resolved room of one space, in metres, square metres and cubic metres.
/// `outline` is the counter-clockwise outer loop (bulges only for explicit outlines), `holes` the clockwise islands inside it.
/// `area` is the outline area minus the holes, `net_floor_area` additionally excludes the columns of the storey, `perimeter` adds the hole boundaries.
/// `clear_height` is the storey height plus the offset minus the thickness of the slab of the storey above that covers `point`, `volume = area * clear_height`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SpaceRoom {
    pub status: SpaceStatus,
    pub outline: Vec<Vertex>,
    pub holes: Vec<Vec<Vertex>>,
    pub point: Point2,
    pub area: f64,
    pub perimeter: f64,
    pub net_floor_area: f64,
    pub floor_z: f64,
    pub clear_height: f64,
    pub volume: f64,
    pub ceiling_slab: String,
    pub bounding_walls: Vec<String>,
}

impl SpaceRoom {
    fn refused(status: SpaceStatus, point: Point2, floor_z: f64) -> Self {
        Self { status, outline: Vec::new(), holes: Vec::new(), point, area: 0.0, perimeter: 0.0, net_floor_area: 0.0, floor_z, clear_height: 0.0, volume: 0.0, ceiling_slab: String::new(), bounding_walls: Vec::new() }
    }
}

/// 🏠️ The rooms of the spaces of one storey, keyed by space id.
pub type StoreyRooms = BTreeMap<String, SpaceRoom>;
//#endregion 🔖️Values

//#region 🔖️Arrangement
fn flat(vertices: &[loops::Vertex]) -> Vec<Vec2> {
    loops::flatten(vertices, FLATTEN_TOLERANCE).iter().map(|point| [point.x, point.y]).collect()
}

fn plan_loop(vertices: &[Vertex]) -> Vec<loops::Vertex> {
    vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect()
}

fn corners(ring: &[Vec2]) -> Vec<Vertex> {
    ring.iter().map(|point| Vertex { point: Point2 { x: point[0], y: point[1] }, bulge: 0.0 }).collect()
}

fn ring_length(ring: &[Vec2]) -> f64 {
    (0..ring.len()).map(|index| (ring[(index + 1) % ring.len()][0] - ring[index][0]).hypot(ring[(index + 1) % ring.len()][1] - ring[index][1])).sum()
}

fn bounds_of<'a>(points: impl Iterator<Item = &'a Vec2>) -> Option<[f64; 4]> {
    points.fold(None, |bounds, point| match bounds {
        None => Some([point[0], point[1], point[0], point[1]]),
        Some([x0, y0, x1, y1]) => Some([x0.min(point[0]), y0.min(point[1]), x1.max(point[0]), y1.max(point[1])]),
    })
}

/// 🧱️ A plan obstacle of a storey: the id of the element, its footprint flattened to a ring and the bounding rectangle of the ring (computed once, tested before any boolean).
#[derive(Clone, Debug, PartialEq)]
pub struct Obstacle {
    pub id: String,
    pub ring: Vec<Vec2>,
    pub bounds: [f64; 4],
}

impl Obstacle {
    fn new(id: &str, ring: Vec<Vec2>) -> Option<Self> {
        let bounds = bounds_of(ring.iter())?;
        Some(Self { id: id.to_string(), ring, bounds })
    }
}

/// 🧱️ The plan obstacles of a storey in id order: the join-trimmed footprints of its walls (taken from their `WallLayout`s) and the bands of its curtain walls.
pub fn obstacles_of(snapshot: &ModelSnapshot, storey: &str, layouts: &BTreeMap<&str, &WallLayout>) -> Vec<Obstacle> {
    let walls = snapshot.walls.iter().filter(|(_, wall)| wall.storey == storey).filter_map(|(id, _)| {
        let layout = layouts.get(id.as_str())?;
        let outline: Vec<loops::Vertex> = layout.footprint.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect();
        (!outline.is_empty()).then(|| Obstacle::new(id, flat(&outline))).flatten()
    });
    let curtains = snapshot.curtain_walls.iter().filter(|(_, curtain)| curtain.storey == storey).filter_map(|(id, curtain)| {
        let depth = depth_of(&curtain.mullion);
        let outline = band_loop(&seg(&curtain.axis), depth / 2.0, depth / 2.0, None, None)?;
        let outline: Vec<loops::Vertex> = outline.iter().map(|(point, bulge)| loops::Vertex::new(*point, *bulge)).collect();
        Obstacle::new(id, flat(&outline))
    });
    let mut obstacles: Vec<Obstacle> = walls.chain(curtains).collect();
    obstacles.sort_by(|a, b| a.id.cmp(&b.id));
    obstacles
}

fn regions_of(obstacles: &[Obstacle]) -> Vec<Region> {
    obstacles.iter().map(|obstacle| Region::new(&obstacle.ring, &[])).collect()
}

fn run(control: &mut dyn FnMut(&BooleanProgress) -> bool, operation: BooleanOperation, subject: &[Region], clip: &[Region]) -> Vec<Region> {
    region_boolean(operation, subject, clip, control).unwrap_or_default()
}

fn inside(ring: &[Vec2], point: Vec2) -> bool {
    let mut crossings = false;
    for index in 0..ring.len() {
        let (a, b) = (ring[index], ring[(index + 1) % ring.len()]);
        if (a[1] > point[1]) != (b[1] > point[1]) && point[0] < a[0] + (point[1] - a[1]) / (b[1] - a[1]) * (b[0] - a[0]) {
            crossings = !crossings;
        }
    }
    crossings
}

fn holds(region: &Region, point: Vec2) -> bool {
    inside(&region.outer, point) && !region.holes.iter().any(|hole| inside(hole, point))
}

/// 📍️ A point strictly inside a region: the middle of the widest span of the horizontal line through the middle of its bounds.
pub fn interior_point(region: &Region) -> Option<Point2> {
    let [_, y0, _, y1] = bounds_of(region.outer.iter())?;
    let y = (y0 + y1) / 2.0;
    let mut crossings: Vec<f64> = std::iter::once(&region.outer)
        .chain(region.holes.iter())
        .flat_map(|ring| (0..ring.len()).filter_map(move |index| {
            let (a, b) = (ring[index], ring[(index + 1) % ring.len()]);
            ((a[1] > y) != (b[1] > y)).then(|| a[0] + (y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]))
        }))
        .collect();
    crossings.sort_by(f64::total_cmp);
    crossings.chunks_exact(2).max_by(|left, right| (left[1] - left[0]).total_cmp(&(right[1] - right[0]))).map(|span| Point2 { x: (span[0] + span[1]) / 2.0, y })
}

fn column_regions(snapshot: &ModelSnapshot, storey: &str) -> Vec<Region> {
    snapshot
        .columns
        .values()
        .filter(|column| column.storey == storey)
        .filter_map(|column| {
            let kind = snapshot.column_types.get(&column.column_type)?;
            let (sin, cos) = column.rotation.sin_cos();
            let placed: Vec<loops::Vertex> = profile_loop(&kind.profile).iter().map(|v| loops::Vertex::new(Point::new(column.position.x + v.point.x * cos - v.point.y * sin, column.position.y + v.point.x * sin + v.point.y * cos), v.bulge)).collect();
            let ring = flat(&placed);
            (ring.len() >= 3).then(|| Region::new(&ring, &[]))
        })
        .collect()
}

fn above_of(snapshot: &ModelSnapshot, storey: &str) -> Option<String> {
    let own = snapshot.storeys.get(storey)?;
    snapshot.storeys.iter().filter(|(_, other)| other.building == own.building && other.level > own.level).min_by_key(|(id, other)| (other.level, (*id).clone())).map(|(id, _)| id.clone())
}

fn slab_thickness(snapshot: &ModelSnapshot, slab_type: &str) -> f64 {
    snapshot.slab_types.get(slab_type).map_or(0.0, |kind| kind.layers.iter().map(|layer| layer.thickness.max(0.0)).sum())
}

fn ceiling_of(snapshot: &ModelSnapshot, storey: &str, point: Point2) -> Option<(String, f64, f64)> {
    let above = above_of(snapshot, storey)?;
    let probe = Point::new(point.x, point.y);
    snapshot
        .slabs
        .iter()
        .filter(|(_, slab)| slab.storey == above && loops::contains(&plan_loop(&slab.boundary), probe) && !slab.holes.iter().any(|hole| loops::contains(&plan_loop(hole), probe)))
        .map(|(id, slab)| (id.clone(), slab_thickness(snapshot, &slab.slab_type), slab.offset))
        .max_by(|left, right| left.1.total_cmp(&right.1).then_with(|| right.0.cmp(&left.0)))
}
//#endregion 🔖️Arrangement

//#region 🔖️Rooms
struct Shape {
    status: SpaceStatus,
    outline: Vec<Vertex>,
    holes: Vec<Vec<Vertex>>,
    area: f64,
    perimeter: f64,
    region: Option<Region>,
    point: Point2,
}

impl Shape {
    fn refused(status: SpaceStatus, point: Point2) -> Self {
        Self { status, outline: Vec::new(), holes: Vec::new(), area: 0.0, perimeter: 0.0, region: None, point }
    }
}

fn explicit_shape(outline: &[Vertex]) -> Shape {
    let geometry = loops::ccw(&plan_loop(outline));
    let area = loops::area(&geometry);
    let origin = outline.first().map_or(Point2 { x: 0.0, y: 0.0 }, |vertex| vertex.point);
    if outline.len() < 2 || !(area > 1e-12) {
        return Shape::refused(SpaceStatus::InvalidOutline, origin);
    }
    let region = Region::new(&flat(&geometry), &[]);
    let point = interior_point(&region).unwrap_or(origin);
    let authored: Vec<Vertex> = geometry.iter().map(|vertex| Vertex { point: Point2 { x: vertex.point.x, y: vertex.point.y }, bulge: vertex.bulge }).collect();
    Shape { status: SpaceStatus::Explicit, outline: authored, holes: Vec::new(), area, perimeter: loops::perimeter(&geometry), region: Some(region), point }
}

fn bounded_shape(faces: &[Region], extent: &[f64; 4], seed: Point2) -> Shape {
    let probe = [seed.x, seed.y];
    let Some(face) = faces.iter().filter(|face| holds(face, probe)).min_by(|left, right| left.area().total_cmp(&right.area())) else {
        return Shape::refused(SpaceStatus::SeedInsideWall, seed);
    };
    let touches = face.outer.iter().any(|point| (point[0] - extent[0]).abs() < 1e-9 || (point[0] - extent[2]).abs() < 1e-9 || (point[1] - extent[1]).abs() < 1e-9 || (point[1] - extent[3]).abs() < 1e-9);
    if touches {
        return Shape::refused(SpaceStatus::NotEnclosed, seed);
    }
    let perimeter = ring_length(&face.outer) + face.holes.iter().map(|hole| ring_length(hole)).sum::<f64>();
    Shape { status: SpaceStatus::Inferred, outline: corners(&face.outer), holes: face.holes.iter().map(|hole| corners(hole)).collect(), area: face.area(), perimeter, region: Some(face.clone()), point: seed }
}

fn finish(snapshot: &ModelSnapshot, storey: &str, level: &StoreyLevel, shape: Shape, obstacles: &[Obstacle], columns: &[Region]) -> SpaceRoom {
    let control = &mut |_: &BooleanProgress| true;
    let Some(region) = shape.region else { return SpaceRoom::refused(shape.status, shape.point, level.elevation) };
    let columns_area: f64 = if columns.is_empty() { 0.0 } else { run(control, BooleanOperation::Intersection, std::slice::from_ref(&region), columns).iter().map(Region::area).sum() };
    let grown = offset_regions(std::slice::from_ref(&region), CONTACT_GROWTH, OffsetJoin::Miter { limit: 2.0 }, control).unwrap_or_default();
    let reach = bounds_of(grown.iter().flat_map(|part| part.outer.iter()));
    let bounding_walls: Vec<String> = obstacles
        .iter()
        .filter(|obstacle| reach.is_some_and(|[x0, y0, x1, y1]| obstacle.bounds[0] <= x1 && obstacle.bounds[2] >= x0 && obstacle.bounds[1] <= y1 && obstacle.bounds[3] >= y0))
        .filter(|obstacle| run(control, BooleanOperation::Intersection, &grown, &[Region::new(&obstacle.ring, &[])]).iter().map(Region::area).sum::<f64>() > CONTACT_AREA)
        .map(|obstacle| obstacle.id.clone())
        .collect();
    let height = level.top_elevation - level.elevation;
    let ceiling = ceiling_of(snapshot, storey, shape.point);
    let clear_height = ceiling.as_ref().map_or(height, |(_, thickness, offset)| (height + offset - thickness).max(0.0));
    SpaceRoom {
        status: shape.status,
        outline: shape.outline,
        holes: shape.holes,
        point: shape.point,
        area: shape.area,
        perimeter: shape.perimeter,
        net_floor_area: (shape.area - columns_area).max(0.0),
        floor_z: level.elevation,
        clear_height,
        volume: shape.area * clear_height,
        ceiling_slab: ceiling.map(|(id, _, _)| id).unwrap_or_default(),
        bounding_walls,
    }
}

/// 🏠️ The rooms of every space on a storey from the storey's level and the plan obstacles of its walls and curtain walls.
pub fn rooms_from(snapshot: &ModelSnapshot, storey: &str, level: &StoreyLevel, obstacles: &[Obstacle]) -> StoreyRooms {
    let spaces: Vec<(&String, &crate::Space)> = snapshot.spaces.iter().filter(|(_, space)| space.storey == storey).collect();
    let seeds: Vec<Vec2> = spaces.iter().filter_map(|(_, space)| if let SpaceBoundary::Bounded { seed } = &space.boundary { Some([seed.x, seed.y]) } else { None }).collect();
    let extent = bounds_of(obstacles.iter().flat_map(|obstacle| obstacle.ring.iter()).chain(seeds.iter())).map(|[x0, y0, x1, y1]| [x0 - OPEN_MARGIN, y0 - OPEN_MARGIN, x1 + OPEN_MARGIN, y1 + OPEN_MARGIN]);
    let faces = extent.filter(|_| !seeds.is_empty()).map(|[x0, y0, x1, y1]| run(&mut |_| true, BooleanOperation::Difference, &[Region::new(&[[x0, y0], [x1, y0], [x1, y1], [x0, y1]], &[])], &regions_of(obstacles))).unwrap_or_default();
    let columns = column_regions(snapshot, storey);
    spaces
        .into_iter()
        .map(|(id, space)| {
            let shape = match (&space.boundary, extent) {
                (SpaceBoundary::Explicit { outline }, _) => explicit_shape(outline),
                (SpaceBoundary::Bounded { seed }, Some(extent)) => bounded_shape(&faces, &extent, *seed),
                (SpaceBoundary::Bounded { seed }, None) => Shape::refused(SpaceStatus::NotEnclosed, *seed),
            };
            (id.clone(), finish(snapshot, storey, level, shape, obstacles, &columns))
        })
        .collect()
}

/// 🏠️ The rooms of a storey of a model on its own (a probe the caller has not inferred): one run of the model graph restricted to the rooms. `level` is ignored, the graph derives it.
pub fn rooms_of(snapshot: &ModelSnapshot, storey: &str, _level: &StoreyLevel) -> StoreyRooms {
    let mut all = compute_spaces(snapshot);
    snapshot.spaces.iter().filter(|(_, space)| space.storey == storey).filter_map(|(id, _)| all.remove(id).map(|room| (id.clone(), room))).collect()
}
//#endregion 🔖️Rooms

//#region 🔖️Dependency
/// 🔑️ Everything `rooms_from` reads of the snapshot for a storey besides its level and the wall footprints (which are parents): the boundaries of its spaces, its curtain wall bands, its columns with their profiles, and the slabs of the storey above with their thickness.
pub fn dependency(snapshot: &ModelSnapshot, storey: &str) -> DslValue {
    let above = above_of(snapshot, storey);
    let spaces = snapshot.spaces.iter().filter(|(_, space)| space.storey == storey).map(|(id, space)| (id.clone(), dep_value(&space.boundary)));
    let curtains = snapshot.curtain_walls.iter().filter(|(_, curtain)| curtain.storey == storey).map(|(id, curtain)| (id.clone(), dep_object([("axis", dep_value(&curtain.axis)), ("mullion", dep_value(&curtain.mullion))])));
    let columns = snapshot.columns.iter().filter(|(_, column)| column.storey == storey).map(|(id, column)| {
        let profile = snapshot.column_types.get(&column.column_type).map(|kind| kind.profile.clone());
        (id.clone(), dep_object([("position", dep_value(&column.position)), ("rotation", dep_value(&column.rotation)), ("profile", dep_value(&profile))]))
    });
    let slabs = snapshot.slabs.iter().filter(|(_, slab)| above.as_ref() == Some(&slab.storey)).map(|(id, slab)| {
        (id.clone(), dep_object([("boundary", dep_value(&slab.boundary)), ("holes", dep_value(&slab.holes)), ("offset", dep_value(&slab.offset)), ("thickness", dep_value(&slab_thickness(snapshot, &slab.slab_type)))]))
    });
    dep_object([("spaces", DslValue::object(spaces)), ("curtain_walls", DslValue::object(curtains)), ("columns", DslValue::object(columns)), ("above", dep_value(&above)), ("slabs", DslValue::object(slabs))])
}
//#endregion 🔖️Dependency

//#region 🔖️Projection




/// 🏠️ The room of every space (the `Room` nodes of the model graph, flattened).
pub fn compute_spaces(snapshot: &ModelSnapshot) -> BTreeMap<String, SpaceRoom> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::ROOMS }>(snapshot).spaces)
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
