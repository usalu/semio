//! 🏠️ `roofs`: the solid of every roof, flat, shed, gable, hip or mansard, from its footprint, pitch, overhang and base offset at the top of its storey.
//!
//! The eave outline is the footprint grown by `overhang`. The underside of the roof at the eave lies at the storey top plus `base_offset`; the layers of the roof type stack
//! upward from the underside surface, last layer lowest, first layer (`layer` 0) the outermost on top, every thickness measured vertically.
//! Flat and shed roofs work on any footprint, curved edges included. Gable, hip and mansard roofs are the pitched surfaces of the weighted straight skeleton of the eave outline
//! (`semio_framework_geometry::roof`), so every simple footprint with straight edges works, concave L, T and U shapes included: every eave edge rises at its pitch, the faces meet in
//! ridges, hips and valleys, and a layer is the closed shell of that surface. The shed `direction` is the direction the roof falls. A gable makes the edges across `ridge_direction` vertical;
//! when such a gable end stands next to a reflex corner the surface would step, so the roof is built as a hip roof of the same pitch and says so ([`RoofFallback::GableEndsAdjustToHip`]).
//! A curved footprint under a pitched shape, a footprint the skeleton cannot be built for or an invalid pitch falls back to a flat roof at the eave height and reports why in
//! [`RoofGeometry::fallback`]; a footprint without area or crossing itself has no roof at all and says so; limits are documented, never silent. A pitch of zero is a flat roof; a pitched roof needs a pitch below a right angle.
//!
//! Related: <https://en.wikipedia.org/wiki/Hip_roof>, <https://en.wikipedia.org/wiki/Straight_skeleton>.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged, direction, layer_thicknesses, stack};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, SolidBuilder, SolidEntry, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{ModelSnapshot, Roof, RoofShape};
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::mesh::{extrude_loops, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::roof::{gable_pitches, roof_surface_controlled, Roof as Pitched, RoofError, RoofSurface, GABLE_END_TOLERANCE, VERTICAL};
pub use semio_framework_geometry::roof::RoofLineKind;
use semio_framework_geometry::skeleton::SkeletonError;
use semio_framework_geometry::vector::{cross, Xyz};
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::f64::consts::FRAC_PI_2;

//#region 🔖️Values
/// 🧭️ Why a roof is not built as its shape asks: flat although the shape asks for more, or a hip roof where a gable roof was asked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoofFallback {
    CurvedFootprint,
    SkeletonFailed,
    DegenerateFootprint,
    InvalidPitch,
    OverhangCollapsed,
    GableEndsAdjustToHip,
}

impl RoofFallback {
    /// 🏷️ The stable diagnostic code.
    pub fn code(self) -> &'static str {
        match self {
            Self::CurvedFootprint => "roof.fallback-flat.curved-footprint",
            Self::SkeletonFailed => "roof.fallback-flat.skeleton",
            Self::DegenerateFootprint => "roof.fallback-flat.degenerate-footprint",
            Self::InvalidPitch => "roof.fallback-flat.invalid-pitch",
            Self::OverhangCollapsed => "roof.overhang-collapsed",
            Self::GableEndsAdjustToHip => "roof.gable-ends-adjust-to-hip",
        }
    }
}

/// 📏️ A line where two roof faces meet (or a verge, a break), on the top surface, in building-local metres.
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
enum Surface {
    Single { outline: Vec<Vertex>, plane: ZPlane },
    Pitched(RoofSurface),
}
//#endregion 🔖️Values

//#region 🔖️Surface
const MITER_LIMIT: f64 = 4.0;
const EPS: f64 = 1e-9;

fn pitch_ok(pitch: f64) -> bool {
    (0.0..FRAC_PI_2 - 1e-6).contains(&pitch)
}

fn straight_ring(outline: &[Vertex]) -> Result<Vec<Point>, RoofFallback> {
    if outline.iter().any(|v| v.bulge.abs() > 1e-12) {
        return Err(RoofFallback::CurvedFootprint);
    }
    let mut ring: Vec<Point> = Vec::new();
    for v in outline {
        if ring.last().is_none_or(|last| last.distance(v.point) > EPS) {
            ring.push(v.point);
        }
    }
    while ring.len() > 1 && ring[0].distance(ring[ring.len() - 1]) <= EPS {
        ring.pop();
    }
    let area = loops::signed_area(&ring.iter().map(|&p| Vertex::new(p, 0.0)).collect::<Vec<_>>());
    if ring.len() < 3 || area.abs() < EPS {
        return Err(RoofFallback::DegenerateFootprint);
    }
    if area < 0.0 {
        ring.reverse();
    }
    Ok(ring)
}

fn reflex(ring: &[Point], vertex: usize) -> bool {
    let n = ring.len();
    cross(ring[vertex] - ring[(vertex + n - 1) % n], ring[(vertex + 1) % n] - ring[vertex]) < -EPS
}

fn gable_ends_next_to_reflex(ring: &[Point], pitch: f64, ridge_direction: f64) -> bool {
    let vertical = &gable_pitches(&[ring.to_vec()], pitch, ridge_direction, GABLE_END_TOLERANCE)[0];
    (0..ring.len()).any(|edge| vertical[edge] >= VERTICAL && (reflex(ring, edge) || reflex(ring, (edge + 1) % ring.len())))
}

fn failure_of(error: &RoofError) -> RoofFallback {
    match error {
        RoofError::InvalidPitch | RoofError::InvalidBreak => RoofFallback::InvalidPitch,
        RoofError::Skeleton(SkeletonError::NoRings | SkeletonError::TooFewVertices { .. } | SkeletonError::NonFinite | SkeletonError::DegenerateEdge { .. } | SkeletonError::Spike { .. } | SkeletonError::SelfIntersecting) => RoofFallback::DegenerateFootprint,
        RoofError::Skeleton(_) => RoofFallback::SkeletonFailed,
    }
}

fn pitched_surface(shape: &RoofShape, eave: &[Vertex], control: &mut dyn FnMut() -> bool, fallback: &mut Option<RoofFallback>) -> Option<Surface> {
    let roof = match shape {
        RoofShape::Hip { pitch } if pitch_ok(*pitch) => Pitched::Hip { pitch: *pitch },
        RoofShape::Gable { pitch, ridge_direction } if pitch_ok(*pitch) => Pitched::Gable { pitch: *pitch, ridge_direction: *ridge_direction },
        RoofShape::Mansard { lower_pitch, upper_pitch, break_height } if pitch_ok(*lower_pitch) && pitch_ok(*upper_pitch) && *upper_pitch > 0.0 && *break_height > 0.0 => Pitched::Mansard { lower_pitch: *lower_pitch, upper_pitch: *upper_pitch, break_height: *break_height },
        _ => {
            *fallback = fallback.or(Some(RoofFallback::InvalidPitch));
            return None;
        }
    };
    let flat = matches!(roof, Pitched::Hip { pitch } | Pitched::Gable { pitch, .. } if pitch == 0.0);
    let ring = match straight_ring(eave) {
        Ok(ring) => ring,
        Err(reason) => {
            *fallback = fallback.or(Some(reason));
            return None;
        }
    };
    if flat {
        return None;
    }
    let roof = match roof {
        Pitched::Gable { pitch, ridge_direction } if gable_ends_next_to_reflex(&ring, pitch, ridge_direction) => {
            *fallback = fallback.or(Some(RoofFallback::GableEndsAdjustToHip));
            Pitched::Hip { pitch }
        }
        other => other,
    };
    match roof_surface_controlled(&[ring], &roof, control) {
        Ok(surface) => Some(Surface::Pitched(surface)),
        Err(error) => {
            *fallback = fallback.or(Some(failure_of(&error)));
            None
        }
    }
}

fn surface_of(shape: &RoofShape, eave: &[Vertex], z0: f64, control: &mut dyn FnMut() -> bool, fallback: &mut Option<RoofFallback>) -> Surface {
    let flat = Surface::Single { outline: eave.to_vec(), plane: ZPlane::flat(z0) };
    match shape {
        RoofShape::Flat => flat,
        RoofShape::Shed { pitch, direction: fall } if pitch_ok(*pitch) => {
            let fall_vector = direction(*fall);
            let anchor = loops::flatten(eave, CHORD_TOLERANCE).into_iter().max_by(|a, b| (a.x * fall_vector.x + a.y * fall_vector.y).total_cmp(&(b.x * fall_vector.x + b.y * fall_vector.y)));
            anchor.map_or(flat.clone(), |anchor| Surface::Single { outline: eave.to_vec(), plane: ZPlane::sloped(anchor, z0, *fall + std::f64::consts::PI, pitch.tan()) })
        }
        RoofShape::Shed { .. } => {
            *fallback = fallback.or(Some(RoofFallback::InvalidPitch));
            flat
        }
        pitched => pitched_surface(pitched, eave, control, fallback).unwrap_or(flat),
    }
}

fn lines_of(surface: &RoofSurface, lift: f64) -> Vec<RoofLine> {
    surface.lines.iter().map(|line| RoofLine { kind: line.kind, start: [line.from[0], line.from[1], line.from[2] + lift], end: [line.to[0], line.to[1], line.to[2] + lift] }).collect()
}
//#endregion 🔖️Surface

//#region 🔖️Geometry
fn grown(footprint: &[Vertex], overhang: f64) -> (Vec<Vertex>, Option<RoofFallback>) {
    if overhang.abs() < 1e-12 {
        return (footprint.to_vec(), None);
    }
    loops::offset(footprint, overhang, MITER_LIMIT).map_or_else(|| (footprint.to_vec(), Some(RoofFallback::OverhangCollapsed)), |outline| (outline, None))
}

/// 🏠️ The geometry of a roof from the level of its storey; layers are measured by `thicknesses` (first layer on top). `control` is polled once per skeleton event and returns `false` to cancel: a cancelled roof is flat.
pub fn roof_geometry_controlled(roof: &Roof, thicknesses: &[f64], own: &StoreyLevel, control: &mut dyn FnMut() -> bool) -> RoofGeometry {
    let z0 = own.top_elevation + roof.base_offset;
    let total: f64 = thicknesses.iter().sum();
    let (eave, mut fallback) = grown(&bulged(&roof.footprint), roof.overhang);
    let absent = |eave: Vec<Vertex>| RoofGeometry { eave_z: z0, ridge_z: z0 + total, eave, lines: Vec::new(), fallback: Some(RoofFallback::DegenerateFootprint), layers: thicknesses.iter().map(|_| TriMesh::new()).collect() };
    if eave.len() < 3 || loops::area(&eave) < EPS {
        return absent(eave);
    }
    let surface = surface_of(&roof.shape, &eave, z0, control, &mut fallback);
    if fallback == Some(RoofFallback::DegenerateFootprint) {
        return absent(eave);
    }
    let layers = stack(thicknesses)
        .iter()
        .zip(thicknesses)
        .map(|(&(from, to), &thickness)| {
            if thickness <= 1e-12 {
                return TriMesh::new();
            }
            match &surface {
                Surface::Single { outline, plane } => extrude_loops(outline, &[], CHORD_TOLERANCE, plane.raised(total - to), plane.raised(total - from)),
                Surface::Pitched(pitched) => pitched.shell(thickness).translated([0.0, 0.0, z0 + total - from]),
            }
        })
        .collect();
    let (ridge_z, lines) = match &surface {
        Surface::Single { outline, plane } => (loops::flatten(outline, CHORD_TOLERANCE).iter().map(|&p| plane.at(p)).fold(f64::NEG_INFINITY, f64::max) + total, Vec::new()),
        Surface::Pitched(pitched) => (z0 + total + pitched.height, lines_of(pitched, z0 + total)),
    };
    RoofGeometry { eave_z: z0, ridge_z, eave, lines, fallback, layers }
}

/// 🏠️ The geometry of a roof from the level of its storey, run to the end.
pub fn roof_geometry(roof: &Roof, thicknesses: &[f64], own: &StoreyLevel) -> RoofGeometry {
    roof_geometry_controlled(roof, thicknesses, own, &mut || true)
}
fn plane_of(vertices: &[[f64; 3]], lift: f64) -> Option<ZPlane> {
    let mut best: Option<(f64, ZPlane)> = None;
    for a in 0..vertices.len() {
        for b in a + 1..vertices.len() {
            for c in b + 1..vertices.len() {
                let (p, q, r) = (vertices[a], vertices[b], vertices[c]);
                let (u, v) = ([q[0] - p[0], q[1] - p[1], q[2] - p[2]], [r[0] - p[0], r[1] - p[1], r[2] - p[2]]);
                let normal = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
                if normal[2].abs() > best.as_ref().map_or(1e-12, |(strength, _)| *strength) {
                    let (slope_x, slope_y) = (-normal[0] / normal[2], -normal[1] / normal[2]);
                    best = Some((normal[2].abs(), ZPlane { a: slope_x, b: slope_y, c: p[2] + lift - slope_x * p[0] - slope_y * p[1] }));
                }
            }
        }
    }
    best.map(|(_, plane)| plane)
}

/// 🔗️ The planar pieces of the underside of a roof (a plan polygon and the plane of the underside above it: one piece for a flat or shed roof, one per sloped face of a pitched roof, gable ends left out) and the height of the underside at the eave.
/// A roof whose footprint has no area has no piece. A wall attached to the roof follows these planes.
pub fn underside_pieces(roof: &Roof, own: &StoreyLevel) -> (Vec<(Vec<Point>, ZPlane)>, f64) {
    let z0 = own.top_elevation + roof.base_offset;
    let (eave, mut fallback) = grown(&bulged(&roof.footprint), roof.overhang);
    if eave.len() < 3 || loops::area(&eave) < EPS {
        return (Vec::new(), z0);
    }
    let surface = surface_of(&roof.shape, &eave, z0, &mut || true, &mut fallback);
    if fallback == Some(RoofFallback::DegenerateFootprint) {
        return (Vec::new(), z0);
    }
    let pieces = match surface {
        Surface::Single { outline, plane } => vec![(loops::flatten(&outline, CHORD_TOLERANCE), plane)],
        Surface::Pitched(pitched) => pitched.faces.iter().filter(|face| !face.vertical).filter_map(|face| plane_of(&face.vertices, z0).map(|plane| (face.vertices.iter().map(|v| Point::new(v[0], v[1])).collect(), plane))).collect(),
    };
    (pieces, z0)
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🏠️ The solid of a roof from the level of its storey, with the reason when it did not become what its shape asks; absent without a type.
pub fn roof_solid_controlled(snapshot: &ModelSnapshot, roof: &Roof, own: &StoreyLevel, control: &mut dyn FnMut() -> bool) -> SolidEntry {
    let mut builder = SolidBuilder::new(SolidFamily::Roof);
    let mut fallback = None;
    if let Some(kind) = snapshot.roof_types.get(&roof.roof_type) {
        let geometry = roof_geometry_controlled(roof, &layer_thicknesses(&kind.layers), own, control);
        for (index, (layer, mesh)) in kind.layers.iter().zip(&geometry.layers).enumerate() {
            builder.add(parts::LAYER, &layer.material, index as u32, mesh);
        }
        fallback = geometry.fallback;
    }
    SolidEntry { solid: builder.build(), fallback }
}

/// 🏠️ The solid of a roof from the level of its storey, run to the end.
pub fn roof_solid(snapshot: &ModelSnapshot, roof: &Roof, own: &StoreyLevel) -> SolidEntry {
    roof_solid_controlled(snapshot, roof, own, &mut || true)
}

/// 🔑️ What `roof_solid` reads besides the level: the roof record and its type.
pub fn dependency(snapshot: &ModelSnapshot, roof: &Roof) -> DslValue {
    dep_object([("roof", dep_value(roof)), ("type", dep_value(&snapshot.roof_types.get(&roof.roof_type).cloned()))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
