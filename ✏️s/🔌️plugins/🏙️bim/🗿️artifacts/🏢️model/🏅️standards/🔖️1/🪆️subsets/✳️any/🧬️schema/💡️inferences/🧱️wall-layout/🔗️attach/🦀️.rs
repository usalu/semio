//! 🔗️ Wall attach: the top of a wall attached to the underside of a roof, a slab or a ceiling, and its base attached to the top of a slab. An attached wall stores only the reference (`TopConstraint::Roof`,
//! `Slab`, `Ceiling` plus an offset, and `Wall::base_slab`); the height of the wall along its axis is inferred here. The target is resolved once into an [`AttachSurface`] (the `Surface` node of the model graph:
//! planar pieces over plan polygons, the underside of the element and its thickness), the axis of the wall is cut by the edges of those pieces, and the heights at the cuts give the
//! elevation edges of the wall (`top_profile`, `base_profile`: piecewise linear in the arc length `s` along the axis). Where the target does not cover the axis the nearest covered height is held; a target
//! that is missing or covers no point of the axis leaves the wall at its reference height (the storey top for a missing target, the eave or reference underside of the surface otherwise).
//!
//! Conventions. The profile is authoritative over the full-thickness extent of the wall (`FaceEnds::full_thickness`, the part between the mitered ends) and is continued with the slope of its end segment over the mitered
//! ends, so the solid of every layer, the cut of the openings and the quantities all read the same function of `s`. A top that would fall below the base is lifted to the base ([`MIN_HEIGHT`]) and reported as clamped.
//! The attach references of walls point at leaf elements (roofs, slabs and ceilings have no attach of their own), so the chain of references cannot loop; [`find_cycle`] guards the graph all the same.

use super::{face_ends, WallLayout};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged, layer_thicknesses, seg};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ceilings, dep_object, dep_value, roofs, slabs, Anonymous, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{Axis, ModelSnapshot, TopConstraint, Wall};
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::vector::{cross, lerp, perp};
use semio_framework_geometry::{Point, Vec2};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

//#region 🔖️Values
/// 📏️ Smallest height an attached wall keeps where its target would push the top below its base, in metres.
pub const MIN_HEIGHT: f64 = 1e-6;

const EPS: f64 = 1e-9;

/// 📈️ One breakpoint of an elevation edge of a wall: the arc length `s` along the axis and the absolute height `z` in building coordinates.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ElevationPoint {
    pub s: f64,
    pub z: f64,
}

/// 🔗️ How one attach of a wall resolved: the target id, whether it exists, the fraction `0..=1` of the axis its surface covers, whether the top was lifted to the base and whether the attach chain loops.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct AttachState {
    pub target: String,
    pub found: bool,
    pub covered: f64,
    pub clamped: bool,
    pub cycle: bool,
}

/// ▭️ One planar piece of an attach surface: a plan polygon with holes and the plane of the underside above it.
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub outline: Vec<Point>,
    pub holes: Vec<Vec<Point>>,
    pub under: ZPlane,
}

/// 🧱️ The resolved surface of a roof, slab or ceiling (the `Surface` node of the model graph): the planar pieces of its underside, its vertical thickness (the top surface lies `thickness` above the underside) and `reference`,
/// the underside height used where the surface covers no point of the wall.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AttachSurface {
    pub found: bool,
    pub pieces: Vec<Piece>,
    pub thickness: f64,
    pub reference: f64,
}
//#endregion 🔖️Values

//#region 🔖️References
/// 🧭️ The roof, slab or ceiling a top constraint is attached to and its offset above the underside of that element.
pub fn top_target(top: &TopConstraint) -> Option<(&str, f64)> {
    match top {
        TopConstraint::Roof { roof, offset } => Some((roof, *offset)),
        TopConstraint::Slab { slab, offset } => Some((slab, *offset)),
        TopConstraint::Ceiling { ceiling, offset } => Some((ceiling, *offset)),
        _ => None,
    }
}

/// 🔗️ The ids a wall is attached to, the top target first, each once.
pub fn targets_of(wall: &Wall) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    if let Some((id, _)) = top_target(&wall.top) {
        out.push(id);
    }
    if let Some(slab) = wall.base_slab.as_deref() {
        if !out.contains(&slab) {
            out.push(slab);
        }
    }
    out
}

/// 🪜️ The storey of the roof, slab or ceiling `id`, `None` when no such element exists.
pub fn storey_of<'a>(snapshot: &'a ModelSnapshot, id: &str) -> Option<&'a String> {
    snapshot.slabs.get(id).map(|row| &row.storey).or_else(|| snapshot.ceilings.get(id).map(|row| &row.storey)).or_else(|| snapshot.roofs.get(id).map(|row| &row.storey))
}

/// 🔁️ The first loop of the reference graph reachable from `start`, the ids on the loop in visiting order; `edges` names the ids a node points at.
pub fn find_cycle(start: &str, edges: &dyn Fn(&str) -> Vec<String>) -> Option<Vec<String>> {
    fn walk(node: &str, edges: &dyn Fn(&str) -> Vec<String>, path: &mut Vec<String>) -> Option<Vec<String>> {
        if let Some(at) = path.iter().position(|seen| seen == node) {
            return Some(path[at..].to_vec());
        }
        path.push(node.to_string());
        for next in edges(node) {
            if let Some(found) = walk(&next, edges, path) {
                return Some(found);
            }
        }
        path.pop();
        None
    }
    walk(start, edges, &mut Vec::new())
}

/// 🔁️ The attach edges of the authored snapshot: a wall points at its targets; a roof, slab or ceiling points at nothing.
pub fn edges(snapshot: &ModelSnapshot) -> impl Fn(&str) -> Vec<String> + '_ {
    move |id| snapshot.walls.get(id).map(|wall| targets_of(wall).into_iter().map(str::to_string).collect()).unwrap_or_default()
}
//#endregion 🔖️References

//#region 🔖️Surface
fn piece(outline: &[Vertex], holes: &[Vec<Vertex>], under: ZPlane) -> Option<Piece> {
    (outline.len() >= 3).then(|| Piece { outline: loops::flatten(outline, CHORD_TOLERANCE), holes: holes.iter().map(|hole| loops::flatten(hole, CHORD_TOLERANCE)).collect(), under })
}

/// 🧱️ The attach surface of the roof, slab or ceiling `id` standing on a storey of level `own`; the default (not found) surface for an id that names none of them.
pub fn surface_of(snapshot: &ModelSnapshot, id: &str, own: &StoreyLevel) -> AttachSurface {
    if let Some(slab) = snapshot.slabs.get(id) {
        let thickness = snapshot.slab_types.get(&slab.slab_type).map_or(0.0, |kind| layer_thicknesses(&kind.layers).iter().sum());
        let (outline, holes) = (bulged(&slab.boundary), slab.holes.iter().map(|hole| bulged(hole)).collect::<Vec<_>>());
        let top = slabs::top_plane(slab, &outline, own.elevation + slab.offset);
        return AttachSurface { found: true, pieces: piece(&outline, &holes, top.raised(-thickness)).into_iter().collect(), thickness, reference: own.elevation + slab.offset - thickness };
    }
    if let Some(ceiling) = snapshot.ceilings.get(id) {
        let thickness = ceilings::thickness(snapshot, ceiling);
        let (outline, holes) = (bulged(&ceiling.boundary), ceiling.holes.iter().map(|hole| bulged(hole)).collect::<Vec<_>>());
        let top = ceilings::top_plane(ceiling.slope, &outline, own.top_elevation - ceiling.offset);
        return AttachSurface { found: true, pieces: piece(&outline, &holes, top.raised(-thickness)).into_iter().collect(), thickness, reference: own.top_elevation - ceiling.offset - thickness };
    }
    if let Some(roof) = snapshot.roofs.get(id) {
        let thicknesses = snapshot.roof_types.get(&roof.roof_type).map(|kind| layer_thicknesses(&kind.layers)).unwrap_or_default();
        let (pieces, eave) = roofs::underside_pieces(roof, own);
        return AttachSurface { found: true, pieces: pieces.into_iter().map(|(outline, under)| Piece { outline, holes: Vec::new(), under }).collect(), thickness: thicknesses.iter().sum(), reference: eave };
    }
    AttachSurface::default()
}

/// 🔑️ What `surface_of` reads of the snapshot besides the level of the storey: the element and its type.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    if let Some(slab) = snapshot.slabs.get(id) {
        return dep_object([("slab", slabs::dependency(snapshot, &slab.anonymous()))]);
    }
    if let Some(ceiling) = snapshot.ceilings.get(id) {
        return dep_object([("ceiling", ceilings::dependency(snapshot, &ceiling.anonymous()))]);
    }
    if let Some(roof) = snapshot.roofs.get(id) {
        return dep_object([("roof", roofs::dependency(snapshot, &roof.anonymous()))]);
    }
    dep_object([("missing", dep_value(&id.to_string()))])
}
//#endregion 🔖️Surface

//#region 🔖️Sampling
fn on_edge(a: Point, b: Point, p: Point) -> bool {
    let (d, v) = (b - a, p - a);
    let l2 = d.dot(d);
    let t = if l2 > 0.0 { (v.dot(d) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (p - (a + d * t)).hypot() <= EPS
}

fn inside(ring: &[Point], p: Point) -> bool {
    let n = ring.len();
    if n < 3 {
        return false;
    }
    if (0..n).any(|k| on_edge(ring[k], ring[(k + 1) % n], p)) {
        return true;
    }
    let mut odd = false;
    for k in 0..n {
        let (a, b) = (ring[k], ring[(k + 1) % n]);
        if (a.y > p.y) != (b.y > p.y) && p.x < a.x + (p.y - a.y) / (b.y - a.y) * (b.x - a.x) {
            odd = !odd;
        }
    }
    odd
}

fn within_hole(ring: &[Point], p: Point) -> bool {
    let n = ring.len();
    inside(ring, p) && !(0..n).any(|k| on_edge(ring[k], ring[(k + 1) % n], p))
}

fn covers(piece: &Piece, p: Point) -> bool {
    inside(&piece.outline, p) && !piece.holes.iter().any(|hole| within_hole(hole, p))
}

fn crossing(p: Point, q: Point, a: Point, b: Point) -> Option<f64> {
    let (r, s) = (q - p, b - a);
    let denominator = cross(r, s);
    if denominator.abs() < 1e-14 {
        return None;
    }
    let t = cross(a - p, s) / denominator;
    let u = cross(a - p, r) / denominator;
    (t > 1e-12 && t < 1.0 - 1e-12 && (-1e-12..=1.0 + 1e-12).contains(&u)).then_some(t)
}

fn polyline(axis: &Axis, length: f64) -> Vec<(f64, Point)> {
    let curve = seg(axis);
    if curve.is_line() {
        return vec![(0.0, curve.start), (length, curve.end)];
    }
    curve.flatten(CHORD_TOLERANCE).into_iter().map(|point| (curve.param_of(point).clamp(0.0, 1.0) * length, point)).collect()
}

/// 📈️ The heights of a surface along an axis: the breakpoints and the fraction of the axis the surface covers.
#[derive(Clone, Debug, PartialEq)]
pub struct Sampled {
    pub points: Vec<ElevationPoint>,
    pub covered: f64,
}

/// 📈️ Cuts `axis` (length `length`) by the edges of the pieces of `surface` and reads the height of the underside (`over` false) or of the top surface (`over` true) at every cut. Gaps between covered stretches are bridged
/// linearly, the stretches before the first and after the last cut hold the nearest covered height; a surface that covers nothing yields no point.
pub fn sample(axis: &Axis, length: f64, surface: &AttachSurface, over: bool) -> Sampled {
    let lift = if over { surface.thickness } else { 0.0 };
    let line = polyline(axis, length);
    let mut raw: Vec<ElevationPoint> = Vec::new();
    let mut covered = 0.0;
    for pair in line.windows(2) {
        let ((s0, p), (s1, q)) = (pair[0], pair[1]);
        if s1 - s0 <= 0.0 {
            continue;
        }
        let mut cuts = vec![0.0, 1.0];
        for piece in &surface.pieces {
            for ring in std::iter::once(&piece.outline).chain(piece.holes.iter()) {
                for k in 0..ring.len() {
                    cuts.extend(crossing(p, q, ring[k], ring[(k + 1) % ring.len()]));
                }
            }
        }
        cuts.sort_by(f64::total_cmp);
        cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
        for span in cuts.windows(2) {
            let mid = lerp(p, q, (span[0] + span[1]) / 2.0);
            if let Some(piece) = surface.pieces.iter().find(|piece| covers(piece, mid)) {
                let (sa, sb) = (s0 + (s1 - s0) * span[0], s0 + (s1 - s0) * span[1]);
                raw.push(ElevationPoint { s: sa, z: piece.under.at(lerp(p, q, span[0])) + lift });
                raw.push(ElevationPoint { s: sb, z: piece.under.at(lerp(p, q, span[1])) + lift });
                covered += sb - sa;
            }
        }
    }
    let (Some(first), Some(last)) = (raw.first().copied(), raw.last().copied()) else { return Sampled { points: Vec::new(), covered: 0.0 } };
    let mut points = Vec::with_capacity(raw.len() + 2);
    if first.s > EPS {
        points.push(ElevationPoint { s: 0.0, z: first.z });
    }
    points.extend(raw);
    if last.s < length - EPS {
        points.push(ElevationPoint { s: length, z: last.z });
    }
    Sampled { points: simplified(points), covered: if length > 0.0 { (covered / length).min(1.0) } else { 0.0 } }
}

fn simplified(points: Vec<ElevationPoint>) -> Vec<ElevationPoint> {
    let mut out: Vec<ElevationPoint> = Vec::with_capacity(points.len());
    for point in points {
        if out.last().is_some_and(|last| (last.s - point.s).abs() < 1e-12 && (last.z - point.z).abs() < 1e-12) {
            continue;
        }
        out.push(point);
        while out.len() >= 3 {
            let (a, b, c) = (out[out.len() - 3], out[out.len() - 2], out[out.len() - 1]);
            let collinear = a.s < b.s - 1e-12 && b.s < c.s - 1e-12 && ((b.z - a.z) * (c.s - b.s) - (c.z - b.z) * (b.s - a.s)).abs() < 1e-12 * (c.s - a.s).max(1.0);
            if !collinear {
                break;
            }
            out.remove(out.len() - 2);
        }
    }
    out
}

/// 📈️ The height of an elevation edge at `s`: linear between the breakpoints, continued with the slope of the first or last segment beyond them. A single point is flat; no point is zero.
pub fn elevation_at(points: &[ElevationPoint], s: f64) -> f64 {
    match points {
        [] => 0.0,
        [only] => only.z,
        _ => {
            let value = |a: &ElevationPoint, b: &ElevationPoint| a.z + (b.z - a.z) * (s - a.s) / (b.s - a.s);
            if let Some(pair) = points.windows(2).find(|pair| pair[1].s - pair[0].s > 1e-12 && s >= pair[0].s - 1e-12 && s <= pair[1].s + 1e-12) {
                return value(&pair[0], &pair[1]);
            }
            if s < points[0].s {
                return points.windows(2).find(|pair| pair[1].s - pair[0].s > 1e-12).map_or(points[0].z, |pair| value(&pair[0], &pair[1]));
            }
            points.windows(2).rev().find(|pair| pair[1].s - pair[0].s > 1e-12).map_or(points[points.len() - 1].z, |pair| value(&pair[0], &pair[1]))
        }
    }
}

/// ✂️ The breakpoints of an edge over the extent `lo..hi`: the heights at both ends and every inner breakpoint.
pub fn restricted(points: &[ElevationPoint], lo: f64, hi: f64) -> Vec<ElevationPoint> {
    let mut out = vec![ElevationPoint { s: lo, z: elevation_at(points, lo) }];
    out.extend(points.iter().filter(|point| point.s > lo + 1e-12 && point.s < hi - 1e-12).copied());
    out.push(ElevationPoint { s: hi, z: elevation_at(points, hi) });
    simplified(out)
}
//#endregion 🔖️Sampling

//#region 🔖️Layout
struct Resolved {
    state: AttachState,
    points: Vec<ElevationPoint>,
    flat: Option<f64>,
}

#[allow(clippy::too_many_arguments)]
fn resolve(id: &str, offset: f64, over: bool, axis: &Axis, length: f64, window: Option<(f64, f64)>, surfaces: &BTreeMap<&str, &AttachSurface>, cycle: bool) -> Resolved {
    let state = |found: bool, covered: f64| AttachState { target: id.to_string(), found, covered, clamped: false, cycle };
    let Some(surface) = surfaces.get(id).filter(|surface| surface.found) else { return Resolved { state: state(false, 0.0), points: Vec::new(), flat: None } };
    let sampled = sample(axis, length, surface, over);
    let reference = surface.reference + if over { surface.thickness } else { 0.0 } + offset;
    if sampled.points.is_empty() {
        return Resolved { state: state(true, 0.0), points: Vec::new(), flat: Some(reference) };
    }
    let shifted: Vec<ElevationPoint> = sampled.points.iter().map(|point| ElevationPoint { s: point.s, z: point.z + offset }).collect();
    Resolved { state: state(true, sampled.covered), points: window.map_or(Vec::new(), |(lo, hi)| restricted(&shifted, lo, hi)), flat: if window.is_none() { shifted.first().map(|point| point.z) } else { None } }
}

struct Edge<'a> {
    points: &'a [ElevationPoint],
    flat: f64,
}

impl Edge<'_> {
    fn at(&self, s: f64) -> f64 {
        if self.points.is_empty() {
            self.flat
        } else {
            elevation_at(self.points, s)
        }
    }
}

fn clip(polygon: &[Point], normal: Vec2, offset: f64) -> Vec<Point> {
    let side = |p: Point| normal.x * p.x + normal.y * p.y - offset;
    let mut out = Vec::with_capacity(polygon.len() + 2);
    for k in 0..polygon.len() {
        let (a, b) = (polygon[k], polygon[(k + 1) % polygon.len()]);
        let (sa, sb) = (side(a), side(b));
        if sa >= 0.0 {
            out.push(a);
        }
        if (sa >= 0.0) != (sb >= 0.0) {
            out.push(lerp(a, b, sa / (sa - sb)));
        }
    }
    out
}

fn area_centroid(polygon: &[Point]) -> (f64, Point) {
    let vertices: Vec<Vertex> = polygon.iter().map(|&point| Vertex::new(point, 0.0)).collect();
    if polygon.len() < 3 {
        return (0.0, polygon.first().copied().unwrap_or(Point::ORIGIN));
    }
    let area = loops::signed_area(&vertices);
    if area.abs() < 1e-14 {
        return (0.0, polygon[0]);
    }
    (area.abs(), loops::centroid(&vertices))
}

/// 🧮️ How the arc length coordinate `s` of the axis cuts the plane: the half-plane `normal · p >= offset` holds the points at or beyond `s` (in the direction of travel).
struct Station<'a> {
    axis: &'a Axis,
    length: f64,
}

impl Station<'_> {
    fn beyond(&self, s: f64) -> (Vec2, f64) {
        let curve = seg(self.axis);
        match (curve.center(), curve.start_angle()) {
            (Some(centre), Some(angle)) => {
                let a = angle + curve.sweep() * s / self.length;
                let normal = perp(Vec2::new(a.cos(), a.sin())) * curve.sweep().signum();
                (normal, normal.x * centre.x + normal.y * centre.y)
            }
            _ => {
                let direction = (curve.end - curve.start) / curve.chord();
                (direction, direction.x * curve.start.x + direction.y * curve.start.y + s * (curve.chord() / self.length))
            }
        }
    }

    fn of(&self, p: Point) -> f64 {
        let curve = seg(self.axis);
        if curve.is_line() {
            let direction = (curve.end - curve.start) / curve.chord();
            (p - curve.start).dot(direction) * self.length / curve.chord()
        } else {
            curve.param_of(p) * self.length
        }
    }
}

fn integral(grid: &[f64], heights: &[f64], from: f64, to: f64) -> f64 {
    grid.windows(2).zip(heights.windows(2)).filter(|(pair, _)| pair[0] >= from - 1e-12 && pair[1] <= to + 1e-12).map(|(pair, h)| (pair[1] - pair[0]) * (h[0] + h[1]) / 2.0).sum()
}

fn interpolated(grid: &[f64], heights: &[f64], s: f64) -> f64 {
    match grid.windows(2).position(|pair| s >= pair[0] - 1e-12 && s <= pair[1] + 1e-12) {
        Some(k) if grid[k + 1] - grid[k] > 1e-12 => heights[k] + (heights[k + 1] - heights[k]) * (s - grid[k]) / (grid[k + 1] - grid[k]),
        Some(k) => heights[k],
        None if s < grid[0] => heights[0],
        None => heights[heights.len() - 1],
    }
}

/// 🔗️ The layout of an attached wall: `layout` is the layout over the flat reference heights, the surfaces are those of the parents of the wall. Sets the elevation edges, the vertical extent (`base_z` the lowest base, `top_z` the highest top,
/// `height` their difference), the exact side areas, the side area along the axis and the volume (the footprint cut into strips at every breakpoint, each strip times the height at its centroid), and reports how every attach resolved.
/// A wall without an attach is returned unchanged.
pub fn apply(snapshot: &ModelSnapshot, id: &str, wall: &Wall, mut layout: WallLayout, surfaces: &BTreeMap<&str, &AttachSurface>) -> WallLayout {
    let (top_target, base_target) = (top_target(&wall.top), wall.base_slab.as_deref());
    if top_target.is_none() && base_target.is_none() {
        return layout;
    }
    let cycle = find_cycle(id, &edges(snapshot)).is_some();
    let ends = face_ends(&layout, &wall.axis);
    let window = ends.as_ref().map(|ends| ends.full_thickness()).filter(|(lo, hi)| hi - lo > EPS);
    let top = top_target.map(|(target, offset)| resolve(target, offset, false, &wall.axis, layout.length, window, surfaces, cycle));
    let base = base_target.map(|target| resolve(target, wall.base_offset, true, &wall.axis, layout.length, window, surfaces, cycle));
    let (flat_top, flat_base) = (top.as_ref().and_then(|row| row.flat).unwrap_or(layout.top_z), base.as_ref().and_then(|row| row.flat).unwrap_or(layout.base_z));
    let (top_points, base_points) = (top.as_ref().map(|row| row.points.clone()).unwrap_or_default(), base.as_ref().map(|row| row.points.clone()).unwrap_or_default());
    let (top_edge, base_edge) = (Edge { points: &top_points, flat: flat_top }, Edge { points: &base_points, flat: flat_base });
    let mut grid: Vec<f64> = vec![0.0, layout.length];
    if let Some(ends) = &ends {
        grid.extend([ends.left.0, ends.left.1, ends.right.0, ends.right.1]);
    }
    if let Some((lo, hi)) = window {
        grid.extend([lo, hi]);
    }
    grid.extend(top_points.iter().chain(base_points.iter()).map(|point| point.s));
    let curve = seg(&wall.axis);
    if !curve.is_line() && layout.length > 0.0 {
        let cells = (curve.sweep().abs() / std::f64::consts::FRAC_PI_2).ceil().max(1.0) as usize;
        grid.extend((1..cells).map(|k| layout.length * k as f64 / cells as f64));
    }
    grid.sort_by(f64::total_cmp);
    grid.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    let bases: Vec<f64> = grid.iter().map(|&s| base_edge.at(s)).collect();
    let mut clamped = false;
    let tops: Vec<f64> = grid
        .iter()
        .zip(&bases)
        .map(|(&s, &floor)| {
            let value = top_edge.at(s);
            if value < floor + MIN_HEIGHT {
                clamped = true;
                floor + MIN_HEIGHT
            } else {
                value
            }
        })
        .collect();
    let heights: Vec<f64> = tops.iter().zip(&bases).map(|(top, base)| top - base).collect();
    layout.base_z = bases.iter().copied().fold(f64::INFINITY, f64::min);
    layout.top_z = tops.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    layout.height = layout.top_z - layout.base_z;
    layout.base_profile = base_points;
    layout.top_profile = if clamped {
        match window {
            Some((lo, hi)) => grid.iter().zip(&tops).filter(|(s, _)| **s >= lo - 1e-12 && **s <= hi + 1e-12).map(|(&s, &z)| ElevationPoint { s, z }).collect(),
            None => Vec::new(),
        }
    } else {
        top_points
    };
    layout.side_area = integral(&grid, &heights, 0.0, layout.length);
    if let Some(ends) = &ends {
        let share = |length: f64, (from, to): (f64, f64)| if to - from > 1e-12 { length / (to - from) } else { 0.0 };
        layout.left_area = share(layout.left_length, ends.left) * integral(&grid, &heights, ends.left.0, ends.left.1);
        layout.right_area = share(layout.right_length, ends.right) * integral(&grid, &heights, ends.right.0, ends.right.1);
    }
    if !layout.footprint.is_empty() {
        layout.volume = strips_volume(&wall.axis, &layout, &grid, &heights);
    }
    layout.top_attach = top.map(|row| AttachState { clamped, ..row.state });
    layout.base_attach = base.map(|row| row.state);
    layout
}

fn strips_volume(axis: &Axis, layout: &WallLayout, grid: &[f64], heights: &[f64]) -> f64 {
    let footprint = loops::flatten(&bulged(&layout.footprint), CHORD_TOLERANCE);
    let station = Station { axis, length: layout.length };
    let mut volume = 0.0;
    for (pair, _) in grid.windows(2).zip(heights.windows(2)) {
        if pair[1] - pair[0] <= 1e-12 {
            continue;
        }
        let (lower, upper) = (station.beyond(pair[0]), station.beyond(pair[1]));
        let strip = clip(&clip(&footprint, lower.0, lower.1), -upper.0, -upper.1);
        let (area, centroid) = area_centroid(&strip);
        if area > 0.0 {
            volume += area * interpolated(grid, heights, station.of(centroid).clamp(pair[0], pair[1]));
        }
    }
    volume
}
//#endregion 🔖️Layout

//#region 🔖️Testing
/// 🧪️ The builders of the small models the wall depth tests share: one building with a ground storey of 3 m, a wall type of one 0.3 m layer, door, window, slab and roof types and the materials `m` and `paint`.
#[cfg(test)]
pub mod testing {
    use crate::ModelSnapshot;
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
    use serde_json::{json, Value};

    /// 🧮️ Equal within a relative tolerance of a nanometre.
    pub fn close(left: f64, right: f64) -> bool {
        (left - right).abs() <= 1e-9 * right.abs().max(1.0)
    }

    /// ▭️ A rectangular loop of the model JSON, counter-clockwise.
    pub fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Value {
        json!([{ "point": { "x": x0, "y": y0 }, "bulge": 0.0 }, { "point": { "x": x1, "y": y0 }, "bulge": 0.0 }, { "point": { "x": x1, "y": y1 }, "bulge": 0.0 }, { "point": { "x": x0, "y": y1 }, "bulge": 0.0 }])
    }

    /// 🧱️ A straight centred wall of type `wt` on the storey `st` from `axis` = `[x0, y0, x1, y1]`, with `top` and the members of `extra` merged in.
    pub fn wall(axis: [f64; 4], top: Value, extra: Value) -> Value {
        let mut row = json!({ "storey": "st", "wall_type": "wt", "axis": { "Line": { "start": { "x": axis[0], "y": axis[1] }, "end": { "x": axis[2], "y": axis[3] } } }, "location": "Center", "base_offset": 0.0, "top": top, "phase": "New", "name": "" });
        row.as_object_mut().expect("a wall object").extend(extra.as_object().expect("extra object").clone());
        row
    }

    /// 🏠️ A gable roof `r` over the footprint 8 m by 6 m with its ridge along x, pitch 0.5 rad, no overhang, on the ground storey.
    pub fn gable_roof() -> Value {
        json!({ "r": { "storey": "st", "roof_type": "rt", "footprint": rect(0.0, 0.0, 8.0, 6.0), "shape": { "Gable": { "pitch": 0.5, "ridge_direction": 0.0 } }, "overhang": 0.0, "base_offset": 0.0, "phase": "New", "name": "" } })
    }

    /// 🏗️ The model with `walls` and the members of `extra` (roofs, slabs, openings, sweeps, …) merged into the base document.
    pub fn model(walls: Value, extra: Value) -> ModelSnapshot {
        let mut document = json!({
            "schema": "s.bim.model@1",
            "project": { "name": "T", "description": "", "author": "", "organization": "", "phase_names": [] },
            "materials": { "m": { "name": "M", "category": "Masonry", "color": { "r": 0.5, "g": 0.5, "b": 0.5 }, "density": 1800.0, "conductivity": 0.8, "specific_heat": 900.0 }, "paint": { "name": "Paint", "category": "Finish", "color": { "r": 1.0, "g": 1.0, "b": 1.0 }, "density": 1200.0, "conductivity": 0.5, "specific_heat": 1000.0 } },
            "wall_types": { "wt": { "name": "W", "layers": [{ "material": "m", "thickness": 0.3, "function": "Structure" }] } },
            "door_types": { "dt": { "name": "D", "width": 0.9, "height": 2.1, "frame_width": 0.05, "frame_depth": 0.1, "leaves": "Single", "swing": "Left", "material": "m" } },
            "window_types": { "wi": { "name": "Win", "width": 1.2, "height": 1.2, "sill": 0.9, "frame_width": 0.06, "frame_depth": 0.08, "panes": 1, "material": "m" } },
            "slab_types": { "slt": { "name": "S", "layers": [{ "material": "m", "thickness": 0.2, "function": "Structure" }] } },
            "roof_types": { "rt": { "name": "R", "layers": [{ "material": "m", "thickness": 0.1, "function": "Finish" }] } },
            "sites": { "s": { "name": "Plot", "latitude": 47.0, "longitude": 8.0, "elevation": 0.0, "true_north": 0.0, "boundary": [] } },
            "buildings": { "b": { "site": "s", "name": "House", "origin": { "x": 0.0, "y": 0.0 }, "rotation": 0.0, "elevation": 0.0 } },
            "storeys": { "st": { "building": "b", "name": "Ground", "level": 0, "height": 3.0 } },
            "walls": walls
        });
        document.as_object_mut().expect("a document object").extend(extra.as_object().expect("extra object").clone());
        from_json_str(&document.to_string(), JsonMemberPolicy::Reject).expect("the test model decodes")
    }
}
//#endregion 🔖️Testing

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
