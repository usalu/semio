//! 🪜️ `stairs`: the solid of every stair, straight, L-turn, U-turn or spiral, as monolithic steps from the storey floor up to the tread level of each step.
//!
//! The run of a stair (rise, riser count and height, tread, flights, landings) is the pure `stair_runs::run_of`, so the solid, the quantities and the code flags can never
//! disagree; this module only turns a [`StairRun`] into prisms. Tread `k` of a flight stands between the foot of riser `k` and the foot of riser `k + 1` and tops out at the
//! flight base plus `k + 1` risers; a landing is one prism up to its own height; a winder flight is one wedge per tread. A stair with a single riser has no tread and is absent.
//! Stringers and tread nosings are not modelled because the stair record carries no parameters for them.
//!
//! Related: <https://en.wikipedia.org/wiki/Stairs>.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{direction, point};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::{StairFlightRun, StairLanding, StairRun, StairWinder};
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::run_of;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::Stair;
use semio_framework_geometry::loops::Vertex;
use semio_framework_geometry::mesh::{extrude_loops, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::vector::perp;
use semio_framework_geometry::{Point, Vec2};
use semio_framework_value::DslValue;

//#region 🔖️Steps
/// 🪜️ One prism of a stair: its plan outline and the height of its top.
#[derive(Clone, Debug, PartialEq)]
pub struct Step {
    pub outline: Vec<Vertex>,
    pub top_z: f64,
}

struct Frame {
    origin: Point,
    axis: Vec2,
}

impl Frame {
    fn rectangle(&self, along: (f64, f64), across: (f64, f64)) -> Vec<Vertex> {
        let at = |x: f64, y: f64| self.origin + self.axis * x + perp(self.axis) * y;
        [(along.0, across.0), (along.1, across.0), (along.1, across.1), (along.0, across.1)].iter().map(|&(x, y)| Vertex::new(at(x, y), 0.0)).collect()
    }
}

fn wedge(winder: &StairWinder, from: f64, to: f64) -> Vec<Vertex> {
    let (centre, bulge) = (point(&winder.centre), ((to - from) / 4.0).tan());
    let at = |radius: f64, angle: f64| centre + direction(angle) * radius;
    if winder.inner_radius < 1e-9 {
        vec![Vertex::new(centre, 0.0), Vertex::new(at(winder.outer_radius, from), bulge), Vertex::new(at(winder.outer_radius, to), 0.0)]
    } else {
        vec![Vertex::new(at(winder.inner_radius, from), 0.0), Vertex::new(at(winder.outer_radius, from), bulge), Vertex::new(at(winder.outer_radius, to), 0.0), Vertex::new(at(winder.inner_radius, to), -bulge)]
    }
}

fn flight_steps(flight: &StairFlightRun, width: f64, riser_height: f64) -> Vec<Step> {
    let frame = Frame { origin: point(&flight.start), axis: direction(flight.direction) };
    (0..flight.treads)
        .map(|index| {
            let top_z = flight.base_z + f64::from(index + 1) * riser_height;
            let outline = match &flight.winder {
                Some(winder) => {
                    let delta = winder.sweep / f64::from(flight.treads);
                    wedge(winder, winder.start_angle + delta * f64::from(index), winder.start_angle + delta * f64::from(index + 1))
                }
                None => frame.rectangle((f64::from(index) * flight.tread, f64::from(index + 1) * flight.tread), (-width / 2.0, width / 2.0)),
            };
            Step { outline, top_z }
        })
        .collect()
}

fn landing_step(landing: &StairLanding) -> Step {
    let frame = Frame { origin: point(&landing.centre), axis: direction(landing.direction) };
    Step { outline: frame.rectangle((-landing.depth / 2.0, landing.depth / 2.0), (-landing.width / 2.0, landing.width / 2.0)), top_z: landing.z }
}

/// 🪜️ The prisms of a run: every plain tread or winder wedge of every flight, then every landing.
pub fn steps_of(run: &StairRun) -> Vec<Step> {
    if run.width < 1e-9 || run.riser_height < 1e-9 {
        return Vec::new();
    }
    run.flights.iter().flat_map(|flight| flight_steps(flight, run.width, run.riser_height)).chain(run.landings.iter().map(landing_step)).collect()
}
//#endregion 🔖️Steps

//#region 🔖️Geometry
/// 🪜️ The run and the mesh of one stair.
#[derive(Clone, Debug, PartialEq)]
pub struct StairGeometry {
    pub run: StairRun,
    pub steps: usize,
    pub mesh: TriMesh,
}

/// 🪜️ The geometry of a stair from the run the `StairRun` node resolved for it.
pub fn stair_geometry_of(run: StairRun) -> StairGeometry {
    let steps = steps_of(&run);
    let mut mesh = TriMesh::new();
    for step in &steps {
        mesh.append(&extrude_loops(&step.outline, &[], CHORD_TOLERANCE, ZPlane::flat(run.base_z), ZPlane::flat(step.top_z)));
    }
    StairGeometry { steps: steps.len(), run, mesh }
}

/// 🪜️ The geometry of a stair from the levels of the storeys it is resolved by (tests only: the graph passes the run of the `StairRun` node to [`stair_geometry_of`]).
#[cfg(test)]
pub fn stair_geometry(stair: &Stair, own: &StoreyLevel, target: Option<&StoreyLevel>) -> StairGeometry {
    stair_geometry_of(run_of(stair, own, target))
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🪜️ The solid of a stair from its run.
pub fn stair_solid(run: &StairRun) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Stair);
    builder.add(parts::STEP, "", 0, &stair_geometry_of(run.clone()).mesh);
    builder.build()
}

/// 🔑️ What `stair_solid` reads besides the run: nothing (the run carries the whole stair).
pub fn dependency(_stair: &Stair) -> DslValue {
    DslValue::Null
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
