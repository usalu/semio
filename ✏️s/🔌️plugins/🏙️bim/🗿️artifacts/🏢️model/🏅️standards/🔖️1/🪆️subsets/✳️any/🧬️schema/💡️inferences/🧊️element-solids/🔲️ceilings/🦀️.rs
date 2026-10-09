//! 🔲️ `ceilings`: the solid of every ceiling, its boundary loop (straight and bulged edges) with holes extruded downward by the layers of its ceiling type.
//!
//! The top of the ceiling hangs `offset` metres below the top of its storey; the layers stack downward from there, first layer topmost (`layer` 0), each with its own material and
//! thickness, so the thickness is the sum of the layer thicknesses measured vertically. A slope tilts the whole ceiling plane: `Slope.direction` is the direction the ceiling falls
//! (radians counter-clockwise from `+X`), `Slope.angle` the fall angle; the top plane keeps its drop at the uphill edge of the boundary, so the ceiling never rises above
//! `storey top - offset`. A ceiling with no layers or an unknown type is absent.
//!
//! The same plane gives the clear height of the rooms below: [`underside_at`] is the z of the visible underside at a plan point, absent where the ceiling has a hole or no extent.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged, direction, layer_thicknesses, stack};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{Ceiling, CeilingType, ModelSnapshot, Point2, Slope};
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::mesh::{extrude_loops, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Geometry
/// 📏️ The top plane of a ceiling whose reference height is `z`: horizontal, or tilted by its slope about the uphill edge of the boundary.
pub fn top_plane(slope: Option<Slope>, outline: &[Vertex], z: f64) -> ZPlane {
    match slope {
        Some(slope) if slope.angle.abs() > 1e-12 => {
            let fall = direction(slope.direction);
            let anchor = loops::flatten(outline, CHORD_TOLERANCE).into_iter().min_by(|a, b| (a.x * fall.x + a.y * fall.y).total_cmp(&(b.x * fall.x + b.y * fall.y)));
            anchor.map_or(ZPlane::flat(z), |anchor| ZPlane::sloped(anchor, z, slope.direction, -slope.angle.tan()))
        }
        _ => ZPlane::flat(z),
    }
}

/// 🔲️ One mesh per layer, topmost first, of a ceiling from the level of its storey.
pub fn ceiling_layers(ceiling: &Ceiling, kind: &CeilingType, own: &StoreyLevel) -> Vec<TriMesh> {
    let (outline, holes) = (bulged(&ceiling.boundary), ceiling.holes.iter().map(|hole| bulged(hole)).collect::<Vec<_>>());
    let top = top_plane(ceiling.slope, &outline, own.top_elevation - ceiling.offset);
    let thicknesses = layer_thicknesses(&kind.layers);
    stack(&thicknesses)
        .into_iter()
        .zip(&thicknesses)
        .map(|((from, to), thickness)| if *thickness > 1e-12 && outline.len() >= 3 { extrude_loops(&outline, &holes, CHORD_TOLERANCE, top.raised(-to), top.raised(-from)) } else { TriMesh::new() })
        .collect()
}

/// 🍰️ The thickness of a ceiling: the sum of the layers of its type, zero for an unknown type.
pub fn thickness(snapshot: &ModelSnapshot, ceiling: &Ceiling) -> f64 {
    snapshot.ceiling_types.get(&ceiling.ceiling_type).map_or(0.0, |kind| layer_thicknesses(&kind.layers).iter().sum())
}

/// 📉️ How far the lowest point of the top plane lies below the highest one: the extent of the flattened boundary along the fall direction times the tangent of the fall angle.
pub fn fall_of(ceiling: &Ceiling) -> f64 {
    ceiling.slope.map_or(0.0, |slope| {
        let fall = direction(slope.direction);
        let along: Vec<f64> = loops::flatten(&bulged(&ceiling.boundary), CHORD_TOLERANCE).iter().map(|point| point.x * fall.x + point.y * fall.y).collect();
        let (low, high) = along.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| (low.min(*value), high.max(*value)));
        if low.is_finite() {
            (high - low) * slope.angle.tan().abs()
        } else {
            0.0
        }
    })
}

/// 🪜️ The vertical span `(bottom, top)` of a ceiling in building coordinates: its top hangs `offset` below the storey top, the layers and a slope reach down from there.
pub fn span(snapshot: &ModelSnapshot, ceiling: &Ceiling, own: &StoreyLevel) -> (f64, f64) {
    let top = own.top_elevation - ceiling.offset;
    (top - fall_of(ceiling) - thickness(snapshot, ceiling), top)
}

/// 🪜️ The z of the visible underside of a ceiling at a plan point, `None` where the point is outside the boundary, inside a hole, or the ceiling has no layers.
pub fn underside_at(snapshot: &ModelSnapshot, ceiling: &Ceiling, own: &StoreyLevel, at: Point2) -> Option<f64> {
    let (outline, probe) = (bulged(&ceiling.boundary), Point::new(at.x, at.y));
    let thick = thickness(snapshot, ceiling);
    let covers = thick > 1e-12 && outline.len() >= 3 && loops::contains(&outline, probe) && !ceiling.holes.iter().any(|hole| loops::contains(&bulged(hole), probe));
    covers.then(|| top_plane(ceiling.slope, &outline, own.top_elevation - ceiling.offset).at(probe) - thick)
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🔲️ The solid of a ceiling from the level of its storey; absent without a type.
pub fn ceiling_solid(snapshot: &ModelSnapshot, ceiling: &Ceiling, own: &StoreyLevel) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Ceiling);
    if let Some(kind) = snapshot.ceiling_types.get(&ceiling.ceiling_type) {
        for (index, (layer, mesh)) in kind.layers.iter().zip(ceiling_layers(ceiling, kind, own)).enumerate() {
            builder.add(parts::LAYER, &layer.material, index as u32, &mesh);
        }
    }
    builder.build()
}

/// 🔑️ What `ceiling_solid` reads besides the level: the ceiling record and its type.
pub fn dependency(snapshot: &ModelSnapshot, ceiling: &Ceiling) -> DslValue {
    dep_object([("ceiling", dep_value(ceiling)), ("type", dep_value(&snapshot.ceiling_types.get(&ceiling.ceiling_type).cloned()))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
