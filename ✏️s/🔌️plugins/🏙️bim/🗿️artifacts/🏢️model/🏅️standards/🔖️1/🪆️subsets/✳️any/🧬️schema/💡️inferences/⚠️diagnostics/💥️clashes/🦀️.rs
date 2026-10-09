//! 💥️ Clashes: pairs of element bodies of one building whose footprints overlap over a shared height.
//!
//! A sweep over the bounding rectangles prefilters the pairs, the heights must overlap, the pair of kinds must be one that can clash (slabs
//! legitimately meet walls and columns, so those pairs are not reported), and then the exact overlap of the flattened footprints
//! decides. Overlaps below [`AREA_EPS`] square metres are ignored: joined walls touch along edges and arcs are flattened within
//! [`CHORD_TOLERANCE`](super::super::super::bodies::CHORD_TOLERANCE). Crossing walls (X joins) are a join and never a clash; a beam that
//! ends inside a column, wall or beam rests on it and does not clash with it.

use super::{Diagnostic, DiagnosticCode, Inputs};
use crate::standards::v1::subsets::any::schema::inferences::bodies::{ceiling_bodies, storey_bodies, Body, BodyKind};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::seg;
use crate::ModelSnapshot;
use semio_framework_2d::booleans::BooleanOperation;
use semio_framework_2d::regions::{region_boolean, Region};
use semio_framework_geometry::bulge::{intersect, Extent};
use semio_framework_geometry::Point;

/// 📏️ Footprint overlaps smaller than this (square metres, 10 cm2) are not clashes.
pub const AREA_EPS: f64 = 1e-3;
/// 📏️ Height overlaps smaller than this (metres) are not clashes.
pub const HEIGHT_EPS: f64 = 1e-6;

/// 🏷️ The code of a pair of kinds, or `None` when the pair may legitimately meet.
pub fn code_of(a: BodyKind, b: BodyKind) -> Option<DiagnosticCode> {
    use BodyKind::*;
    use DiagnosticCode::*;
    let (low, high) = if a <= b { (a, b) } else { (b, a) };
    match (low, high) {
        (Wall, Wall) => Some(ClashWallWall),
        (Wall, Column) => Some(ClashWallColumn),
        (Column, Column) => Some(ClashColumnColumn),
        (Wall, Beam) => Some(ClashWallBeam),
        (Column, Beam) => Some(ClashBeamColumn),
        (Beam, Beam) => Some(ClashBeamBeam),
        (Beam, Slab) => Some(ClashBeamSlab),
        (Wall, Stair) => Some(ClashStairWall),
        (Column, Stair) => Some(ClashStairColumn),
        (Beam, Stair) => Some(ClashStairBeam),
        (Stair, Stair) => Some(ClashStairStair),
        (Slab, Slab) => Some(ClashSlabSlab),
        (Beam, Ceiling) => Some(ClashBeamCeiling),
        (Ceiling, Ceiling) => Some(ClashCeilingCeiling),
        _ => None,
    }
}

fn intersection_area(a: &[Region], b: &[Region]) -> f64 {
    region_boolean(BooleanOperation::Intersection, a, b, &mut |_| true).map_or(0.0, |rows| rows.iter().map(Region::area).sum())
}

fn inside(regions: &[Region], p: Point) -> bool {
    let within = |ring: &[[f64; 2]]| {
        let n = ring.len();
        (0..n).fold(false, |odd, i| {
            let (a, b) = (ring[i], ring[(i + 1) % n]);
            odd ^ ((a[1] > p.y) != (b[1] > p.y) && p.x < a[0] + (p.y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]))
        })
    };
    let near_edge = |ring: &[[f64; 2]]| {
        let n = ring.len();
        (0..n).any(|i| {
            let (a, b) = (ring[i], ring[(i + 1) % n]);
            let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
            let square = dx * dx + dy * dy;
            let t = if square > 0.0 { (((p.x - a[0]) * dx + (p.y - a[1]) * dy) / square).clamp(0.0, 1.0) } else { 0.0 };
            (a[0] + t * dx - p.x).hypot(a[1] + t * dy - p.y) <= 1e-6
        })
    };
    regions.iter().any(|region| (within(&region.outer) || near_edge(&region.outer)) && !region.holes.iter().any(|hole| within(hole) && !near_edge(hole)))
}

fn ends_of(snapshot: &ModelSnapshot, id: &str) -> Option<[Point; 2]> {
    snapshot.beams.get(id).map(|beam| {
        let (crate::Axis::Line { start, end } | crate::Axis::Arc { start, end, .. }) = &beam.axis;
        [Point::new(start.x, start.y), Point::new(end.x, end.y)]
    })
}

fn rests(snapshot: &ModelSnapshot, a: &Body, b: &Body) -> bool {
    let support = |other: &Body| matches!(other.kind, BodyKind::Wall | BodyKind::Column | BodyKind::Beam);
    let ends_in = |beam: &Body, other: &Body| support(other) && ends_of(snapshot, &beam.id).is_some_and(|ends| ends.iter().any(|end| inside(&other.regions, *end)));
    match (a.kind, b.kind) {
        (BodyKind::Beam, BodyKind::Beam) => ends_in(a, b) || ends_in(b, a),
        (BodyKind::Beam, _) => ends_in(a, b),
        (_, BodyKind::Beam) => ends_in(b, a),
        _ => false,
    }
}

fn crossing(snapshot: &ModelSnapshot, a: &Body, b: &Body) -> bool {
    let (Some(first), Some(second)) = (snapshot.walls.get(&a.id), snapshot.walls.get(&b.id)) else { return false };
    let interior = |t: f64| t > 1e-9 && t < 1.0 - 1e-9;
    intersect(&seg(&first.axis), &seg(&second.axis), Extent::Bounded).iter().any(|hit| interior(hit.ta) && interior(hit.tb))
}

/// 💥️ The clashes among `bodies`, one finding per clashing pair.
pub fn among(snapshot: &ModelSnapshot, bodies: &[Body]) -> Vec<Diagnostic> {
    let mut order: Vec<usize> = (0..bodies.len()).collect();
    order.sort_by(|&a, &b| bodies[a].bounds[0].total_cmp(&bodies[b].bounds[0]).then_with(|| bodies[a].id.cmp(&bodies[b].id)));
    let mut found = Vec::new();
    for (rank, &i) in order.iter().enumerate() {
        let a = &bodies[i];
        for &j in &order[rank + 1..] {
            let b = &bodies[j];
            if b.bounds[0] > a.bounds[2] + 1e-9 {
                break;
            }
            let height = a.z_max.min(b.z_max) - a.z_min.max(b.z_min);
            let separate = b.bounds[1] > a.bounds[3] + 1e-9 || a.bounds[1] > b.bounds[3] + 1e-9;
            let Some(code) = code_of(a.kind, b.kind) else { continue };
            if height <= HEIGHT_EPS || separate || rests(snapshot, a, b) || crossing(snapshot, a, b) {
                continue;
            }
            let area = intersection_area(&a.regions, &b.regions);
            if area > AREA_EPS {
                let (first, second) = if a.id <= b.id { (a, b) } else { (b, a) };
                found.push(Diagnostic::new(code, &[&first.id, &second.id]).on(&first.storey).with("overlap_area", area).with("overlap_height", height).with("overlap_volume", area * height));
            }
        }
    }
    found
}

/// 💥️ The clashes among the bodies of every storey of a building.
pub fn building(snapshot: &ModelSnapshot, building: &str, inputs: &Inputs<'_>) -> Vec<Diagnostic> {
    let bodies: Vec<Body> = snapshot
        .storeys
        .iter()
        .filter(|(_, storey)| storey.building == building)
        .flat_map(|(id, _)| storey_bodies(snapshot, id, &inputs.levels, &inputs.layouts, &inputs.runs).into_iter().chain(ceiling_bodies(snapshot, id, &inputs.levels)))
        .collect();
    among(snapshot, &bodies)
}
