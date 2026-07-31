//! ⚙️ FEM 3D app — headless compute (constitutional: engine).

use fem3d::{Fem3dDocument, FemElement, FemLoad};
use fem_core::{analyses, Bar3, Dof, Element, Frame3, MemberUdl, Model, NodalLoad, Node, Support};
use std::collections::HashMap;

pub fn empty_fem3d_projection() -> Fem3dDocument {
    Fem3dDocument::default()
}

// #region 🔖️Bridge

// #region 🔖️Errors
/// ⚠️ Everything that can go wrong resolving or solving a `Fem3dDocument`.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Fem3dError {
    #[error("material not found: {0}")]
    MaterialNotFound(String),
    #[error("section not found: {0}")]
    SectionNotFound(String),
    #[error("node not found: {0}")]
    NodeNotFound(String),
    #[error("unknown solid id: {0}")]
    UnknownSolidId(String),
    #[error("solid {solid_id} failed to mesh: {reason}")]
    MeshFailed { solid_id: String, reason: String },
    #[error("load case not found: {0}")]
    LoadCaseNotFound(String),
    #[error("mode index out of range: {0}")]
    ModeIndexOutOfRange(usize),
    #[error(transparent)]
    Fem(#[from] fem_core::FemError),
}
// #endregion 🔖️Errors

// #region 🔖️SolidMeshing
/// 📐️ Unsigned area of triangle `(p0, p1, p2)` via the shoelace formula — mirrors `fem_2d`'s helper of
/// the same purpose.
fn triangle_area_2d(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> f64 {
    (0.5 * ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]))).abs()
}

/// 🌐️ One meshed `FemSolid`'s resolved geometry, reused by `resolve_geometry`'s caller for area-load
/// tributary-area computation. `node_ids`/`points` cover the FULL volume mesh (every layer); the top
/// surface (needed for `FemLoad::Area`) is exactly `top_footprint_tris` over `top_footprint_points`
/// (the ORIGINAL flat triangulation, since `extrude_tri_mesh` numbers each layer's points in that same
/// order — see its doc — so the top layer's node ids are `node_ids[top_offset + footprint_point_index]`).
struct MeshedSolid {
    solid_id: String,
    node_ids: Vec<String>,
    top_offset: usize,
    top_footprint_points: Vec<[f64; 2]>,
    top_footprint_tris: Vec<[u32; 3]>,
}

/// 🧩️ `resolve_geometry`'s resolved `(nodes, elements, meshed solids, supports)` quadruple.
type ResolvedGeometry = (Vec<Node>, Vec<Box<dyn Element>>, Vec<MeshedSolid>, Vec<Support>);

/// 🌉️ Resolves a `Fem3dDocument`'s nodes/elements/supports (materials/sections looked up by id) plus
/// every `FemSolid` meshed into `Tet4` elements (footprint triangulated via `fem_core::mesh::triangulate`,
/// extruded via `extrude_tri_mesh`, split via `split_to_tets` — mirrors `fem_2d::build_nodes_and_elements`'s
/// `Tri3Cst` region meshing) — the geometry shared by `build_model`, `fem3d_solve_all`, modal, and
/// buckling. A solid boundary point coinciding (within `1e-9`, all of x/y/z) with an existing document
/// node reuses that node's id; otherwise a node is synthesized once per unique mesh point as
/// `{solid_id}_m{point_index}`.
fn resolve_geometry(doc: &Fem3dDocument) -> Result<ResolvedGeometry, Fem3dError> {
    let mut nodes: Vec<Node> = doc.nodes.iter().map(|node| Node { id: node.id.clone(), pos: [node.x, node.y, node.z] }).collect();
    let node_exists = |id: &str| doc.nodes.iter().any(|n| n.id == id);
    let mut elements: Vec<Box<dyn Element>> = Vec::with_capacity(doc.elements.len());
    for element in &doc.elements {
        match element {
            FemElement::Bar { id, start, end, material_id, section_id } => {
                let material = doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| Fem3dError::MaterialNotFound(material_id.clone()))?;
                let section = doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| Fem3dError::SectionNotFound(section_id.clone()))?;
                if !node_exists(start) {
                    return Err(Fem3dError::NodeNotFound(start.clone()));
                }
                if !node_exists(end) {
                    return Err(Fem3dError::NodeNotFound(end.clone()));
                }
                elements.push(Box::new(Bar3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, a: section.area, density: material.rho }));
            }
            FemElement::Frame { id, start, end, material_id, section_id, roll } => {
                let material = doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| Fem3dError::MaterialNotFound(material_id.clone()))?;
                let section = doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| Fem3dError::SectionNotFound(section_id.clone()))?;
                if !node_exists(start) {
                    return Err(Fem3dError::NodeNotFound(start.clone()));
                }
                if !node_exists(end) {
                    return Err(Fem3dError::NodeNotFound(end.clone()));
                }
                elements.push(Box::new(Frame3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, g: material.g, a: section.area, iy: section.iy, iz: section.iz, j: section.j, roll: *roll, density: material.rho }));
            }
        }
    }

    let mut meshed_solids = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let material = doc.materials.iter().find(|m| m.id == solid.material_id).ok_or_else(|| Fem3dError::MaterialNotFound(solid.material_id.clone()))?;
        let domain = fem_core::mesh::PlanarDomain { outer: solid.outline.clone(), holes: solid.holes.clone() };
        let opts = fem_core::mesh::MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = fem_core::mesh::triangulate(&domain, &opts).map_err(|e| Fem3dError::MeshFailed { solid_id: solid.id.clone(), reason: e.to_string() })?;
        let layers = solid.layers.max(1);
        let volume_mesh = fem_core::mesh::extrude_tri_mesh(&tri_mesh, solid.height, layers);
        let tet_mesh = fem_core::mesh::split_to_tets(&volume_mesh);
        let points: Vec<[f64; 3]> = tet_mesh.points.iter().map(|p| [p[0], p[1], p[2] + solid.base_z]).collect();

        let mut node_ids = Vec::with_capacity(points.len());
        for (point_index, p) in points.iter().enumerate() {
            let id = match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9 && (n.z - p[2]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => {
                    let synthetic_id = format!("{}_m{}", solid.id, point_index);
                    nodes.push(Node { id: synthetic_id.clone(), pos: *p });
                    synthetic_id
                }
            };
            node_ids.push(id);
        }

        for (cell_index, cell) in tet_mesh.cells.iter().enumerate() {
            let fem_core::mesh::Cell::Tet4(t) = cell else { continue };
            let tet_nodes = [node_ids[t[0] as usize].clone(), node_ids[t[1] as usize].clone(), node_ids[t[2] as usize].clone(), node_ids[t[3] as usize].clone()];
            elements.push(Box::new(fem_core::elements3d::Tet4 { id: format!("{}_c{}", solid.id, cell_index), nodes: tet_nodes, e: material.e, nu: material.nu, density: material.rho }));
        }

        // The LAST extrusion layer's points are the top surface — `extrude_tri_mesh` numbers points
        // layer-by-layer in `tri_mesh.points`' own order (see its doc), so index `top_offset + i` is
        // footprint point `i`'s top-layer node, and `tri_mesh.tris` is directly the top surface's
        // triangulation (that layer's wedges pass their top cap through `split_to_tets` unsplit).
        let top_offset = layers * tri_mesh.points.len();
        meshed_solids.push(MeshedSolid { solid_id: solid.id.clone(), node_ids, top_offset, top_footprint_points: tri_mesh.points, top_footprint_tris: tri_mesh.tris });
    }

    let supports = doc.supports.iter().map(|support| Support { node_id: support.node_id.clone(), fixed: support.fixed.iter().map(|dof| Dof::from(*dof)).collect() }).collect();
    Ok((nodes, elements, meshed_solids, supports))
}

/// 🌬️ Converts a `FemLoad::Area` (uniform pressure, Pa) into per-node global `-Z` nodal loads on a
/// solid's TOP surface — `pressure * tributaryArea` at each top node, tributary area `(1/3)` of the
/// summed area of every top-surface triangle touching that node. Mirrors `fem_2d::area_load_nodal_loads`.
fn area_load_nodal_loads_3d(solid: &MeshedSolid, pressure: f64) -> Vec<NodalLoad> {
    let mut tributary: HashMap<String, f64> = HashMap::new();
    for tri in &solid.top_footprint_tris {
        let area = triangle_area_2d(solid.top_footprint_points[tri[0] as usize], solid.top_footprint_points[tri[1] as usize], solid.top_footprint_points[tri[2] as usize]);
        for &idx in tri {
            let node_id = solid.node_ids[solid.top_offset + idx as usize].clone();
            *tributary.entry(node_id).or_insert(0.0) += area / 3.0;
        }
    }
    tributary.into_iter().map(|(node_id, trib)| NodalLoad { node_id, dof: Dof::Tz, value: -pressure * trib }).collect()
}

/// 🌬️ Translates one `FemLoadCase`'s loads into `(nodal_loads, member_loads)`, resolving `Area` loads
/// against the already-meshed `solids` — shared by `build_model`, `fem3d_solve_all`, and buckling's
/// reference-case resolution.
fn translate_loads(loads: &[FemLoad], solids: &[MeshedSolid]) -> Result<(Vec<NodalLoad>, Vec<(String, MemberUdl)>), Fem3dError> {
    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: Dof::from(*dof), value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, wz, .. } => member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: *wz })),
            FemLoad::Area { solid_id, pressure, .. } => {
                let solid = solids.iter().find(|s| &s.solid_id == solid_id).ok_or_else(|| Fem3dError::UnknownSolidId(solid_id.clone()))?;
                nodal_loads.extend(area_load_nodal_loads_3d(solid, *pressure));
            }
        }
    }
    Ok((nodal_loads, member_loads))
}
// #endregion 🔖️SolidMeshing

/// 🌉️ Resolves a `Fem3dDocument` load case into a `fem_core::Model`: nodes, `Bar3`/`Frame3`/`Tet4`
/// elements (materials/sections looked up by id), supports, and the named load case's translated loads.
pub fn build_model(doc: &Fem3dDocument, case_id: &str) -> Result<Model, Fem3dError> {
    let (nodes, elements, solids, supports) = resolve_geometry(doc)?;
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    let (nodal_loads, member_loads) = translate_loads(&case.loads, &solids)?;
    Ok(Model { nodes, elements, supports, nodal_loads, member_loads })
}

/// 🚀️ Frozen entry point: builds the model for `case_id` and runs `fem_core::solve_linear_static`.
/// Consumed directly by `fem-plugin`; do not rename or change this signature.
pub fn fem3d_solve(doc: &Fem3dDocument, case_id: &str) -> Result<fem_core::StaticResult, String> {
    let model = build_model(doc, case_id).map_err(|e| e.to_string())?;
    fem_core::solve_linear_static(&model).map_err(|e| e.to_string())
}

/// 🌉️ Builds an `AnalysisModel` plus one `analyses::LoadCase` per `doc.load_cases` entry and one
/// `analyses::Combination` per `doc.combinations` entry, solving them ALL at once via
/// `fem_core::analyses::solve_multi_case` (self-weight honored via `doc.materials`' `rho`, gravity
/// fixed at `[0.0, 0.0, -9.81]` — this crate is Z-up, per `FemNode`'s `{x,y,z}` fields and the existing
/// cantilever test's `Dof::Tz` tip load). Returns results keyed by case id ∪ combination id.
pub fn fem3d_solve_all(doc: &Fem3dDocument) -> Result<HashMap<String, fem_core::StaticResult>, Fem3dError> {
    let (nodes, elements, solids, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let mut cases = Vec::with_capacity(doc.load_cases.len());
    for case in &doc.load_cases {
        let (nodal_loads, member_loads) = translate_loads(&case.loads, &solids)?;
        cases.push(analyses::LoadCase { id: case.id.clone(), nodal_loads, member_loads, self_weight: case.self_weight });
    }
    let combinations: Vec<analyses::Combination> =
        doc.combinations.iter().map(|combination| analyses::Combination { id: combination.id.clone(), terms: combination.terms.iter().map(|(id, factor)| (id.clone(), *factor)).collect() }).collect();
    analyses::solve_multi_case(&model, &cases, &combinations, [0.0, 0.0, -9.81]).map_err(Fem3dError::from)
}

// #region 🔖️ModalBuckling
/// 🔢️ Node-major, active-DOF-filtered ordering matching `fem_core::analyses::ModalResult`/
/// `BucklingResult`'s documented shape-vector layout — mirrors `fem_2d`'s identically named helper
/// (both are small local reimplementations of `analyses::build_dof_map`, which isn't `pub`).
fn mode_dof_order(nodes: &[Node], elements: &[Box<dyn Element>]) -> Vec<(String, Dof)> {
    let mut order = Vec::new();
    for node in nodes {
        let mut active: Vec<Dof> = Vec::new();
        for element in elements {
            if element.node_ids().iter().any(|id| id == &node.id) {
                for &dof in element.dofs_per_node() {
                    if !active.contains(&dof) {
                        active.push(dof);
                    }
                }
            }
        }
        active.sort_by_key(|d| d.index());
        for dof in active {
            order.push((node.id.clone(), dof));
        }
    }
    order
}

/// 🎵️ Modal analysis: lowest `doc.analysis.modal_count` natural frequencies/mode shapes.
pub fn fem3d_modal(doc: &Fem3dDocument) -> Result<analyses::ModalResult, Fem3dError> {
    let (nodes, elements, _solids, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    analyses::modal(&model, doc.analysis.modal_count).map_err(Fem3dError::from)
}

/// 🌉️ Richer modal entry point: solves the same modal analysis as `fem3d_modal` but also unpacks mode
/// `mode_index`'s shape into a per-node `[f64;6]` displacement map. Returns
/// `(frequency_hz, node_id -> displacement values)`.
pub fn fem3d_modal_mode_values(doc: &Fem3dDocument, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem3dError> {
    let (nodes, elements, _solids, supports) = resolve_geometry(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let result = analyses::modal(&model, doc.analysis.modal_count)?;
    let freq = *result.frequencies_hz.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((freq, values))
}

/// 🌉️ Shared buckling-case resolution for `fem3d_buckling`/`fem3d_buckling_mode_values`, mirroring
/// `fem2d`'s `buckling_inputs` — translates the named case's loads (incl. `Area` against `solids`).
fn buckling_case(doc: &Fem3dDocument, case_id: &str, solids: &[MeshedSolid]) -> Result<analyses::LoadCase, Fem3dError> {
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    let (nodal_loads, member_loads) = translate_loads(&case.loads, solids)?;
    Ok(analyses::LoadCase { id: case.id.clone(), nodal_loads, member_loads, self_weight: case.self_weight })
}

/// 🏛️ Linear buckling: lowest `doc.analysis.buckling_count` load factors/mode shapes for `case_id`.
pub fn fem3d_buckling(doc: &Fem3dDocument, case_id: &str) -> Result<analyses::BucklingResult, Fem3dError> {
    let (nodes, elements, solids, supports) = resolve_geometry(doc)?;
    let case = buckling_case(doc, case_id, &solids)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    analyses::buckling(&model, &case, doc.analysis.buckling_count).map_err(Fem3dError::from)
}

/// 🌉️ Richer buckling entry point: mirrors `fem3d_modal_mode_values` — solves the same buckling
/// analysis as `fem3d_buckling` but also unpacks mode `mode_index`'s shape into a per-node
/// displacement map. Returns `(load_factor, node_id -> displacement values)`.
pub fn fem3d_buckling_mode_values(doc: &Fem3dDocument, case_id: &str, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem3dError> {
    let (nodes, elements, solids, supports) = resolve_geometry(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let case = buckling_case(doc, case_id, &solids)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let result = analyses::buckling(&model, &case, doc.analysis.buckling_count)?;
    let factor = *result.factors.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((factor, values))
}
// #endregion 🔖️ModalBuckling

// #region 🔖️SolidMeshPreview
/// 🗺️ One meshed solid's cheap preview geometry — the full volume mesh (points/tets) plus its outer
/// boundary triangulation (via `fem_core::mesh::boundary_faces`) for surface rendering, WITHOUT
/// building any `fem_core::Element`. Mirrors `fem_2d::RegionMesh`/`fem2d_mesh_preview`.
pub struct SolidMesh {
    pub solid_id: String,
    pub points: Vec<[f64; 3]>,
    pub tets: Vec<[u32; 4]>,
    pub boundary_tris: Vec<[u32; 3]>,
    pub node_ids: Vec<String>,
}

/// 🗺️ Triangulates+extrudes+tet-splits every `FemSolid` in `doc` (same deterministic
/// `fem_core::mesh` calls as `resolve_geometry`, so tet indices line up with `"{solid_id}_c{i}"`
/// element ids) and returns just the geometry plus its outer surface — cheap enough for every render.
pub fn fem3d_mesh_preview(doc: &Fem3dDocument) -> Result<Vec<SolidMesh>, Fem3dError> {
    let mut out = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let domain = fem_core::mesh::PlanarDomain { outer: solid.outline.clone(), holes: solid.holes.clone() };
        let opts = fem_core::mesh::MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = fem_core::mesh::triangulate(&domain, &opts).map_err(|e| Fem3dError::MeshFailed { solid_id: solid.id.clone(), reason: e.to_string() })?;
        let volume_mesh = fem_core::mesh::extrude_tri_mesh(&tri_mesh, solid.height, solid.layers.max(1));
        let tet_mesh = fem_core::mesh::split_to_tets(&volume_mesh);
        let points: Vec<[f64; 3]> = tet_mesh.points.iter().map(|p| [p[0], p[1], p[2] + solid.base_z]).collect();
        let node_ids = points
            .iter()
            .enumerate()
            .map(|(point_index, p)| match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9 && (n.z - p[2]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => format!("{}_m{}", solid.id, point_index),
            })
            .collect();
        let tets: Vec<[u32; 4]> = tet_mesh.cells.iter().filter_map(|c| match c { fem_core::mesh::Cell::Tet4(t) => Some(*t), _ => None }).collect();
        let boundary_mesh = fem_core::mesh::VolumeMesh { points: points.clone(), cells: tet_mesh.cells };
        let boundary_tris = fem_core::mesh::boundary_faces(&boundary_mesh);
        out.push(SolidMesh { solid_id: solid.id.clone(), points, tets, boundary_tris, node_ids });
    }
    Ok(out)
}

/// 🎨️ Nodal-averaged von Mises stress for `case_id`'s solved result, keyed by node id — the
/// document-layer bridge to `fem_core::analyses::nodal_averaged_scalar`, mirroring `fem_2d`'s
/// `fem2d_nodal_von_mises`, feeding `fem-plugin`'s solid stress contour rendering.
pub fn fem3d_nodal_von_mises(doc: &Fem3dDocument, case_id: &str) -> Result<HashMap<String, f64>, Fem3dError> {
    let (nodes, elements, _solids, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let results = fem3d_solve_all(doc)?;
    let result = results.get(case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    Ok(analyses::nodal_averaged_scalar(&model, result, analyses::StressScalar::VonMises))
}
// #endregion 🔖️SolidMeshPreview
// #endregion 🔖️Bridge

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use fem3d::{FemAnalysisSettings, FemCombination, FemDof, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
    use fem_core::ElementResult;
    use std::collections::BTreeMap;

    // #region 🔖️Fixtures
    fn cantilever_fixture() -> (Fem3dDocument, f64, f64, f64, f64, f64) {
        let e = 210e9;
        let g = 80.77e9;
        let a = 0.00538;
        let iy = 0.0000369;
        let iz = 0.0000133;
        let j = 0.00000060;
        let l = 3.0;
        let p = 5000.0;
        let doc = Fem3dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: l, y: 0.0, z: 0.0 }],
            elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e, g, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: a, iy, iz, j }],
            solids: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -p }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        };
        (doc, e, iy, l, p, iz)
    }

    /// 🔺️ A free 3D joint needs at least 3 non-coplanar bars to be kinematically determinate — two
    /// bars only span a plane, leaving one direction with zero stiffness (a mechanism). Hence n4/b3.
    fn truss_fixture() -> Fem3dDocument {
        Fem3dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNode { id: "n3".into(), x: 1.0, y: 1.0, z: 2.0 }, FemNode { id: "n4".into(), x: 1.0, y: -1.0, z: 0.0 }],
            elements: vec![
                FemElement::Bar { id: "b1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b3".into(), start: "n4".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "rod".into(), name: "Rod".into(), area: 0.001, iy: 1e-6, iz: 1e-6, j: 1e-6 }],
            solids: vec![],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() },
                FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: FemDof::ALL.to_vec() },
                FemSupport { id: "s3".into(), node_id: "n4".into(), fixed: FemDof::ALL.to_vec() },
            ],
            load_cases: vec![FemLoadCase { id: "drop".into(), name: "Drop".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: FemDof::Tz, value: -1000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    /// 🧱️ A 2m x 1m x 0.5m slab footprint at the origin, meshed at `mesh_size`, with all 4 footprint
    /// corners as pre-placed document nodes fully fixed in translation (`Tet4` has no rotational DOF) —
    /// mirrors `fem_2d`'s `rectangle_region_doc` fixture pattern for `FemSolid`.
    fn solid_slab_doc() -> Fem3dDocument {
        Fem3dDocument {
            nodes: vec![
                FemNode { id: "sc0".into(), x: 0.0, y: 0.0, z: 0.0 },
                FemNode { id: "sc1".into(), x: 2.0, y: 0.0, z: 0.0 },
                FemNode { id: "sc2".into(), x: 2.0, y: 1.0, z: 0.0 },
                FemNode { id: "sc3".into(), x: 0.0, y: 1.0, z: 0.0 },
            ],
            elements: vec![],
            materials: vec![FemMaterial { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
            sections: vec![],
            solids: vec![FemSolid { id: "sol1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.5, layers: 1, mesh_size: 1.0, material_id: "concrete".into() }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "sc0".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s2".into(), node_id: "sc1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s3".into(), node_id: "sc2".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s4".into(), node_id: "sc3".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
            ],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️BuildModel
    #[test]
    fn build_model_rejects_dangling_material() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { material_id, .. } = &mut doc.elements[0] {
            *material_id = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }

    #[test]
    fn build_model_rejects_dangling_section() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { section_id, .. } = &mut doc.elements[0] {
            *section_id = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }

    #[test]
    fn build_model_rejects_dangling_node() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { end, .. } = &mut doc.elements[0] {
            *end = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }
    // #endregion 🔖️BuildModel

    // #region 🔖️CantileverBenchmark
    #[test]
    fn cantilever_tip_load_matches_analytical_solution() {
        let (doc, e, iy, l, p, _iz) = cantilever_fixture();
        let result = fem3d_solve(&doc, "point").expect("solves");

        let expected_deflection = p * l.powi(3) / (3.0 * e * iy);
        let expected_rotation = p * l.powi(2) / (2.0 * e * iy);
        let expected_base_moment = p * l;

        let n2 = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
        let deflection = n2.values[Dof::Tz.index()].abs();
        let rotation = n2.values[Dof::Ry.index()].abs();
        assert!((deflection - expected_deflection).abs() / expected_deflection < 0.01, "deflection {deflection} vs {expected_deflection}");
        assert!((rotation - expected_rotation).abs() / expected_rotation < 0.01, "rotation {rotation} vs {expected_rotation}");

        let reaction_tz = result.reactions.iter().find(|r| r.node_id == "n1" && r.dof == Dof::Tz).unwrap();
        assert!((reaction_tz.value - p).abs() < p * 0.01 || (reaction_tz.value + p).abs() < p * 0.01, "reaction {}", reaction_tz.value);
        assert!((reaction_tz.value + (-p)).abs() < p * 0.01, "reaction + applied load should be ~0: {}", reaction_tz.value);

        let reaction_ry = result.reactions.iter().find(|r| r.node_id == "n1" && r.dof == Dof::Ry).unwrap();
        assert!((reaction_ry.value.abs() - expected_base_moment).abs() / expected_base_moment < 0.01, "base moment {} vs {}", reaction_ry.value, expected_base_moment);

        let (_, element_result) = result.elements.iter().find(|(id, _)| id == "e1").unwrap();
        match element_result {
            ElementResult::Beam { stations } => {
                let base = stations.first().unwrap();
                let tip = stations.last().unwrap();
                let base_tol = (expected_base_moment * 0.01).max(1.0);
                assert!((base.m.abs() - expected_base_moment).abs() < base_tol, "base moment {} vs {}", base.m, expected_base_moment);
                assert!(tip.m.abs() < base_tol, "tip moment should be ~0: {}", tip.m);
            }
            _ => panic!("expected beam result"),
        }
    }

    #[test]
    fn truss_3d_solve_is_finite_and_balanced() {
        let doc = truss_fixture();
        let result = fem3d_solve(&doc, "drop").expect("solves");
        for &v in &result.checks.reaction_sum {
            assert!(v.abs() < 1e-6, "reaction_sum should balance the applied load: {:?}", result.checks.reaction_sum);
        }
        for (_, element_result) in &result.elements {
            match element_result {
                ElementResult::Bar { n } => {
                    assert!(n.is_finite());
                    assert!(n.abs() > 1e-6, "bar force should be nonzero under load");
                }
                _ => panic!("expected bar result"),
            }
        }
    }

    #[test]
    fn fem3d_solve_unknown_case_id_errors() {
        let (doc, ..) = cantilever_fixture();
        let err = fem3d_solve(&doc, "missing-case").unwrap_err();
        assert!(err.contains("load case not found"), "error was: {err}");
    }
    // #endregion 🔖️CantileverBenchmark

    // #region 🔖️SolveAll
    #[test]
    fn fem3d_solve_all_returns_case_and_combination_results() {
        let (mut doc, ..) = cantilever_fixture();
        doc.load_cases.push(FemLoadCase { id: "point2".into(), name: "Point Load 2".into(), loads: vec![FemLoad::Nodal { id: "l2".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -2000.0 }], self_weight: false });
        doc.combinations = vec![FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35), ("point2".into(), 1.0)]) }];

        let results = fem3d_solve_all(&doc).expect("solves");
        let mut keys: Vec<&String> = results.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["point", "point2", "uls"], "expected exactly the case and combination ids");

        let point = results.get("point").unwrap();
        let point2 = results.get("point2").unwrap();
        let uls = results.get("uls").unwrap();
        for pd in &point.displacements {
            let p2d = point2.displacements.iter().find(|d| d.node_id == pd.node_id).unwrap();
            let ud = uls.displacements.iter().find(|d| d.node_id == pd.node_id).unwrap();
            for k in 0..6 {
                let expected = 1.35 * pd.values[k] + 1.0 * p2d.values[k];
                assert!((ud.values[k] - expected).abs() < 1e-8, "combo displacement mismatch at {} dof {k}: {} vs {}", pd.node_id, ud.values[k], expected);
            }
        }
    }

    #[test]
    fn self_weight_case_produces_nonzero_reactions() {
        let (mut doc, _e, _iy, l, _p, _iz) = cantilever_fixture();
        let (area, rho) = (doc.sections[0].area, doc.materials[0].rho);
        doc.load_cases = vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }];

        let results = fem3d_solve_all(&doc).expect("solves");
        let result = results.get("self").unwrap();

        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = rho * area * l * 9.81;
        assert!(total_tz_reaction.abs() > 1e-6, "self-weight reaction should be nonzero");
        assert!((total_tz_reaction - expected).abs() / expected < 0.02, "reaction sum {total_tz_reaction} vs expected {expected}");
    }

    /// 🌬️ A `FemLoad::MemberUdl` on the cantilever fixture's `Frame3`: base shear must equal the
    /// classical `wL` total, same benchmark `elements3d::tests::frame3_udl_cantilever_matches_hand_calc`
    /// checks headlessly, now exercised through the document bridge's load translation.
    #[test]
    fn member_udl_load_matches_total_wl() {
        let (mut doc, _e, _iy, l, _p, _iz) = cantilever_fixture();
        let w = 800.0;
        doc.load_cases = vec![FemLoadCase { id: "udl".into(), name: "UDL".into(), loads: vec![FemLoad::MemberUdl { id: "u1".into(), element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -w }], self_weight: false }];
        let results = fem3d_solve_all(&doc).expect("solves");
        let result = results.get("udl").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = w * l;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }
    // #endregion 🔖️SolveAll

    // #region 🔖️Solids
    #[test]
    fn solid_self_weight_matches_total_mass_times_gravity() {
        let doc = solid_slab_doc();
        let results = fem3d_solve_all(&doc).expect("solid self-weight solves");
        let result = results.get("self").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let (footprint_area, height, rho, g) = (2.0 * 1.0, 0.5, 2400.0, 9.81);
        let expected = rho * footprint_area * height * g;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }

    /// ⚖️ A uniform pressure over the solid's top face must balance EXACTLY (mesh-independent, since
    /// tributary-area nodal loads sum to `pressure * footprintArea` regardless of triangulation) —
    /// possible only now that `fem_3d` meshes solids at all.
    #[test]
    fn solid_area_load_matches_pressure_times_footprint_area() {
        let mut doc = solid_slab_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "Pressure".into(), loads: vec![FemLoad::Area { id: "a1".into(), solid_id: "sol1".into(), pressure: 8000.0 }], self_weight: false }];
        let results = fem3d_solve_all(&doc).expect("solid pressure load solves");
        let result = results.get("pressure").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = 8000.0 * 2.0 * 1.0;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }

    #[test]
    fn fem3d_mesh_preview_returns_solid_tets_and_boundary() {
        let doc = solid_slab_doc();
        let previews = fem3d_mesh_preview(&doc).expect("mesh preview succeeds");
        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].solid_id, "sol1");
        assert!(!previews[0].tets.is_empty(), "expected at least one tet");
        assert!(!previews[0].boundary_tris.is_empty(), "expected boundary triangles");
        assert_eq!(previews[0].node_ids.len(), previews[0].points.len());
        for corner_id in ["sc0", "sc1", "sc2", "sc3"] {
            assert!(previews[0].node_ids.contains(&corner_id.to_string()), "expected corner {corner_id} to be reused");
        }
    }

    #[test]
    fn fem3d_nodal_von_mises_returns_finite_values_for_solid() {
        let mut doc = solid_slab_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "Pressure".into(), loads: vec![FemLoad::Area { id: "a1".into(), solid_id: "sol1".into(), pressure: 8000.0 }], self_weight: false }];
        let averaged = fem3d_nodal_von_mises(&doc, "pressure").expect("nodal von mises solves");
        assert!(!averaged.is_empty());
        for v in averaged.values() {
            assert!(v.is_finite() && *v >= 0.0, "von mises {v} should be finite and non-negative");
        }
    }
    // #endregion 🔖️Solids

    // #region 🔖️ModalBuckling
    #[test]
    fn fem3d_modal_returns_requested_mode_count() {
        let (doc, ..) = cantilever_fixture();
        let result = fem3d_modal(&doc).expect("modal solves");
        assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
        for w in result.frequencies_hz.windows(2) {
            assert!(w[0] <= w[1], "frequencies should be ascending: {:?}", result.frequencies_hz);
        }
        for &f in &result.frequencies_hz {
            assert!(f.is_finite() && f >= 0.0, "frequency should be finite and non-negative: {f}");
        }
    }

    #[test]
    fn fem3d_modal_mode_values_returns_node_displacements() {
        let (doc, ..) = cantilever_fixture();
        let (freq, values) = fem3d_modal_mode_values(&doc, 0).expect("modal mode values solves");
        assert!(freq.is_finite() && freq >= 0.0);
        assert!(values.contains_key("n1"));
        assert!(values.contains_key("n2"));
    }

    #[test]
    fn fem3d_buckling_returns_requested_mode_count() {
        let (doc, ..) = cantilever_fixture();
        let result = fem3d_buckling(&doc, "point").expect("buckling solves");
        assert_eq!(result.factors.len(), doc.analysis.buckling_count);
        for &f in &result.factors {
            assert!(f.is_finite(), "buckling factor should be finite: {f}");
        }
    }

    #[test]
    fn fem3d_buckling_mode_values_returns_node_displacements() {
        let (doc, ..) = cantilever_fixture();
        let (factor, values) = fem3d_buckling_mode_values(&doc, "point", 0).expect("buckling mode values solves");
        assert!(factor.is_finite());
        assert!(values.contains_key("n1"));
        assert!(values.contains_key("n2"));
    }

    #[test]
    fn fem3d_buckling_unknown_case_errors() {
        let (doc, ..) = cantilever_fixture();
        let err = fem3d_buckling(&doc, "missing").err().expect("expected error");
        assert!(err.to_string().contains("load case not found"), "unexpected error: {err}");
    }
    // #endregion 🔖️ModalBuckling

    // #region 🔖️ExampleFixture
    #[test]
    fn example_fixture_parses() {
        use store::DocumentDsl;
        let doc: Fem3dDocument = Fem3dDocument::parse_dsl(fem3d_dsl::FEM3D_EXAMPLE_TEXT).expect("example fixture parses");
        assert_eq!(doc.nodes.len(), 16);
        assert_eq!(doc.elements.len(), 16);
        assert_eq!(doc.solids.len(), 1);
        let result = fem3d_solve(&doc, "dead").expect("example fixture solves");
        assert!(result.checks.residual_norm < 1e-6);

        let all_results = fem3d_solve_all(&doc).expect("example fixture solves all");
        assert!(all_results.contains_key("dead"), "expected dead case result");
        assert!(all_results.contains_key("live"), "expected live case result");
        assert!(all_results.contains_key("uls"), "expected uls combination result");
        let dead = all_results.get("dead").expect("expected dead case result (solid area load + member UDL + self-weight)");
        assert!(dead.checks.residual_norm < 1e-6, "residual {}", dead.checks.residual_norm);

        let previews = fem3d_mesh_preview(&doc).expect("mesh preview succeeds");
        assert_eq!(previews.len(), 1);
        assert!(!previews[0].tets.is_empty(), "expected at least one tet");
        assert!(!previews[0].boundary_tris.is_empty(), "expected boundary triangles");

        let averaged = fem3d_nodal_von_mises(&doc, "dead").expect("nodal von mises solves");
        assert!(!averaged.is_empty(), "expected at least one averaged nodal value");
        for v in averaged.values() {
            assert!(v.is_finite() && *v >= 0.0, "von mises {v} should be finite and non-negative");
        }

        let buckling = fem3d_buckling(&doc, "dead").expect("buckling resolves for the dead case's compressed column");
        assert!(buckling.factors[0].is_finite() && buckling.factors[0] > 1.0, "expected an illustrative (finite, >1) load factor: {:?}", buckling.factors);
    }
    // #endregion 🔖️ExampleFixture
}
// #endregion 🧪️Tests
