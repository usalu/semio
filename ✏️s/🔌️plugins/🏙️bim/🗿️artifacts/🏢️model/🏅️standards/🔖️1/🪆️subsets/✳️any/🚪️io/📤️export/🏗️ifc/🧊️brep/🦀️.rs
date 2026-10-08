//! 🧊️ Triangle meshes as `IfcFacetedBrep` bodies, for elements without an extruded-profile form (roofs, stairs, railings, sloped slabs, curtain-wall parts).

use super::writer::{flag, refs, rf, Ifc};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidGroup};
use semio_framework_geometry::mesh::TriMesh;

/// 🧊️ An `IfcFacetedBrep` of `mesh`, one triangular face per mesh triangle; `None` when the mesh has no triangle.
pub fn faceted_brep(ifc: &mut Ifc, mesh: &TriMesh) -> Option<u64> {
    let welded = mesh.welded();
    if welded.indices.is_empty() {
        return None;
    }
    let points: Vec<u64> = welded.positions.iter().map(|position| ifc.point3(*position)).collect();
    let faces: Vec<u64> = welded
        .indices
        .iter()
        .map(|triangle| {
            let corners = triangle.map(|index| points[index as usize]);
            let polygon = ifc.add("IFCPOLYLOOP", vec![refs(&corners)]);
            let bound = ifc.add("IFCFACEOUTERBOUND", vec![rf(polygon), flag(true)]);
            ifc.add("IFCFACE", vec![refs(&[bound])])
        })
        .collect();
    let shell = ifc.add("IFCCLOSEDSHELL", vec![refs(&faces)]);
    Some(ifc.add("IFCFACETEDBREP", vec![rf(shell)]))
}

/// 🧊️ The triangles of `solid` that `keep` accepts (given the face group and the triangle centroid in building coordinates), shifted down by `drop` into the storey frame.
pub fn mesh_where(solid: &ElementSolid, drop: f64, keep: impl Fn(&SolidGroup, [f64; 3]) -> bool) -> TriMesh {
    let corner = |index: u32| {
        let at = index as usize * 3;
        [solid.positions[at], solid.positions[at + 1], solid.positions[at + 2]]
    };
    let mut mesh = TriMesh::new();
    for (triangle, group) in solid.indices.chunks_exact(3).zip(&solid.face_groups) {
        let (a, b, c) = (corner(triangle[0]), corner(triangle[1]), corner(triangle[2]));
        let centroid = [(a[0] + b[0] + c[0]) / 3.0, (a[1] + b[1] + c[1]) / 3.0, (a[2] + b[2] + c[2]) / 3.0];
        if keep(&solid.groups[*group as usize], centroid) {
            let lowered = |p: [f64; 3]| [p[0], p[1], p[2] - drop];
            mesh.push_triangle(lowered(a), lowered(b), lowered(c));
        }
    }
    mesh
}

/// 🧱️ A product definition shape with one `Body`/`Brep` representation of `mesh`.
pub fn brep_definition(ifc: &mut Ifc, mesh: &TriMesh) -> Option<u64> {
    let brep = faceted_brep(ifc, mesh)?;
    let shape = ifc.shape(ifc.body, "Body", "Brep", &[brep]);
    Some(ifc.definition(&[shape]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
