//! 🎨️ `finishes`: the finish areas of a room. A space authors only the materials that finish its floor, walls and ceiling; every area is derived here from the
//! resolved room, the same figures the wall and slab solids carry.
//!
//! Definitions. Lengths in metres, areas in square metres.
//! * floor: the net floor area of the room (the outline without its islands and without the columns).
//! * walls: the perimeter of the room (outline plus the boundary of every island) times its clear height, minus the openings. Every **valid** opening frame whose
//!   face lies on the room boundary (the middle of the face of its host on that side is within [`FACE_TOLERANCE`] of the outline or of an island) removes the part of
//!   its rectangle between the floor and the ceiling: exactly the hole the host wall solid cuts out of that face. A wall with a face on both sides of an island counts
//!   both faces, as the perimeter does.
//! * ceiling: the part of the net floor under the hung ceiling of the room (the lowest ceiling of the storey over its point, see `spaces`) over the cosine of that ceiling's slope, plus the rest of the net floor over the cosine of the slope of the slab above that closes the room (its soffit); a room under no hung ceiling is the soffit alone, the net floor area itself when no slab closes it either.
//!
//! The finish of a surface is the material the space names for it; an unfinished surface keeps its area with an empty material.
//!
//! Related: shapely `Polygon.boundary.distance` and `buffer` reproduce the face test, <https://shapely.readthedocs.io/en/stable/manual.html#object.distance>.

use super::super::element_solids::{dep_object, dep_value};
use super::super::opening_frames::OpeningFrame;
use super::super::quantities::{ElementQuantity, ModelQuantities, QuantityKind};
use super::super::spaces::{self, SpaceRoom, SpaceStatus};
use crate::{Ceiling, ModelSnapshot, Slope, Space, Vertex};
use semio_framework_2d::booleans::{BooleanOperation, BooleanProgress};
use semio_framework_2d::regions::{region_boolean, Region};
use semio_framework_2d::Vec2;
use semio_framework_geometry::loops;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

/// 📏️ How far the middle of an opening face may lie from the room boundary to count as on it, in metres (below the sagitta of any flattened arc a wall can have).
pub const FACE_TOLERANCE: f64 = 1e-4;
/// 📏️ Chord tolerance (sagitta, metres) used to flatten the room boundary before the face test.
pub const FLATTEN_TOLERANCE: f64 = 1e-6;

//#region 🔖️Values
/// 🎨️ Which surface of a room a finish covers.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum FinishSurface {
    #[default]
    Floor,
    Wall,
    Ceiling,
}

impl FinishSurface {
    /// 🏷️ The stable key of the surface in the totals tables.
    pub fn key(self) -> &'static str {
        match self {
            Self::Floor => "floor",
            Self::Wall => "wall",
            Self::Ceiling => "ceiling",
        }
    }
}

/// 🎨️ The finish of one surface of a room: the material the space names (empty when unfinished) and the area it covers.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct FinishQuantity {
    pub surface: FinishSurface,
    pub material: String,
    pub area: f64,
}
//#endregion 🔖️Values

//#region 🔖️Geometry
fn rings_of(room: &SpaceRoom) -> Vec<Vec<[f64; 2]>> {
    std::iter::once(&room.outline)
        .chain(room.holes.iter())
        .filter(|ring| ring.len() >= 2)
        .map(|ring| {
            let vertices: Vec<loops::Vertex> = ring.iter().map(|vertex: &Vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect();
            loops::flatten(&vertices, FLATTEN_TOLERANCE).iter().map(|point| [point.x, point.y]).collect()
        })
        .collect()
}

fn distance(point: [f64; 2], from: [f64; 2], to: [f64; 2]) -> f64 {
    let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
    let length = dx * dx + dy * dy;
    let along = if length <= 0.0 { 0.0 } else { (((point[0] - from[0]) * dx + (point[1] - from[1]) * dy) / length).clamp(0.0, 1.0) };
    (point[0] - (from[0] + along * dx)).hypot(point[1] - (from[1] + along * dy))
}

fn on_boundary(rings: &[Vec<[f64; 2]>], point: [f64; 2]) -> bool {
    rings.iter().any(|ring| (0..ring.len()).any(|index| distance(point, ring[index], ring[(index + 1) % ring.len()]) <= FACE_TOLERANCE))
}

/// 🪟️ The number of faces of the opening on the room boundary: the face on each side of its host counts when its middle lies on the boundary.
fn faces_on(rings: &[Vec<[f64; 2]>], frame: &OpeningFrame) -> usize {
    let (origin, side) = (frame.local.origin, frame.local.y_axis);
    [frame.face_front, -frame.face_back].into_iter().filter(|lateral| on_boundary(rings, [origin.x + side.x * lateral, origin.y + side.y * lateral])).count()
}

/// ✂️ The area the valid openings on the boundary of the room cut out of its walls, between the floor and the ceiling.
pub fn opening_area(room: &SpaceRoom, frames: &[&OpeningFrame]) -> f64 {
    let rings = rings_of(room);
    let (floor, ceiling) = (room.floor_z, room.floor_z + room.clear_height);
    frames
        .iter()
        .filter(|frame| frame.valid)
        .map(|frame| {
            let (bottom, top) = (frame.local.origin.z.max(floor), (frame.local.origin.z + frame.height).min(ceiling));
            faces_on(&rings, frame) as f64 * frame.width * (top - bottom).max(0.0)
        })
        .sum()
}

fn tilt_factor(slope: Option<Slope>) -> f64 {
    slope.map_or(1.0, |slope| slope.angle.cos().abs().max(1e-9))
}

fn soffit_factor(snapshot: &ModelSnapshot, room: &SpaceRoom) -> f64 {
    tilt_factor(snapshot.slabs.get(&room.ceiling_slab).and_then(|slab| slab.slope))
}

fn flat_ring(vertices: &[Vertex]) -> Vec<Vec2> {
    let ring: Vec<loops::Vertex> = vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect();
    loops::flatten(&ring, FLATTEN_TOLERANCE).iter().map(|point| [point.x, point.y]).collect()
}

/// 🔲️ The part of the net floor of a room that its hung ceiling covers, in plan: the room (outline without islands) meets the boundary of the ceiling without its holes, minus the columns of the storey.
pub fn covered_area(snapshot: &ModelSnapshot, space: &Space, room: &SpaceRoom, ceiling: &Ceiling) -> f64 {
    let rings = rings_of(room);
    let Some((outer, islands)) = rings.split_first() else { return 0.0 };
    let control = &mut |_: &BooleanProgress| true;
    let (floor, hung) = (Region::new(outer, islands), Region::new(&flat_ring(&ceiling.boundary), &ceiling.holes.iter().map(|hole| flat_ring(hole)).collect::<Vec<_>>()));
    let covered = region_boolean(BooleanOperation::Intersection, &[floor], &[hung], control).unwrap_or_default();
    let columns = spaces::column_regions(snapshot, &space.storey);
    let net = if columns.is_empty() { covered } else { region_boolean(BooleanOperation::Difference, &covered, &columns, control).unwrap_or_default() };
    net.iter().map(Region::area).sum()
}

/// 🔲️ The area of the ceiling surface of a room: the part under its hung ceiling (the lowest one over the room point) over the cosine of the slope of that ceiling, the rest under the soffit of the slab above.
pub fn ceiling_area(snapshot: &ModelSnapshot, space: &Space, room: &SpaceRoom) -> f64 {
    let soffit = soffit_factor(snapshot, room);
    let Some(hung) = snapshot.ceilings.get(&room.ceiling) else { return room.net_floor_area / soffit };
    let covered = covered_area(snapshot, space, room, hung).min(room.net_floor_area);
    covered / tilt_factor(hung.slope) + (room.net_floor_area - covered) / soffit
}
//#endregion 🔖️Geometry

//#region 🔖️Rows
/// 🎨️ The wall area of a room before any finish: perimeter times clear height minus the openings on its boundary.
pub fn wall_area(room: &SpaceRoom, frames: &[&OpeningFrame]) -> f64 {
    (room.perimeter * room.clear_height - opening_area(room, frames)).max(0.0)
}

/// 🎨️ The three finish rows of a resolved room (floor, walls, ceiling), each with the material the space names; none while the room is unresolved.
/// `frames` are the frames of the openings hosted on the walls of the storey of the space.
pub fn finish_rows(snapshot: &ModelSnapshot, space: &Space, room: &SpaceRoom, frames: &[&OpeningFrame]) -> Vec<FinishQuantity> {
    if !matches!(room.status, SpaceStatus::Inferred | SpaceStatus::Explicit) {
        return Vec::new();
    }
    let material = |finish: &Option<String>| finish.clone().unwrap_or_default();
    vec![
        FinishQuantity { surface: FinishSurface::Floor, material: material(&space.floor_finish), area: room.net_floor_area },
        FinishQuantity { surface: FinishSurface::Wall, material: material(&space.wall_finish), area: wall_area(room, frames) },
        FinishQuantity { surface: FinishSurface::Ceiling, material: material(&space.ceiling_finish), area: ceiling_area(snapshot, space, room) },
    ]
}
//#endregion 🔖️Rows

//#region 🔖️Dependency
/// 🔑️ What `finish_rows` reads of the snapshot besides the room and the frames (which are parents): the three finish references of the space, the slope of every slab (the slab that closes the room is only known from the room), and the boundary, holes and slope of every ceiling of the storey with the columns of the storey (the covered part of the floor).
pub fn dependency(snapshot: &ModelSnapshot, space: &Space) -> DslValue {
    let slopes = DslValue::object(snapshot.slabs.iter().filter(|(_, slab)| slab.slope.is_some()).map(|(id, slab)| (id.clone(), dep_value(&slab.slope))));
    let hung = DslValue::object(snapshot.ceilings.iter().filter(|(_, ceiling)| ceiling.storey == space.storey).map(|(id, ceiling)| (id.clone(), dep_object([("boundary", dep_value(&ceiling.boundary)), ("holes", dep_value(&ceiling.holes)), ("slope", dep_value(&ceiling.slope))]))));
    dep_object([("floor", dep_value(&space.floor_finish)), ("wall", dep_value(&space.wall_finish)), ("ceiling", dep_value(&space.ceiling_finish)), ("slopes", slopes), ("ceilings", hung), ("columns", spaces::column_dependency(snapshot, &space.storey))])
}
//#endregion 🔖️Dependency

//#region 🔖️Projection
/// 🧾️ One row of the table the third-party oracle reproduces.
#[derive(value_derive::ToValue)]
struct FinishRow {
    floor_area: f64,
    wall_gross_area: f64,
    wall_opening_area: f64,
    wall_area: f64,
    ceiling_area: f64,
}

fn area_of(quantity: &ElementQuantity, surface: FinishSurface) -> f64 {
    quantity.finishes.iter().filter(|row| row.surface == surface).map(|row| row.area).sum()
}

/// 🧾️ The table the third-party oracle reproduces: floor, wall (gross, openings, net) and ceiling area of every resolved space.
pub fn table_json(quantities: &ModelQuantities) -> String {
    let rows: BTreeMap<String, FinishRow> = quantities
        .elements
        .iter()
        .filter(|(_, quantity)| quantity.kind == QuantityKind::Space && !quantity.finishes.is_empty())
        .map(|(id, quantity)| {
            let (gross, wall) = (quantity.perimeter * quantity.height, area_of(quantity, FinishSurface::Wall));
            (id.clone(), FinishRow { floor_area: area_of(quantity, FinishSurface::Floor), wall_gross_area: gross, wall_opening_area: gross - wall, wall_area: wall, ceiling_area: area_of(quantity, FinishSurface::Ceiling) })
        })
        .collect();
    semio_framework_pack_json::to_json_string(&rows)
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
