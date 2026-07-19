//! 🏗️ FEM core: the headless finite-element calculation library — model, element trait, assembly
//! pipeline, and linear-static analysis, extended by sibling modules for sparse solvers,
//! quadrature/element formulation, the 2D/3D element libraries, meshing, and multi-case analyses.
//! No UI, no VCS, no framework dependency — `fem_2d`/`fem_3d`/`fem-plugin` are the UI layer built
//! on top of this crate.

pub mod analyses;
pub mod elements2d;
pub mod elements3d;
pub mod formulation;
pub mod mesh;
pub mod sparse;

pub use elements2d::{Bar2, BeamEb2};
pub use elements3d::{Bar3, Frame3};

use mathematical_algebra::{MatD, VecD};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;

// #region 🔖Dof
/// 🧭 Nodal degree of freedom kind, shared by 2D (Tx, Ty, Rz) and 3D (all six) models.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dof {
    Tx,
    Ty,
    Tz,
    Rx,
    Ry,
    Rz,
}

impl Dof {
    pub const ALL: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];

    pub fn index(self) -> usize {
        match self {
            Dof::Tx => 0,
            Dof::Ty => 1,
            Dof::Tz => 2,
            Dof::Rx => 3,
            Dof::Ry => 4,
            Dof::Rz => 5,
        }
    }
}
// #endregion 🔖Dof

// #region 🔖Model
/// 📍 A structural node: a stable id and a global position (2D models keep `pos[2] == 0`).
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub pos: [f64; 3],
}

/// 🔒 A support: the subset of a node's active DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq)]
pub struct Support {
    pub node_id: String,
    pub fixed: Vec<Dof>,
}

/// 🏋️ A concentrated load applied directly to one node's global DOF.
#[derive(Clone, Debug, PartialEq)]
pub struct NodalLoad {
    pub node_id: String,
    pub dof: Dof,
    pub value: f64,
}

/// 🌬️ A uniformly distributed member load, components in global directions per unit length.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MemberUdl {
    pub wx: f64,
    pub wy: f64,
    pub wz: f64,
}

/// 📦 Resolved geometry handed to an element when it builds its stiffness/loads/results —
/// node positions in the same order as `Element::node_ids`.
pub struct ElementContext {
    pub positions: Vec<[f64; 3]>,
}

/// 🔩 One finite element: contributes a global-coordinate stiffness matrix, optional equivalent
/// nodal loads for a member UDL, and recovers internal forces from the solved displacement vector.
/// Every vector this trait produces or consumes is node-major, DOF-minor ordered, matching
/// `node_ids()` paired with `dofs_per_node()`.
pub trait Element {
    fn id(&self) -> &str;
    fn node_ids(&self) -> Vec<String>;
    fn dofs_per_node(&self) -> &[Dof];
    fn stiffness_global(&self, ctx: &ElementContext) -> MatD;
    fn equivalent_nodal_loads(&self, _ctx: &ElementContext, _udl: &MemberUdl) -> Option<VecD> {
        None
    }
    fn recover(&self, ctx: &ElementContext, u_local: &VecD, udl: Option<&MemberUdl>) -> ElementResult;
    /// 🏋️ Consistent element mass matrix in GLOBAL coordinates, same DOF order as `stiffness_global`.
    /// `None` means this element contributes no mass (the default — self-weight/modal analysis skips
    /// it, so massless elements never spuriously restrain a modal shape).
    fn mass(&self, _ctx: &ElementContext) -> Option<MatD> {
        None
    }
    /// 🌀 Geometric ("stress") stiffness in GLOBAL coordinates for the element's current axial/stress
    /// state at displacement `u_element` — used by linear buckling. `None` means unsupported.
    fn geometric_stiffness(&self, _ctx: &ElementContext, _u_element: &VecD) -> Option<MatD> {
        None
    }
}

/// 🏗️ The assembled structural model handed to `solve_linear_static`.
#[derive(Default)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub elements: Vec<Box<dyn Element>>,
    pub supports: Vec<Support>,
    pub nodal_loads: Vec<NodalLoad>,
    pub member_loads: Vec<(String, MemberUdl)>,
}

/// 🔍 Element trait objects aren't `Debug`, so print element ids/count instead — this is what lets
/// `Result<Model, _>` be used with `unwrap_err()`/`expect_err()` in caller test code.
impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Model")
            .field("nodes", &self.nodes)
            .field("elements", &self.elements.iter().map(|e| e.id()).collect::<Vec<_>>())
            .field("supports", &self.supports)
            .field("nodal_loads", &self.nodal_loads)
            .field("member_loads", &self.member_loads)
            .finish()
    }
}
// #endregion 🔖Model

// #region 🔖Results
/// 📐 Per-node displacement, indexed by `Dof::index()`; inactive DOFs stay `0.0`.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeDisplacement {
    pub node_id: String,
    pub values: [f64; 6],
}

/// ⚖️ A support reaction at one restrained, active DOF.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeReaction {
    pub node_id: String,
    pub dof: Dof,
    pub value: f64,
}

/// 📊 A station along a beam's length: internal axial/shear/moment at `x` from the start node.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BeamStation {
    pub x: f64,
    pub n: f64,
    pub v: f64,
    pub m: f64,
}

/// 🧮 In-plane stress state at one Gauss point of a plane-stress/plane-strain continuum element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaneStress {
    pub sxx: f64,
    pub syy: f64,
    pub sxy: f64,
    pub von_mises: f64,
}

/// 🧊 Full 3D stress state at one Gauss point of a solid element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolidStress {
    pub sxx: f64,
    pub syy: f64,
    pub szz: f64,
    pub sxy: f64,
    pub syz: f64,
    pub sxz: f64,
    pub von_mises: f64,
}

/// 🧊 Bending moments per unit width at one Gauss point of a plate element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlateMoments {
    pub mx: f64,
    pub my: f64,
    pub mxy: f64,
}

/// 🐚 Membrane forces + bending moments per unit width at one Gauss point of a facet shell element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShellState {
    pub nxx: f64,
    pub nyy: f64,
    pub nxy: f64,
    pub mxx: f64,
    pub myy: f64,
    pub mxy: f64,
    pub von_mises_top: f64,
    pub von_mises_bottom: f64,
}

/// 📤 Element-kind-specific internal-force recovery. Only Gauss-point values live here — nodal
/// extrapolation/averaging across elements (for contour rendering) is a mesh-wide post-processing
/// step owned by `analyses`, not by any single element.
#[derive(Clone, Debug, PartialEq)]
pub enum ElementResult {
    Bar { n: f64 },
    Beam { stations: Vec<BeamStation> },
    Plane { gauss: Vec<PlaneStress> },
    Plate { gauss: Vec<PlateMoments> },
    Solid { gauss: Vec<SolidStress> },
    Shell { gauss: Vec<ShellState> },
}

/// ✅ Global sanity checks on the solved system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolutionChecks {
    pub residual_norm: f64,
    pub reaction_sum: [f64; 6],
}

/// 📈 The full result of a linear-static solve.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticResult {
    pub displacements: Vec<NodeDisplacement>,
    pub reactions: Vec<NodeReaction>,
    pub elements: Vec<(String, ElementResult)>,
    pub checks: SolutionChecks,
}
// #endregion 🔖Results

//#region ⚠️ Errors
/// ⚠️ Everything that can go wrong building or solving a [`Model`].
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum FemError {
    #[error("model has no nodes")]
    EmptyModel,
    #[error("duplicate node id: {0}")]
    DuplicateNodeId(String),
    #[error("reference to unknown node id: {0}")]
    DanglingNodeRef(String),
    #[error("stiffness matrix is singular — model is a mechanism or under-constrained")]
    Singular,
}
//#endregion ⚠️ Errors

// #region 🔖DofMap
struct DofMap {
    index: HashMap<(String, Dof), usize>,
    order: Vec<(String, Dof)>,
}

impl DofMap {
    fn get(&self, node_id: &str, dof: Dof) -> Option<usize> {
        self.index.get(&(node_id.to_string(), dof)).copied()
    }

    fn len(&self) -> usize {
        self.order.len()
    }
}

/// 🔢 Numbers each node's active DOFs (the union of `dofs_per_node()` over elements touching it),
/// so nodes with no rotational stiffness never get a spurious, singular rotational equation.
fn build_dof_map(model: &Model) -> DofMap {
    let mut order = Vec::new();
    let mut index = HashMap::new();
    for node in &model.nodes {
        let mut active: Vec<Dof> = Vec::new();
        for element in &model.elements {
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
            index.insert((node.id.clone(), dof), order.len());
            order.push((node.id.clone(), dof));
        }
    }
    DofMap { index, order }
}
// #endregion 🔖DofMap

// #region 🔖Validate
fn validate(model: &Model) -> Result<(), FemError> {
    if model.nodes.is_empty() {
        return Err(FemError::EmptyModel);
    }
    let mut seen = HashSet::new();
    for node in &model.nodes {
        if !seen.insert(node.id.clone()) {
            return Err(FemError::DuplicateNodeId(node.id.clone()));
        }
    }
    let node_exists = |id: &str| model.nodes.iter().any(|n| n.id == id);
    for element in &model.elements {
        for id in element.node_ids() {
            if !node_exists(&id) {
                return Err(FemError::DanglingNodeRef(id));
            }
        }
    }
    for support in &model.supports {
        if !node_exists(&support.node_id) {
            return Err(FemError::DanglingNodeRef(support.node_id.clone()));
        }
    }
    for load in &model.nodal_loads {
        if !node_exists(&load.node_id) {
            return Err(FemError::DanglingNodeRef(load.node_id.clone()));
        }
    }
    Ok(())
}
// #endregion 🔖Validate

// #region 🔖Assembly
fn positions_of(model: &Model, node_ids: &[String]) -> Vec<[f64; 3]> {
    node_ids.iter().map(|id| model.nodes.iter().find(|n| &n.id == id).map(|n| n.pos).unwrap_or_default()).collect()
}

fn element_global_indices(dof_map: &DofMap, node_ids: &[String], dofs: &[Dof]) -> Option<Vec<usize>> {
    let mut indices = Vec::with_capacity(node_ids.len() * dofs.len());
    for node_id in node_ids {
        for &dof in dofs {
            indices.push(dof_map.get(node_id, dof)?);
        }
    }
    Some(indices)
}

fn extract_submatrix(k: &MatD, rows: &[usize], cols: &[usize]) -> MatD {
    let mut out = MatD::zeros(rows.len(), cols.len());
    for (i, &row) in rows.iter().enumerate() {
        for (j, &col) in cols.iter().enumerate() {
            out.set(i, j, k.get(row, col));
        }
    }
    out
}
// #endregion 🔖Assembly

// #region 🔖Solve
/// 🧮 Assembles and solves the model for linear-static equilibrium `Ku = F`, partitioned by
/// support restraints (dense elimination — small/single-case scale; `analyses::solve_multi_case`
/// is the sparse multi-case/combination pipeline for larger models), then recovers reactions and
/// per-element internal forces from the solved displacement vector.
pub fn solve_linear_static(model: &Model) -> Result<StaticResult, FemError> {
    validate(model)?;
    let dof_map = build_dof_map(model);
    let ndof = dof_map.len();
    let mut k = MatD::zeros(ndof, ndof);
    let mut f = VecD::zeros(ndof);

    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let indices = match element_global_indices(&dof_map, &node_ids, dofs) {
            Some(indices) => indices,
            None => continue,
        };
        let ctx = ElementContext { positions: positions_of(model, &node_ids) };
        let ke = element.stiffness_global(&ctx);
        for (local_row, &global_row) in indices.iter().enumerate() {
            for (local_col, &global_col) in indices.iter().enumerate() {
                k.add_at(global_row, global_col, ke.get(local_row, local_col));
            }
        }
        if let Some((_, udl)) = model.member_loads.iter().find(|(id, _)| id.as_str() == element.id()) {
            if let Some(fe) = element.equivalent_nodal_loads(&ctx, udl) {
                for (local_row, &global_row) in indices.iter().enumerate() {
                    f.add_at(global_row, fe.get(local_row));
                }
            }
        }
    }

    for load in &model.nodal_loads {
        if let Some(idx) = dof_map.get(&load.node_id, load.dof) {
            f.add_at(idx, load.value);
        }
    }

    let mut constrained_set = BTreeSet::new();
    for support in &model.supports {
        for &dof in &support.fixed {
            if let Some(idx) = dof_map.get(&support.node_id, dof) {
                constrained_set.insert(idx);
            }
        }
    }
    let free: Vec<usize> = (0..ndof).filter(|i| !constrained_set.contains(i)).collect();
    let constrained: Vec<usize> = constrained_set.into_iter().collect();

    let kff = extract_submatrix(&k, &free, &free);
    let ff = VecD::from_vec(free.iter().map(|&i| f.get(i)).collect());
    let uf = kff.lu_solve(&ff).ok_or(FemError::Singular)?;

    let mut u = VecD::zeros(ndof);
    for (i, &idx) in free.iter().enumerate() {
        u.set(idx, uf.get(i));
    }

    let mut reactions = Vec::with_capacity(constrained.len());
    for &c in &constrained {
        let mut r = 0.0;
        for j in 0..ndof {
            r += k.get(c, j) * u.get(j);
        }
        r -= f.get(c);
        let (node_id, dof) = dof_map.order[c].clone();
        reactions.push(NodeReaction { node_id, dof, value: r });
    }

    let mut displacements: Vec<NodeDisplacement> = model.nodes.iter().map(|node| NodeDisplacement { node_id: node.id.clone(), values: [0.0; 6] }).collect();
    for (idx, (node_id, dof)) in dof_map.order.iter().enumerate() {
        if let Some(entry) = displacements.iter_mut().find(|d| &d.node_id == node_id) {
            entry.values[dof.index()] = u.get(idx);
        }
    }

    let mut elements = Vec::with_capacity(model.elements.len());
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let indices = match element_global_indices(&dof_map, &node_ids, dofs) {
            Some(indices) => indices,
            None => continue,
        };
        let ctx = ElementContext { positions: positions_of(model, &node_ids) };
        let u_local = VecD::from_vec(indices.iter().map(|&idx| u.get(idx)).collect());
        let udl = model.member_loads.iter().find(|(id, _)| id.as_str() == element.id()).map(|(_, udl)| udl);
        elements.push((element.id().to_string(), element.recover(&ctx, &u_local, udl)));
    }

    let mut reaction_sum = [0.0; 6];
    for r in &reactions {
        reaction_sum[r.dof.index()] += r.value;
    }
    for (idx, (_, dof)) in dof_map.order.iter().enumerate() {
        reaction_sum[dof.index()] += f.get(idx);
    }
    let ku = k.mul_vec(&u);
    let ku_free = VecD::from_vec(free.iter().map(|&i| ku.get(i)).collect());
    let residual_norm = ku_free.sub(&ff).norm2() / ff.norm2().max(1e-9);

    Ok(StaticResult { displacements, reactions, elements, checks: SolutionChecks { residual_norm, reaction_sum } })
}
// #endregion 🔖Solve

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    struct AxialSpring {
        id: String,
        a: String,
        b: String,
        k: f64,
    }

    impl Element for AxialSpring {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            vec![self.a.clone(), self.b.clone()]
        }

        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx]
        }

        fn stiffness_global(&self, _ctx: &ElementContext) -> MatD {
            let mut m = MatD::zeros(2, 2);
            m.set(0, 0, self.k);
            m.set(0, 1, -self.k);
            m.set(1, 0, -self.k);
            m.set(1, 1, self.k);
            m
        }

        fn recover(&self, _ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            ElementResult::Bar { n: self.k * (u_local.get(1) - u_local.get(0)) }
        }
    }

    fn two_spring_model() -> Model {
        Model {
            nodes: vec![Node { id: "n1".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "n2".into(), pos: [1.0, 0.0, 0.0] }],
            elements: vec![Box::new(AxialSpring { id: "e1".into(), a: "n1".into(), b: "n2".into(), k: 1000.0 })],
            supports: vec![Support { node_id: "n1".into(), fixed: vec![Dof::Tx] }],
            nodal_loads: vec![NodalLoad { node_id: "n2".into(), dof: Dof::Tx, value: 10.0 }],
            member_loads: vec![],
        }
    }

    #[test]
    fn solves_single_spring_against_hand_calc() {
        let model = two_spring_model();
        let result = solve_linear_static(&model).expect("solves");
        let n2 = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
        assert!((n2.values[Dof::Tx.index()] - 0.01).abs() < 1e-9);
        let reaction = result.reactions.iter().find(|r| r.node_id == "n1").unwrap();
        assert!((reaction.value + 10.0).abs() < 1e-9);
        match &result.elements[0].1 {
            ElementResult::Bar { n } => assert!((n - 10.0).abs() < 1e-9),
            _ => panic!("expected bar result"),
        }
    }

    #[test]
    fn equilibrium_checks_are_near_zero() {
        let model = two_spring_model();
        let result = solve_linear_static(&model).expect("solves");
        assert!(result.checks.residual_norm < 1e-9);
        assert!(result.checks.reaction_sum[Dof::Tx.index()].abs() < 1e-9);
    }

    #[test]
    fn empty_model_is_rejected() {
        let model = Model::default();
        assert_eq!(solve_linear_static(&model), Err(FemError::EmptyModel));
    }

    #[test]
    fn dangling_node_ref_is_rejected() {
        let mut model = two_spring_model();
        model.supports.push(Support { node_id: "missing".into(), fixed: vec![Dof::Tx] });
        assert_eq!(solve_linear_static(&model), Err(FemError::DanglingNodeRef("missing".into())));
    }

    #[test]
    fn unconstrained_model_is_singular() {
        let mut model = two_spring_model();
        model.supports.clear();
        assert_eq!(solve_linear_static(&model), Err(FemError::Singular));
    }

    #[test]
    fn load_on_inactive_dof_is_silently_skipped() {
        let mut model = two_spring_model();
        model.nodal_loads.push(NodalLoad { node_id: "n2".into(), dof: Dof::Ty, value: 999.0 });
        let result = solve_linear_static(&model).expect("solves despite inactive-dof load");
        let n2 = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
        assert!((n2.values[Dof::Tx.index()] - 0.01).abs() < 1e-9);
    }
}
// #endregion 🔖Tests
