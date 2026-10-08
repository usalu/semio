//! 🪜️ `stair-runs`: the resolved run of every stair. A stair stores its start, direction, width, flight kind, top constraint and the
//! comfort limits `max_riser` and `min_tread`; the rise, the riser count and height, the tread, the flights, the landings and the code
//! flags are derived here. Its parents are the storey nodes it is resolved by (own storey and the target of a `TopConstraint::Storey`),
//! so one storey height edit re-infers exactly the stairs that stand on or reach that storey.
//!
//! Placement: `start` is the middle of the foot edge of the first riser, `direction` the travel angle (counter-clockwise from +X). A
//! flight's riser `k` (0-based) stands at `start + k * tread` along the flight direction and rises `riser_height`; the last riser of the
//! last flight arrives on the upper floor, so a stair has `riser_count - 1` treads and landings count as one tread each.
//!
//! Related: Blondel's rule `2R + T`, <https://en.wikipedia.org/wiki/Stairs#Rise_and_run>.

use super::super::storey_levels::{vertical_of, StoreyLevel};
use crate::{ModelSnapshot, Point2, Stair, StairFlight, Turn};
use std::collections::BTreeMap;
use std::f64::consts::{FRAC_PI_2, PI};

//#region 🔖️Values
/// 📏️ Lower bound of Blondel's comfort rule `2 * riser + tread`, in metres.
pub const BLONDEL_MIN: f64 = 0.59;
/// 📏️ Upper bound of Blondel's comfort rule `2 * riser + tread`, in metres.
pub const BLONDEL_MAX: f64 = 0.65;
/// 📏️ Target of Blondel's rule used to size the tread: the middle of the comfort band.
pub const BLONDEL_TARGET: f64 = 0.62;
/// 🔢️ Hard cap of the riser count of one stair; a stair that needs more is reported with `riser_ok == false`.
pub const MAX_RISERS: u32 = 512;
const EPS: f64 = 1e-9;

/// 🌀️ The circular walking line of a winding flight (spiral stair).
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct StairWinder {
    pub centre: Point2,
    pub inner_radius: f64,
    pub outer_radius: f64,
    pub start_angle: f64,
    pub sweep: f64,
}

/// 🪜️ One flight: `risers` risers and `treads` plain treads, starting at the foot of riser `first_riser` (1-based). `length` is the horizontal distance
/// from the first to the last riser along the walking line; `tread` is the going per step on that line.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct StairFlightRun {
    pub first_riser: u32,
    pub risers: u32,
    pub treads: u32,
    pub start: Point2,
    pub direction: f64,
    pub tread: f64,
    pub base_z: f64,
    pub length: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub winder: Option<StairWinder>,
}

/// 🟫️ A landing: a `depth` by `width` rectangle centred on `centre`, its depth along `direction` (the direction of the arriving flight), at height `z`.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct StairLanding {
    pub after_flight: u32,
    pub z: f64,
    pub centre: Point2,
    pub direction: f64,
    pub width: f64,
    pub depth: f64,
}

/// 🚦️ The code flags of one stair run.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct StairCompliance {
    pub rise_positive: bool,
    pub riser_ok: bool,
    pub tread_ok: bool,
    pub blondel_ok: bool,
    pub compliant: bool,
}

/// 🪜️ Resolved run of one stair, in metres and radians.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct StairRun {
    pub base_z: f64,
    pub top_z: f64,
    pub rise: f64,
    pub riser_count: u32,
    pub riser_height: f64,
    pub tread_count: u32,
    pub tread: f64,
    pub stride: f64,
    pub width: f64,
    pub run_length: f64,
    pub flights: Vec<StairFlightRun>,
    pub landings: Vec<StairLanding>,
    pub compliance: StairCompliance,
}
//#endregion 🔖️Values

//#region 🔖️Geometry
fn along(origin: Point2, angle: f64, distance: f64) -> Point2 {
    Point2 { x: origin.x + distance * angle.cos(), y: origin.y + distance * angle.sin() }
}

fn aside(origin: Point2, angle: f64, distance: f64) -> Point2 {
    along(origin, angle + FRAC_PI_2, distance)
}

/// 🔢️ The riser count for a rise: the fewest equal risers that stay within `max_riser`.
pub fn riser_count_for(rise: f64, max_riser: f64) -> (u32, bool) {
    if !(rise > EPS && max_riser.is_finite() && max_riser > EPS) {
        return (0, false);
    }
    let wanted = (rise / max_riser - EPS).ceil().max(1.0);
    if wanted > f64::from(MAX_RISERS) {
        (MAX_RISERS, false)
    } else {
        (wanted as u32, true)
    }
}

/// 📏️ The tread that satisfies Blondel's rule for a riser height, never below `min_tread`.
pub fn tread_for(riser_height: f64, min_tread: f64) -> f64 {
    (BLONDEL_TARGET - 2.0 * riser_height).max(min_tread).max(0.0)
}

fn flight(first_riser: u32, risers: u32, start: Point2, direction: f64, tread: f64, riser_height: f64, base_z: f64) -> StairFlightRun {
    let treads = risers.saturating_sub(1);
    StairFlightRun { first_riser, risers, treads, start, direction, tread, base_z: base_z + f64::from(first_riser - 1) * riser_height, length: f64::from(treads) * tread, winder: None }
}

fn layout(stair: &Stair, count: u32, riser_height: f64, tread: f64, base_z: f64) -> (Vec<StairFlightRun>, Vec<StairLanding>) {
    if count == 0 {
        return (Vec::new(), Vec::new());
    }
    let (start, direction, width) = (stair.start, stair.direction, stair.width);
    let single = || (vec![flight(1, count, start, direction, tread, riser_height, base_z)], Vec::new());
    match &stair.flight {
        StairFlight::Straight => single(),
        StairFlight::LTurn { split, turn } if count >= 2 => {
            let fraction = if split.is_finite() { *split } else { 0.5 };
            let first = ((fraction * f64::from(count)).round().clamp(1.0, f64::from(count - 1))) as u32;
            let sign = if *turn == Turn::Left { 1.0 } else { -1.0 };
            let foot = along(start, direction, f64::from(first - 1) * tread);
            let centre = along(foot, direction, width / 2.0);
            let turned = direction + sign * FRAC_PI_2;
            let second_start = along(centre, turned, width / 2.0);
            let landing = StairLanding { after_flight: 0, z: base_z + f64::from(first) * riser_height, centre, direction, width, depth: width };
            (vec![flight(1, first, start, direction, tread, riser_height, base_z), flight(first + 1, count - first, second_start, turned, tread, riser_height, base_z)], vec![landing])
        }
        StairFlight::UTurn { gap } if count >= 2 => {
            let gap = if gap.is_finite() { gap.max(0.0) } else { 0.0 };
            let first = count.div_ceil(2);
            let foot = along(start, direction, f64::from(first - 1) * tread);
            let second_start = aside(foot, direction, width + gap);
            let centre = aside(along(foot, direction, width / 2.0), direction, (width + gap) / 2.0);
            let landing = StairLanding { after_flight: 0, z: base_z + f64::from(first) * riser_height, centre, direction, width: 2.0 * width + gap, depth: width };
            (vec![flight(1, first, start, direction, tread, riser_height, base_z), flight(first + 1, count - first, second_start, direction + PI, tread, riser_height, base_z)], vec![landing])
        }
        StairFlight::Spiral { radius, sweep } => {
            let outer = radius.max(width);
            let walking = outer - width / 2.0;
            let sign = if *sweep < 0.0 { -1.0 } else { 1.0 };
            let centre = aside(start, direction, sign * walking);
            let treads = count - 1;
            let arc = if treads == 0 { 0.0 } else { sweep.abs() * walking / f64::from(treads) };
            let mut spiral = flight(1, count, start, direction, arc, riser_height, base_z);
            spiral.winder = Some(StairWinder { centre, inner_radius: (outer - width).max(0.0), outer_radius: outer, start_angle: direction - sign * FRAC_PI_2, sweep: *sweep });
            (vec![spiral], Vec::new())
        }
        _ => single(),
    }
}

/// 🧮️ The run of a stair from the levels of the storeys it is resolved by.
pub fn run_of(stair: &Stair, own: &StoreyLevel, target: Option<&StoreyLevel>) -> StairRun {
    let (base_z, top_z) = vertical_of(0.0, &stair.top, own, target);
    let rise = top_z - base_z;
    let (count, within_cap) = riser_count_for(rise, stair.max_riser);
    let riser_height = if count == 0 { 0.0 } else { rise / f64::from(count) };
    let designed = tread_for(riser_height, stair.min_tread);
    let (flights, landings) = layout(stair, count, riser_height, designed, base_z);
    let tread = match &stair.flight {
        StairFlight::Spiral { .. } => flights.first().map_or(designed, |run| run.tread),
        _ => designed,
    };
    let stride = 2.0 * riser_height + tread;
    let rise_positive = rise > EPS;
    let riser_ok = rise_positive && within_cap && riser_height <= stair.max_riser + EPS;
    let tread_ok = count <= 1 || tread >= stair.min_tread - EPS;
    let blondel_ok = count <= 1 || (BLONDEL_MIN - EPS..=BLONDEL_MAX + EPS).contains(&stride);
    StairRun {
        base_z,
        top_z,
        rise,
        riser_count: count,
        riser_height,
        tread_count: count.saturating_sub(1),
        tread,
        stride,
        width: stair.width,
        run_length: flights.iter().map(|run| run.length).sum(),
        flights,
        landings,
        compliance: StairCompliance { rise_positive, riser_ok, tread_ok, blondel_ok, compliant: rise_positive && riser_ok && tread_ok && blondel_ok },
    }
}

/// 🔑️ What `run_of` reads of a stair: its record.
pub fn dependency(stair: &Stair) -> semio_framework_value::DslValue {
    semio_framework_value::ToValue::to_value(stair)
}
//#endregion 🔖️Geometry

//#region 🔖️Projection


/// 🪜️ The run of every stair (the `StairRun` nodes of the model graph).
pub fn compute_stair_runs(snapshot: &ModelSnapshot) -> BTreeMap<String, StairRun> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::RUNS }>(snapshot).stair_runs)
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
