//! 🌐️ Explicit installed FEM document to Semio mesh conversion owned by foreign export composition.

use crate::Fem2dSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology};

//#region 🔖️SemioMeshBridge
/// 🌉️ Builds a real `SemioMeshSnapshot` from every `FemRegion`'s genuinely triangulated,
/// thickness-extruded solid boundary: `crate::mesh::triangulate` the footprint (same call
/// `build_nodes_and_elements` above already makes for its `Tri3Cst` regions), `extrude_tri_mesh`
/// by the region's OWN `thickness`, `split_to_tets`, then `boundary_faces` for the outward-
/// oriented outer surface — real, tested geometry (`crate::mesh`'s own volume/area-preservation
/// tests), not fabricated bytes. Feeds the `s.stdio.obj`/`s.stdio.stl` export leaves, which hand
/// this to stdio's real `SemioMeshToObj`/`SemioMeshToStl` bridge + `encode_obj`/`encode_stl_ascii`
/// grammar — this function does no byte-level encoding itself.
///
/// `FemElement::Bar`/`Beam` line members carry no real cross-section PROFILE in the persisted
/// data (only `area`/`iy`, scalar section properties) so no honest 3D solid can be derived from
/// them — they contribute no geometry here. A pure bar/beam model (no `regions`) yields an empty,
/// still-structurally-valid mesh (and thus an empty .obj/.stl) rather than a fabricated shape.
pub(crate) fn build_semio_mesh_snapshot(doc: &Fem2dSnapshot) -> SemioMeshSnapshot {
    let mut meshes = Vec::with_capacity(doc.regions.len());
    for region in &doc.regions {
        let domain = crate::mesh::PlanarDomain { outer: region.outline.clone(), holes: region.holes.clone() };
        let opts = crate::mesh::MeshOpts { max_edge: region.mesh_size, min_angle_deg: 20.0 };
        let Ok(tri_mesh) = crate::mesh::triangulate(&domain, &opts) else { continue };
        let volume = crate::mesh::extrude_tri_mesh(&tri_mesh, region.thickness, 1);
        let tets = crate::mesh::split_to_tets(&volume);
        let faces = crate::mesh::boundary_faces(&tets);
        let positions: Vec<SemioPoint3> = tets.points.iter().map(|p| SemioPoint3 { x: p[0], y: p[1], z: p[2] }).collect();
        let indices: Vec<u32> = faces.iter().flat_map(|f| f.iter().copied()).collect();
        meshes.push(SemioMesh { id: region.id.clone(), primitives: vec![SemioPrimitive { id: format!("{}-surface", region.id), topology: SemioTopology::Triangles, positions, indices, ..Default::default() }] });
    }
    SemioMeshSnapshot { meshes, ..Default::default() }
}
//#endregion 🔖️SemioMeshBridge
