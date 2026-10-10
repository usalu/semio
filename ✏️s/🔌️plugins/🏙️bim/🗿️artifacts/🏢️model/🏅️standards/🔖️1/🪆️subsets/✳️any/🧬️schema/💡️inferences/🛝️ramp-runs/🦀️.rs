//! 🛝️ `ramp-runs`: the resolved run of every ramp. A ramp stores its centre-line path (vertices with bulges), width, the lengths of its flat landings, the slope limit, thickness, material, base offset, top constraint
//! and which sides carry a railing; the rise, the length, the sloped run, the slope, the landings, the flights and the compliance are derived here. Its parents are the storey nodes it is resolved by (own
//! storey and the target of a `TopConstraint::Storey`), so one storey height edit re-infers exactly the ramps that stand on or reach that storey.
//!
//! The path runs from the foot (height `base`) to the head (height `base + rise`, the top constraint). Flat landings lie at the foot (`landing_start`), at the head (`landing_end`) and centred on every corner of
//! the path where its direction changes (`landing_turn`); overlapping landings merge and are clipped to the path. The rest of the path is sloped: the rise is spread evenly over the sloped length, so
//! `slope = |rise| / run_length`, the same on every flight. A ramp whose slope exceeds its limit, or which has a rise but no sloped length left, is not compliant.
//!
//! Related: accessibility ramps rise at most 1:12, <https://en.wikipedia.org/wiki/Wheelchair_ramp>.

use super::super::element_solids::plan_kit::point;
use super::super::storey_levels::{vertical_of, StoreyLevel};
use crate::{ModelSnapshot, Ramp};
use semio_framework_geometry::bulge::{corner_join, BulgeSeg};
use semio_framework_geometry::loops::Vertex as Corner;
use semio_framework_geometry::vector::cross;
use semio_framework_geometry::Point;
use std::collections::BTreeMap;

//#region 🔖️Values
/// 📏️ Slopes and rises below this are zero; the tolerance of every comparison with the slope limit.
pub const SLOPE_EPS: f64 = 1e-9;
/// 📏️ Lengths below this (metres) are no length.
pub const LENGTH_EPS: f64 = 1e-9;
/// 🔄️ A vertex turns the path when the tangents on both sides differ by more than this many radians; smaller turns are tangent-continuous joins.
pub const TURN_EPS: f64 = 1e-3;

/// 🛝️ One sloped stretch of the path between two landings (or the path ends): arc lengths `from` and `to` along the centre line, the heights at both ends.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RampFlight {
    pub from: f64,
    pub to: f64,
    pub length: f64,
    pub z_from: f64,
    pub z_to: f64,
}

/// 🟫️ One flat stretch of the path: arc lengths `from` and `to` along the centre line and its height.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RampLanding {
    pub from: f64,
    pub to: f64,
    pub length: f64,
    pub z: f64,
}

/// 🚦️ The code flags of one ramp run: it has a sloped run when it has a rise, the slope is within the limit, and both together.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RampCompliance {
    pub run_ok: bool,
    pub slope_ok: bool,
    pub compliant: bool,
}

/// 🛝️ Resolved run of one ramp, in metres and radians: `length` the whole path, `run_length` its sloped part, `slope = |rise| / run_length`, `angle = atan(slope)`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RampRun {
    pub base_z: f64,
    pub top_z: f64,
    pub rise: f64,
    pub length: f64,
    pub run_length: f64,
    pub slope: f64,
    pub angle: f64,
    pub width: f64,
    pub flights: Vec<RampFlight>,
    pub landings: Vec<RampLanding>,
    pub compliance: RampCompliance,
}

impl RampRun {
    /// 📐️ The height of the walking surface on the centre line at arc length `s` (clamped to the path): the foot plus the rise in proportion to the sloped length passed so far.
    pub fn z_at(&self, s: f64) -> f64 {
        if self.run_length <= LENGTH_EPS {
            return self.base_z + if self.length > LENGTH_EPS { self.rise * (s / self.length).clamp(0.0, 1.0) } else { 0.0 };
        }
        let passed: f64 = self.flights.iter().map(|flight| (s.min(flight.to) - flight.from).max(0.0)).sum();
        self.base_z + self.rise * (passed / self.run_length).clamp(0.0, 1.0)
    }
}
//#endregion 🔖️Values

//#region 🔖️Path
/// 〰️ The segments of an authored path: consecutive vertices, a vertex that repeats the previous one adds nothing.
pub fn segments_of(ramp: &Ramp) -> Vec<BulgeSeg> {
    ramp.path
        .windows(2)
        .filter_map(|pair| {
            let (start, end) = (point(&pair[0].point), point(&pair[1].point));
            (start.distance(end) > LENGTH_EPS).then(|| BulgeSeg::new(start, end, pair[0].bulge))
        })
        .collect()
}

/// 📏️ The arc length at which every segment starts, and the whole length.
pub fn starts_of(segments: &[BulgeSeg]) -> (Vec<f64>, f64) {
    let mut at = 0.0;
    let starts = segments
        .iter()
        .map(|segment| {
            let start = at;
            at += segment.length();
            start
        })
        .collect();
    (starts, at)
}

fn corners_of(segments: &[BulgeSeg], starts: &[f64]) -> Vec<f64> {
    (1..segments.len())
        .filter(|&index| {
            let (before, after) = (segments[index - 1].tangent_at(1.0), segments[index].tangent_at(0.0));
            cross(before, after).atan2(before.dot(after)).abs() > TURN_EPS
        })
        .map(|index| starts[index])
        .collect()
}

fn flats(ramp: &Ramp, length: f64, corners: &[f64]) -> Vec<(f64, f64)> {
    let mut raw = vec![(0.0, ramp.landing_start), (length - ramp.landing_end, length)];
    raw.extend(corners.iter().map(|corner| (corner - ramp.landing_turn / 2.0, corner + ramp.landing_turn / 2.0)));
    let mut clipped: Vec<(f64, f64)> = raw.into_iter().map(|(from, to)| (from.max(0.0), to.min(length))).filter(|(from, to)| to - from > LENGTH_EPS).collect();
    clipped.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut merged: Vec<(f64, f64)> = Vec::new();
    for (from, to) in clipped {
        match merged.last_mut() {
            Some(last) if from <= last.1 + LENGTH_EPS => last.1 = last.1.max(to),
            _ => merged.push((from, to)),
        }
    }
    merged
}
//#endregion 🔖️Path

//#region 🔖️Run
/// 🧮️ The run of a ramp from the levels of the storeys it is resolved by.
pub fn run_of(ramp: &Ramp, own: &StoreyLevel, target: Option<&StoreyLevel>) -> RampRun {
    let (base_z, top_z) = vertical_of(ramp.base_offset, &ramp.top, own, target);
    let rise = top_z - base_z;
    let segments = segments_of(ramp);
    let (starts, length) = starts_of(&segments);
    let corners = corners_of(&segments, &starts);
    let landed = flats(ramp, length, &corners);
    let mut sloped: Vec<(f64, f64)> = Vec::new();
    let mut at = 0.0;
    for (from, to) in landed.iter().copied().chain(std::iter::once((length, length))) {
        if from - at > LENGTH_EPS {
            sloped.push((at, from));
        }
        at = at.max(to);
    }
    let run_length: f64 = sloped.iter().map(|(from, to)| to - from).sum();
    let mut run = RampRun { base_z, top_z, rise, length, run_length, width: ramp.width, ..RampRun::default() };
    if run_length > LENGTH_EPS {
        run.slope = rise.abs() / run_length;
        run.angle = run.slope.atan();
    }
    let mut passed = 0.0;
    run.flights = sloped
        .iter()
        .map(|&(from, to)| {
            let z_from = base_z + if run_length > LENGTH_EPS { rise * passed / run_length } else { 0.0 };
            passed += to - from;
            RampFlight { from, to, length: to - from, z_from, z_to: base_z + if run_length > LENGTH_EPS { rise * passed / run_length } else { 0.0 } }
        })
        .collect();
    run.landings = landed.iter().map(|&(from, to)| RampLanding { from, to, length: to - from, z: run.z_at(from) }).collect();
    let run_ok = rise.abs() <= SLOPE_EPS || run_length > LENGTH_EPS;
    let slope_ok = run_ok && run.slope <= ramp.max_slope + SLOPE_EPS;
    run.compliance = RampCompliance { run_ok, slope_ok, compliant: slope_ok };
    run
}

/// 🔑️ What `run_of` reads of a ramp: its record.
pub fn dependency(ramp: &Ramp) -> semio_framework_value::DslValue {
    semio_framework_value::ToValue::to_value(ramp)
}
//#endregion 🔖️Run

//#region 🔖️Strip
/// ▭️ The plan strip of a ramp: the centre line, its left and right edges (offset by half the width, joined by miters at the corners) and the arc length at which every segment starts.
/// Empty when the path or an offset collapses.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RampStrip {
    pub centre: Vec<BulgeSeg>,
    pub left: Vec<BulgeSeg>,
    pub right: Vec<BulgeSeg>,
    pub starts: Vec<f64>,
    pub length: f64,
    pub half: f64,
}

/// ▭️ The strip of a ramp.
pub fn strip_of(ramp: &Ramp) -> RampStrip {
    let centre = segments_of(ramp);
    let half = ramp.width / 2.0;
    if centre.is_empty() || !(half > LENGTH_EPS) {
        return RampStrip::default();
    }
    let (starts, length) = starts_of(&centre);
    let (mut left, mut right) = (Vec::new(), Vec::new());
    for (index, segment) in centre.iter().enumerate() {
        let (Some(l), Some(r)) = (segment.offset(half), segment.offset(-half)) else { return RampStrip::default() };
        let before = (index > 0).then(|| corner_join(&centre[index - 1], (half, half), segment, (half, half)));
        let after = centre.get(index + 1).map(|next| corner_join(segment, (half, half), next, (half, half)));
        left.push(l.retarget(before.and_then(|corner| corner.left).unwrap_or(l.start), after.and_then(|corner| corner.left).unwrap_or(l.end)));
        right.push(r.retarget(before.and_then(|corner| corner.right).unwrap_or(r.start), after.and_then(|corner| corner.right).unwrap_or(r.end)));
    }
    RampStrip { centre, left, right, starts, length, half }
}

impl RampStrip {
    /// 🕳️ Whether the strip has no geometry.
    pub fn is_empty(&self) -> bool {
        self.centre.is_empty()
    }

    /// 🔷️ The closed counter-clockwise outline with bulges: the right edge forward, the cap, the left edge backward, the cap.
    pub fn outline(&self) -> Vec<Corner> {
        let (Some(right_last), Some(left_first)) = (self.right.last(), self.left.first()) else { return Vec::new() };
        let mut outline: Vec<Corner> = self.right.iter().map(|edge| Corner::new(edge.start, edge.bulge)).collect();
        outline.push(Corner::new(right_last.end, 0.0));
        outline.extend(self.left.iter().rev().map(|edge| edge.reversed()).map(|edge| Corner::new(edge.start, edge.bulge)));
        outline.push(Corner::new(left_first.start, 0.0));
        outline
    }

    /// 📍️ The segment index and the parameter of the centre line at arc length `s` (clamped to the path).
    pub fn locate(&self, s: f64) -> (usize, f64) {
        let s = s.clamp(0.0, self.length);
        let index = self.starts.iter().rposition(|start| *start <= s + LENGTH_EPS).unwrap_or(0).min(self.centre.len().saturating_sub(1));
        let length = self.centre[index].length();
        (index, if length > LENGTH_EPS { ((s - self.starts[index]) / length).clamp(0.0, 1.0) } else { 0.0 })
    }

    /// 📍️ The left and right edge points and the centre point at arc length `s`; the ends of a segment use the mitred corner points.
    pub fn section_at(&self, s: f64) -> (Point, Point, Point) {
        let (index, t) = self.locate(s);
        let (centre, left, right) = (&self.centre[index], &self.left[index], &self.right[index]);
        if t <= 0.0 {
            return (left.start, right.start, centre.start);
        }
        if t >= 1.0 {
            return (left.end, right.end, centre.end);
        }
        let tangent = centre.tangent_at(t);
        let normal = semio_framework_geometry::vector::perp(tangent);
        let middle = centre.point_at(t);
        (middle + normal * self.half, middle - normal * self.half, middle)
    }
}
//#endregion 🔖️Strip

//#region 🔖️Projection
/// 🛝️ The run of every ramp (the `RampRun` nodes of the model graph).
#[cfg(test)]
pub fn compute_ramp_runs(snapshot: &ModelSnapshot) -> BTreeMap<String, RampRun> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::RAMP_RUNS }>(snapshot).ramp_runs)
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
