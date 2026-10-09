//! ✂️ The pure curve geometry of the modify kernel on authored axes and loops: the axis a wall takes when one of its ends is trimmed or
//! extended to the axis of another wall, the parallel axis of an offset wall, and the two loops a straight cut leaves of a closed loop.
//! Points snap to a nanometre and bulges to 1e-12, so every device derives the same numbers from the same payload. All curve work is the
//! framework bulge kernel (`semio-framework-geometry`); this file only decides which of its results is the one a tool means.

use crate::mutations::wall_geometry::snap;
use super::WallEnd;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::segment_of;
use crate::{Axis, Point2, Vertex};
use semio_framework_geometry::bulge::{bulge_from_sweep, intersect, nearest_intersection, BulgeSeg, Extent};
use semio_framework_geometry::loops;
use semio_framework_geometry::vector::{angle_between, cross, LENGTH_EPS};
use semio_framework_geometry::Point;

fn snap_bulge(value: f64) -> f64 {
    (value * 1e12).round() / 1e12 + 0.0
}

fn point2(point: Point) -> Point2 {
    Point2 { x: snap(point.x), y: snap(point.y) }
}

fn axis_from(start: Point, end: Point, bulge: f64) -> Axis {
    if bulge == 0.0 {
        Axis::Line { start: point2(start), end: point2(end) }
    } else {
        Axis::Arc { start: point2(start), end: point2(end), bulge }
    }
}

//#region 🔖️Trim
/// ✂️ Why an end cannot be trimmed or extended to a target axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrimFlaw {
    Parallel,
    Reversed,
    Unchanged,
    Degenerate,
}

impl TrimFlaw {
    /// 💬️ The human message of the flaw.
    pub fn message(&self) -> &'static str {
        match self {
            TrimFlaw::Parallel => "The axis of the wall never meets the axis of the target.",
            TrimFlaw::Reversed => "The new end would lie behind the other end of the wall.",
            TrimFlaw::Unchanged => "The end of the wall already lies on the axis of the target.",
            TrimFlaw::Degenerate => "The wall has no length.",
        }
    }
}

/// ✂️ A trimmed or extended axis and how far its start moved along the old axis: positive towards the old end (a trim), negative away
/// from it (an extension), zero when the end moved. Hosted openings keep their place in the world by moving their offsets by that much.
#[derive(Clone, Debug, PartialEq)]
pub struct Trimmed {
    pub axis: Axis,
    pub shift: f64,
}

/// 📏️ The signed fraction of the length of a segment at which `point` projects onto its carrier: the angle fraction for an arc (measured
/// within half a turn of the start), the line fraction otherwise; below zero before the start, above one beyond the end.
fn fraction(segment: &BulgeSeg, point: Point) -> f64 {
    match segment.center() {
        Some(centre) => angle_between(segment.start - centre, point - centre) * segment.sweep().signum() / segment.sweep().abs(),
        None => segment.param_of(point),
    }
}

/// ✂️ The axis of a wall whose `end` is moved onto the axis of the `target`: the intersection of the two carriers (the line or the full
/// circle of each axis) nearest to the old end becomes the new end, the other end stays, an arc keeps its circle. An end that moves
/// towards the other end is a trim, one that moves away an extension.
pub fn trim_extend(axis: &Axis, end: WallEnd, target: &Axis) -> Result<Trimmed, TrimFlaw> {
    let (own, other) = (segment_of(axis), segment_of(target));
    let length = own.length();
    if length <= LENGTH_EPS {
        return Err(TrimFlaw::Degenerate);
    }
    let moving = if end == WallEnd::Start { own.start } else { own.end };
    let hit = nearest_intersection(&own, &other, Extent::Unbounded, moving).ok_or(TrimFlaw::Parallel)?;
    let hit = Point::new(snap(hit.x), snap(hit.y));
    if (hit - moving).hypot() <= 2.0 * LENGTH_EPS {
        return Err(TrimFlaw::Unchanged);
    }
    let along = fraction(&own, hit);
    let (from, to) = if end == WallEnd::Start { (along, 1.0) } else { (0.0, along) };
    if (to - from) * length <= LENGTH_EPS {
        return Err(TrimFlaw::Reversed);
    }
    let bulge = if own.is_line() { 0.0 } else { snap_bulge(bulge_from_sweep(own.sweep() * (to - from))) };
    let (start, finish) = if end == WallEnd::Start { (hit, own.end) } else { (own.start, hit) };
    Ok(Trimmed { axis: axis_from(start, finish, bulge), shift: if end == WallEnd::Start { snap(along * length) } else { 0.0 } })
}

/// 📏️ The signed fraction of the length of `axis` at which `point` projects onto it: below zero before its start, above one beyond its end.
pub fn locate(axis: &Axis, point: Point2) -> f64 {
    fraction(&segment_of(axis), Point::new(point.x, point.y))
}

/// 🎯️ The signed fraction, along `axis`, of the intersection of its carrier with the carrier of `target` that lies nearest to `near`; none when the two never meet.
pub fn meeting(axis: &Axis, target: &Axis, near: Point2) -> Option<f64> {
    let own = segment_of(axis);
    let hit = nearest_intersection(&own, &segment_of(target), Extent::Unbounded, Point::new(near.x, near.y))?;
    Some(fraction(&own, hit))
}
//#endregion 🔖️Trim

//#region 🔖️Offset
/// ↔️ The axis parallel to `axis` at `distance` metres to the left of its direction (to the right when negative): a shifted line, or a
/// concentric arc of the same sweep. None when an arc would collapse into its centre or the axis has no length.
pub fn offset_axis(axis: &Axis, distance: f64) -> Option<Axis> {
    let shifted = segment_of(axis).offset(distance)?;
    Some(axis_from(shifted.start, shifted.end, shifted.bulge))
}
//#endregion 🔖️Offset

//#region 🔖️Loop
/// ✂️ Why a loop cannot be cut by a line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CutFlaw {
    Line,
    Crossings(usize),
    Degenerate,
    Pieces,
}

impl CutFlaw {
    /// 💬️ The human message of the flaw.
    pub fn message(&self) -> String {
        match self {
            CutFlaw::Line => "A cut line needs two different finite points.".to_string(),
            CutFlaw::Crossings(count) => format!("The cut line must cross the outline exactly twice, it crosses it {count} time(s)."),
            CutFlaw::Degenerate => "The cut line runs along the outline.".to_string(),
            CutFlaw::Pieces => "The cut does not leave two loops that together fill the outline.".to_string(),
        }
    }
}

/// ✂️ The two loops a cut leaves, the one on the left of the cut line (seen from its first point to its second) and the one on the right.
#[derive(Clone, Debug, PartialEq)]
pub struct Cut {
    pub left: Vec<Vertex>,
    pub right: Vec<Vertex>,
}

fn ring(vertices: &[Vertex]) -> Vec<loops::Vertex> {
    vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect()
}

fn carrier(inner: &[loops::Vertex], from: Point, to: Point) -> Option<BulgeSeg> {
    let direction = to - from;
    let length = direction.hypot();
    if !(length > LENGTH_EPS && length.is_finite()) {
        return None;
    }
    let bounds = loops::bounds(inner)?;
    let centre = Point::new((bounds.x0() + bounds.x1()) / 2.0, (bounds.y0() + bounds.y1()) / 2.0);
    let reach = (bounds.width().hypot(bounds.height()) + (from - centre).hypot() + length + 1.0) / length;
    Some(BulgeSeg::line(from - direction * reach, from + direction * reach))
}

type Hit = (usize, f64, Point);

fn hits(segments: &[BulgeSeg], carrier: &BulgeSeg) -> Vec<Hit> {
    let count = segments.len();
    let mut found: Vec<Hit> = Vec::new();
    for (index, segment) in segments.iter().enumerate() {
        for hit in intersect(segment, carrier, Extent::Bounded) {
            let place = if (hit.point - segment.end).hypot() <= 2.0 * LENGTH_EPS {
                ((index + 1) % count, 0.0, segments[(index + 1) % count].start)
            } else if (hit.point - segment.start).hypot() <= 2.0 * LENGTH_EPS {
                (index, 0.0, segment.start)
            } else {
                (index, hit.ta, Point::new(snap(hit.point.x), snap(hit.point.y)))
            };
            if !found.iter().any(|(other, t, point)| *other == place.0 && (t - place.1).abs() <= 1e-9 && (*point - place.2).hypot() <= 2.0 * LENGTH_EPS) {
                found.push(place);
            }
        }
    }
    found.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)));
    found
}

fn part(segment: &BulgeSeg, from: f64, to: f64, start: Point, end: Point) -> Option<BulgeSeg> {
    let start = if from == 0.0 { segment.start } else { start };
    let end = if to == 1.0 { segment.end } else { end };
    ((end - start).hypot() > LENGTH_EPS).then(|| BulgeSeg::new(start, end, if segment.is_line() { 0.0 } else { snap_bulge(bulge_from_sweep(segment.sweep() * (to - from))) }))
}

fn walk(segments: &[BulgeSeg], from: Hit, to: Hit) -> Vec<BulgeSeg> {
    let ((i, t, p), (j, u, q)) = (from, to);
    if i == j && t <= u {
        return part(&segments[i], t, u, p, q).into_iter().collect();
    }
    let mut path: Vec<BulgeSeg> = part(&segments[i], t, 1.0, p, segments[i].end).into_iter().collect();
    let mut index = (i + 1) % segments.len();
    while index != j {
        path.push(segments[index]);
        index = (index + 1) % segments.len();
    }
    path.extend(part(&segments[j], 0.0, u, segments[j].start, q));
    path
}

fn closed(path: &[BulgeSeg], from: Point, to: Point) -> Vec<loops::Vertex> {
    path.iter().copied().chain(std::iter::once(BulgeSeg::line(to, from))).map(|segment| loops::Vertex::new(segment.start, segment.bulge)).collect()
}

fn mark(vertices: Vec<loops::Vertex>) -> Vec<Vertex> {
    vertices.into_iter().map(|vertex| Vertex { point: point2(vertex.point), bulge: vertex.bulge }).collect()
}

/// ✂️ Whether the infinite line through `from` and `to` meets the outline `vertices`.
pub fn crosses(vertices: &[Vertex], from: Point2, to: Point2) -> bool {
    let inner = ring(vertices);
    carrier(&inner, Point::new(from.x, from.y), Point::new(to.x, to.y)).is_some_and(|line| !hits(&loops::segments(&inner), &line).is_empty())
}

/// 🧭️ Whether `point` lies inside the closed loop `vertices`.
pub fn contains(vertices: &[Vertex], point: Point2) -> bool {
    loops::contains(&ring(vertices), Point::new(point.x, point.y))
}

/// ✂️ The two loops the infinite line through `from` and `to` cuts out of the closed counter-clockwise loop `vertices`. The line must
/// cross the outline exactly twice (a vertex it passes through counts once); the pieces are counter-clockwise, split arcs keep their
/// circle, and together they fill exactly the area of the loop.
pub fn cut_loop(vertices: &[Vertex], from: Point2, to: Point2) -> Result<Cut, CutFlaw> {
    let (start, end) = (Point::new(from.x, from.y), Point::new(to.x, to.y));
    let inner = ring(vertices);
    let line = carrier(&inner, start, end).ok_or(CutFlaw::Line)?;
    let segments = loops::segments(&inner);
    let found = hits(&segments, &line);
    let &[a, b] = found.as_slice() else { return Err(CutFlaw::Crossings(found.len())) };
    let (first, second) = (walk(&segments, a, b), walk(&segments, b, a));
    let (one, two) = (closed(&first, a.2, b.2), closed(&second, b.2, a.2));
    if first.is_empty() || second.is_empty() || one.len() < 3 || two.len() < 3 {
        return Err(CutFlaw::Degenerate);
    }
    let direction = end - start;
    let side = cross(direction, first[0].point_at(0.5) - start) / direction.hypot();
    if side.abs() <= LENGTH_EPS {
        return Err(CutFlaw::Degenerate);
    }
    let (areas, whole) = ((loops::signed_area(&one), loops::signed_area(&two)), loops::signed_area(&inner));
    if !(areas.0 > LENGTH_EPS && areas.1 > LENGTH_EPS) || ((areas.0 + areas.1) - whole).abs() > 1e-9 * whole.abs().max(1.0) {
        return Err(CutFlaw::Pieces);
    }
    Ok(if side > 0.0 { Cut { left: mark(one), right: mark(two) } } else { Cut { left: mark(two), right: mark(one) } })
}
//#endregion 🔖️Loop
