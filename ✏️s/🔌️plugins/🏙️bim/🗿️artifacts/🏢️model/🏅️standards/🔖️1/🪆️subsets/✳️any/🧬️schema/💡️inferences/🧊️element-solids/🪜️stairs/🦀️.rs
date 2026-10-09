//! 🪜️ `stairs`: the solid of every stair, straight, L-turn, U-turn or spiral, built from its parts: tread slabs, riser boards, stringers and landing slabs.
//!
//! The run of a stair (rise, riser count and height, tread, flights, landings) is the pure `stair_runs::run_of`, so the solid, the quantities and the code flags can never disagree; this module
//! only turns a [`StairRun`] and the authored construction of the stair into parts. Per straight flight the parts stand in the flight frame: `u` along the travel from the foot of the first riser,
//! `w` across from `-width / 2` to `width / 2`, `z` up from the base of the flight; `g` is the tread, `h` the riser height, `t` the tread thickness and `n` the nosing. Riser `k` (0-based) stands at
//! `u = k g`, tread `k` has its top at `z = (k + 1) h`.
//! * tread `k`: a slab `u in [k g - n, (k + 1) g]` over the full width, `z in [(k + 1) h - t, (k + 1) h]`, volume `(g + n) * width * t`.
//! * riser `k` (closed risers only): a board `u in [k g, k g + t]`, `z in [k h, (k + 1) h - t]`, full width, volume `(h - t) * width * t`. A flight that arrives on a landing also has the riser `treads`,
//!   the front of the landing; the last flight arrives on the upper floor, whose edge needs no board.
//! * stringer: boards of width `stringer.width` whose side profile is a band of vertical depth `stringer.depth`, cut by the floor at the foot and by the arrival level at the head.
//!   Closed: outside the treads, `w in [width / 2, width / 2 + sw]` and its mirror, the band under the pitch line through the noses. Open: inside the treads, `w in [width / 2 - sw, width / 2]` and its
//!   mirror, the band under the saw tooth of the tread undersides (the boards start behind the first riser board when risers are closed). Mono: one central board `w in [-sw / 2, sw / 2]` under the
//!   pitch line through the back lower corners of the treads. Stringers end at the edge of the landing; a landing is a self-supporting slab.
//! * landing: a slab `landing_depth x width` (U-turn: `landing_depth x (2 width + gap)`) of thickness `t`, its top at the landing height.
//!
//! Winding flights (spiral stairs) are one wedge slab of thickness `t` per tread; their risers and stringers are out of scope (the code flag `stair.stringer-ignored-on-spiral` says so).
//! A stair with a single riser has no tread and is absent.
//!
//! Related: <https://en.wikipedia.org/wiki/Stairs>.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{direction, point};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::{StairFlightRun, StairLanding, StairRun, StairWinder};
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::run_of;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{RiserKind, Stair, StairFlight, StringerKind};
use semio_framework_geometry::loops::Vertex;
use semio_framework_geometry::mesh::{extrude, extrude_loops, prism_between, TriMesh};
use semio_framework_geometry::placement::{Affine3, ZPlane};
use semio_framework_geometry::vector::cross;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

const EPS: f64 = 1e-9;

//#region 🔖️Profiles
/// 🪜️ The measures of one straight flight in its own frame: the tread `g`, the riser height `h`, the tread thickness `t`, the nosing `n`, the number of treads and whether the risers are closed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Flight {
    pub going: f64,
    pub rise: f64,
    pub thickness: f64,
    pub nosing: f64,
    pub treads: u32,
    pub closed: bool,
}

/// ✂️ The part of a polygon on the side of a line where `side` is not negative (Sutherland-Hodgman; exact for the monotone bands of a stringer).
fn clipped(polygon: &[Point], side: impl Fn(Point) -> f64) -> Vec<Point> {
    let mut out = Vec::new();
    for (index, &current) in polygon.iter().enumerate() {
        let next = polygon[(index + 1) % polygon.len()];
        let (a, b) = (side(current), side(next));
        if a >= 0.0 {
            out.push(current);
        }
        if (a >= 0.0) != (b >= 0.0) {
            out.push(current + (next - current) * (a / (a - b)));
        }
    }
    out
}

fn area_of(polygon: &[Point]) -> f64 {
    (0..polygon.len()).map(|index| cross(polygon[index] - Point::ZERO, polygon[(index + 1) % polygon.len()] - Point::ZERO)).sum::<f64>() / 2.0
}

impl Flight {
    /// 🪜️ The height of the arrival level above the base of the flight.
    pub fn arrival(&self) -> f64 {
        f64::from(self.treads + 1) * self.rise
    }

    /// 🪜️ The length of the flight from the foot of the first riser to the arrival edge.
    pub fn length(&self) -> f64 {
        f64::from(self.treads) * self.going
    }

    /// 🪵️ The side profile `(u, z)` of one board of a stringer of `kind` and vertical `depth`, counter-clockwise; empty for none, a flight without treads or a board that vanishes.
    pub fn profile(&self, kind: StringerKind, depth: f64) -> Vec<Point> {
        let Self { going, rise, thickness, nosing, treads, closed } = *self;
        let (slope, end, level) = (rise / going, self.length(), self.arrival());
        let band = |from: f64, line: &dyn Fn(f64) -> f64| vec![Point::new(from, line(from)), Point::new(end, line(end)), Point::new(end, line(end) - depth), Point::new(from, line(from) - depth)];
        let polygon = match kind {
            StringerKind::None => Vec::new(),
            StringerKind::Closed => clipped(&clipped(&band(-nosing, &|u| rise + (u + nosing) * slope), |p| p.y), |p| level - p.y),
            StringerKind::Mono => clipped(&band(0.0, &|u| u * slope - thickness), |p| p.y),
            StringerKind::Open => {
                let lead = if closed { thickness } else { 0.0 };
                let mut outline = Vec::new();
                for tread in 0..treads {
                    let z = f64::from(tread + 1) * rise - thickness;
                    outline.push(Point::new(f64::from(tread) * going + lead, z));
                    outline.push(Point::new((f64::from(tread + 1) * going + lead).min(end), z));
                }
                let low = |u: f64| (u - lead) * slope - thickness - depth;
                outline.extend([Point::new(end, low(end)), Point::new(lead, low(lead))]);
                clipped(&outline, |p| p.y)
            }
        };
        if treads > 0 && polygon.len() >= 3 && area_of(&polygon).abs() > EPS { polygon } else { Vec::new() }
    }

    /// 🪵️ The across ranges `(from, to)` of the boards of a stringer on a stair of `width`.
    pub fn boards(kind: StringerKind, stringer_width: f64, width: f64) -> Vec<(f64, f64)> {
        let (half, sw) = (width / 2.0, stringer_width);
        match kind {
            StringerKind::None => Vec::new(),
            StringerKind::Closed => vec![(half, half + sw), (-half - sw, -half)],
            StringerKind::Open => vec![(half - sw, half), (-half, -half + sw)],
            StringerKind::Mono => vec![(-sw / 2.0, sw / 2.0)],
        }
    }
}
//#endregion 🔖️Profiles

//#region 🔖️Parts
fn slab(u: (f64, f64), w: (f64, f64), z: (f64, f64)) -> TriMesh {
    extrude(&[Point::new(u.0, w.0), Point::new(u.1, w.0), Point::new(u.1, w.1), Point::new(u.0, w.1)], &[], ZPlane::flat(z.0), ZPlane::flat(z.1))
}

fn board(profile: &[Point], w: (f64, f64)) -> TriMesh {
    let ring = |across: f64| profile.iter().map(|p| [p.x, across, p.y]).collect::<Vec<_>>();
    prism_between(&ring(w.0), &ring(w.1))
}

fn frame_at(origin: Point, z: f64, angle: f64) -> Affine3 {
    let (sin, cos) = angle.sin_cos();
    Affine3::from_frame([origin.x, origin.y, z], [cos, sin, 0.0], [-sin, cos, 0.0], [0.0, 0.0, 1.0])
}

/// 🪜️ The measures of a flight of a run with the construction of a stair.
pub fn flight_of(stair: &Stair, run: &StairRun, flight: &StairFlightRun) -> Flight {
    Flight { going: flight.tread, rise: run.riser_height, thickness: stair.tread_thickness, nosing: stair.nosing, treads: flight.treads, closed: stair.riser == RiserKind::Closed }
}

/// 🪜️ The meshes of the parts of a stair.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StairParts {
    pub treads: TriMesh,
    pub risers: TriMesh,
    pub stringers: TriMesh,
    pub landings: TriMesh,
}

fn straight(stair: &Stair, run: &StairRun, flight: &StairFlightRun, landed: bool) -> StairParts {
    let measures = flight_of(stair, run, flight);
    let Flight { going: g, rise: h, thickness: t, nosing: n, treads, closed } = measures;
    let (half, mut parts) = (run.width / 2.0, StairParts::default());
    let level = |index: u32| f64::from(index) * h;
    for tread in 0..treads {
        parts.treads.append(&slab((f64::from(tread) * g - n, f64::from(tread + 1) * g), (-half, half), (level(tread + 1) - t, level(tread + 1))));
    }
    if closed {
        for riser in 0..treads + u32::from(landed) {
            parts.risers.append(&slab((f64::from(riser) * g, f64::from(riser) * g + t), (-half, half), (level(riser), level(riser + 1) - t)));
        }
    }
    let profile = measures.profile(stair.stringer.kind, stair.stringer.depth);
    for across in Flight::boards(stair.stringer.kind, stair.stringer.width, run.width).into_iter().filter(|_| !profile.is_empty()) {
        parts.stringers.append(&board(&profile, across));
    }
    let frame = frame_at(point(&flight.start), flight.base_z, flight.direction);
    StairParts { treads: parts.treads.transformed(&frame), risers: parts.risers.transformed(&frame), stringers: parts.stringers.transformed(&frame), landings: TriMesh::new() }
}

fn wedge(winder: &StairWinder, from: f64, to: f64) -> Vec<Vertex> {
    let (centre, bulge) = (point(&winder.centre), ((to - from) / 4.0).tan());
    let at = |radius: f64, angle: f64| centre + direction(angle) * radius;
    if winder.inner_radius < EPS {
        vec![Vertex::new(centre, 0.0), Vertex::new(at(winder.outer_radius, from), bulge), Vertex::new(at(winder.outer_radius, to), 0.0)]
    } else {
        vec![Vertex::new(at(winder.inner_radius, from), 0.0), Vertex::new(at(winder.outer_radius, from), bulge), Vertex::new(at(winder.outer_radius, to), 0.0), Vertex::new(at(winder.inner_radius, to), -bulge)]
    }
}

fn winding(stair: &Stair, run: &StairRun, flight: &StairFlightRun, winder: &StairWinder) -> StairParts {
    let delta = winder.sweep / f64::from(flight.treads.max(1));
    let mut treads = TriMesh::new();
    for index in 0..flight.treads {
        let top = flight.base_z + f64::from(index + 1) * run.riser_height;
        treads.append(&extrude_loops(&wedge(winder, winder.start_angle + delta * f64::from(index), winder.start_angle + delta * f64::from(index + 1)), &[], CHORD_TOLERANCE, ZPlane::flat(top - stair.tread_thickness), ZPlane::flat(top)));
    }
    StairParts { treads, ..StairParts::default() }
}

fn landing(stair: &Stair, landing: &StairLanding) -> TriMesh {
    slab((-landing.depth / 2.0, landing.depth / 2.0), (-landing.width / 2.0, landing.width / 2.0), (-stair.tread_thickness, 0.0)).transformed(&frame_at(point(&landing.centre), landing.z, landing.direction))
}

/// 🪜️ The parts of a stair from its authored construction and its run.
pub fn parts_of(stair: &Stair, run: &StairRun) -> StairParts {
    let mut built = StairParts::default();
    if run.width < EPS || run.riser_height < EPS {
        return built;
    }
    for (index, flight) in run.flights.iter().enumerate() {
        let part = match &flight.winder {
            Some(winder) => winding(stair, run, flight, winder),
            None => straight(stair, run, flight, index + 1 < run.flights.len()),
        };
        built.treads.append(&part.treads);
        built.risers.append(&part.risers);
        built.stringers.append(&part.stringers);
    }
    for row in &run.landings {
        built.landings.append(&landing(stair, row));
    }
    built
}

/// 🪵️ The plan footprints `(flight index, corners)` of the stringer boards of a stair: a rectangle per board over the length of its flight.
pub fn stringer_footprints(stair: &Stair, run: &StairRun) -> Vec<(usize, [Point; 4])> {
    let mut found = Vec::new();
    for (index, flight) in run.flights.iter().enumerate().filter(|(_, flight)| flight.winder.is_none() && flight.treads > 0) {
        let measures = flight_of(stair, run, flight);
        if measures.profile(stair.stringer.kind, stair.stringer.depth).is_empty() {
            continue;
        }
        let (origin, (sin, cos)) = (point(&flight.start), flight.direction.sin_cos());
        let at = |u: f64, w: f64| Point::new(origin.x + u * cos - w * sin, origin.y + u * sin + w * cos);
        let from = if stair.stringer.kind == StringerKind::Closed { -stair.nosing } else { 0.0 };
        for (low, high) in Flight::boards(stair.stringer.kind, stair.stringer.width, run.width) {
            found.push((index, [at(from, low), at(measures.length(), low), at(measures.length(), high), at(from, high)]));
        }
    }
    found
}
//#endregion 🔖️Parts

//#region 🔖️Geometry
/// 🪜️ The run and the meshes of one stair.
#[derive(Clone, Debug, PartialEq)]
pub struct StairGeometry {
    pub run: StairRun,
    pub steps: usize,
    pub parts: StairParts,
}

/// 🪜️ The geometry of a stair from its record and the run the `StairRun` node resolved for it.
pub fn stair_geometry_of(stair: &Stair, run: StairRun) -> StairGeometry {
    let parts = parts_of(stair, &run);
    let steps = run.flights.iter().map(|flight| flight.treads as usize).sum::<usize>() + run.landings.len();
    StairGeometry { steps: if parts.treads.indices.is_empty() && parts.landings.indices.is_empty() { 0 } else { steps }, run, parts }
}

/// 🪜️ The geometry of a stair from the levels of the storeys it is resolved by (tests only: the graph passes the run of the `StairRun` node to [`stair_geometry_of`]).
#[cfg(test)]
pub fn stair_geometry(stair: &Stair, own: &StoreyLevel, target: Option<&StoreyLevel>) -> StairGeometry {
    stair_geometry_of(stair, run_of(stair, own, target))
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🪜️ The solid of a stair from its record and its run.
pub fn stair_solid(stair: &Stair, run: &StairRun) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Stair);
    let built = parts_of(stair, run);
    builder.add(parts::STEP, "", 0, &built.treads).add(parts::RISER, "", 0, &built.risers).add(parts::STRINGER, "", 0, &built.stringers).add(parts::LANDING, "", 0, &built.landings);
    builder.build()
}

/// 🔑️ What `stair_solid` reads besides the run: the construction of the stair (the run already carries its landing depth).
pub fn dependency(stair: &Stair) -> DslValue {
    dep_object([("stringer", dep_value(&stair.stringer)), ("nosing", dep_value(&stair.nosing)), ("tread_thickness", dep_value(&stair.tread_thickness)), ("riser", dep_value(&stair.riser))])
}

/// 🪜️ Whether the stringer kind of a stair has no effect on its flight kind: winding flights have neither stringers nor risers.
pub fn stringer_ignored(stair: &Stair) -> bool {
    stair.stringer.kind != StringerKind::None && matches!(stair.flight, StairFlight::Spiral { .. })
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
