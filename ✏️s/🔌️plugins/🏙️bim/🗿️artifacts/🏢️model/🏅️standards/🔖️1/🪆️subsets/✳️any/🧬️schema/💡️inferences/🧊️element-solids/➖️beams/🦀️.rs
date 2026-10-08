//! ➖️ `beams`: the solid of every beam, its profile swept along the straight axis from `start` to `end`.
//!
//! The beam hangs below the top of its storey: the top of the profile lies at the storey top plus `top_offset`, so a storey height edit lifts the beam with the storey.
//! The profile is laid out like a column profile ([`profile_loop`]) with `x` read as the horizontal offset to the left of the direction of travel and `y` as the height,
//! and then dropped so that its highest point touches the beam top.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::columns::profile_loop;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::point;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{Beam, ModelSnapshot, Profile};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::loops;
use semio_framework_geometry::mesh::{sweep_profile, TriMesh};
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Geometry
/// 📏️ The resolved top, the axis length and the mesh of one beam.
#[derive(Clone, Debug, PartialEq)]
pub struct BeamGeometry {
    pub top_z: f64,
    pub length: f64,
    pub mesh: TriMesh,
}

/// ➖️ The sweep section `(left, up)` of a profile with its highest point on the reference line.
pub fn section_of(profile: &Profile) -> Vec<Point> {
    let ring = loops::flatten(&profile_loop(profile), CHORD_TOLERANCE);
    let top = ring.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    ring.into_iter().map(|p| Point::new(p.x, p.y - top)).collect()
}

/// ➖️ The geometry of a beam from the level of its storey; a zero-length beam has an empty mesh.
pub fn beam_geometry(beam: &Beam, profile: &Profile, own: &StoreyLevel) -> BeamGeometry {
    let top_z = own.top_elevation + beam.top_offset;
    let axis = BulgeSeg::line(point(&beam.start), point(&beam.end));
    let length = axis.length();
    let section = section_of(profile);
    let mesh = if length > 1e-9 && section.len() >= 3 { sweep_profile(&section, &[], &[axis], top_z, CHORD_TOLERANCE) } else { TriMesh::new() };
    BeamGeometry { top_z, length, mesh }
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// ➖️ The solid of a beam from the level of its storey; absent without a type.
pub fn beam_solid(snapshot: &ModelSnapshot, beam: &Beam, own: &StoreyLevel) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Beam);
    if let Some(kind) = snapshot.beam_types.get(&beam.beam_type) {
        builder.add(parts::BODY, &kind.material, 0, &beam_geometry(beam, &kind.profile, own).mesh);
    }
    builder.build()
}

/// 🔑️ What `beam_solid` reads besides the level: the beam record and its type.
pub fn dependency(snapshot: &ModelSnapshot, beam: &Beam) -> DslValue {
    dep_object([("beam", dep_value(beam)), ("type", dep_value(&snapshot.beam_types.get(&beam.beam_type).cloned()))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
