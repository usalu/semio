//! ⬜️ `slabs`: the solid of every slab, its boundary loop (straight and bulged edges) with holes extruded downward by the layers of its slab type.
//!
//! The top of the slab lies at the storey elevation plus `offset`; the layers stack downward from there, first layer topmost (`layer` 0), each with its own material and thickness,
//! so the thickness is the sum of the layer thicknesses measured vertically. A slope tilts the whole slab plane: `Slope.direction` is the direction the slab falls (radians
//! counter-clockwise from `+X`), `Slope.angle` the fall angle; the top plane keeps the reference height at the uphill edge of the boundary, so the slab never rises above
//! `storey elevation + offset`. A slab with no layers or an unknown type is absent.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged, direction, layer_thicknesses, stack};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{ModelSnapshot, Slab, SlabType};
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::mesh::{extrude_loops, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_value::DslValue;

//#region 🔖️Geometry
/// 📏️ The top plane of a slab whose reference height is `z`: horizontal, or tilted by its slope about the uphill edge of the boundary.
pub fn top_plane(slab: &Slab, outline: &[Vertex], z: f64) -> ZPlane {
    match slab.slope {
        Some(slope) if slope.angle.abs() > 1e-12 => {
            let fall = direction(slope.direction);
            let anchor = loops::flatten(outline, CHORD_TOLERANCE).into_iter().min_by(|a, b| (a.x * fall.x + a.y * fall.y).total_cmp(&(b.x * fall.x + b.y * fall.y)));
            anchor.map_or(ZPlane::flat(z), |anchor| ZPlane::sloped(anchor, z, slope.direction, -slope.angle.tan()))
        }
        _ => ZPlane::flat(z),
    }
}

/// ⬜️ One mesh per layer, topmost first, of a slab from the level of its storey.
pub fn slab_layers(slab: &Slab, kind: &SlabType, own: &StoreyLevel) -> Vec<TriMesh> {
    let (outline, holes) = (bulged(&slab.boundary), slab.holes.iter().map(|hole| bulged(hole)).collect::<Vec<_>>());
    let top = top_plane(slab, &outline, own.elevation + slab.offset);
    let thicknesses = layer_thicknesses(&kind.layers);
    stack(&thicknesses)
        .into_iter()
        .zip(&thicknesses)
        .map(|((from, to), thickness)| if *thickness > 1e-12 && outline.len() >= 3 { extrude_loops(&outline, &holes, CHORD_TOLERANCE, top.raised(-to), top.raised(-from)) } else { TriMesh::new() })
        .collect()
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// ⬜️ The solid of a slab from the level of its storey; absent without a type.
pub fn slab_solid(snapshot: &ModelSnapshot, slab: &Slab, own: &StoreyLevel) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Slab);
    if let Some(kind) = snapshot.slab_types.get(&slab.slab_type) {
        for (index, (layer, mesh)) in kind.layers.iter().zip(slab_layers(slab, kind, own)).enumerate() {
            builder.add(parts::LAYER, &layer.material, index as u32, &mesh);
        }
    }
    builder.build()
}

/// 🔑️ What `slab_solid` reads besides the level: the slab record and its type.
pub fn dependency(snapshot: &ModelSnapshot, slab: &Slab) -> DslValue {
    dep_object([("slab", dep_value(slab)), ("type", dep_value(&snapshot.slab_types.get(&slab.slab_type).cloned()))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
