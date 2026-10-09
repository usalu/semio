//! 🛝️ `ramps`: the solid of every ramp, a slab of constant vertical thickness swept along the centre line with the walking surface following the inferred run (flat landings, an even slope between them), plus the posts and
//! rail of the side railings the ramp asks for.
//!
//! The run of a ramp (rise, sloped length, landings) is the pure `ramp_runs::run_of`, so the solid, the quantities and the code flags can never disagree; this module only turns a [`RampRun`] and the plan strip of the ramp into
//! prisms. Stations lie at every segment end, at every landing and flight end and along arcs within the chord tolerance, so the surface is exact between them. The module also owns the rising rail builder
//! ([`railing_meshes`]) that the hosted railings of stairs, ramps and slab edges share: a rail swept along a polyline whose height follows the host.
//!
//! Related: <https://en.wikipedia.org/wiki/Wheelchair_ramp>.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{extents_of, placed, rectangle};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::ramp_runs::{strip_of, RampRun, RampStrip};
use crate::{standard_post_profile, standard_rail_profile, Ramp};
use semio_framework_geometry::mesh::{extrude_loops, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::vector::Xyz;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Stations
/// 🛝️ One cross-section of the ramp: the left and right edge and the centre point in plan, the arc length along the centre line and the height of the walking surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Station {
    pub left: Point,
    pub right: Point,
    pub centre: Point,
    pub s: f64,
    pub z: f64,
}

fn chords(strip: &RampStrip, index: usize) -> usize {
    let segment = &strip.centre[index];
    if segment.is_line() {
        return 1;
    }
    let radius = segment.radius() + strip.half;
    let step = 2.0 * (1.0 - (CHORD_TOLERANCE / radius).clamp(1e-12, 1.0)).acos();
    ((segment.sweep().abs() / step.max(1e-6)).ceil() as usize).clamp(1, 4096)
}

/// 📍️ The stations of a ramp: every segment end, every flight and landing end and the chord points of arcs, in arc-length order.
pub fn stations_of(strip: &RampStrip, run: &RampRun) -> Vec<Station> {
    if strip.is_empty() {
        return Vec::new();
    }
    let mut marks: Vec<f64> = vec![strip.length];
    for index in 0..strip.centre.len() {
        let (start, length, count) = (strip.starts[index], strip.centre[index].length(), chords(strip, index));
        marks.extend((0..count).map(|step| start + length * step as f64 / count as f64));
    }
    marks.extend(run.flights.iter().flat_map(|flight| [flight.from, flight.to]));
    marks.extend(run.landings.iter().flat_map(|landing| [landing.from, landing.to]));
    marks.iter_mut().for_each(|mark| *mark = mark.clamp(0.0, strip.length));
    marks.sort_by(f64::total_cmp);
    marks.dedup_by(|a, b| (*a - *b).abs() <= 1e-9);
    marks
        .into_iter()
        .map(|s| {
            let (left, right, centre) = strip.section_at(s);
            Station { left, right, centre, s, z: run.z_at(s) }
        })
        .collect()
}
//#endregion 🔖️Stations

//#region 🔖️Sweep
/// 🧱️ The four corners of one cross-section of a swept rectangle: left and right, top and bottom.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ring {
    pub left_top: Xyz,
    pub right_top: Xyz,
    pub left_bottom: Xyz,
    pub right_bottom: Xyz,
}

/// 🧱️ The closed solid through consecutive rings (top, bottom, left and right faces, and both end caps), counter-clockwise outward when the rings run in the direction of travel with the left corners on the left.
pub fn sweep_rings(rings: &[Ring]) -> TriMesh {
    let mut mesh = TriMesh::new();
    let (Some(first), Some(last)) = (rings.first(), rings.last()) else { return mesh };
    if rings.len() < 2 {
        return mesh;
    }
    for pair in rings.windows(2) {
        let (a, b) = (&pair[0], &pair[1]);
        mesh.push_quad(a.right_top, b.right_top, b.left_top, a.left_top);
        mesh.push_quad(a.left_bottom, b.left_bottom, b.right_bottom, a.right_bottom);
        mesh.push_quad(a.left_bottom, a.left_top, b.left_top, b.left_bottom);
        mesh.push_quad(a.right_bottom, b.right_bottom, b.right_top, a.right_top);
    }
    mesh.push_quad(first.left_bottom, first.right_bottom, first.right_top, first.left_top);
    mesh.push_quad(last.right_bottom, last.left_bottom, last.left_top, last.right_top);
    mesh
}

fn lift(point: Point, z: f64) -> Xyz {
    [point.x, point.y, z]
}

/// 🛝️ The slab of a ramp: its walking surface on the stations, `thickness` below it.
pub fn body_mesh(stations: &[Station], thickness: f64) -> TriMesh {
    let rings: Vec<Ring> = stations.iter().map(|station| Ring { left_top: lift(station.left, station.z), right_top: lift(station.right, station.z), left_bottom: lift(station.left, station.z - thickness), right_bottom: lift(station.right, station.z - thickness) }).collect();
    sweep_rings(&rings)
}
//#endregion 🔖️Sweep

//#region 🔖️Rails
/// 📍️ A point of a rail path: its plan position and the height of the surface the railing stands on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🛤️ The measures of a railing that its solid needs: the height of the top of the rail above its base, the largest post spacing and the section extents (across, depth) of rail and post.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailSpec {
    pub height: f64,
    pub spacing: f64,
    pub rail: (f64, f64),
    pub post: (f64, f64),
}

impl RailSpec {
    /// 🛤️ The standard railing of a ramp: the standard rail and post sections, 1 m high, a post every 1.2 m.
    pub fn standard() -> Self {
        Self { height: RAMP_RAIL_HEIGHT, spacing: RAMP_POST_SPACING, rail: extents_of(&standard_rail_profile()), post: extents_of(&standard_post_profile()) }
    }
}

/// 📏️ Height of the rail of a ramp's side railing above the walking surface, in metres.
pub const RAMP_RAIL_HEIGHT: f64 = 1.0;
/// 📏️ Post spacing of a ramp's side railing, in metres.
pub const RAMP_POST_SPACING: f64 = 1.2;
/// 📏️ How far a ramp's side railing stands inside the edge of the ramp, in metres.
pub const RAMP_RAIL_INSET: f64 = 0.05;
const CORNER_TURN: f64 = 0.35;
const SLOPE_STEP: f64 = 1e-3;
const EPS: f64 = 1e-9;

fn horizontal(a: &RailPoint, b: &RailPoint) -> f64 {
    (b.x - a.x).hypot(b.y - a.y)
}

fn heading(a: &RailPoint, b: &RailPoint) -> f64 {
    (b.y - a.y).atan2(b.x - a.x)
}

fn tidy(run: &[RailPoint]) -> Vec<RailPoint> {
    let mut out: Vec<RailPoint> = Vec::with_capacity(run.len());
    for point in run {
        if out.last().is_none_or(|last| horizontal(last, point) > EPS) {
            out.push(*point);
        }
    }
    out
}

fn corner(path: &[RailPoint], index: usize) -> bool {
    let (before, here, after) = (&path[index - 1], &path[index], &path[index + 1]);
    let turn = (heading(here, after) - heading(before, here) + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI;
    let slope = |a: &RailPoint, b: &RailPoint| (b.z - a.z) / horizontal(a, b);
    turn.abs() > CORNER_TURN || (slope(here, after) - slope(before, here)).abs() > SLOPE_STEP
}

/// 🛤️ The post positions of one continuous rail path with the plan heading and the surface height at each: a post at both ends, at every corner (a turn of the path or a change of its slope) and equal subdivisions of at most `spacing` of the horizontal length between them.
pub fn post_positions(path: &[RailPoint], spacing: f64) -> Vec<(Point, f64, f64)> {
    let path = tidy(path);
    if path.len() < 2 {
        return Vec::new();
    }
    let mut breaks: Vec<usize> = vec![0];
    breaks.extend((1..path.len() - 1).filter(|&index| corner(&path, index)));
    breaks.push(path.len() - 1);
    let mut posts = Vec::new();
    for (span_index, span) in breaks.windows(2).enumerate() {
        let points = &path[span[0]..=span[1]];
        let lengths: Vec<f64> = points.windows(2).map(|pair| horizontal(&pair[0], &pair[1])).collect();
        let total: f64 = lengths.iter().sum();
        let divisions = if spacing > EPS { ((total / spacing - EPS).ceil() as usize).max(1) } else { 1 };
        for step in (if span_index == 0 { 0 } else { 1 })..=divisions {
            let target = total * step as f64 / divisions as f64;
            let mut passed = 0.0;
            let mut found = (points[0], points[1], 0.0);
            for (index, length) in lengths.iter().enumerate() {
                found = (points[index], points[index + 1], if *length > EPS { ((target - passed) / length).clamp(0.0, 1.0) } else { 0.0 });
                if target <= passed + length + EPS {
                    break;
                }
                passed += length;
            }
            let (a, b, t) = found;
            posts.push((Point::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t), heading(&a, &b), a.z + (b.z - a.z) * t));
        }
    }
    posts
}

fn lateral(path: &[RailPoint], index: usize) -> (f64, f64, f64) {
    let normal = |a: &RailPoint, b: &RailPoint| {
        let angle = heading(a, b);
        (-angle.sin(), angle.cos())
    };
    if index == 0 {
        let (x, y) = normal(&path[0], &path[1]);
        return (x, y, 1.0);
    }
    if index == path.len() - 1 {
        let (x, y) = normal(&path[index - 1], &path[index]);
        return (x, y, 1.0);
    }
    let ((ax, ay), (bx, by)) = (normal(&path[index - 1], &path[index]), normal(&path[index], &path[index + 1]));
    let (sx, sy) = (ax + bx, ay + by);
    let length = sx.hypot(sy);
    if length < 1e-9 {
        return (ax, ay, 1.0);
    }
    let (mx, my) = (sx / length, sy / length);
    (mx, my, 1.0 / (mx * ax + my * ay).max(0.2))
}

/// 🛤️ The rail swept along one rail path: a rectangle of `spec.rail` whose top lies `spec.height` above the surface, mitred at the corners, closed at both ends.
pub fn rail_mesh(path: &[RailPoint], spec: &RailSpec) -> TriMesh {
    let path = tidy(path);
    if path.len() < 2 {
        return TriMesh::new();
    }
    let (width, depth) = spec.rail;
    let rings: Vec<Ring> = (0..path.len())
        .map(|index| {
            let (nx, ny, scale) = lateral(&path, index);
            let (point, half) = (path[index], width / 2.0 * scale);
            let top = point.z + spec.height;
            Ring {
                left_top: [point.x + nx * half, point.y + ny * half, top],
                right_top: [point.x - nx * half, point.y - ny * half, top],
                left_bottom: [point.x + nx * half, point.y + ny * half, top - depth],
                right_bottom: [point.x - nx * half, point.y - ny * half, top - depth],
            }
        })
        .collect();
    sweep_rings(&rings)
}

/// 🛤️ The posts and the rail of a railing over its rail paths (each path is continuous; a jump in height starts a new path): `(posts, rail)`.
pub fn railing_meshes(paths: &[Vec<RailPoint>], spec: &RailSpec) -> (TriMesh, TriMesh) {
    let (mut posts, mut rails) = (TriMesh::new(), TriMesh::new());
    for path in paths {
        rails.append(&rail_mesh(path, spec));
        let reach = spec.height - spec.rail.1;
        if reach <= EPS {
            continue;
        }
        for (position, angle, z) in post_positions(path, spec.spacing) {
            posts.append(&extrude_loops(&placed(&rectangle(spec.post.0, spec.post.1), position, angle), &[], CHORD_TOLERANCE, ZPlane::flat(z), ZPlane::flat(z + reach)));
        }
    }
    (posts, rails)
}

/// 🛝️ The rail path along one edge of a ramp, `inset` metres inside it, on the walking surface of the stations.
pub fn edge_path(stations: &[Station], width: f64, left: bool, inset: f64) -> Vec<RailPoint> {
    let inset = (inset / width.max(EPS)).clamp(0.0, 0.5);
    stations
        .iter()
        .map(|station| {
            let (from, to) = if left { (station.left, station.right) } else { (station.right, station.left) };
            RailPoint { x: from.x + (to.x - from.x) * inset, y: from.y + (to.y - from.y) * inset, z: station.z }
        })
        .collect()
}
//#endregion 🔖️Rails

//#region 🔖️Solid
/// 🛝️ The solid of a ramp from its run: the slab and, for every side that asks for one, the posts and the rail of its railing.
pub fn ramp_solid(ramp: &Ramp, run: &RampRun) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Ramp);
    let strip = strip_of(ramp);
    let stations = stations_of(&strip, run);
    builder.add(parts::BODY, &ramp.material, 0, &body_mesh(&stations, ramp.thickness));
    let spec = RailSpec::standard();
    let paths: Vec<Vec<RailPoint>> = [(ramp.railing_left, true), (ramp.railing_right, false)].into_iter().filter(|(wanted, _)| *wanted).map(|(_, left)| edge_path(&stations, ramp.width, left, RAMP_RAIL_INSET)).collect();
    let (posts, rails) = railing_meshes(&paths, &spec);
    builder.add(parts::POST, "", 1, &posts).add(parts::RAIL, "", 1, &rails);
    builder.build()
}

/// 🔑️ What `ramp_solid` reads besides the run: the ramp record (its path, width, thickness, material and railing flags).
pub fn dependency(ramp: &Ramp) -> DslValue {
    dep_value(ramp)
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
