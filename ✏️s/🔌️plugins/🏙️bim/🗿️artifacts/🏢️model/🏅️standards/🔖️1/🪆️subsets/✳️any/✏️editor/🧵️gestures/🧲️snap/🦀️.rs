//! 🧲️ Snapping of the authoring tools: the model point a pointer means, drawn from the wall endpoints and axis midpoints, the column positions, the beam, slab and roof corners, the
//! grid lines with their intersections and the orthogonal directions from the last point of a chain. Candidates compete by kind (a corner beats a midpoint beats a grid line ...)
//! inside a pixel tolerance expressed in metres; nothing within reach leaves the pointer free.

use super::plane::{angle, axis_ends, dist, from_point2, polar, pt, P};
use crate::standards::v1::subsets::any::schema::authored::plan::segment_of;
use crate::ModelSnapshot;
use semio_framework_geometry::bulge::{intersect, BulgeSeg, Extent};
use std::f64::consts::FRAC_PI_4;
use value_derive::{FromValue, ToValue};

/// 🧲️ What a snapped point sits on, ranked from the strongest to the weakest.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ToValue, FromValue)]
pub enum SnapKind {
    Endpoint,
    Intersection,
    Midpoint,
    Grid,
    Edge,
    Orthogonal,
    Free,
}

/// 🧲️ The point a pointer snapped to, its kind and the element it came from (empty when free).
#[derive(Clone, Debug, PartialEq)]
pub struct SnapHit {
    pub point: P,
    pub kind: SnapKind,
    pub source: String,
}

/// 🧲️ What a snap may use: the storey whose elements count, the reach in metres, the last point of a chain (the origin of orthogonal directions), whether the direction is locked
/// to the nearest orthogonal one, the elements that must not snap to themselves and extra candidate corners.
#[derive(Clone, Debug, Default)]
pub struct SnapRequest<'a> {
    pub storey: Option<&'a str>,
    pub tolerance: f64,
    pub anchor: Option<P>,
    pub lock_orthogonal: bool,
    pub exclude: &'a [String],
    pub extra: &'a [P],
}

struct Candidate {
    point: P,
    kind: SnapKind,
    source: String,
}

fn candidate(point: P, kind: SnapKind, source: &str) -> Candidate {
    Candidate { point, kind, source: source.to_string() }
}

fn storey_elements(snapshot: &ModelSnapshot, storey: &str) -> Vec<(String, Vec<P>, Option<BulgeSeg>)> {
    let mut rows = Vec::new();
    for (id, wall) in snapshot.walls.iter().filter(|(_, wall)| wall.storey == storey) {
        let (start, end) = axis_ends(&wall.axis);
        rows.push((id.clone(), vec![start, end], Some(segment_of(&wall.axis))));
    }
    for (id, wall) in snapshot.curtain_walls.iter().filter(|(_, wall)| wall.storey == storey) {
        let (start, end) = axis_ends(&wall.axis);
        rows.push((id.clone(), vec![start, end], Some(segment_of(&wall.axis))));
    }
    for (id, column) in snapshot.columns.iter().filter(|(_, column)| column.storey == storey) {
        rows.push((id.clone(), vec![from_point2(column.position)], None));
    }
    for (id, beam) in snapshot.beams.iter().filter(|(_, beam)| beam.storey == storey) {
        let (start, end) = axis_ends(&beam.axis);
        rows.push((id.clone(), vec![start, end], Some(segment_of(&beam.axis))));
    }
    for (id, slab) in snapshot.slabs.iter().filter(|(_, slab)| slab.storey == storey) {
        rows.push((id.clone(), slab.boundary.iter().map(|vertex| from_point2(vertex.point)).collect(), None));
    }
    for (id, roof) in snapshot.roofs.iter().filter(|(_, roof)| roof.storey == storey) {
        rows.push((id.clone(), roof.footprint.iter().map(|vertex| from_point2(vertex.point)).collect(), None));
    }
    for (id, stair) in snapshot.stairs.iter().filter(|(_, stair)| stair.storey == storey) {
        rows.push((id.clone(), vec![from_point2(stair.start)], None));
    }
    rows
}

fn grid_segments(snapshot: &ModelSnapshot, storey: &str) -> Vec<(String, BulgeSeg)> {
    let Some(building) = snapshot.storeys.get(storey).map(|row| row.building.as_str()) else { return Vec::new() };
    snapshot.grids.iter().filter(|(_, grid)| grid.building == building).map(|(id, grid)| (id.clone(), BulgeSeg::line(pt(from_point2(grid.start)), pt(from_point2(grid.end))))).collect()
}

fn candidates(snapshot: &ModelSnapshot, request: &SnapRequest<'_>, raw: P) -> Vec<Candidate> {
    let mut found: Vec<Candidate> = request.extra.iter().map(|point| candidate(*point, SnapKind::Endpoint, "")).collect();
    let Some(storey) = request.storey else { return found };
    let elements: Vec<_> = storey_elements(snapshot, storey).into_iter().filter(|(id, ..)| !request.exclude.contains(id)).collect();
    for (id, corners, segment) in &elements {
        found.extend(corners.iter().map(|corner| candidate(*corner, SnapKind::Endpoint, id)));
        if let Some(segment) = segment {
            let middle = segment.point_at(0.5);
            found.push(candidate([middle.x, middle.y], SnapKind::Midpoint, id));
            let closest = segment.closest(pt(raw));
            found.push(candidate([closest.point.x, closest.point.y], SnapKind::Edge, id));
        }
    }
    let grids = grid_segments(snapshot, storey);
    for (id, segment) in &grids {
        found.extend([segment.start, segment.end].map(|end| candidate([end.x, end.y], SnapKind::Endpoint, id)));
        let closest = segment.closest(pt(raw));
        found.push(candidate([closest.point.x, closest.point.y], SnapKind::Grid, id));
    }
    let carriers: Vec<(&str, BulgeSeg, Extent)> = grids.iter().map(|(id, segment)| (id.as_str(), *segment, Extent::Unbounded)).chain(elements.iter().filter_map(|(id, _, segment)| segment.map(|segment| (id.as_str(), segment, Extent::Bounded)))).collect();
    for (index, (left_id, left, left_extent)) in carriers.iter().enumerate() {
        for (right_id, right, right_extent) in carriers.iter().skip(index + 1) {
            let extent = if *left_extent == Extent::Unbounded || *right_extent == Extent::Unbounded { Extent::Unbounded } else { Extent::Bounded };
            let bounded = |hit: &semio_framework_geometry::bulge::SegHit| (*left_extent == Extent::Unbounded || (-1e-9..=1.0 + 1e-9).contains(&hit.ta)) && (*right_extent == Extent::Unbounded || (-1e-9..=1.0 + 1e-9).contains(&hit.tb));
            for hit in intersect(left, right, extent).into_iter().filter(bounded) {
                found.push(candidate([hit.point.x, hit.point.y], SnapKind::Intersection, &format!("{left_id}+{right_id}")));
            }
        }
    }
    found
}

fn orthogonal(anchor: P, raw: P, tolerance: f64, locked: bool) -> Option<SnapHit> {
    let length = dist(anchor, raw);
    if length < tolerance {
        return None;
    }
    let step = (angle(anchor, raw) / FRAC_PI_4).round() as i64;
    let point = match step.rem_euclid(4) {
        0 => [raw[0], anchor[1]],
        2 => [anchor[0], raw[1]],
        _ => polar(anchor, step as f64 * FRAC_PI_4, (raw[0] - anchor[0]) * (step as f64 * FRAC_PI_4).cos() + (raw[1] - anchor[1]) * (step as f64 * FRAC_PI_4).sin()),
    };
    (locked || dist(point, raw) <= tolerance).then(|| SnapHit { point, kind: SnapKind::Orthogonal, source: String::new() })
}

/// 🧲️ The point `raw` snaps to under `request`: the strongest candidate within reach (nearest among equals), else the orthogonal direction from the anchor, else `raw` itself.
pub fn snap(snapshot: &ModelSnapshot, raw: P, request: &SnapRequest<'_>) -> SnapHit {
    let locked = request.anchor.and_then(|anchor| orthogonal(anchor, raw, request.tolerance, true)).filter(|_| request.lock_orthogonal);
    let reference = locked.as_ref().map_or(raw, |hit| hit.point);
    let best = candidates(snapshot, request, reference)
        .into_iter()
        .filter(|candidate| dist(candidate.point, reference) <= request.tolerance)
        .min_by(|a, b| a.kind.cmp(&b.kind).then(dist(a.point, reference).total_cmp(&dist(b.point, reference))));
    if let Some(best) = best {
        if !locked.is_some() || best.kind <= SnapKind::Intersection {
            return SnapHit { point: best.point, kind: best.kind, source: best.source };
        }
    }
    if let Some(hit) = locked {
        return hit;
    }
    request.anchor.and_then(|anchor| orthogonal(anchor, raw, request.tolerance, false)).unwrap_or(SnapHit { point: raw, kind: SnapKind::Free, source: String::new() })
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
