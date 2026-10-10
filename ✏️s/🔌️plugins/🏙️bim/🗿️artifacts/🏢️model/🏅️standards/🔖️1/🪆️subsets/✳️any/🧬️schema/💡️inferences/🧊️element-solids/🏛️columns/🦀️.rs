//! 🏛️ `columns`: the solid of every column, its profile (rectangle, circle, I-shape or custom outline) swept vertically from the resolved base to the resolved top, or, for a leaning column, along its tilted axis.
//!
//! A column stores its position, rotation, tilt, base offset and a `TopConstraint`; its height is never stored. The height resolves from the storey levels exactly like a wall
//! (`StoreyTop` follows the own storey, `Storey` the constrained storey, `Unconnected` is free), so a storey height edit re-infers exactly the columns that follow it.
//! The profile is centred on the column position with its width along the rotated `+X`; beams reuse [`profile_loop`]. A tilt leans the top of the column towards its direction
//! by an angle from the vertical about the base point; the profile is the cross-section perpendicular to the leaning axis and both ends are cut by the horizontal planes of the
//! base and the top: the horizontal section is the profile stretched by `1 / cos(angle)` along the lean, the top section is the base section moved by `rise * tan(angle)` along it, and
//! the solid is the sheared prism between them (volume `area * rise / cos(angle)`).

use crate::standards::v1::subsets::any::schema::inferences::families::FamilyProfiles;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged, placed, point, rectangle};
use crate::standards::v1::subsets::any::schema::authored::profile::profile_polygon;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{vertical_of, StoreyLevel};
use crate::{Column, ModelSnapshot, Profile, Slope};
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::mesh::{extrude_loops, prism_between, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Geometry

/// 🔷️ The centred cross-section of a profile as a bulged loop in `(x, y)`; an invalid profile yields an empty loop.
pub fn profile_loop(profile: &Profile) -> Vec<Vertex> {
    match profile {
        Profile::Rectangle { width, depth } if *width > 0.0 && *depth > 0.0 => rectangle(*width, *depth),
        Profile::Circle { diameter } if *diameter > 0.0 => {
            let radius = diameter / 2.0;
            vec![Vertex::new(Point::new(radius, 0.0), 1.0), Vertex::new(Point::new(-radius, 0.0), 1.0)]
        }
        Profile::IShape { width, depth, web, flange } if *width > 0.0 && *depth > 0.0 && *web > 0.0 && *flange > 0.0 && web < width && 2.0 * flange < *depth => {
            let (x, y, w, f) = (width / 2.0, depth / 2.0, web / 2.0, flange);
            [(-x, -y), (x, -y), (x, -y + f), (w, -y + f), (w, y - f), (x, y - f), (x, y), (-x, y), (-x, y - f), (-w, y - f), (-w, -y + f), (-x, -y + f)].iter().map(|&(px, py)| Vertex::corner(px, py)).collect()
        }
        Profile::Custom { outline } => bulged(outline),
        _ => Vec::new(),
    }
}

/// 📏️ The resolved vertical extent and the mesh of one column.
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnGeometry {
    pub base_z: f64,
    pub top_z: f64,
    pub mesh: TriMesh,
}

/// 🧭️ The lean of a tilt: the unit direction in plan, `1 / cos(angle)` and `tan(angle)`.
fn lean(tilt: &Slope) -> (Point, f64, f64) {
    (Point::new(tilt.direction.cos(), tilt.direction.sin()), 1.0 / tilt.angle.cos(), tilt.angle.tan())
}

/// 🔷️ The horizontal section of a column at the height `z`, a counter-clockwise ring in plan: the profile placed at the column and, for a tilted column, stretched by `1 / cos(angle)` along the lean
/// and moved by `(z - base_z) * tan(angle)` along it. A plumb column has the same section at every height.
pub fn footprint(column: &Column, profile: &Profile, base_z: f64, z: f64) -> Vec<Point> {
    let outline = profile_loop(profile);
    let ring = loops::flatten(&placed(&outline, point(&column.position), column.rotation), CHORD_TOLERANCE);
    let Some(tilt) = column.tilt.as_ref().filter(|tilt| tilt.angle.is_finite() && tilt.direction.is_finite() && tilt.angle.abs() < std::f64::consts::FRAC_PI_2) else { return ring };
    let (u, stretch, slope) = lean(tilt);
    let (centre, shift) = (point(&column.position), (z - base_z) * slope);
    ring.into_iter()
        .map(|p| {
            let (dx, dy) = (p.x - centre.x, p.y - centre.y);
            let along = dx * u.x + dy * u.y;
            let (wx, wy) = (dx - along * u.x, dy - along * u.y);
            Point::new(centre.x + wx + (along * stretch + shift) * u.x, centre.y + wy + (along * stretch + shift) * u.y)
        })
        .collect()
}

/// 🏛️ The geometry of a column from the levels of the storeys it is resolved by; a column that does not rise (top at or below base) has an empty mesh.
pub fn column_geometry(column: &Column, profile: &Profile, own: &StoreyLevel, target: Option<&StoreyLevel>) -> ColumnGeometry {
    let (base_z, top_z) = vertical_of(column.base_offset, &column.top, own, target);
    let outline = profile_loop(profile);
    let rises = top_z - base_z > 1e-9 && outline.len() >= 2;
    let mesh = match (rises, column.tilt.as_ref().filter(|tilt| tilt.angle.is_finite() && tilt.direction.is_finite() && tilt.angle.abs() < std::f64::consts::FRAC_PI_2)) {
        (false, _) => TriMesh::new(),
        (true, None) => extrude_loops(&placed(&outline, point(&column.position), column.rotation), &[], CHORD_TOLERANCE, ZPlane::flat(base_z), ZPlane::flat(top_z)),
        (true, Some(_)) => {
            let (lower, upper) = (footprint(column, profile, base_z, base_z), footprint(column, profile, base_z, top_z));
            let ring = |points: &[Point], z: f64| points.iter().map(|p| [p.x, p.y, z]).collect::<Vec<_>>();
            let mesh = prism_between(&ring(&lower, base_z), &ring(&upper, top_z));
            if outline.iter().any(|vertex| vertex.bulge != 0.0) {
                mesh.crease_normals(35f64.to_radians())
            } else {
                mesh
            }
        }
    };
    ColumnGeometry { base_z, top_z, mesh }
}

/// 🔗️ A column that a beam can join: the column, its profile and its resolved vertical extent.
#[derive(Clone, Copy, Debug)]
pub struct Joiner<'a> {
    pub column: &'a Column,
    pub profile: &'a Profile,
    pub base_z: f64,
    pub top_z: f64,
}

/// 📏️ How far from its position a column can reach in plan (the largest distance of its profile outline, plus the most a lean can move the section within the tallest storey assumed), none without a profile.
pub fn reach(snapshot: &ModelSnapshot, column: &Column) -> Option<f64> {
    let kind = snapshot.column_types.get(&column.column_type)?;
    let radius = profile_polygon(&kind.profile).iter().map(|p| p.x.hypot(p.y)).fold(0.0, f64::max);
    let stretch = column.tilt.as_ref().map_or(0.0, |tilt| if tilt.angle.is_finite() { radius * (1.0 / tilt.angle.cos().max(0.5) - 1.0) + 20.0 * tilt.angle.tan().abs().min(100.0) } else { 0.0 });
    Some(radius + stretch)
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🏛️ The solid of a column from the levels of the storeys it is resolved by; absent without a type.
pub fn column_solid(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>) -> ElementSolid {
    column_solid_in(snapshot, column, own, target, &FamilyProfiles::new())
}

/// 🏛️ [`column_solid`] where a type that names a profile family gets that family's outline from `profiles`.
pub fn column_solid_in(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>, profiles: &FamilyProfiles<'_>) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Column);
    if let Some(kind) = snapshot.column_types.get(&column.column_type) {
        builder.add(parts::BODY, &kind.material, 0, &column_geometry(column, &profiles.resolve(&kind.profile), own, target).mesh);
    }
    builder.build()
}

/// 🔑️ What `column_solid` reads besides the levels: the column record and its type.
pub fn dependency(snapshot: &ModelSnapshot, column: &Column) -> DslValue {
    dep_object([("column", dep_value(column)), ("type", dep_value(&snapshot.column_types.get(&column.column_type).cloned()))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
