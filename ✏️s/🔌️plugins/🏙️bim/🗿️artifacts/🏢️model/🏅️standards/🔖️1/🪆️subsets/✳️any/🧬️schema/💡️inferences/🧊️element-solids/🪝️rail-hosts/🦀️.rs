//! 🪝️ `rail-hosts`: the solid of a railing that is hosted by a stair, a ramp or a slab edge. A hosted railing authors no path: its path is the edge of its host, `inset` metres inside it, and its base follows the host surface plus
//! its base offset. The host geometry is the already inferred run of the stair or the ramp (so the rail climbs exactly as the flights climb) or the top plane of the slab, never a second derivation of them.
//!
//! * stair: along the `side` edge of every flight on the pitch line through the tread nosings; across a landing the rail follows the landing boundary on that side, flat at the landing height, and a new rail path starts
//!   at the next flight (a step of one riser height is a break of the rail, as with a newel).
//! * ramp: along the `side` edge of the ramp, on the walking surface, stations as in the ramp solid.
//! * slab: along edge `edge` of the boundary (the edge from that vertex to the next), on the top plane of the slab; left is inward, right outward.
//!
//! Related: <https://en.wikipedia.org/wiki/Baluster>.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged, extents_of, point};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ramps::{edge_path, railing_meshes, stations_of, RailPoint, RailSpec};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::slabs::top_plane;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_value, parts, Anonymous, ElementSolid, SolidBuilder, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::ramp_runs::{strip_of, RampRun};
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::{StairLanding, StairRun, StairWinder};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{HostSide, ModelSnapshot, Railing, RailingHost, Ramp, Slab};
use semio_framework_geometry::loops;
use semio_framework_value::DslValue;
use std::f64::consts::PI;

const EPS: f64 = 1e-9;
const WINDER_STEP: f64 = 5.0 * PI / 180.0;

//#region 🔖️Hosts
/// 🪝️ The inferred geometry a railing is hosted by.
#[derive(Clone, Copy, Debug)]
pub enum Host<'a> {
    Stair(&'a StairRun),
    Ramp { ramp: &'a Ramp, run: &'a RampRun },
    Slab { slab: &'a Slab, own: &'a StoreyLevel },
}

/// ⚠️ Why a host yields no rail path: the slab has no such edge, or the edge is curved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostFault {
    Edge,
}
//#endregion 🔖️Hosts

//#region 🔖️Stairs
fn at(origin: (f64, f64), angle: f64, along: f64, aside: f64) -> (f64, f64) {
    let (sin, cos) = angle.sin_cos();
    (origin.0 + along * cos - aside * sin, origin.1 + along * sin + aside * cos)
}

fn perimeter_parameter(a: f64, b: f64, p: (f64, f64)) -> f64 {
    let edges = [((-a, -b), (a, -b), 0.0), ((a, -b), (a, b), 2.0 * a), ((a, b), (-a, b), 2.0 * a + 2.0 * b), ((-a, b), (-a, -b), 4.0 * a + 2.0 * b)];
    edges
        .iter()
        .map(|&(from, to, offset)| {
            let (dx, dy) = (to.0 - from.0, to.1 - from.1);
            let length2 = dx * dx + dy * dy;
            let t = if length2 > EPS { (((p.0 - from.0) * dx + (p.1 - from.1) * dy) / length2).clamp(0.0, 1.0) } else { 0.0 };
            let (cx, cy) = (from.0 + dx * t, from.1 + dy * t);
            ((p.0 - cx).hypot(p.1 - cy), offset + t * length2.sqrt())
        })
        .min_by(|x, y| x.0.total_cmp(&y.0))
        .map_or(0.0, |best| best.1)
}

/// 🟫️ The corners of the boundary of a landing, shrunk by `inset`, that a rail visits between the point where it arrives and the point where it leaves: counter-clockwise on the right side of the stair, clockwise on the left; none when both points coincide.
pub fn landing_walk(landing: &StairLanding, entry: (f64, f64), exit: (f64, f64), side: HostSide, inset: f64) -> Vec<(f64, f64)> {
    let (a, b) = ((landing.depth / 2.0 - inset).max(0.0), (landing.width / 2.0 - inset).max(0.0));
    let centre = (landing.centre.x, landing.centre.y);
    let local = |p: (f64, f64)| {
        let (sin, cos) = landing.direction.sin_cos();
        let (dx, dy) = (p.0 - centre.0, p.1 - centre.1);
        (dx * cos + dy * sin, -dx * sin + dy * cos)
    };
    let perimeter = 4.0 * a + 4.0 * b;
    if perimeter <= EPS {
        return Vec::new();
    }
    let (from, to) = (perimeter_parameter(a, b, local(entry)), perimeter_parameter(a, b, local(exit)));
    let corners = [((-a, -b), 0.0), ((a, -b), 2.0 * a), ((a, b), 2.0 * a + 2.0 * b), ((-a, b), 4.0 * a + 2.0 * b)];
    let counter_clockwise = side == HostSide::Right;
    let travel = |origin: f64, target: f64| if counter_clockwise { (target - origin).rem_euclid(perimeter) } else { (origin - target).rem_euclid(perimeter) };
    let span = travel(from, to);
    if span <= EPS || perimeter - span <= EPS {
        return Vec::new();
    }
    let mut passed: Vec<(f64, (f64, f64))> = corners.iter().map(|&(corner, parameter)| (travel(from, parameter), corner)).filter(|(distance, _)| *distance > EPS && *distance < span - EPS).collect();
    passed.sort_by(|x, y| x.0.total_cmp(&y.0));
    passed
        .into_iter()
        .map(|(_, corner)| {
            let (sin, cos) = landing.direction.sin_cos();
            (centre.0 + corner.0 * cos - corner.1 * sin, centre.1 + corner.0 * sin + corner.1 * cos)
        })
        .collect()
}

fn winder_points(winder: &StairWinder, riser_height: f64, treads: u32, base_z: f64, side: HostSide, inset: f64) -> Vec<RailPoint> {
    let inner = (side == HostSide::Left) == (winder.sweep > 0.0);
    let radius = if inner { winder.inner_radius + inset } else { (winder.outer_radius - inset).max(0.0) };
    let count = ((winder.sweep.abs() / WINDER_STEP).ceil() as usize).max(1);
    (0..=count)
        .map(|step| {
            let fraction = step as f64 / count as f64;
            let angle = winder.start_angle + winder.sweep * fraction;
            RailPoint { x: winder.centre.x + radius * angle.cos(), y: winder.centre.y + radius * angle.sin(), z: base_z + riser_height + fraction * f64::from(treads) * riser_height }
        })
        .collect()
}

fn stair_paths(run: &StairRun, side: HostSide, inset: f64, base_offset: f64) -> Vec<Vec<RailPoint>> {
    let reach = (run.width / 2.0 - inset).max(0.0);
    let aside = if side == HostSide::Left { reach } else { -reach };
    let rise = run.riser_height;
    let (mut paths, mut current): (Vec<Vec<RailPoint>>, Vec<RailPoint>) = (Vec::new(), Vec::new());
    for (index, flight) in run.flights.iter().enumerate() {
        match &flight.winder {
            Some(winder) => current.extend(winder_points(winder, rise, flight.treads, flight.base_z, side, inset).into_iter().map(|p| RailPoint { z: p.z + base_offset, ..p })),
            None => {
                let (start, end) = (at((flight.start.x, flight.start.y), flight.direction, 0.0, aside), at((flight.start.x, flight.start.y), flight.direction, flight.length, aside));
                let z0 = flight.base_z + rise;
                current.push(RailPoint { x: start.0, y: start.1, z: z0 + base_offset });
                current.push(RailPoint { x: end.0, y: end.1, z: z0 + f64::from(flight.treads) * rise + base_offset });
            }
        }
        let Some(next) = run.flights.get(index + 1) else { continue };
        let exit = at((next.start.x, next.start.y), next.direction, 0.0, aside);
        let z = run.landings.get(index).map_or(flight.base_z + f64::from(flight.risers) * rise, |landing| landing.z) + base_offset;
        if let (Some(landing), Some(last)) = (run.landings.get(index), current.last().copied()) {
            current.extend(landing_walk(landing, (last.x, last.y), exit, side, inset).into_iter().map(|(x, y)| RailPoint { x, y, z }));
        }
        current.push(RailPoint { x: exit.0, y: exit.1, z });
        paths.push(std::mem::take(&mut current));
    }
    if !current.is_empty() {
        paths.push(current);
    }
    paths
}
//#endregion 🔖️Stairs

//#region 🔖️Slabs
fn slab_path(slab: &Slab, own: &StoreyLevel, host: &RailingHost, base_offset: f64) -> Result<Vec<Vec<RailPoint>>, HostFault> {
    let count = slab.boundary.len();
    let index = host.edge as usize;
    if index >= count || slab.boundary[index].bulge != 0.0 {
        return Err(HostFault::Edge);
    }
    let (start, end) = (point(&slab.boundary[index].point), point(&slab.boundary[(index + 1) % count].point));
    let length = start.distance(end);
    if length <= EPS {
        return Err(HostFault::Edge);
    }
    let outline = bulged(&slab.boundary);
    let top = top_plane(slab, &outline, own.elevation + slab.offset);
    let (dx, dy) = ((end.x - start.x) / length, (end.y - start.y) / length);
    let inward = if loops::is_ccw(&outline) { (-dy, dx) } else { (dy, -dx) };
    let shift = if host.side == HostSide::Left { host.inset } else { -host.inset };
    let place = |p: semio_framework_geometry::Point| {
        let moved = semio_framework_geometry::Point::new(p.x + inward.0 * shift, p.y + inward.1 * shift);
        RailPoint { x: moved.x, y: moved.y, z: top.at(p) + base_offset }
    };
    Ok(vec![vec![place(start), place(end)]])
}
//#endregion 🔖️Slabs

//#region 🔖️Solid
/// 🪝️ The rail paths of a hosted railing over the geometry of its host: continuous paths of plan positions with the height of the surface plus the base offset of the railing.
pub fn host_paths(railing: &Railing, host_spec: &RailingHost, host: &Host<'_>) -> Result<Vec<Vec<RailPoint>>, HostFault> {
    match host {
        Host::Stair(run) => Ok(stair_paths(run, host_spec.side, host_spec.inset, railing.base_offset)),
        Host::Ramp { ramp, run } => {
            let stations = stations_of(&strip_of(ramp), run);
            let path = edge_path(&stations, ramp.width, host_spec.side == HostSide::Left, host_spec.inset);
            Ok(vec![path.into_iter().map(|p| RailPoint { z: p.z + railing.base_offset, ..p }).collect()])
        }
        Host::Slab { slab, own } => slab_path(slab, own, host_spec, railing.base_offset),
    }
}

/// 🛤️ The measures of the solid of a railing: its height, post spacing and the extents of its authored rail and post sections.
pub fn spec_of(railing: &Railing) -> RailSpec {
    RailSpec { height: railing.height, spacing: railing.post_spacing, rail: extents_of(&railing.profile), post: extents_of(&railing.post_profile) }
}

/// 🪝️ The solid of a hosted railing; absent when the host yields no path.
pub fn hosted_solid(railing: &Railing, host: Option<&Host<'_>>) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Railing);
    if let (Some(spec), Some(host)) = (railing.host.as_ref(), host) {
        if let Ok(paths) = host_paths(railing, spec, host) {
            let (posts, rails) = railing_meshes(&paths, &spec_of(railing));
            builder.add(parts::POST, &railing.material, 0, &posts).add(parts::RAIL, &railing.material, 0, &rails);
        }
    }
    builder.build()
}

/// 📏️ The length of the rail along its paths, measured in three dimensions.
pub fn length_of(paths: &[Vec<RailPoint>]) -> f64 {
    paths.iter().flat_map(|path| path.windows(2)).map(|pair| (pair[1].x - pair[0].x).hypot(pair[1].y - pair[0].y).hypot(pair[1].z - pair[0].z)).sum()
}

/// 🔑️ What the solid and the quantity of a hosted railing read of its host besides the run in their parents: the record of a ramp or slab host (a stair host is its run).
pub fn dependency(snapshot: &ModelSnapshot, railing: &Railing) -> DslValue {
    let Some(host) = railing.host.as_ref() else { return DslValue::Null };
    match (snapshot.ramps.get(&host.element), snapshot.slabs.get(&host.element)) {
        (Some(ramp), _) => dep_value(&ramp.anonymous()),
        (None, Some(slab)) => dep_value(&slab.anonymous()),
        (None, None) => DslValue::Null,
    }
}

/// 🪝️ Whether a hosted railing cannot follow its host: the host is a slab edge that does not exist or is curved.
pub fn unresolved(railing: &Railing, slab: Option<&Slab>) -> bool {
    match (railing.host.as_ref(), slab) {
        (Some(host), Some(slab)) => {
            let count = slab.boundary.len();
            let index = host.edge as usize;
            index >= count || slab.boundary[index].bulge != 0.0 || point(&slab.boundary[index].point).distance(point(&slab.boundary[(index + 1) % count].point)) <= EPS
        }
        _ => false,
    }
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
