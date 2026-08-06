//! 📈️ Multi-case/combination linear-static analysis, self-weight load generation, modal analysis
//! (frequencies/shapes), linear buckling, and nodal-averaged stress recovery for contour rendering
//! (`nodal_averaged_scalar`) — all sparse-backed (RCM-ordered, single LDLT factorization shared
//! across every load case / eigen-solve).

use crate::core::sparse::{ldlt_factor, rcm_order, subspace_iteration, Coo, Csr, EigenPairs, LdltFactor};
use crate::core::{BeamStation, Dof, Element, ElementContext, ElementResult, FemError, MemberUdl, NodalLoad, Node, NodeDisplacement, NodeReaction, PlaneStress, PlateMoments, ShellState, SolidStress, SolutionChecks, StaticResult, Support};
use math::algebra::{MatD, VecD};
use std::collections::{HashMap, HashSet};

// #region 🔖️Model
/// 📦️ A named load case: nodal loads, member UDLs, and an optional self-weight contribution.
pub struct LoadCase {
    pub id: String,
    pub nodal_loads: Vec<NodalLoad>,
    pub member_loads: Vec<(String, MemberUdl)>,
    pub self_weight: bool,
}

/// 📦️ A linear combination of load cases — `Σ factor_i * case_i`, superposed from already-solved
/// case results (no re-solve).
pub struct Combination {
    pub id: String,
    pub terms: Vec<(String, f64)>,
}

/// 🏗️ Model geometry for multi-case/modal/buckling analysis — no loads (those come from `LoadCase`).
pub struct AnalysisModel {
    pub nodes: Vec<Node>,
    pub elements: Vec<Box<dyn Element>>,
    pub supports: Vec<Support>,
}

/// 📐️ The lowest modes of a `modal` analysis — `shapes[i]` is node-major matching `model.nodes`
/// order, DOF sub-order `Tx,Ty,Tz,Rx,Ry,Rz` filtered to each node's active DOFs (the same layout
/// `StaticResult`'s node list implies), zero at every constrained DOF.
pub struct ModalResult {
    pub frequencies_hz: Vec<f64>,
    pub shapes: Vec<VecD>,
}

/// 📐️ The lowest linear-buckling load factors of a `buckling` analysis — `factors[i] * reference_case`
/// is the critical load; `shapes[i]` uses the same layout as `ModalResult::shapes`.
pub struct BucklingResult {
    pub factors: Vec<f64>,
    pub shapes: Vec<VecD>,
}
// #endregion 🔖️Model

// #region 🔖️DofMap
/// 🔢️ Numbers each node's active DOFs (the union of `dofs_per_node()` over elements touching it) —
/// a small, self-contained reimplementation of `lib.rs`'s private `build_dof_map`/`DofMap` (not
/// `pub`, so not importable here), kept byte-for-byte equivalent in ordering behavior.
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

fn build_dof_map(nodes: &[Node], elements: &[Box<dyn Element>]) -> DofMap {
    let mut order = Vec::new();
    let mut index = HashMap::new();
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
            index.insert((node.id.clone(), dof), order.len());
            order.push((node.id.clone(), dof));
        }
    }
    DofMap { index, order }
}

fn positions_of(nodes: &[Node], node_ids: &[String]) -> Vec<[f64; 3]> {
    node_ids.iter().map(|id| nodes.iter().find(|n| &n.id == id).map(|n| n.pos).unwrap_or_default()).collect()
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
// #endregion 🔖️DofMap

// #region 🔖️Validate
fn validate(model: &AnalysisModel) -> Result<(), FemError> {
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
    Ok(())
}

fn validate_case(model: &AnalysisModel, case: &LoadCase) -> Result<(), FemError> {
    let node_exists = |id: &str| model.nodes.iter().any(|n| n.id == id);
    for load in &case.nodal_loads {
        if !node_exists(&load.node_id) {
            return Err(FemError::DanglingNodeRef(load.node_id.clone()));
        }
    }
    Ok(())
}
// #endregion 🔖️Validate

// #region 🔖️Rcm
/// 🌀️ Node-index RCM permutation, expanded to DOF granularity: each node's active DOFs stay
/// contiguous, positioned at its node's new RCM slot. `inv_perm[old_idx] = new_idx` (the only
/// direction callers need — un-permuting walks `old_idx` and looks up its new slot).
struct RcmPermutation {
    inv_perm: Vec<usize>,
}

fn build_rcm_permutation(nodes: &[Node], elements: &[Box<dyn Element>], dof_map: &DofMap) -> RcmPermutation {
    let n_nodes = nodes.len();
    let node_index: HashMap<&str, usize> = nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n_nodes];
    let mut seen_edges: HashSet<(usize, usize)> = HashSet::new();
    for element in elements {
        let ids = element.node_ids();
        let idxs: Vec<usize> = ids.iter().filter_map(|id| node_index.get(id.as_str()).copied()).collect();
        for i in 0..idxs.len() {
            for j in (i + 1)..idxs.len() {
                let (a, b) = (idxs[i], idxs[j]);
                if a != b {
                    let key = (a.min(b), a.max(b));
                    if seen_edges.insert(key) {
                        adjacency[a].push(b);
                        adjacency[b].push(a);
                    }
                }
            }
        }
    }
    let node_perm = rcm_order(&adjacency);

    // `dof_map.order` is grouped by node in `nodes`' own iteration order (see `build_dof_map`), so
    // each original node index owns one contiguous run — walk it once to find each run's bounds.
    let mut node_dof_ranges: Vec<(usize, usize)> = vec![(0, 0); n_nodes];
    let mut cursor = 0usize;
    for (i, node) in nodes.iter().enumerate() {
        let mut count = 0;
        while cursor + count < dof_map.order.len() && dof_map.order[cursor + count].0 == node.id {
            count += 1;
        }
        node_dof_ranges[i] = (cursor, count);
        cursor += count;
    }

    let ndof = dof_map.len();
    let mut rcm_perm = Vec::with_capacity(ndof);
    for &old_node_idx in &node_perm {
        let (start, count) = node_dof_ranges[old_node_idx];
        for k in 0..count {
            rcm_perm.push(start + k);
        }
    }
    let mut inv_perm = vec![0usize; ndof];
    for (new_idx, &old_idx) in rcm_perm.iter().enumerate() {
        inv_perm[old_idx] = new_idx;
    }
    RcmPermutation { inv_perm }
}
// #endregion 🔖️Rcm

// #region 🔖️Assembly
/// 🧮️ The shared, once-per-model assembly: DOF map, RCM permutation, free/constrained partition
/// (partitioned BEFORE assembly per the design — only free×free entries feed the LDLT factor), and
/// both the free-free `LdltFactor` (for solves) and the full `Csr` (for reactions/residuals).
struct AssembledSystem {
    dof_map: DofMap,
    inv_perm: Vec<usize>,
    ndof: usize,
    free_new: Vec<usize>,
    compact_of_new: Vec<Option<usize>>,
    k_factor: LdltFactor,
    k_full: Csr,
}

impl AssembledSystem {
    fn n_free(&self) -> usize {
        self.free_new.len()
    }
}

fn assemble_system(model: &AnalysisModel) -> Result<AssembledSystem, FemError> {
    validate(model)?;
    let dof_map = build_dof_map(&model.nodes, &model.elements);
    let ndof = dof_map.len();
    let perm = build_rcm_permutation(&model.nodes, &model.elements, &dof_map);

    let mut constrained_old: HashSet<usize> = HashSet::new();
    for support in &model.supports {
        for &dof in &support.fixed {
            if let Some(idx) = dof_map.get(&support.node_id, dof) {
                constrained_old.insert(idx);
            }
        }
    }
    let constrained_new: HashSet<usize> = constrained_old.iter().map(|&old| perm.inv_perm[old]).collect();
    let free_new: Vec<usize> = (0..ndof).filter(|new_idx| !constrained_new.contains(new_idx)).collect();
    let mut compact_of_new: Vec<Option<usize>> = vec![None; ndof];
    for (k, &new_idx) in free_new.iter().enumerate() {
        compact_of_new[new_idx] = Some(k);
    }
    let n_free = free_new.len();

    let mut k_full_coo = Coo::new(ndof);
    let mut k_ff_coo = Coo::new(n_free);

    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices_old) = element_global_indices(&dof_map, &node_ids, dofs) else { continue };
        let indices_new: Vec<usize> = indices_old.iter().map(|&old| perm.inv_perm[old]).collect();
        let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };
        let ke = element.stiffness_global(&ctx);
        k_full_coo.add_block(&indices_new, &ke);

        for (local_row, &new_row) in indices_new.iter().enumerate() {
            let Some(compact_row) = compact_of_new[new_row] else { continue };
            for (local_col, &new_col) in indices_new.iter().enumerate() {
                let Some(compact_col) = compact_of_new[new_col] else { continue };
                let v = ke.get(local_row, local_col);
                if v != 0.0 {
                    k_ff_coo.add(compact_row, compact_col, v);
                }
            }
        }
    }

    let k_full = k_full_coo.to_csr();
    let k_factor = ldlt_factor(&k_ff_coo.to_csc_sym_upper()).map_err(|_| FemError::Singular)?;

    Ok(AssembledSystem { dof_map, inv_perm: perm.inv_perm, ndof, free_new, compact_of_new, k_factor, k_full })
}

/// 🌬️ Per-node gravity pattern for an element's own `dofs_per_node()` layout — `[gx,gy,gz]` placed at
/// each node's active `Tx/Ty/Tz` slots, `0.0` at any `Rx/Ry/Rz` slots, repeated node-major.
fn gravity_pattern(node_count: usize, dofs: &[Dof], gravity: [f64; 3]) -> VecD {
    let mut out = VecD::zeros(node_count * dofs.len());
    for n in 0..node_count {
        for (i, &dof) in dofs.iter().enumerate() {
            let g = match dof {
                Dof::Tx => gravity[0],
                Dof::Ty => gravity[1],
                Dof::Tz => gravity[2],
                Dof::Rx | Dof::Ry | Dof::Rz => 0.0,
            };
            out.set(n * dofs.len() + i, g);
        }
    }
    out
}

/// 🌬️ Assembles one load case's RHS in ORIGINAL (old) DOF-index space — nodal loads, member-UDL
/// equivalent loads, and (if `self_weight`) `element.mass() · gravity_pattern` self-weight loads.
fn case_rhs_old(model: &AnalysisModel, dof_map: &DofMap, case: &LoadCase, gravity: [f64; 3]) -> VecD {
    let ndof = dof_map.len();
    let mut f = VecD::zeros(ndof);

    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices) = element_global_indices(dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };

        if let Some((_, udl)) = case.member_loads.iter().find(|(id, _)| id.as_str() == element.id()) {
            if let Some(fe) = element.equivalent_nodal_loads(&ctx, udl) {
                for (local_row, &global_row) in indices.iter().enumerate() {
                    f.add_at(global_row, fe.get(local_row));
                }
            }
        }

        if case.self_weight {
            if let Some(me) = element.mass(&ctx) {
                let gpat = gravity_pattern(node_ids.len(), dofs, gravity);
                let fw = me.mul_vec(&gpat);
                for (local_row, &global_row) in indices.iter().enumerate() {
                    f.add_at(global_row, fw.get(local_row));
                }
            }
        }
    }

    for load in &case.nodal_loads {
        if let Some(idx) = dof_map.get(&load.node_id, load.dof) {
            f.add_at(idx, load.value);
        }
    }

    f
}
// #endregion 🔖️Assembly

// #region 🔖️Combine
/// 🌱️ A zero-valued `ElementResult` of the same variant/shape as `result` — the seed for superposition.
fn zero_like(result: &ElementResult) -> ElementResult {
    match result {
        ElementResult::Bar { .. } => ElementResult::Bar { n: 0.0 },
        ElementResult::Beam { stations } => ElementResult::Beam { stations: stations.iter().map(|s| BeamStation { x: s.x, n: 0.0, v: 0.0, m: 0.0 }).collect() },
        ElementResult::Plane { gauss } => ElementResult::Plane { gauss: gauss.iter().map(|_| PlaneStress { sxx: 0.0, syy: 0.0, sxy: 0.0, von_mises: 0.0 }).collect() },
        ElementResult::Plate { gauss } => ElementResult::Plate { gauss: gauss.iter().map(|_| PlateMoments { mx: 0.0, my: 0.0, mxy: 0.0 }).collect() },
        ElementResult::Solid { gauss } => ElementResult::Solid { gauss: gauss.iter().map(|_| SolidStress { sxx: 0.0, syy: 0.0, szz: 0.0, sxy: 0.0, syz: 0.0, sxz: 0.0, von_mises: 0.0 }).collect() },
        ElementResult::Shell { gauss } => ElementResult::Shell { gauss: gauss.iter().map(|_| ShellState { nxx: 0.0, nyy: 0.0, nxy: 0.0, mxx: 0.0, myy: 0.0, mxy: 0.0, von_mises_top: 0.0, von_mises_bottom: 0.0 }).collect() },
    }
}

/// ➕️ `acc + factor * term`, field-by-field, matched by `ElementResult` variant and Gauss-point index.
fn add_scaled_element_result(acc: &ElementResult, term: &ElementResult, factor: f64) -> ElementResult {
    match (acc, term) {
        (ElementResult::Bar { n: an }, ElementResult::Bar { n: tn }) => ElementResult::Bar { n: an + factor * tn },
        (ElementResult::Beam { stations: acc_s }, ElementResult::Beam { stations: term_s }) => {
            ElementResult::Beam { stations: acc_s.iter().zip(term_s.iter()).map(|(a, t)| BeamStation { x: a.x, n: a.n + factor * t.n, v: a.v + factor * t.v, m: a.m + factor * t.m }).collect() }
        }
        (ElementResult::Plane { gauss: acc_g }, ElementResult::Plane { gauss: term_g }) => ElementResult::Plane {
            gauss: acc_g.iter().zip(term_g.iter()).map(|(a, t)| PlaneStress { sxx: a.sxx + factor * t.sxx, syy: a.syy + factor * t.syy, sxy: a.sxy + factor * t.sxy, von_mises: a.von_mises + factor * t.von_mises }).collect(),
        },
        (ElementResult::Plate { gauss: acc_g }, ElementResult::Plate { gauss: term_g }) => {
            ElementResult::Plate { gauss: acc_g.iter().zip(term_g.iter()).map(|(a, t)| PlateMoments { mx: a.mx + factor * t.mx, my: a.my + factor * t.my, mxy: a.mxy + factor * t.mxy }).collect() }
        }
        (ElementResult::Solid { gauss: acc_g }, ElementResult::Solid { gauss: term_g }) => ElementResult::Solid {
            gauss: acc_g
                .iter()
                .zip(term_g.iter())
                .map(|(a, t)| SolidStress {
                    sxx: a.sxx + factor * t.sxx,
                    syy: a.syy + factor * t.syy,
                    szz: a.szz + factor * t.szz,
                    sxy: a.sxy + factor * t.sxy,
                    syz: a.syz + factor * t.syz,
                    sxz: a.sxz + factor * t.sxz,
                    von_mises: a.von_mises + factor * t.von_mises,
                })
                .collect(),
        },
        (ElementResult::Shell { gauss: acc_g }, ElementResult::Shell { gauss: term_g }) => ElementResult::Shell {
            gauss: acc_g
                .iter()
                .zip(term_g.iter())
                .map(|(a, t)| ShellState {
                    nxx: a.nxx + factor * t.nxx,
                    nyy: a.nyy + factor * t.nyy,
                    nxy: a.nxy + factor * t.nxy,
                    mxx: a.mxx + factor * t.mxx,
                    myy: a.myy + factor * t.myy,
                    mxy: a.mxy + factor * t.mxy,
                    von_mises_top: a.von_mises_top + factor * t.von_mises_top,
                    von_mises_bottom: a.von_mises_bottom + factor * t.von_mises_bottom,
                })
                .collect(),
        },
        _ => acc.clone(),
    }
}

fn combine_results(case_results: &[StaticResult], cases: &[LoadCase], combo: &Combination) -> Result<StaticResult, FemError> {
    let mut displacements: Vec<NodeDisplacement> = Vec::new();
    let mut reactions: Vec<NodeReaction> = Vec::new();
    let mut elements: Vec<(String, ElementResult)> = Vec::new();
    let mut reaction_sum = [0.0; 6];
    let mut residual_norm = 0.0;
    let mut seeded = false;

    for (case_id, factor) in &combo.terms {
        let idx = cases.iter().position(|c| &c.id == case_id).ok_or_else(|| FemError::DanglingNodeRef(case_id.clone()))?;
        let cr = &case_results[idx];
        if !seeded {
            displacements = cr.displacements.iter().map(|d| NodeDisplacement { node_id: d.node_id.clone(), values: [0.0; 6] }).collect();
            elements = cr.elements.iter().map(|(id, r)| (id.clone(), zero_like(r))).collect();
            seeded = true;
        }
        for (i, d) in cr.displacements.iter().enumerate() {
            for k in 0..6 {
                displacements[i].values[k] += factor * d.values[k];
            }
        }
        for r in &cr.reactions {
            if let Some(existing) = reactions.iter_mut().find(|e: &&mut NodeReaction| e.node_id == r.node_id && e.dof == r.dof) {
                existing.value += factor * r.value;
            } else {
                reactions.push(NodeReaction { node_id: r.node_id.clone(), dof: r.dof, value: factor * r.value });
            }
        }
        for (i, (_, res)) in cr.elements.iter().enumerate() {
            elements[i].1 = add_scaled_element_result(&elements[i].1, res, *factor);
        }
        for k in 0..6 {
            reaction_sum[k] += factor * cr.checks.reaction_sum[k];
        }
        residual_norm += factor.abs() * cr.checks.residual_norm;
    }

    Ok(StaticResult { displacements, reactions, elements, checks: SolutionChecks { residual_norm, reaction_sum } })
}
// #endregion 🔖️Combine

// #region 🔖️SolveMultiCase
/// 🧮️ Assembles the model ONCE (sparse, RCM-ordered, free-free LDLT factored once), then solves every
/// load case as one shared multi-RHS `solve_many` call, superposes `combinations` from the already-
/// solved case results, and un-permutes everything back to original node identity.
pub fn solve_multi_case(model: &AnalysisModel, cases: &[LoadCase], combinations: &[Combination], gravity: [f64; 3]) -> Result<HashMap<String, StaticResult>, FemError> {
    for case in cases {
        validate_case(model, case)?;
    }
    let system = assemble_system(model)?;
    let dof_map = &system.dof_map;
    let ndof = system.ndof;
    let n_free = system.n_free();

    let rhs_full_old: Vec<VecD> = cases.iter().map(|case| case_rhs_old(model, dof_map, case, gravity)).collect();

    let mut rhs_compact = MatD::zeros(n_free, cases.len().max(1));
    for (c, f_old) in rhs_full_old.iter().enumerate() {
        for old_idx in 0..ndof {
            let new_idx = system.inv_perm[old_idx];
            if let Some(compact) = system.compact_of_new[new_idx] {
                rhs_compact.set(compact, c, f_old.get(old_idx));
            }
        }
    }
    let u_compact = system.k_factor.solve_many(&rhs_compact);

    let mut results: HashMap<String, StaticResult> = HashMap::new();
    let mut case_results: Vec<StaticResult> = Vec::with_capacity(cases.len());

    for (c, case) in cases.iter().enumerate() {
        let mut u_new = VecD::zeros(ndof);
        for (k, &new_idx) in system.free_new.iter().enumerate() {
            u_new.set(new_idx, u_compact.get(k, c));
        }
        let f_old = &rhs_full_old[c];
        let mut f_new = VecD::zeros(ndof);
        for old_idx in 0..ndof {
            f_new.set(system.inv_perm[old_idx], f_old.get(old_idx));
        }
        let ku_new = system.k_full.mul_vec(&u_new);

        let mut reactions = Vec::new();
        for old_idx in 0..ndof {
            let new_idx = system.inv_perm[old_idx];
            if system.compact_of_new[new_idx].is_none() {
                let r = ku_new.get(new_idx) - f_new.get(new_idx);
                let (node_id, dof) = dof_map.order[old_idx].clone();
                reactions.push(NodeReaction { node_id, dof, value: r });
            }
        }

        let mut displacements: Vec<NodeDisplacement> = model.nodes.iter().map(|n| NodeDisplacement { node_id: n.id.clone(), values: [0.0; 6] }).collect();
        for (old_idx, (node_id, dof)) in dof_map.order.iter().enumerate() {
            let new_idx = system.inv_perm[old_idx];
            if let Some(entry) = displacements.iter_mut().find(|d| &d.node_id == node_id) {
                entry.values[dof.index()] = u_new.get(new_idx);
            }
        }

        let mut elements_out = Vec::with_capacity(model.elements.len());
        for element in &model.elements {
            let node_ids = element.node_ids();
            let dofs = element.dofs_per_node();
            let Some(indices_old) = element_global_indices(dof_map, &node_ids, dofs) else { continue };
            let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };
            let u_local = VecD::from_vec(indices_old.iter().map(|&old| u_new.get(system.inv_perm[old])).collect());
            let udl = case.member_loads.iter().find(|(id, _)| id.as_str() == element.id()).map(|(_, udl)| udl);
            elements_out.push((element.id().to_string(), element.recover(&ctx, &u_local, udl)));
        }

        let mut reaction_sum = [0.0; 6];
        for r in &reactions {
            reaction_sum[r.dof.index()] += r.value;
        }
        for old_idx in 0..ndof {
            let (_, dof) = &dof_map.order[old_idx];
            reaction_sum[dof.index()] += f_old.get(old_idx);
        }
        let free_ku = VecD::from_vec(system.free_new.iter().map(|&new_idx| ku_new.get(new_idx)).collect());
        let free_f = VecD::from_vec(system.free_new.iter().map(|&new_idx| f_new.get(new_idx)).collect());
        let residual_norm = free_ku.sub(&free_f).norm2() / free_f.norm2().max(1e-9);

        let result = StaticResult { displacements, reactions, elements: elements_out, checks: SolutionChecks { residual_norm, reaction_sum } };
        case_results.push(result.clone());
        results.insert(case.id.clone(), result);
    }

    for combo in combinations {
        let combined = combine_results(&case_results, cases, combo)?;
        results.insert(combo.id.clone(), combined);
    }

    Ok(results)
}
// #endregion 🔖️SolveMultiCase

// #region 🔖️Modal
/// 🎯️ Modal analysis: shares `solve_multi_case`'s sparse RCM-ordered free-free LDLT factor, assembles
/// the global mass matrix over the SAME free DOFs (elements with `mass() == None` contribute nothing),
/// and calls `subspace_iteration` for the lowest `count` frequencies/shapes.
pub fn modal(model: &AnalysisModel, count: usize) -> Result<ModalResult, FemError> {
    let system = assemble_system(model)?;
    let ndof = system.ndof;
    let n_free = system.n_free();

    let mut m_coo = Coo::new(n_free);
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices_old) = element_global_indices(&system.dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };
        let Some(me) = element.mass(&ctx) else { continue };
        let indices_new: Vec<usize> = indices_old.iter().map(|&old| system.inv_perm[old]).collect();
        for (local_row, &new_row) in indices_new.iter().enumerate() {
            let Some(compact_row) = system.compact_of_new[new_row] else { continue };
            for (local_col, &new_col) in indices_new.iter().enumerate() {
                let Some(compact_col) = system.compact_of_new[new_col] else { continue };
                let v = me.get(local_row, local_col);
                if v != 0.0 {
                    m_coo.add(compact_row, compact_col, v);
                }
            }
        }
    }
    let m_csr = m_coo.to_csr();

    let pairs: EigenPairs = subspace_iteration(&system.k_factor, &m_csr, n_free, count, 30);
    let frequencies_hz: Vec<f64> = pairs.values.iter().map(|&lambda| lambda.max(0.0).sqrt() / (2.0 * std::f64::consts::PI)).collect();
    let shapes = unpermute_shapes(&system, ndof, &pairs.vectors);

    Ok(ModalResult { frequencies_hz, shapes })
}

/// 🔁️ Expands each compact free-DOF eigenvector back to full `ndof` (zero at constrained slots), then
/// un-permutes RCM (new) index space back to the ORIGINAL `dof_map` order (node-major, matching
/// `model.nodes`, DOF sub-order filtered to active DOFs).
fn unpermute_shapes(system: &AssembledSystem, ndof: usize, vectors: &[VecD]) -> Vec<VecD> {
    vectors
        .iter()
        .map(|vec_compact| {
            let mut u_new = VecD::zeros(ndof);
            for (k, &new_idx) in system.free_new.iter().enumerate() {
                u_new.set(new_idx, vec_compact.get(k));
            }
            let mut shape = VecD::zeros(ndof);
            for old_idx in 0..ndof {
                shape.set(old_idx, u_new.get(system.inv_perm[old_idx]));
            }
            shape
        })
        .collect()
}
// #endregion 🔖️Modal

// #region 🔖️Buckling
/// 🌀️ Linear buckling: solves `reference_case` (via `solve_multi_case`) for `u_ref`, assembles the
/// geometric stiffness `Kg` from every element's own axial state under `u_ref`, then solves
/// `K φ = λ (−Kg) φ` via `subspace_iteration` — `factors[i] * reference_case` is the i-th critical load.
pub fn buckling(model: &AnalysisModel, reference_case: &LoadCase, count: usize) -> Result<BucklingResult, FemError> {
    let ref_results = solve_multi_case(model, std::slice::from_ref(reference_case), &[], [0.0, 0.0, 0.0])?;
    let ref_result = ref_results.get(&reference_case.id).expect("reference case was just solved");

    let system = assemble_system(model)?;
    let ndof = system.ndof;
    let n_free = system.n_free();

    let mut neg_kg_coo = Coo::new(n_free);
    let mut diag_estimate = vec![0.0f64; n_free];
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices_old) = element_global_indices(&system.dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };

        let mut u_element = VecD::zeros(indices_old.len());
        for (i, &old_idx) in indices_old.iter().enumerate() {
            let (node_id, dof) = &system.dof_map.order[old_idx];
            let d = ref_result.displacements.iter().find(|d| &d.node_id == node_id).expect("node exists in reference result");
            u_element.set(i, d.values[dof.index()]);
        }

        let Some(kg) = element.geometric_stiffness(&ctx, &u_element) else { continue };
        let indices_new: Vec<usize> = indices_old.iter().map(|&old| system.inv_perm[old]).collect();
        for (local_row, &new_row) in indices_new.iter().enumerate() {
            let Some(compact_row) = system.compact_of_new[new_row] else { continue };
            for (local_col, &new_col) in indices_new.iter().enumerate() {
                let Some(compact_col) = system.compact_of_new[new_col] else { continue };
                let v = kg.get(local_row, local_col);
                if v != 0.0 {
                    neg_kg_coo.add(compact_row, compact_col, -v);
                    if compact_row == compact_col {
                        diag_estimate[compact_row] += v.abs();
                    }
                }
            }
        }
    }
    // 🩹️ Frame/truss `geometric_stiffness` (bar/beam bending block, truss `N/L·(I−ccᵀ)` transverse
    // projector) still leaves SOME directions exactly unstressed (bending elements' own axial DOF,
    // `PlateDkt`'s entire DOF set — see its struct doc — and any drilling/rotational DOF no element's
    // Kg touches), so the assembled `−Kg` can still be singular or near-singular along those
    // directions even now that continuum/solid/shell elements contribute a full Kg of their own.
    // `subspace_iteration`'s B-orthonormalization divides by `sqrt(x·Bx)`, which blows up (→ NaN) for
    // any seed vector with a nonzero component in an exact null space. A tiny diagonal regularization
    // (Tikhonov-style, scaled off the assembled `−Kg`'s own diagonal magnitude) makes `−Kg` strictly
    // positive-definite everywhere without perturbing the physically meaningful lowest eigenvalues,
    // which are orders of magnitude below the huge spurious eigenvalues this regularization assigns
    // to the null-space directions.
    let max_diag = diag_estimate.iter().cloned().fold(0.0_f64, f64::max);
    let eps = max_diag.max(1e-12) * 1e-6;
    for i in 0..n_free {
        neg_kg_coo.add(i, i, eps);
    }
    let neg_kg_csr = neg_kg_coo.to_csr();

    let pairs: EigenPairs = subspace_iteration(&system.k_factor, &neg_kg_csr, n_free, count, 30);
    let shapes = unpermute_shapes(&system, ndof, &pairs.vectors);

    Ok(BucklingResult { factors: pairs.values, shapes })
}
// #endregion 🔖️Buckling

// #region 🔖️NodalAveraging
/// 🎨️ A scalar quantity `nodal_averaged_scalar` can recover from an `ElementResult`, for contour
/// rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StressScalar {
    VonMises,
    Sxx,
    Syy,
    Sxy,
    Szz,
    Syz,
    Sxz,
    VonMisesTop,
    VonMisesBottom,
}

/// 📊️ An element's own Gauss-point-averaged value of `scalar`, or `None` if that element kind/scalar
/// combination isn't defined (e.g. `VonMisesTop` on a `Plane` result, or any scalar on a `Bar`/`Beam`
/// result — those carry no stress tensor to project).
fn element_scalar_average(result: &ElementResult, scalar: StressScalar) -> Option<f64> {
    fn avg(values: impl Iterator<Item = f64>) -> f64 {
        let mut sum = 0.0;
        let mut count = 0usize;
        for v in values {
            sum += v;
            count += 1;
        }
        sum / (count.max(1) as f64)
    }
    match result {
        ElementResult::Plane { gauss } => match scalar {
            StressScalar::VonMises => Some(avg(gauss.iter().map(|g| g.von_mises))),
            StressScalar::Sxx => Some(avg(gauss.iter().map(|g| g.sxx))),
            StressScalar::Syy => Some(avg(gauss.iter().map(|g| g.syy))),
            StressScalar::Sxy => Some(avg(gauss.iter().map(|g| g.sxy))),
            _ => None,
        },
        ElementResult::Solid { gauss } => match scalar {
            StressScalar::VonMises => Some(avg(gauss.iter().map(|g| g.von_mises))),
            StressScalar::Sxx => Some(avg(gauss.iter().map(|g| g.sxx))),
            StressScalar::Syy => Some(avg(gauss.iter().map(|g| g.syy))),
            StressScalar::Szz => Some(avg(gauss.iter().map(|g| g.szz))),
            StressScalar::Sxy => Some(avg(gauss.iter().map(|g| g.sxy))),
            StressScalar::Syz => Some(avg(gauss.iter().map(|g| g.syz))),
            StressScalar::Sxz => Some(avg(gauss.iter().map(|g| g.sxz))),
            _ => None,
        },
        ElementResult::Shell { gauss } => match scalar {
            StressScalar::VonMisesTop => Some(avg(gauss.iter().map(|g| g.von_mises_top))),
            StressScalar::VonMisesBottom => Some(avg(gauss.iter().map(|g| g.von_mises_bottom))),
            _ => None,
        },
        _ => None,
    }
}

/// 🎨️ Nodal-averaged contour values: each element's OWN Gauss-point average of `scalar` (constant
/// across Gauss points for a 1-point-integrated `Tri3Cst`, a genuine average for higher-order
/// elements — deliberately NOT a polynomial extrapolation-to-nodes, a simple scope choice) is
/// accumulated, UNWEIGHTED (by element count, not by tributary area/volume), into every node it
/// touches; the returned value per node is that accumulation's mean. A node touched only by elements
/// that report no value for `scalar` (e.g. a `Bar` in a mixed mesh) simply never appears in the map.
/// Element-to-model matching is by `element.id()` against `result.elements`' ids.
pub fn nodal_averaged_scalar(model: &AnalysisModel, result: &StaticResult, scalar: StressScalar) -> HashMap<String, f64> {
    let mut sums: HashMap<String, (f64, usize)> = HashMap::new();
    for (element_id, element_result) in &result.elements {
        let Some(value) = element_scalar_average(element_result, scalar) else { continue };
        let Some(element) = model.elements.iter().find(|e| e.id() == element_id) else { continue };
        for node_id in element.node_ids() {
            let entry = sums.entry(node_id).or_insert((0.0, 0));
            entry.0 += value;
            entry.1 += 1;
        }
    }
    sums.into_iter().map(|(node_id, (sum, count))| (node_id, sum / count as f64)).collect()
}
// #endregion 🔖️NodalAveraging

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::elements2d::{Bar2, BeamEb2};
    use crate::core::{solve_linear_static, Model};

    fn cantilever_analysis_model(e: f64, area: f64, iy: f64, l: f64, density: f64) -> (AnalysisModel, Vec<LoadCase>) {
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Box::new(BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
        };
        let cases = vec![LoadCase { id: "tip_load".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -1000.0 }], member_loads: vec![], self_weight: false }];
        (model, cases)
    }

    /// 🧮️ Cross-validates `solve_multi_case`'s sparse RCM-ordered pipeline (single case) against
    /// `solve_linear_static`'s already-correct dense pipeline on an equivalent model — same oracle
    /// strategy already used elsewhere in this crate.
    #[test]
    fn solve_multi_case_matches_single_case_dense_solve() {
        let (e, area, iy, l) = (200e9, 0.01, 1e-5, 2.0);
        let (model, cases) = cantilever_analysis_model(e, area, iy, l, 0.0);
        let results = solve_multi_case(&model, &cases, &[], [0.0, 0.0, 0.0]).expect("solves");
        let sparse_result = results.get("tip_load").expect("case present");

        let dense_model = Model {
            nodes: model.nodes.clone(),
            elements: vec![Box::new(BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 })],
            supports: model.supports.clone(),
            nodal_loads: cases[0].nodal_loads.clone(),
            member_loads: vec![],
        };
        let dense_result = solve_linear_static(&dense_model).expect("dense solves");

        for sd in &sparse_result.displacements {
            let dd = dense_result.displacements.iter().find(|d| d.node_id == sd.node_id).unwrap();
            for k in 0..6 {
                assert!((sd.values[k] - dd.values[k]).abs() < 1e-8, "displacement mismatch at {} dof {k}: {} vs {}", sd.node_id, sd.values[k], dd.values[k]);
            }
        }
        for sr in &sparse_result.reactions {
            let dr = dense_result.reactions.iter().find(|r| r.node_id == sr.node_id && r.dof == sr.dof).unwrap();
            assert!((sr.value - dr.value).abs() < 1e-8, "reaction mismatch at {} {:?}: {} vs {}", sr.node_id, sr.dof, sr.value, dr.value);
        }
    }

    /// ➕️ A `Combination` must equal hand-computed superposition of the individually-solved case results.
    #[test]
    fn combination_equals_manual_superposition() {
        let (e, area, iy, l) = (200e9, 0.01, 1e-5, 2.0);
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Box::new(BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
        };
        let case_a = LoadCase { id: "a_case".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -1000.0 }], member_loads: vec![], self_weight: false };
        let case_b = LoadCase { id: "b_case".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Rz, value: 500.0 }], member_loads: vec![], self_weight: false };
        let combo = Combination { id: "combo".into(), terms: vec![("a_case".into(), 1.35), ("b_case".into(), 1.5)] };

        let results = solve_multi_case(&model, &[case_a, case_b], &[combo], [0.0, 0.0, 0.0]).expect("solves");
        let ra = results.get("a_case").unwrap().clone();
        let rb = results.get("b_case").unwrap().clone();
        let combined = results.get("combo").unwrap();

        for cd in &combined.displacements {
            let ad = ra.displacements.iter().find(|d| d.node_id == cd.node_id).unwrap();
            let bd = rb.displacements.iter().find(|d| d.node_id == cd.node_id).unwrap();
            for k in 0..6 {
                let expected = 1.35 * ad.values[k] + 1.5 * bd.values[k];
                assert!((cd.values[k] - expected).abs() < 1e-8, "combo displacement mismatch at {} dof {k}", cd.node_id);
            }
        }
        for cr in &combined.reactions {
            let ar = ra.reactions.iter().find(|r| r.node_id == cr.node_id && r.dof == cr.dof).unwrap();
            let br = rb.reactions.iter().find(|r| r.node_id == cr.node_id && r.dof == cr.dof).unwrap();
            let expected = 1.35 * ar.value + 1.5 * br.value;
            assert!((cr.value - expected).abs() < 1e-8, "combo reaction mismatch at {} {:?}", cr.node_id, cr.dof);
        }
    }

    /// ⚖️ Self-weight-only equilibrium: the sum of vertical reactions must equal `ρAL * g` — a
    /// strong, simple physical check independent of the moment distribution.
    #[test]
    fn self_weight_matches_total_mass_times_gravity() {
        let (e, area, iy, l, density) = (30e9, 0.05, 1e-4, 6.0, 2400.0);
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Box::new(BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
        };
        let case = LoadCase { id: "self_weight".into(), nodal_loads: vec![], member_loads: vec![], self_weight: true };
        let results = solve_multi_case(&model, &[case], &[], [0.0, -9.81, 0.0]).expect("solves");
        let result = results.get("self_weight").unwrap();

        let total_ty_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Ty).map(|r| r.value).sum();
        let expected = density * area * l * 9.81;
        // Reactions balance the applied (downward, negative) self-weight load, so they sum positive.
        assert!((total_ty_reaction - expected).abs() / expected < 0.01, "reaction sum {total_ty_reaction} vs expected {expected}");
    }

    /// 🎯️ Cantilever modal frequencies vs the classical closed form `f_i = (β_iL)²/(2πL²) · sqrt(EI/ρA)`.
    #[test]
    fn modal_cantilever_matches_analytical_frequencies() {
        let (e, iy, area, density, total_l) = (200e9, 1e-5, 0.01, 7850.0, 3.0);
        let n = 9;
        let dl = total_l / n as f64;
        let nodes: Vec<Node> = (0..=n).map(|i| Node { id: format!("n{i}"), pos: [dl * i as f64, 0.0, 0.0] }).collect();
        let elements: Vec<Box<dyn Element>> = (0..n).map(|i| Box::new(BeamEb2 { id: format!("e{i}"), start: format!("n{i}"), end: format!("n{}", i + 1), e, area, iy, density }) as Box<dyn Element>).collect();
        let model = AnalysisModel { nodes, elements, supports: vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }] };

        let result = modal(&model, 3).expect("modal solves");
        let beta_l = [1.875104_f64, 4.694091, 7.854757];
        for i in 0..3 {
            let expected = (beta_l[i] * beta_l[i]) / (2.0 * std::f64::consts::PI * total_l * total_l) * (e * iy / (density * area)).sqrt();
            let actual = result.frequencies_hz[i];
            assert!((actual - expected).abs() / expected < 0.10, "mode {i}: {actual} Hz vs analytical {expected} Hz");
        }
    }

    /// 🌀️ Euler pinned-pinned column buckling load vs `π²EI/L²` (K=1.0).
    #[test]
    fn buckling_euler_column_matches_analytical_load() {
        let (e, iy, area, density, total_l) = (200e9, 8e-6, 0.005, 7850.0, 3.0);
        let n = 7;
        let dl = total_l / n as f64;
        let nodes: Vec<Node> = (0..=n).map(|i| Node { id: format!("n{i}"), pos: [dl * i as f64, 0.0, 0.0] }).collect();
        let elements: Vec<Box<dyn Element>> = (0..n).map(|i| Box::new(BeamEb2 { id: format!("e{i}"), start: format!("n{i}"), end: format!("n{}", i + 1), e, area, iy, density }) as Box<dyn Element>).collect();
        let supports = vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: format!("n{n}"), fixed: vec![Dof::Ty] }];
        let model = AnalysisModel { nodes, elements, supports };

        let p_ref = 1.0;
        let reference_case = LoadCase { id: "axial_compression".into(), nodal_loads: vec![NodalLoad { node_id: format!("n{n}"), dof: Dof::Tx, value: -p_ref }], member_loads: vec![], self_weight: false };

        // Sanity-check the reference static solve first: pure axial compression should give nonzero Tx
        // displacement at the loaded end and ~zero Ty/Rz everywhere (no bending under a concentric load).
        let static_results = solve_multi_case(&model, std::slice::from_ref(&reference_case), &[], [0.0, 0.0, 0.0]).expect("reference solves");
        let static_result = static_results.get("axial_compression").unwrap();
        for d in &static_result.displacements {
            assert!(d.values[Dof::Ty.index()].abs() < 1e-9, "unexpected transverse displacement at {}: {}", d.node_id, d.values[Dof::Ty.index()]);
        }

        let result = buckling(&model, &reference_case, 1).expect("buckling solves");
        let factor = result.factors[0];
        let critical_load = factor * p_ref;
        let expected = std::f64::consts::PI.powi(2) * e * iy / (total_l * total_l);
        assert!(critical_load > 0.0, "critical load should be positive, got {critical_load}");
        assert!((critical_load - expected).abs() / expected < 0.10, "critical load {critical_load} vs analytical {expected}");
    }

    /// 🔍️ Duplicate-node-id models are rejected the same way `lib.rs::validate` rejects them.
    #[test]
    fn duplicate_node_id_is_rejected() {
        let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "a".into(), pos: [1.0, 0.0, 0.0] }], elements: vec![], supports: vec![] };
        let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::DuplicateNodeId("a".into()));
    }

    /// 🔍️ A `Bar2` model works fine through the multi-case pipeline too (not just `BeamEb2`).
    #[test]
    fn solve_multi_case_supports_bar2_truss() {
        let (e, area, l, p) = (200e9, 0.001, 2.0, 5000.0);
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Box::new(Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
        };
        let case = LoadCase { id: "axial".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p }], member_loads: vec![], self_weight: false };
        let results = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).expect("solves");
        let result = results.get("axial").unwrap();
        let expected = p * l / (e * area);
        let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
        assert!((b.values[Dof::Tx.index()] - expected).abs() / expected < 1e-8);
    }

    /// 🎨️ Patch test for `nodal_averaged_scalar`: TWO `Tri3Cst` triangles splitting a square along its
    /// diagonal, both under the SAME uniform uniaxial strain field (`u=a*x`, `v=-nu*a*y`) — every
    /// node's averaged von Mises must equal the exact analytical `E*a` (a constant field averages to
    /// itself regardless of how many elements touch a node).
    #[test]
    fn nodal_averaged_scalar_patch_test_is_exact_under_uniform_stress() {
        use crate::core::elements2d::{PlaneKind, Tri3Cst};
        let (e, nu, t) = (1000.0, 0.25, 1.0);
        let coords = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
        let nodes: Vec<Node> = (0..4).map(|i| Node { id: format!("n{i}"), pos: [coords[i][0], coords[i][1], 0.0] }).collect();
        let el1 = Tri3Cst { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };
        let el2 = Tri3Cst { id: "t2".into(), nodes: ["n0".into(), "n2".into(), "n3".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };

        let a = 0.01;
        let u_of = |ids: [usize; 3]| VecD::from_vec(ids.iter().flat_map(|&i| [a * coords[i][0], -nu * a * coords[i][1]]).collect());
        let ctx_of = |ids: [usize; 3]| ElementContext { positions: ids.iter().map(|&i| [coords[i][0], coords[i][1], 0.0]).collect() };

        let r1 = el1.recover(&ctx_of([0, 1, 2]), &u_of([0, 1, 2]), None);
        let r2 = el2.recover(&ctx_of([0, 2, 3]), &u_of([0, 2, 3]), None);

        let model = AnalysisModel { nodes, elements: vec![Box::new(el1), Box::new(el2)], supports: vec![] };
        let result = StaticResult { displacements: vec![], reactions: vec![], elements: vec![("t1".into(), r1), ("t2".into(), r2)], checks: SolutionChecks { residual_norm: 0.0, reaction_sum: [0.0; 6] } };

        let averaged = nodal_averaged_scalar(&model, &result, StressScalar::VonMises);
        let expected_vm = (e * a).abs();
        for id in ["n0", "n1", "n2", "n3"] {
            let v = *averaged.get(id).unwrap_or_else(|| panic!("node {id} missing from averaged map"));
            assert!((v - expected_vm).abs() / expected_vm < 1e-8, "node {id}: {v} vs {expected_vm}");
        }
    }

    /// 🎨️ `nodal_averaged_scalar` on two elements sharing exactly one node but reporting DIFFERENT
    /// constant von Mises values: the shared node's averaged value must land strictly between the
    /// two elements' own values, while each element's exclusive nodes keep that element's exact value.
    #[test]
    fn nodal_averaged_scalar_shared_node_is_between_neighboring_element_values() {
        use crate::core::elements2d::{PlaneKind, Tri3Cst};
        let (e, nu, t) = (1000.0, 0.25, 1.0);
        let el_a = Tri3Cst { id: "a".into(), nodes: ["shared".into(), "a1".into(), "a2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };
        let el_b = Tri3Cst { id: "b".into(), nodes: ["shared".into(), "b1".into(), "b2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };

        let ctx_a = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]] };
        let ctx_b = ElementContext { positions: vec![[0.0, 0.0, 0.0], [-2.0, 0.0, 0.0], [0.0, -2.0, 0.0]] };
        // `u = k*x` uniaxial fields with distinct magnitudes `k_a=0.02`, `k_b=0.05`, both zero at the
        // shared origin node so they stay purely constant-strain (patch-test-exact) on each triangle.
        let u_a = VecD::from_vec(vec![0.0, 0.0, 0.04, 0.0, 0.0, 0.0]);
        let u_b = VecD::from_vec(vec![0.0, 0.0, -0.1, 0.0, 0.0, 0.0]);

        let r_a = el_a.recover(&ctx_a, &u_a, None);
        let r_b = el_b.recover(&ctx_b, &u_b, None);
        let (va, vb) = match (&r_a, &r_b) {
            (ElementResult::Plane { gauss: ga }, ElementResult::Plane { gauss: gb }) => (ga[0].von_mises, gb[0].von_mises),
            _ => panic!("expected plane results"),
        };
        assert!(va < vb, "test setup should give distinct, ordered element values, got {va} vs {vb}");

        let nodes = vec![
            Node { id: "shared".into(), pos: [0.0, 0.0, 0.0] },
            Node { id: "a1".into(), pos: [2.0, 0.0, 0.0] },
            Node { id: "a2".into(), pos: [0.0, 2.0, 0.0] },
            Node { id: "b1".into(), pos: [-2.0, 0.0, 0.0] },
            Node { id: "b2".into(), pos: [0.0, -2.0, 0.0] },
        ];
        let model = AnalysisModel { nodes, elements: vec![Box::new(el_a), Box::new(el_b)], supports: vec![] };
        let result = StaticResult { displacements: vec![], reactions: vec![], elements: vec![("a".into(), r_a), ("b".into(), r_b)], checks: SolutionChecks { residual_norm: 0.0, reaction_sum: [0.0; 6] } };

        let averaged = nodal_averaged_scalar(&model, &result, StressScalar::VonMises);
        let shared = *averaged.get("shared").unwrap();
        assert!(shared > va && shared < vb, "shared node value {shared} should be strictly between {va} and {vb}");
        assert!((*averaged.get("a1").unwrap() - va).abs() < 1e-9);
        assert!((*averaged.get("a2").unwrap() - va).abs() < 1e-9);
        assert!((*averaged.get("b1").unwrap() - vb).abs() < 1e-9);
        assert!((*averaged.get("b2").unwrap() - vb).abs() < 1e-9);
    }

    /// 🔍️ An empty `AnalysisModel` is rejected the same way `Model`'s top-level `validate` rejects it.
    #[test]
    fn empty_model_is_rejected() {
        let model = AnalysisModel { nodes: vec![], elements: vec![], supports: vec![] };
        let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::EmptyModel);
    }

    /// 🔍️ An element referencing a node id absent from `model.nodes` is rejected.
    #[test]
    fn dangling_element_node_ref_is_rejected() {
        let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![Box::new(Bar2 { id: "e1".into(), start: "a".into(), end: "missing".into(), e: 1.0, area: 1.0, density: 0.0 })], supports: vec![] };
        let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
    }

    /// 🔍️ A support referencing a node id absent from `model.nodes` is rejected.
    #[test]
    fn dangling_support_node_ref_is_rejected() {
        let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![], supports: vec![Support { node_id: "missing".into(), fixed: vec![Dof::Tx] }] };
        let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
    }

    /// 🔍️ A `LoadCase` nodal load referencing a node id absent from `model.nodes` is rejected —
    /// `validate_case`'s own check, distinct from `validate`'s model-wide checks above.
    #[test]
    fn dangling_load_case_node_ref_is_rejected() {
        let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![], supports: vec![] };
        let case = LoadCase { id: "bad".into(), nodal_loads: vec![NodalLoad { node_id: "missing".into(), dof: Dof::Tx, value: 1.0 }], member_loads: vec![], self_weight: false };
        let err = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
    }

    /// 🌬️ `solve_multi_case`'s member-UDL branch (`case_rhs_old`'s `equivalent_nodal_loads` path) must
    /// match `solve_linear_static`'s dense pipeline (`model.member_loads`) on an equivalent model.
    #[test]
    fn solve_multi_case_applies_member_udl_equivalent_loads() {
        let (e, area, iy, l, w) = (200e9, 0.01, 1e-5, 2.0, 500.0);
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Box::new(BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
        };
        let case = LoadCase { id: "udl".into(), nodal_loads: vec![], member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: -w, wz: 0.0 })], self_weight: false };
        let results = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).expect("solves");
        let sparse_result = results.get("udl").unwrap();

        let dense_model = Model {
            nodes: model.nodes.clone(),
            elements: vec![Box::new(BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 })],
            supports: model.supports.clone(),
            nodal_loads: vec![],
            member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: -w, wz: 0.0 })],
        };
        let dense_result = solve_linear_static(&dense_model).expect("dense solves");

        for sd in &sparse_result.displacements {
            let dd = dense_result.displacements.iter().find(|d| d.node_id == sd.node_id).unwrap();
            for k in 0..6 {
                assert!((sd.values[k] - dd.values[k]).abs() < 1e-8, "displacement mismatch at {} dof {k}", sd.node_id);
            }
        }
    }

    /// 🌱️ `zero_like` zero-initializes every non-`Beam` `ElementResult` variant (the `Beam` variant is
    /// covered by `combination_equals_manual_superposition` above), and `add_scaled_element_result` on
    /// a freshly-zeroed accumulator reduces to exactly `factor * term`, field-by-field, per variant.
    #[test]
    fn zero_like_and_add_scaled_element_result_handle_every_non_beam_variant() {
        let factor = 2.5;

        let bar = ElementResult::Bar { n: 4.0 };
        let zero_bar = zero_like(&bar);
        assert_eq!(zero_bar, ElementResult::Bar { n: 0.0 });
        match add_scaled_element_result(&zero_bar, &bar, factor) {
            ElementResult::Bar { n } => assert!((n - factor * 4.0).abs() < 1e-12),
            other => panic!("expected bar, got {other:?}"),
        }

        let plane = ElementResult::Plane { gauss: vec![PlaneStress { sxx: 1.0, syy: 2.0, sxy: 3.0, von_mises: 4.0 }] };
        let zero_plane = zero_like(&plane);
        match &zero_plane {
            ElementResult::Plane { gauss } => assert_eq!(gauss[0], PlaneStress { sxx: 0.0, syy: 0.0, sxy: 0.0, von_mises: 0.0 }),
            other => panic!("expected plane, got {other:?}"),
        }
        match add_scaled_element_result(&zero_plane, &plane, factor) {
            ElementResult::Plane { gauss } => {
                assert!((gauss[0].sxx - factor * 1.0).abs() < 1e-12);
                assert!((gauss[0].syy - factor * 2.0).abs() < 1e-12);
                assert!((gauss[0].sxy - factor * 3.0).abs() < 1e-12);
            }
            other => panic!("expected plane, got {other:?}"),
        }

        let plate = ElementResult::Plate { gauss: vec![PlateMoments { mx: 1.0, my: 2.0, mxy: 3.0 }] };
        let zero_plate = zero_like(&plate);
        match add_scaled_element_result(&zero_plate, &plate, factor) {
            ElementResult::Plate { gauss } => {
                assert!((gauss[0].mx - factor * 1.0).abs() < 1e-12);
                assert!((gauss[0].my - factor * 2.0).abs() < 1e-12);
                assert!((gauss[0].mxy - factor * 3.0).abs() < 1e-12);
            }
            other => panic!("expected plate, got {other:?}"),
        }

        let solid = ElementResult::Solid { gauss: vec![SolidStress { sxx: 1.0, syy: 2.0, szz: 3.0, sxy: 4.0, syz: 5.0, sxz: 6.0, von_mises: 7.0 }] };
        let zero_solid = zero_like(&solid);
        match add_scaled_element_result(&zero_solid, &solid, factor) {
            ElementResult::Solid { gauss } => {
                assert!((gauss[0].sxx - factor * 1.0).abs() < 1e-12);
                assert!((gauss[0].szz - factor * 3.0).abs() < 1e-12);
                assert!((gauss[0].syz - factor * 5.0).abs() < 1e-12);
            }
            other => panic!("expected solid, got {other:?}"),
        }

        let shell = ElementResult::Shell { gauss: vec![ShellState { nxx: 1.0, nyy: 2.0, nxy: 3.0, mxx: 4.0, myy: 5.0, mxy: 6.0, von_mises_top: 7.0, von_mises_bottom: 8.0 }] };
        let zero_shell = zero_like(&shell);
        match add_scaled_element_result(&zero_shell, &shell, factor) {
            ElementResult::Shell { gauss } => {
                assert!((gauss[0].nxx - factor * 1.0).abs() < 1e-12);
                assert!((gauss[0].mxy - factor * 6.0).abs() < 1e-12);
                assert!((gauss[0].von_mises_bottom - factor * 8.0).abs() < 1e-12);
            }
            other => panic!("expected shell, got {other:?}"),
        }
    }

    /// 📊️ `element_scalar_average` covers every element-kind/scalar combination it recognizes (`Some`)
    /// and every mismatched combination (`None`) — arms `nodal_averaged_scalar`'s own patch tests never
    /// happen to exercise (those only touch `Plane`/`VonMises`).
    #[test]
    fn element_scalar_average_covers_every_variant_and_scalar_combination() {
        let plane = ElementResult::Plane { gauss: vec![PlaneStress { sxx: 1.0, syy: 2.0, sxy: 3.0, von_mises: 4.0 }] };
        assert_eq!(element_scalar_average(&plane, StressScalar::VonMises), Some(4.0));
        assert_eq!(element_scalar_average(&plane, StressScalar::Sxx), Some(1.0));
        assert_eq!(element_scalar_average(&plane, StressScalar::Syy), Some(2.0));
        assert_eq!(element_scalar_average(&plane, StressScalar::Sxy), Some(3.0));
        assert_eq!(element_scalar_average(&plane, StressScalar::Szz), None);
        assert_eq!(element_scalar_average(&plane, StressScalar::VonMisesTop), None);

        let solid = ElementResult::Solid { gauss: vec![SolidStress { sxx: 1.0, syy: 2.0, szz: 3.0, sxy: 4.0, syz: 5.0, sxz: 6.0, von_mises: 7.0 }] };
        assert_eq!(element_scalar_average(&solid, StressScalar::VonMises), Some(7.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Sxx), Some(1.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Syy), Some(2.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Szz), Some(3.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Sxy), Some(4.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Syz), Some(5.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Sxz), Some(6.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::VonMisesTop), None);

        let shell = ElementResult::Shell { gauss: vec![ShellState { nxx: 0.0, nyy: 0.0, nxy: 0.0, mxx: 0.0, myy: 0.0, mxy: 0.0, von_mises_top: 8.0, von_mises_bottom: 9.0 }] };
        assert_eq!(element_scalar_average(&shell, StressScalar::VonMisesTop), Some(8.0));
        assert_eq!(element_scalar_average(&shell, StressScalar::VonMisesBottom), Some(9.0));
        assert_eq!(element_scalar_average(&shell, StressScalar::VonMises), None);

        let bar = ElementResult::Bar { n: 42.0 };
        assert_eq!(element_scalar_average(&bar, StressScalar::VonMises), None);
    }
}
// #endregion 🔖️Tests
