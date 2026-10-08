//! 🏠️ `roofs`: the solid of every roof, flat, shed, gable, hip or mansard, from its footprint, pitch, overhang and base offset at the top of its storey.
//!
//! The eave outline is the footprint grown by `overhang`. The underside of the roof at the eave lies at the storey top plus `base_offset`; the layers of the roof type stack
//! upward from the underside surface, last layer lowest, first layer (`layer` 0) the outermost on top, every thickness measured vertically.
//! Flat and shed roofs work on any footprint, curved edges included. Gable, hip and mansard roofs are the lower envelope of planes over a convex footprint with straight
//! edges: hip planes rise from every eave edge at the pitch, a gable rises from the two extremes perpendicular to `ridge_direction`, a mansard rises at the lower pitch up
//! to the break height and at the upper pitch above it. The shed `direction` is the direction the roof falls. For any other footprint (curved, concave, degenerate) or an
//! invalid pitch the roof falls back to a flat roof at the eave height and reports why in [`RoofGeometry::fallback`]; limits are documented, never silent.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged, direction, layer_thicknesses, stack};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidEntry, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{ModelSnapshot, Roof, RoofShape};
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::mesh::{extrude_loops, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::triangulation::triangulate;
use semio_framework_geometry::vector::{cross, perp, unit, Xyz};
use semio_framework_geometry::{Point, Vec2};
use semio_framework_value::DslValue;
use std::f64::consts::FRAC_PI_2;

//#region 🔖️Values
/// 🧭️ Why a roof is flat although its shape asks for more.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoofFallback {
    CurvedFootprint,
    NonConvexFootprint,
    DegenerateFootprint,
    InvalidPitch,
    OverhangCollapsed,
}

impl RoofFallback {
    /// 🏷️ The stable diagnostic code.
    pub fn code(self) -> &'static str {
        match self {
            Self::CurvedFootprint => "roof.fallback-flat.curved-footprint",
            Self::NonConvexFootprint => "roof.fallback-flat.non-convex-footprint",
            Self::DegenerateFootprint => "roof.fallback-flat.degenerate-footprint",
            Self::InvalidPitch => "roof.fallback-flat.invalid-pitch",
            Self::OverhangCollapsed => "roof.overhang-collapsed",
        }
    }
}

/// 📏️ What a roof line is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoofLineKind {
    Ridge,
    Hip,
    Break,
}

/// 📏️ A line where two roof faces meet, on the top surface, in building-local metres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofLine {
    pub kind: RoofLineKind,
    pub start: Xyz,
    pub end: Xyz,
}

/// 🏠️ The resolved heights, the plan outputs and the layer meshes of one roof.
#[derive(Clone, Debug, PartialEq)]
pub struct RoofGeometry {
    pub eave_z: f64,
    pub ridge_z: f64,
    pub eave: Vec<Vertex>,
    pub lines: Vec<RoofLine>,
    pub fallback: Option<RoofFallback>,
    pub layers: Vec<TriMesh>,
}

#[derive(Clone, Debug)]
struct Face {
    ring: Vec<Point>,
    boundary: Vec<bool>,
    plane: ZPlane,
}

#[derive(Clone, Debug)]
enum Surface {
    Single { outline: Vec<Vertex>, plane: ZPlane },
    Faces(Vec<Face>),
}
//#endregion 🔖️Values

//#region 🔖️Planes
const MITER_LIMIT: f64 = 4.0;
const EPS: f64 = 1e-10;

fn pitch_ok(pitch: f64) -> bool {
    pitch >= 0.0 && pitch < FRAC_PI_2 - 1e-6
}

fn plane_rising(normal: Vec2, anchor: Point, base: f64, slope: f64) -> ZPlane {
    ZPlane { a: slope * normal.x, b: slope * normal.y, c: base - slope * (normal.x * anchor.x + normal.y * anchor.y) }
}

fn same_plane(a: &ZPlane, b: &ZPlane) -> bool {
    (a.a - b.a).abs() < 1e-12 && (a.b - b.b).abs() < 1e-12 && (a.c - b.c).abs() < 1e-12
}

fn convex_ring(outline: &[Vertex]) -> Result<Vec<Point>, RoofFallback> {
    if outline.iter().any(|v| v.bulge.abs() > 1e-12) {
        return Err(RoofFallback::CurvedFootprint);
    }
    let mut ring: Vec<Point> = Vec::new();
    for v in outline {
        if ring.last().map_or(true, |last| last.distance(v.point) > 1e-9) {
            ring.push(v.point);
        }
    }
    while ring.len() > 1 && ring[0].distance(ring[ring.len() - 1]) <= 1e-9 {
        ring.pop();
    }
    if ring.len() < 3 {
        return Err(RoofFallback::DegenerateFootprint);
    }
    let area = loops::signed_area(&ring.iter().map(|&p| Vertex::new(p, 0.0)).collect::<Vec<_>>());
    if area.abs() < 1e-9 {
        return Err(RoofFallback::DegenerateFootprint);
    }
    if area < 0.0 {
        ring.reverse();
    }
    let n = ring.len();
    let convex = (0..n).all(|i| cross(ring[(i + 1) % n] - ring[i], ring[(i + 2) % n] - ring[(i + 1) % n]) >= -1e-9);
    if convex {
        Ok(ring)
    } else {
        Err(RoofFallback::NonConvexFootprint)
    }
}

fn edge_planes(ring: &[Point], z0: f64, rises: &[(f64, f64)]) -> Vec<ZPlane> {
    let n = ring.len();
    (0..n)
        .filter_map(|i| unit(perp(ring[(i + 1) % n] - ring[i]), 1e-12).map(|normal| (ring[i], normal)))
        .flat_map(|(anchor, normal)| rises.iter().map(move |&(offset, slope)| plane_rising(normal, anchor, z0 + offset, slope)))
        .collect()
}

fn gable_planes(ring: &[Point], z0: f64, pitch: f64, ridge_direction: f64) -> Vec<ZPlane> {
    let normal = perp(direction(ridge_direction));
    let projections: Vec<f64> = ring.iter().map(|p| normal.x * p.x + normal.y * p.y).collect();
    let (low, high) = (projections.iter().copied().fold(f64::INFINITY, f64::min), projections.iter().copied().fold(f64::NEG_INFINITY, f64::max));
    let slope = pitch.tan();
    vec![ZPlane { a: slope * normal.x, b: slope * normal.y, c: z0 - slope * low }, ZPlane { a: -slope * normal.x, b: -slope * normal.y, c: z0 + slope * high }]
}
//#endregion 🔖️Planes

//#region 🔖️Faces
type Edged = Vec<(Point, bool)>;

fn clip(polygon: &Edged, value: impl Fn(Point) -> f64) -> Edged {
    let mut out: Edged = Vec::new();
    let n = polygon.len();
    for i in 0..n {
        let ((current, boundary), (next, _)) = (polygon[i], polygon[(i + 1) % n]);
        let (fc, fnext) = (value(current), value(next));
        let (inside, next_inside) = (fc <= EPS, fnext <= EPS);
        let crossing = || current + (next - current) * (fc / (fc - fnext));
        match (inside, next_inside) {
            (true, true) => out.push((current, boundary)),
            (true, false) => {
                out.push((current, boundary));
                out.push((crossing(), false));
            }
            (false, true) => out.push((crossing(), boundary)),
            (false, false) => {}
        }
    }
    let mut cleaned: Edged = Vec::new();
    for (point, flag) in out {
        if let Some(last) = cleaned.last_mut() {
            if last.0.distance(point) <= 1e-10 {
                *last = (point, flag);
                continue;
            }
        }
        cleaned.push((point, flag));
    }
    while cleaned.len() > 1 && cleaned[0].0.distance(cleaned[cleaned.len() - 1].0) <= 1e-10 {
        cleaned.pop();
    }
    cleaned
}

fn ring_area(ring: &[(Point, bool)]) -> f64 {
    let n = ring.len();
    (0..n).map(|i| cross(ring[i].0 - Point::ZERO, ring[(i + 1) % n].0 - Point::ZERO)).sum::<f64>() / 2.0
}

fn envelope_faces(ring: &[Point], planes: Vec<ZPlane>) -> Vec<Face> {
    let mut unique: Vec<ZPlane> = Vec::new();
    for plane in planes {
        if !unique.iter().any(|known| same_plane(known, &plane)) {
            unique.push(plane);
        }
    }
    let start: Edged = ring.iter().map(|&p| (p, true)).collect();
    unique
        .iter()
        .enumerate()
        .filter_map(|(k, plane)| {
            let mut polygon = start.clone();
            for other in unique.iter().enumerate().filter(|(j, _)| *j != k).map(|(_, other)| other) {
                polygon = clip(&polygon, |p| (plane.a - other.a) * p.x + (plane.b - other.b) * p.y + (plane.c - other.c));
                if polygon.len() < 3 {
                    return None;
                }
            }
            (polygon.len() >= 3 && ring_area(&polygon) > 1e-9).then(|| Face { ring: polygon.iter().map(|row| row.0).collect(), boundary: polygon.iter().map(|row| row.1).collect(), plane: *plane })
        })
        .collect()
}

fn faces_mesh(faces: &[Face], lift: f64, thickness: f64) -> TriMesh {
    let mut mesh = TriMesh::new();
    for face in faces {
        let (low, high) = (face.plane.raised(lift), face.plane.raised(lift + thickness));
        let at = |plane: &ZPlane, p: Point| [p.x, p.y, plane.at(p)];
        let triangulation = triangulate(&face.ring, &[]);
        for t in &triangulation.triangles {
            let [a, b, c] = t.map(|i| triangulation.vertices[i as usize]);
            mesh.push_triangle(at(&low, a), at(&low, c), at(&low, b));
            mesh.push_triangle(at(&high, a), at(&high, b), at(&high, c));
        }
        let n = face.ring.len();
        for i in (0..n).filter(|&i| face.boundary[i]) {
            let (a, b) = (face.ring[i], face.ring[(i + 1) % n]);
            mesh.push_quad(at(&low, a), at(&low, b), at(&high, b), at(&high, a));
        }
    }
    mesh
}

fn lines_of(faces: &[Face], raise: f64) -> Vec<RoofLine> {
    let key = |p: Point| ((p.x / 1e-9).round() as i64, (p.y / 1e-9).round() as i64);
    let top = faces.iter().flat_map(|face| face.ring.iter().map(|&p| face.plane.at(p))).fold(f64::NEG_INFINITY, f64::max);
    faces
        .iter()
        .flat_map(|face| {
            let n = face.ring.len();
            (0..n).filter(|&i| !face.boundary[i]).map(move |i| (face, face.ring[i], face.ring[(i + 1) % n]))
        })
        .filter(|(_, a, b)| key(*a) < key(*b))
        .map(|(face, a, b)| {
            let (start, end) = ([a.x, a.y, face.plane.at(a) + raise], [b.x, b.y, face.plane.at(b) + raise]);
            let level = (start[2] - end[2]).abs() < 1e-9;
            let kind = if !level { RoofLineKind::Hip } else if (face.plane.at(a) - top).abs() < 1e-9 { RoofLineKind::Ridge } else { RoofLineKind::Break };
            RoofLine { kind, start, end }
        })
        .collect()
}
//#endregion 🔖️Faces

//#region 🔖️Geometry
fn grown(footprint: &[Vertex], overhang: f64) -> (Vec<Vertex>, Option<RoofFallback>) {
    if overhang.abs() < 1e-12 {
        return (footprint.to_vec(), None);
    }
    loops::offset(footprint, overhang, MITER_LIMIT).map_or_else(|| (footprint.to_vec(), Some(RoofFallback::OverhangCollapsed)), |outline| (outline, None))
}

fn surface_of(shape: &RoofShape, eave: &[Vertex], z0: f64, fallback: &mut Option<RoofFallback>) -> Surface {
    let flat = Surface::Single { outline: eave.to_vec(), plane: ZPlane::flat(z0) };
    let faces = |ring: Result<Vec<Point>, RoofFallback>, build: &dyn Fn(&[Point]) -> Vec<ZPlane>, fallback: &mut Option<RoofFallback>| match ring {
        Ok(ring) => Some(Surface::Faces(envelope_faces(&ring, build(&ring)))),
        Err(reason) => {
            *fallback = fallback.or(Some(reason));
            None
        }
    };
    let invalid = |fallback: &mut Option<RoofFallback>| {
        *fallback = fallback.or(Some(RoofFallback::InvalidPitch));
    };
    match shape {
        RoofShape::Flat => flat,
        RoofShape::Shed { pitch, direction: fall } if pitch_ok(*pitch) => {
            let fall_vector = direction(*fall);
            let anchor = loops::flatten(eave, CHORD_TOLERANCE).into_iter().max_by(|a, b| (a.x * fall_vector.x + a.y * fall_vector.y).total_cmp(&(b.x * fall_vector.x + b.y * fall_vector.y)));
            anchor.map_or(flat.clone(), |anchor| Surface::Single { outline: eave.to_vec(), plane: ZPlane::sloped(anchor, z0, *fall + std::f64::consts::PI, pitch.tan()) })
        }
        RoofShape::Gable { pitch, ridge_direction } if pitch_ok(*pitch) => faces(convex_ring(eave), &|ring| gable_planes(ring, z0, *pitch, *ridge_direction), fallback).unwrap_or(flat),
        RoofShape::Hip { pitch } if pitch_ok(*pitch) => faces(convex_ring(eave), &|ring| edge_planes(ring, z0, &[(0.0, pitch.tan())]), fallback).unwrap_or(flat),
        RoofShape::Mansard { lower_pitch, upper_pitch, break_height } if pitch_ok(*lower_pitch) && *lower_pitch > 0.0 && pitch_ok(*upper_pitch) && *break_height >= 0.0 => {
            let (lower, upper) = (lower_pitch.tan(), upper_pitch.tan());
            faces(convex_ring(eave), &|ring| edge_planes(ring, z0, &[(0.0, lower), (break_height * (1.0 - upper / lower), upper)]), fallback).unwrap_or(flat)
        }
        _ => {
            invalid(fallback);
            flat
        }
    }
}

/// 🏠️ The geometry of a roof from the level of its storey; layers are measured by `thicknesses` (first layer on top).
pub fn roof_geometry(roof: &Roof, thicknesses: &[f64], own: &StoreyLevel) -> RoofGeometry {
    let z0 = own.top_elevation + roof.base_offset;
    let total: f64 = thicknesses.iter().sum();
    let (eave, mut fallback) = grown(&bulged(&roof.footprint), roof.overhang);
    if eave.len() < 3 || loops::area(&eave) < 1e-9 {
        return RoofGeometry { eave_z: z0, ridge_z: z0 + total, eave, lines: Vec::new(), fallback: Some(RoofFallback::DegenerateFootprint), layers: thicknesses.iter().map(|_| TriMesh::new()).collect() };
    }
    let surface = surface_of(&roof.shape, &eave, z0, &mut fallback);
    let bands = stack(thicknesses);
    let layers = bands
        .iter()
        .zip(thicknesses)
        .map(|(&(from, to), &thickness)| {
            if thickness <= 1e-12 {
                return TriMesh::new();
            }
            let lift = total - to;
            match &surface {
                Surface::Single { outline, plane } => extrude_loops(outline, &[], CHORD_TOLERANCE, plane.raised(lift), plane.raised(total - from)),
                Surface::Faces(faces) => faces_mesh(faces, lift, thickness),
            }
        })
        .collect();
    let (ridge_z, lines) = match &surface {
        Surface::Single { outline, plane } => (loops::flatten(outline, CHORD_TOLERANCE).iter().map(|&p| plane.at(p)).fold(f64::NEG_INFINITY, f64::max) + total, Vec::new()),
        Surface::Faces(faces) => (faces.iter().flat_map(|face| face.ring.iter().map(|&p| face.plane.at(p))).fold(f64::NEG_INFINITY, f64::max) + total, lines_of(faces, total)),
    };
    RoofGeometry { eave_z: z0, ridge_z, eave, lines, fallback, layers }
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🏠️ The solid of a roof from the level of its storey, with the reason when it fell back to a flat roof; absent without a type.
pub fn roof_solid(snapshot: &ModelSnapshot, roof: &Roof, own: &StoreyLevel) -> SolidEntry {
    let mut builder = SolidBuilder::new(SolidFamily::Roof);
    let mut fallback = None;
    if let Some(kind) = snapshot.roof_types.get(&roof.roof_type) {
        let geometry = roof_geometry(roof, &layer_thicknesses(&kind.layers), own);
        for (index, (layer, mesh)) in kind.layers.iter().zip(&geometry.layers).enumerate() {
            builder.add(parts::LAYER, &layer.material, index as u32, mesh);
        }
        fallback = geometry.fallback;
    }
    SolidEntry { solid: builder.build(), fallback }
}

/// 🔑️ What `roof_solid` reads besides the level: the roof record and its type.
pub fn dependency(snapshot: &ModelSnapshot, roof: &Roof) -> DslValue {
    dep_object([("roof", dep_value(roof)), ("type", dep_value(&snapshot.roof_types.get(&roof.roof_type).cloned()))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
