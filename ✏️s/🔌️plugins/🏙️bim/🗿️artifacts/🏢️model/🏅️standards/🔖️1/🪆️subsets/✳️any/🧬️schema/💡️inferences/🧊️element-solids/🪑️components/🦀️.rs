//! 🪑️ Components as solids: the visible solids of the family under the overrides of the instance, moved from the family frame (`x` right, `y` depth, `z` up) into the building frame by the placement of the component
//! (mirror `x`, turn by the yaw, move the origin; a mirror reverses the winding so faces stay outward). Each family solid keeps its evaluated material, so volumes and material rows follow the same rules as every other element.

use crate::standards::v1::subsets::any::schema::inferences::components::{ComponentEntry, ComponentIssueCode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{parts, ElementSolid, SolidBuilder, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::families::FamilySolidMesh;
use semio_framework_geometry::mesh::TriMesh;
use semio_framework_geometry::placement::Affine3;

fn triples(flat: &[f64]) -> Vec<[f64; 3]> {
    flat.chunks_exact(3).map(|chunk| [chunk[0], chunk[1], chunk[2]]).collect()
}

/// 🕸️ The mesh of a family solid in the family frame.
pub fn mesh_of(solid: &FamilySolidMesh) -> TriMesh {
    TriMesh { positions: triples(&solid.positions), normals: triples(&solid.normals), indices: solid.indices.chunks_exact(3).map(|chunk| [chunk[0], chunk[1], chunk[2]]).collect() }
}

/// 🧭️ The transform of the family frame into the building frame for a placement.
pub fn affine_of(entry: &ComponentEntry) -> Affine3 {
    let placement = &entry.value.placement;
    let (sin, cos) = placement.yaw.sin_cos();
    let side = if placement.mirrored { -1.0 } else { 1.0 };
    Affine3::from_frame([placement.x, placement.y, placement.z], [side * cos, side * sin, 0.0], [-sin, cos, 0.0], [0.0, 0.0, 1.0])
}

/// 🪑️ The solid of a component: empty while the component has no geometry.
pub fn component_solid(entry: &ComponentEntry) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Component);
    if entry.value.has(ComponentIssueCode::NonFinite) {
        return builder.build();
    }
    let affine = affine_of(entry);
    for (_, solid) in entry.family.visible().filter(|(_, solid)| !solid.indices.is_empty()) {
        builder.add(parts::BODY, &solid.material, 0, &mesh_of(solid).transformed(&affine));
    }
    builder.build()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
