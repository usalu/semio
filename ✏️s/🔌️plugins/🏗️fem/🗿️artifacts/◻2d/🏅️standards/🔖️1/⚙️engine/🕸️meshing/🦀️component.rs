//! 🌐️ FEM 2D artifact engine — region meshing bridge (was the old engine crate's `RegionMeshing`
//! region). `build_nodes_and_elements`/`self_weight_nodal_loads`/`area_load_nodal_loads`/`GRAVITY_G`
//! are `pub(crate)`: they are called cross-node from `crate::artifacts::fem2d::engine` (the top-level
//! `build_model`/`fem2d_solve_all` entry points) and from `crate::artifacts::fem2d::engine::modal_buckling`
//! /`crate::artifacts::fem2d::engine::mesh_preview`.

use crate::artifacts::fem2d::engine::Fem2dError;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemElement};
use crate::model::{Bar2, BeamEb2, Dof, Element, NodalLoad, Node};
use std::collections::HashMap;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::SemioPoint3;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology};

/// ⚖️ Gravitational acceleration (m/s²) used both by the document-bridge's own lumped self-weight
/// translation (`self_weight_nodal_loads`, feeding the frozen `fem2d_solve`) and as the `gravity`
/// argument to `crate::analyses::solve_multi_case` (`fem2d_solve_all`).
pub(crate) const GRAVITY_G: f64 = 9.81;

/// 🌐️ One meshed `FemRegion` — resolved node ids (mesh point-index → doc/synthesized node id, ONE
/// per unique mesh point, matching `points`/`tris` index order), reused by `build_nodes_and_elements`'s
/// caller for area-load tributary-area and self-weight computation.
pub(crate) struct MeshedRegion {
    pub(crate) region_id: String,
    pub(crate) material_id: String,
    pub(crate) thickness: f64,
    pub(crate) node_ids: Vec<String>,
    pub(crate) points: Vec<[f64; 2]>,
    pub(crate) tris: Vec<[u32; 3]>,
}

/// 🧩️ `build_nodes_and_elements`'s resolved `(nodes, elements, meshed regions)` triple.
pub(crate) type ResolvedGeometry = (Vec<Node>, Vec<Box<dyn Element>>, Vec<MeshedRegion>);

/// 📐️ Unsigned area of triangle `(p0, p1, p2)` via the shoelace formula.
fn triangle_area(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> f64 {
    (0.5 * ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]))).abs()
}

/// 🌉️ Shared node/element resolution for `build_model` and `fem2d_solve_all`: base document
/// nodes/elements (`Bar2`/`BeamEb2`, `density: material.rho`) plus every `FemRegion` meshed into
/// `Tri3Cst` elements (plane-stress, `PlaneKind::Stress`) — a region boundary point that coincides
/// (within `1e-9`, both x and y) with an existing document node reuses that node's id, so supports and
/// loads placed on that node reach the mesh; otherwise a node is synthesized once per unique mesh
/// point as `{region_id}_m{point_index}`.
pub(crate) fn build_nodes_and_elements(doc: &Fem2dSnapshot) -> Result<ResolvedGeometry, Fem2dError> {
    let node_exists = |id: &str| doc.nodes.iter().any(|n| n.id == id);
    let mut nodes: Vec<Node> = doc.nodes.iter().map(|n| Node { id: n.id.clone(), pos: [n.x, n.y, 0.0] }).collect();

    let mut elements: Vec<Box<dyn Element>> = Vec::with_capacity(doc.elements.len());
    for element in &doc.elements {
        let (id, start, end, material_id, section_id) = match element {
            FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id),
            FemElement::Beam { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id),
        };
        if !node_exists(start) {
            return Err(Fem2dError::UnknownNodeId(start.clone()));
        }
        if !node_exists(end) {
            return Err(Fem2dError::UnknownNodeId(end.clone()));
        }
        let material = doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| Fem2dError::UnknownMaterialId(material_id.clone()))?;
        let section = doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| Fem2dError::UnknownSectionId(section_id.clone()))?;
        match element {
            FemElement::Bar { .. } => {
                elements.push(Box::new(Bar2 { id: id.clone(), start: start.clone(), end: end.clone(), e: material.e, area: section.area, density: material.rho }));
            }
            FemElement::Beam { .. } => {
                elements.push(Box::new(BeamEb2 { id: id.clone(), start: start.clone(), end: end.clone(), e: material.e, area: section.area, iy: section.iy, density: material.rho }));
            }
        }
    }

    let mut meshed_regions = Vec::with_capacity(doc.regions.len());
    for region in &doc.regions {
        let domain = crate::mesh::PlanarDomain { outer: region.outline.clone(), holes: region.holes.clone() };
        let opts = crate::mesh::MeshOpts { max_edge: region.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = crate::mesh::triangulate(&domain, &opts).map_err(|e| Fem2dError::MeshFailed { region_id: region.id.clone(), reason: e.to_string() })?;
        let material = doc.materials.iter().find(|m| m.id == region.material_id).ok_or_else(|| Fem2dError::UnknownMaterialId(region.material_id.clone()))?;

        let mut node_ids = Vec::with_capacity(tri_mesh.points.len());
        for (point_index, p) in tri_mesh.points.iter().enumerate() {
            let id = match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => {
                    let synthetic_id = format!("{}_m{}", region.id, point_index);
                    nodes.push(Node { id: synthetic_id.clone(), pos: [p[0], p[1], 0.0] });
                    synthetic_id
                }
            };
            node_ids.push(id);
        }

        for (tri_index, tri) in tri_mesh.tris.iter().enumerate() {
            let tri_nodes = [node_ids[tri[0] as usize].clone(), node_ids[tri[1] as usize].clone(), node_ids[tri[2] as usize].clone()];
            elements.push(Box::new(crate::elements2d::Tri3Cst {
                id: format!("{}_t{}", region.id, tri_index),
                nodes: tri_nodes,
                e: material.e,
                nu: material.nu,
                thickness: region.thickness,
                kind: crate::elements2d::PlaneKind::Stress,
                density: material.rho,
            }));
        }

        meshed_regions.push(MeshedRegion { region_id: region.id.clone(), material_id: region.material_id.clone(), thickness: region.thickness, node_ids, points: tri_mesh.points, tris: tri_mesh.tris });
    }

    Ok((nodes, elements, meshed_regions))
}

/// ⚖️ Lumped self-weight nodal loads (downward, global `-Y`) — `ρ·A·L` split evenly at a bar/beam's
/// two end nodes, `ρ·thickness·triangleArea` split evenly at each region triangle's 3 nodes, summed
/// per node. A simple document-bridge translation feeding ONLY the frozen `fem2d_solve`/`build_model`
/// path (which has no native self-weight concept) — `fem2d_solve_all` never calls this helper, since it
/// gets self-weight natively through `crate::analyses`' own `element.mass()`-based pipeline for
/// EVERY massed element (`Bar2`/`BeamEb2` and now `Tri3Cst` regions too), so the two paths never overlap.
pub(crate) fn self_weight_nodal_loads(doc: &Fem2dSnapshot, regions: &[MeshedRegion]) -> Vec<NodalLoad> {
    let mut totals: HashMap<String, f64> = HashMap::new();

    for element in &doc.elements {
        let (start, end, material_id, section_id) = match element {
            FemElement::Bar { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
            FemElement::Beam { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
        };
        let (Some(material), Some(section), Some(n0), Some(n1)) =
            (doc.materials.iter().find(|m| &m.id == material_id), doc.sections.iter().find(|s| &s.id == section_id), doc.nodes.iter().find(|n| &n.id == start), doc.nodes.iter().find(|n| &n.id == end))
        else {
            continue;
        };
        let length = ((n1.x - n0.x).powi(2) + (n1.y - n0.y).powi(2)).sqrt();
        let weight = material.rho * section.area * length * GRAVITY_G;
        *totals.entry(start.clone()).or_insert(0.0) += weight / 2.0;
        *totals.entry(end.clone()).or_insert(0.0) += weight / 2.0;
    }

    for region in regions {
        let Some(material) = doc.materials.iter().find(|m| m.id == region.material_id) else { continue };
        for tri in &region.tris {
            let area = triangle_area(region.points[tri[0] as usize], region.points[tri[1] as usize], region.points[tri[2] as usize]);
            let weight = material.rho * region.thickness * area * GRAVITY_G;
            for &idx in tri {
                *totals.entry(region.node_ids[idx as usize].clone()).or_insert(0.0) += weight / 3.0;
            }
        }
    }

    totals.into_iter().map(|(node_id, weight)| NodalLoad { node_id, dof: Dof::Ty, value: -weight }).collect()
}

/// 🌬️ Converts a `FemLoad::Area` (uniform pressure, Pa) into per-node global `-Y` nodal loads —
/// `pressure * tributaryArea` at each region node, where tributary area is `(1/3)` of the summed area
/// of every triangle touching that node.
pub(crate) fn area_load_nodal_loads(region: &MeshedRegion, pressure: f64) -> Vec<NodalLoad> {
    let mut tributary: HashMap<String, f64> = HashMap::new();
    for tri in &region.tris {
        let area = triangle_area(region.points[tri[0] as usize], region.points[tri[1] as usize], region.points[tri[2] as usize]);
        for &idx in tri {
            *tributary.entry(region.node_ids[idx as usize].clone()).or_insert(0.0) += area / 3.0;
        }
    }
    tributary.into_iter().map(|(node_id, trib)| NodalLoad { node_id, dof: Dof::Ty, value: -pressure * trib }).collect()
}

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
        meshes.push(SemioMesh {
            id: region.id.clone(),
            primitives: vec![SemioPrimitive { id: format!("{}-surface", region.id), topology: SemioTopology::Triangles, positions, indices, ..Default::default() }],
        });
    }
    SemioMeshSnapshot { meshes, ..Default::default() }
}
//#endregion 🔖️SemioMeshBridge
