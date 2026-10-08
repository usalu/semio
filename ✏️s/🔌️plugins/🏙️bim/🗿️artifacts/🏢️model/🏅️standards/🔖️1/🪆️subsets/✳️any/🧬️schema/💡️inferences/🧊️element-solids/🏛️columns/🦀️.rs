//! 🏛️ `columns`: the solid of every column, its profile (rectangle, circle, I-shape or custom outline) swept vertically from the resolved base to the resolved top.
//!
//! A column stores its position, rotation, base offset and a `TopConstraint`; its height is never stored. The height resolves from the storey levels exactly like a wall
//! (`StoreyTop` follows the own storey, `Storey` the constrained storey, `Unconnected` is free), so a storey height edit re-infers exactly the columns that follow it.
//! The profile is centred on the column position with its width along the rotated `+X`; beams reuse [`profile_loop`].

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged, placed, point, rectangle};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{vertical_of, StoreyLevel};
use crate::{Column, ModelSnapshot, Profile};
use semio_framework_geometry::loops::Vertex;
use semio_framework_geometry::mesh::{extrude_loops, TriMesh};
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

/// 🏛️ The geometry of a column from the levels of the storeys it is resolved by; a column that does not rise (top at or below base) has an empty mesh.
pub fn column_geometry(column: &Column, profile: &Profile, own: &StoreyLevel, target: Option<&StoreyLevel>) -> ColumnGeometry {
    let (base_z, top_z) = vertical_of(column.base_offset, &column.top, own, target);
    let outline = profile_loop(profile);
    let mesh = if top_z - base_z > 1e-9 && outline.len() >= 2 { extrude_loops(&placed(&outline, point(&column.position), column.rotation), &[], CHORD_TOLERANCE, ZPlane::flat(base_z), ZPlane::flat(top_z)) } else { TriMesh::new() };
    ColumnGeometry { base_z, top_z, mesh }
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🏛️ The solid of a column from the levels of the storeys it is resolved by; absent without a type.
pub fn column_solid(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Column);
    if let Some(kind) = snapshot.column_types.get(&column.column_type) {
        builder.add(parts::BODY, &kind.material, 0, &column_geometry(column, &kind.profile, own, target).mesh);
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
