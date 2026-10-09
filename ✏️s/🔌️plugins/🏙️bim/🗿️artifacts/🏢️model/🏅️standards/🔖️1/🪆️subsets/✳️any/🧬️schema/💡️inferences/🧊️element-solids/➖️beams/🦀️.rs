//! ➖️ `beams`: the solid of every beam, its profile swept along the axis from `start` to `end`, a line or an arc, level or inclined, and trimmed to the faces of the columns it joins.
//!
//! The beam hangs below the top of its storey: the top of the profile lies at the storey top plus `top_offset` at the start of the axis and plus `end_top_offset` at its end
//! (the same offset when none is authored), linear in arc length, so a storey height edit lifts the beam with the storey. The profile is laid out like a column profile
//! ([`profile_loop`]) with `x` read as the horizontal offset to the left of the direction of travel and `y` as the height, and then dropped so that its highest point touches the beam top;
//! on an inclined beam the section stays perpendicular to the climbing axis.
//!
//! A beam end that lies inside the section of a column of its storey (at mid-depth of the beam) is joined to it: the axis is cut back to the face where it leaves the column, so the beam
//! solid and the column solid share a face instead of overlapping. The joins are derived here from the authored columns, never stored.

use crate::standards::v1::subsets::any::schema::inferences::families::FamilyProfiles;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::columns::{footprint, profile_loop, reach, Joiner};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{point, seg};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, Anonymous, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{Beam, Column, ModelSnapshot, Profile};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::loops;
use semio_framework_geometry::mesh::{sweep_profile_ramped, TriMesh};
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Geometry
/// 📏️ The resolved tops, the axis length, the lengths cut off at both ends by joins and the mesh of one beam.
#[derive(Clone, Debug, PartialEq)]
pub struct BeamGeometry {
    pub top_z: f64,
    pub end_top_z: f64,
    pub length: f64,
    pub trimmed: [f64; 2],
    pub mesh: TriMesh,
}

/// ➖️ The sweep section `(left, up)` of a profile with its highest point on the reference line.
pub fn section_of(profile: &Profile) -> Vec<Point> {
    let ring = loops::flatten(&profile_loop(profile), CHORD_TOLERANCE);
    let top = ring.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    ring.into_iter().map(|p| Point::new(p.x, p.y - top)).collect()
}

/// 🔎️ Whether a point lies strictly inside a ring (even-odd rule, either orientation).
fn inside(at: Point, ring: &[Point]) -> bool {
    let mut crossings = false;
    let mut j = ring.len().wrapping_sub(1);
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[j]);
        if (a.y > at.y) != (b.y > at.y) && at.x < (b.x - a.x) * (at.y - a.y) / (b.y - a.y) + a.x {
            crossings = !crossings;
        }
        j = i;
    }
    crossings
}

/// ✂️ How far from one end along the axis the beam leaves every ring it starts inside, `0` when the end lies in none; the walk is sampled and refined by bisection.
fn cut_from(axis: &BulgeSeg, length: f64, from_start: bool, rings: &[Vec<Point>]) -> f64 {
    let at = |s: f64| axis.point_at_length(if from_start { s } else { length - s });
    let held = |s: f64| rings.iter().any(|ring| inside(at(s), ring));
    if length <= 1e-9 || !held(0.0) {
        return 0.0;
    }
    let step = (length / 400.0).max(1e-3).min(length);
    let mut s = 0.0;
    while s < length {
        let next = (s + step).min(length);
        if !held(next) {
            let (mut low, mut high) = (s, next);
            for _ in 0..40 {
                let middle = (low + high) / 2.0;
                if held(middle) {
                    low = middle;
                } else {
                    high = middle;
                }
            }
            return high;
        }
        s = next;
    }
    length
}

/// 🔗️ The lengths cut off the start and the end of the axis by the joiners whose vertical extent holds the mid-depth of the beam at that end. Joins that would swallow the beam are ignored.
pub fn trims(axis: &BulgeSeg, depth: f64, tops: [f64; 2], joiners: &[Joiner<'_>]) -> [f64; 2] {
    let length = axis.length();
    let rings = |top: f64| -> Vec<Vec<Point>> {
        let middle = top - depth / 2.0;
        joiners.iter().filter(|joiner| joiner.base_z - 1e-9 <= middle && middle <= joiner.top_z + 1e-9).map(|joiner| footprint(joiner.column, joiner.profile, joiner.base_z, middle)).collect()
    };
    let cut = [cut_from(axis, length, true, &rings(tops[0])), cut_from(axis, length, false, &rings(tops[1]))];
    if cut[0] + cut[1] >= length - 1e-6 {
        [0.0, 0.0]
    } else {
        cut
    }
}

/// ➖️ The geometry of a beam from the level of its storey and the columns it can join; a zero-length beam has an empty mesh.
pub fn beam_geometry(beam: &Beam, profile: &Profile, own: &StoreyLevel, joiners: &[Joiner<'_>]) -> BeamGeometry {
    let top_z = own.top_elevation + beam.top_offset;
    let end_top_z = own.top_elevation + beam.end_top_offset.unwrap_or(beam.top_offset);
    let axis = seg(&beam.axis);
    let length = axis.length();
    let section = section_of(profile);
    let depth = -section.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let trimmed = if length > 1e-9 && section.len() >= 3 { trims(&axis, depth, [top_z, end_top_z], joiners) } else { [0.0, 0.0] };
    let mesh = if length > 1e-9 && section.len() >= 3 {
        let (from, to) = (trimmed[0] / length, 1.0 - trimmed[1] / length);
        let kept = if trimmed == [0.0, 0.0] { axis } else { axis.subsegment(from, to) };
        let (z0, z1) = (top_z + (end_top_z - top_z) * from, top_z + (end_top_z - top_z) * to);
        sweep_profile_ramped(&section, &[], &[kept], z0, z1 - z0, CHORD_TOLERANCE)
    } else {
        TriMesh::new()
    };
    BeamGeometry { top_z, end_top_z, length, trimmed, mesh }
}

/// 🔗️ The columns of the storey of a beam whose reach touches an end of its axis: the candidates of its joins, from authored data only.
pub fn joining<'a>(snapshot: &'a ModelSnapshot, beam: &Beam) -> Vec<(&'a String, &'a Column)> {
    let axis = seg(&beam.axis);
    let ends = [axis.start, axis.end];
    snapshot
        .columns
        .iter()
        .filter(|(_, column)| column.storey == beam.storey)
        .filter(|(_, column)| reach(snapshot, column).is_some_and(|reach| ends.iter().any(|end| (point(&column.position) - *end).hypot() <= reach)))
        .collect()
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// ➖️ The solid of a beam from the level of its storey and the columns it joins; absent without a type.
pub fn beam_solid(snapshot: &ModelSnapshot, beam: &Beam, own: &StoreyLevel, joiners: &[Joiner<'_>]) -> ElementSolid {
    beam_solid_in(snapshot, beam, own, joiners, &FamilyProfiles::new())
}

/// ➖️ [`beam_solid`] where a type that names a profile family gets that family's outline from `profiles`.
pub fn beam_solid_in(snapshot: &ModelSnapshot, beam: &Beam, own: &StoreyLevel, joiners: &[Joiner<'_>], profiles: &FamilyProfiles<'_>) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Beam);
    if let Some(kind) = snapshot.beam_types.get(&beam.beam_type) {
        builder.add(parts::BODY, &kind.material, 0, &beam_geometry(beam, &profiles.resolve(&kind.profile), own, joiners).mesh);
    }
    builder.build()
}

/// 🔑️ What `beam_solid` reads besides the levels: the beam record, its type and the columns it can join with their types.
pub fn dependency(snapshot: &ModelSnapshot, beam: &Beam) -> DslValue {
    let columns = joining(snapshot, beam).into_iter().map(|(id, column)| (id.clone(), dep_object([("column", dep_value(&column.anonymous())), ("type", dep_value(&snapshot.column_types.get(&column.column_type).cloned()))])));
    dep_object([("beam", dep_value(beam)), ("type", dep_value(&snapshot.beam_types.get(&beam.beam_type).cloned())), ("columns", DslValue::object(columns))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
