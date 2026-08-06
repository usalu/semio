//! 🏗️ FEM core: the headless finite-element calculation library — model, element trait, assembly
//! pipeline, and linear-static analysis, extended by sibling modules for sparse solvers,
//! quadrature/element formulation, the 2D/3D element libraries, meshing, and multi-case analyses.
//! No UI, no VCS, no framework dependency — `fem_2d`/`fem_3d`/`fem-plugin` are the UI layer built
//! on top of this crate.

pub mod analyses {
    //! 📈️ Multi-case/combination linear-static analysis, self-weight load generation, modal analysis
    //! (frequencies/shapes), linear buckling, and nodal-averaged stress recovery for contour rendering
    //! (`nodal_averaged_scalar`) — all sparse-backed (RCM-ordered, single LDLT factorization shared
    //! across every load case / eigen-solve).

    use crate::sparse::{ldlt_factor, rcm_order, subspace_iteration, Coo, Csr, EigenPairs, LdltFactor};
    use crate::{BeamStation, Dof, Element, ElementContext, ElementResult, FemError, MemberUdl, NodalLoad, Node, NodeDisplacement, NodeReaction, PlaneStress, PlateMoments, ShellState, SolidStress, SolutionChecks, StaticResult, Support};
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
        use crate::elements2d::{Bar2, BeamEb2};
        use crate::{solve_linear_static, Model};

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
            use crate::elements2d::{PlaneKind, Tri3Cst};
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
            use crate::elements2d::{PlaneKind, Tri3Cst};
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
}

pub mod elements2d {
    //! 📐️ 2D structural elements: axial `Bar2` truss, Euler-Bernoulli `BeamEb2` frame member, the
    //! Tri3/Tri6/Quad4/Quad8 plane-stress/plane-strain continuum family, and the `PlateDkt` Batoz
    //! Discrete Kirchhoff Triangle thin-plate bending element.

    use crate::formulation::{b_matrix_plane, d_matrix_plane_strain, d_matrix_plane_stress, gauss_quad, gauss_tri, jacobian_2d, shape_quad4, shape_quad8, shape_tri3, shape_tri6};
    use crate::{Dof, Element, ElementContext, ElementResult, MemberUdl, PlaneStress, PlateMoments};
    use math::algebra::{MatD, VecD};

    // #region 🔖️Geometry
    fn segment_geometry(ctx: &ElementContext) -> (f64, f64, f64) {
        let p1 = ctx.positions[0];
        let p2 = ctx.positions[1];
        let dx = p2[0] - p1[0];
        let dy = p2[1] - p1[1];
        let l = (dx * dx + dy * dy).sqrt();
        (l, dx / l, dy / l)
    }
    // #endregion 🔖️Geometry

    // #region 🔖️Bar2
    /// 🪢️ 2-node axial truss element — DOFs `[Tx, Ty]` per node.
    pub struct Bar2 {
        pub id: String,
        pub start: String,
        pub end: String,
        pub e: f64,
        pub area: f64,
        pub density: f64,
    }

    impl Element for Bar2 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            vec![self.start.clone(), self.end.clone()]
        }

        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty]
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let (l, cx, cy) = segment_geometry(ctx);
            let k = self.e * self.area / l;
            let mut m = MatD::zeros(4, 4);
            let terms = [[cx * cx, cx * cy, -cx * cx, -cx * cy], [cx * cy, cy * cy, -cx * cy, -cy * cy]];
            for row in 0..2 {
                for col in 0..4 {
                    m.set(row, col, k * terms[row][col]);
                    m.set(row + 2, col, if col < 2 { -k * terms[row][col] } else { k * terms[row][col - 2] });
                }
            }
            m
        }

        fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let (l, cx, cy) = segment_geometry(ctx);
            let k = self.e * self.area / l;
            let n = k * ((u_local.get(2) - u_local.get(0)) * cx + (u_local.get(3) - u_local.get(1)) * cy);
            ElementResult::Bar { n }
        }

        /// 🏋️ Isotropic lumped-consistent mass — same in both directions since a bar has no bending
        /// stiffness to give mass a preferred orientation. `m = ρAL/6`, block form `[[2m,0,m,0],[0,2m,0,m],
        /// [m,0,2m,0],[0,m,0,2m]]` (node-major `[u1,v1,u2,v2]`).
        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let (l, _, _) = segment_geometry(ctx);
            let m = self.density * self.area * l / 6.0;
            let mut out = MatD::zeros(4, 4);
            for i in 0..4 {
                out.set(i, i, 2.0 * m);
            }
            out.set(0, 2, m);
            out.set(2, 0, m);
            out.set(1, 3, m);
            out.set(3, 1, m);
            Some(out)
        }

        /// 🌬️ Consistent end-load `wL/2` at each node from a global member UDL `(wx,wy)` — a 2-node
        /// linear axial element has no bending stiffness to redistribute the load unevenly, so the
        /// lumped-consistent split is exact.
        fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
            let (l, _, _) = segment_geometry(ctx);
            let half = l / 2.0;
            Some(VecD::from_vec(vec![udl.wx * half, udl.wy * half, udl.wx * half, udl.wy * half]))
        }

        /// 🌀️ Truss geometric ("stability") stiffness under the member's own axial force `n` (tension-
        /// positive, same convention as `recover`): `N/L·(I − ccᵀ)` on each 2x2 node block, `ccᵀ` the
        /// outer product of the unit axial direction — the transverse-projector form (Przemieniecki,
        /// "Theory of Matrix Structural Analysis") that only destabilizes displacement PERPENDICULAR to
        /// the bar's own axis, vanishing identically for a rigid translation (which the projector kills).
        fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
            let (l, cx, cy) = segment_geometry(ctx);
            let k = self.e * self.area / l;
            let n = k * ((u_element.get(2) - u_element.get(0)) * cx + (u_element.get(3) - u_element.get(1)) * cy);
            let coeff = n / l;
            let proj = [[1.0 - cx * cx, -cx * cy], [-cx * cy, 1.0 - cy * cy]];
            let mut kg = MatD::zeros(4, 4);
            for row in 0..2 {
                for col in 0..2 {
                    let v = coeff * proj[row][col];
                    kg.set(row, col, v);
                    kg.set(row, col + 2, -v);
                    kg.set(row + 2, col, -v);
                    kg.set(row + 2, col + 2, v);
                }
            }
            Some(kg)
        }
    }
    // #endregion 🔖️Bar2

    // #region 🔖️BeamEb2
    /// 🧭️ 2D frame transformation matrix — block-diagonal 3 copies of the planar rotation, mapping
    /// GLOBAL `[u1,v1,θ1,u2,v2,θ2]` to LOCAL coordinates.
    fn beam_transform(c: f64, s: f64) -> MatD {
        let mut t = MatD::zeros(6, 6);
        for block in 0..2 {
            let o = block * 3;
            t.set(o, o, c);
            t.set(o, o + 1, s);
            t.set(o + 1, o, -s);
            t.set(o + 1, o + 1, c);
            t.set(o + 2, o + 2, 1.0);
        }
        t
    }

    /// 🧮️ Local 6x6 Euler-Bernoulli beam stiffness, dof order `[u1,v1,θ1,u2,v2,θ2]`.
    fn beam_local_stiffness(l: f64, axial_k: f64, bend_k: f64) -> MatD {
        let mut k = MatD::zeros(6, 6);
        k.set(0, 0, axial_k);
        k.set(0, 3, -axial_k);
        k.set(3, 0, -axial_k);
        k.set(3, 3, axial_k);

        let l2 = l * l;
        let bending = [
            (1, 1, 12.0 * bend_k / l2),
            (1, 2, 6.0 * bend_k / l),
            (1, 4, -12.0 * bend_k / l2),
            (1, 5, 6.0 * bend_k / l),
            (2, 1, 6.0 * bend_k / l),
            (2, 2, 4.0 * bend_k),
            (2, 4, -6.0 * bend_k / l),
            (2, 5, 2.0 * bend_k),
            (4, 1, -12.0 * bend_k / l2),
            (4, 2, -6.0 * bend_k / l),
            (4, 4, 12.0 * bend_k / l2),
            (4, 5, -6.0 * bend_k / l),
            (5, 1, 6.0 * bend_k / l),
            (5, 2, 2.0 * bend_k),
            (5, 4, -6.0 * bend_k / l),
            (5, 5, 4.0 * bend_k),
        ];
        for (row, col, value) in bending {
            k.set(row, col, value);
        }
        k
    }

    /// 🌬️ Local fixed-end load vector `[u1,v1,θ1,u2,v2,θ2]` for a local-frame UDL `(wx_local, wy_local)`.
    fn beam_local_udl(l: f64, wx_local: f64, wy_local: f64) -> VecD {
        VecD::from_vec(vec![wx_local * l / 2.0, wy_local * l / 2.0, wy_local * l * l / 12.0, wx_local * l / 2.0, wy_local * l / 2.0, -wy_local * l * l / 12.0])
    }

    /// 🏋️ Consistent local mass matrix, dof order `[u1,v1,θ1,u2,v2,θ2]` — axial `ρAL/6*[[2,1],[1,2]]` at
    /// `(0,3)`, standard Euler-Bernoulli consistent bending mass at `[1,2,4,5]` (rotary inertia of the
    /// cross-section neglected — see Cook/Malkus/Plesha "Concepts and Applications of Finite Element
    /// Analysis" for the closed form).
    fn beam_local_mass(l: f64, area: f64, density: f64) -> MatD {
        let mut m = MatD::zeros(6, 6);
        let axial = density * area * l / 6.0;
        m.set(0, 0, 2.0 * axial);
        m.set(0, 3, axial);
        m.set(3, 0, axial);
        m.set(3, 3, 2.0 * axial);

        let l2 = l * l;
        let factor = density * area * l / 420.0;
        let idx = [1usize, 2, 4, 5];
        let block = [[156.0, 22.0 * l, 54.0, -13.0 * l], [22.0 * l, 4.0 * l2, 13.0 * l, -3.0 * l2], [54.0, 13.0 * l, 156.0, -22.0 * l], [-13.0 * l, -3.0 * l2, -22.0 * l, 4.0 * l2]];
        for (bi, &gi) in idx.iter().enumerate() {
            for (bj, &gj) in idx.iter().enumerate() {
                m.set(gi, gj, factor * block[bi][bj]);
            }
        }
        m
    }

    /// 🌀️ Local geometric ("stress") stiffness for a 2D Euler-Bernoulli beam-column under axial force `n`
    /// (tension-positive, same convention `recover` reports), bending block `[v1,θ1,v2,θ2]` only — no
    /// axial/geometric coupling at this scope. Standard textbook beam-column geometric stiffness.
    fn beam_local_geometric_stiffness(l: f64, n: f64) -> MatD {
        let mut kg = MatD::zeros(6, 6);
        let l2 = l * l;
        let coeff = n / l;
        let idx = [1usize, 2, 4, 5];
        let block = [[6.0 / 5.0, l / 10.0, -6.0 / 5.0, l / 10.0], [l / 10.0, 2.0 * l2 / 15.0, -l / 10.0, -l2 / 30.0], [-6.0 / 5.0, -l / 10.0, 6.0 / 5.0, -l / 10.0], [l / 10.0, -l2 / 30.0, -l / 10.0, 2.0 * l2 / 15.0]];
        for (bi, &gi) in idx.iter().enumerate() {
            for (bj, &gj) in idx.iter().enumerate() {
                kg.set(gi, gj, coeff * block[bi][bj]);
            }
        }
        kg
    }

    /// 🏗️ 2-node Euler-Bernoulli frame element — DOFs `[Tx, Ty, Rz]` per node.
    pub struct BeamEb2 {
        pub id: String,
        pub start: String,
        pub end: String,
        pub e: f64,
        pub area: f64,
        pub iy: f64,
        pub density: f64,
    }

    impl Element for BeamEb2 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            vec![self.start.clone(), self.end.clone()]
        }

        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty, Dof::Rz]
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let (l, c, s) = segment_geometry(ctx);
            let axial_k = self.e * self.area / l;
            let bend_k = self.e * self.iy / l;
            let k_local = beam_local_stiffness(l, axial_k, bend_k);
            let t = beam_transform(c, s);
            t.transpose().matmul(&k_local).matmul(&t)
        }

        fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
            let (l, c, s) = segment_geometry(ctx);
            let wx_local = udl.wx * c + udl.wy * s;
            let wy_local = -udl.wx * s + udl.wy * c;
            let f_local = beam_local_udl(l, wx_local, wy_local);
            let t = beam_transform(c, s);
            Some(t.transpose().mul_vec(&f_local))
        }

        fn recover(&self, ctx: &ElementContext, u_local: &VecD, udl: Option<&MemberUdl>) -> ElementResult {
            let (l, c, s) = segment_geometry(ctx);
            let axial_k = self.e * self.area / l;
            let bend_k = self.e * self.iy / l;
            let t = beam_transform(c, s);
            let u_loc = t.mul_vec(u_local);
            let k_local = beam_local_stiffness(l, axial_k, bend_k);

            let (wx_local, wy_local) = match udl {
                Some(u) => (u.wx * c + u.wy * s, -u.wx * s + u.wy * c),
                None => (0.0, 0.0),
            };
            let f_udl_local = beam_local_udl(l, wx_local, wy_local);
            let f_end = k_local.mul_vec(&u_loc).sub(&f_udl_local);

            let n1 = f_end.get(0);
            let v1 = f_end.get(1);
            let m1 = f_end.get(2);

            let stations = (0..11)
                .map(|i| {
                    let x = l * (i as f64) / 10.0;
                    crate::BeamStation { x, n: -n1, v: v1 + wy_local * x, m: m1 + v1 * x + wy_local * x * x / 2.0 }
                })
                .collect();
            ElementResult::Beam { stations }
        }

        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let (l, c, s) = segment_geometry(ctx);
            let m_local = beam_local_mass(l, self.area, self.density);
            let t = beam_transform(c, s);
            Some(t.transpose().matmul(&m_local).matmul(&t))
        }

        /// 🌀️ Buckling geometric stiffness from the member's own axial force under `u_element` — same
        /// sign convention as `recover`'s `n` (tension-positive): `n = -k_local.mul_vec(u_loc).get(0)`.
        fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
            let (l, c, s) = segment_geometry(ctx);
            let axial_k = self.e * self.area / l;
            let bend_k = self.e * self.iy / l;
            let t = beam_transform(c, s);
            let u_loc = t.mul_vec(u_element);
            let k_local = beam_local_stiffness(l, axial_k, bend_k);
            let f_end = k_local.mul_vec(&u_loc);
            let n = -f_end.get(0);
            let kg_local = beam_local_geometric_stiffness(l, n);
            Some(t.transpose().matmul(&kg_local).matmul(&t))
        }
    }
    // #endregion 🔖️BeamEb2

    // #region 🔖️Continuum
    /// 🧱️ Plane-stress vs plane-strain constitutive assumption, shared by the Tri3/Tri6/Quad4/Quad8
    /// continuum elements.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PlaneKind {
        Stress,
        Strain,
    }

    impl PlaneKind {
        fn d_matrix(self, e: f64, nu: f64) -> MatD {
            match self {
                PlaneKind::Stress => d_matrix_plane_stress(e, nu),
                PlaneKind::Strain => d_matrix_plane_strain(e, nu),
            }
        }
    }

    fn plane_coords(ctx: &ElementContext) -> Vec<[f64; 2]> {
        ctx.positions.iter().map(|p| [p[0], p[1]]).collect()
    }

    /// 🧮️ Physical B-matrix + `weight * det(J)` at every Gauss point of a rule, shared by
    /// `stiffness_global` and `recover` so both walk the SAME Gauss points in the SAME order.
    fn plane_b_and_weights(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>) -> Vec<(MatD, f64)> {
        rule.iter()
            .map(|&(xi, eta, w)| {
                let d_n_param = shape(xi, eta);
                let (_, det_j, d_n_xy) = jacobian_2d(coords, &d_n_param);
                (b_matrix_plane(&d_n_xy), w * det_j)
            })
            .collect()
    }

    fn plane_stiffness(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, thickness: f64, ndof: usize) -> MatD {
        let mut ke = MatD::zeros(ndof, ndof);
        for (b, w) in plane_b_and_weights(coords, rule, shape) {
            ke.add_triple_product(&b, d, w * thickness);
        }
        ke
    }

    fn plane_recover(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, u_local: &VecD) -> ElementResult {
        let gauss = plane_b_and_weights(coords, rule, shape)
            .into_iter()
            .map(|(b, _)| {
                let eps = b.mul_vec(u_local);
                let sigma = d.mul_vec(&eps);
                let (sxx, syy, sxy) = (sigma.get(0), sigma.get(1), sigma.get(2));
                let von_mises = (sxx * sxx - sxx * syy + syy * syy + 3.0 * sxy * sxy).sqrt();
                PlaneStress { sxx, syy, sxy, von_mises }
            })
            .collect();
        ElementResult::Plane { gauss }
    }

    /// 🏋️ Consistent plane-continuum mass `ρ·t·∫Nᵀ·N·dA`, evaluated at the SAME Gauss rule as
    /// `plane_stiffness` — `shape_full` returns BOTH shape values (for `Nᵀ·N`) and parametric
    /// derivatives (for `jacobian_2d`'s `det(J)`), unlike `plane_b_and_weights`'s gradient-only closure.
    fn plane_mass(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape_full: impl Fn(f64, f64) -> (Vec<f64>, Vec<[f64; 2]>), density: f64, thickness: f64, n_nodes: usize) -> MatD {
        let mut m = MatD::zeros(n_nodes * 2, n_nodes * 2);
        for (xi, eta, w) in rule.iter().copied() {
            let (n_vals, d_n_param) = shape_full(xi, eta);
            let (_, det_j, _) = jacobian_2d(coords, &d_n_param);
            let scale = density * thickness * w * det_j;
            for i in 0..n_nodes {
                for j in 0..n_nodes {
                    let v = n_vals[i] * n_vals[j] * scale;
                    m.add_at(2 * i, 2 * j, v);
                    m.add_at(2 * i + 1, 2 * j + 1, v);
                }
            }
        }
        m
    }

    /// 🌀️ Plane-continuum initial-stress geometric stiffness `Kg = ∫Gᵀ(σ⊗I₂)G·t·dA` (Cook, Malkus,
    /// Plesha & Witt, "Concepts and Applications of Finite Element Analysis") — recovers the Cauchy
    /// stress `σ=Dε` from `u_local` at each Gauss point, then couples node `i`/`j`'s shape gradients
    /// through `σ` identically in BOTH the `u` and `v` directions (no `u`-`v` cross-coupling, since `G`
    /// is block-diagonal by direction).
    fn plane_geometric_stiffness(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, thickness: f64, u_local: &VecD, n_nodes: usize) -> MatD {
        let mut kg = MatD::zeros(n_nodes * 2, n_nodes * 2);
        for (xi, eta, w) in rule.iter().copied() {
            let d_n_param = shape(xi, eta);
            let (_, det_j, d_n_xy) = jacobian_2d(coords, &d_n_param);
            let b = b_matrix_plane(&d_n_xy);
            let eps = b.mul_vec(u_local);
            let sigma = d.mul_vec(&eps);
            let (sxx, syy, sxy) = (sigma.get(0), sigma.get(1), sigma.get(2));
            let scale = w * det_j * thickness;
            for i in 0..n_nodes {
                let (dix, diy) = (d_n_xy[i][0], d_n_xy[i][1]);
                for j in 0..n_nodes {
                    let (djx, djy) = (d_n_xy[j][0], d_n_xy[j][1]);
                    let s = dix * sxx * djx + dix * sxy * djy + diy * sxy * djx + diy * syy * djy;
                    kg.add_at(2 * i, 2 * j, s * scale);
                    kg.add_at(2 * i + 1, 2 * j + 1, s * scale);
                }
            }
        }
        kg
    }

    // #region 🔖️Tri3Cst
    /// 🔺️ 3-node constant-strain triangle — DOFs `[Tx, Ty]` per node, 1-point Gauss-tri integration
    /// (exact for constant strain).
    pub struct Tri3Cst {
        pub id: String,
        pub nodes: [String; 3],
        pub e: f64,
        pub nu: f64,
        pub thickness: f64,
        pub kind: PlaneKind,
        pub density: f64,
    }

    impl Tri3Cst {
        fn rule(&self) -> Vec<(f64, f64, f64)> {
            gauss_tri(1)
        }

        fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
            shape_tri3(xi, eta).1.to_vec()
        }

        fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
            let (n, dn) = shape_tri3(xi, eta);
            (n.to_vec(), dn.to_vec())
        }
    }

    impl Element for Tri3Cst {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }

        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty]
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 6)
        }

        fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
        }

        /// 🏋️ Consistent CST mass `ρtA/12·[[2,1,1],[1,2,1],[1,1,2]]` (both directions) — Tri3's shape
        /// functions ARE the area coordinates (`Ni=Li`), so `Ni·Nj` is a complete quadratic in area
        /// coordinates, integrated EXACTLY by the degree-2-precision 3-point rule (own stiffness rule
        /// `self.rule()` is only 1-point, adequate for the constant-strain stiffness but NOT exact here).
        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let coords = plane_coords(ctx);
            Some(plane_mass(&coords, &gauss_tri(3), Self::shape_full, self.density, self.thickness, 3))
        }

        fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            Some(plane_geometric_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, u_element, 3))
        }
    }
    // #endregion 🔖️Tri3Cst

    // #region 🔖️Tri6Lst
    /// 🔺️ 6-node linear-strain triangle — DOFs `[Tx, Ty]` per node, 3-point Gauss-tri integration.
    /// Node order `[n0,n1,n2,n01,n12,n20]` — see `formulation::shape_tri6` for the exact convention.
    pub struct Tri6Lst {
        pub id: String,
        pub nodes: [String; 6],
        pub e: f64,
        pub nu: f64,
        pub thickness: f64,
        pub kind: PlaneKind,
        pub density: f64,
    }

    impl Tri6Lst {
        fn rule(&self) -> Vec<(f64, f64, f64)> {
            gauss_tri(3)
        }

        /// 🎯️ A 7-point rule (degree-5 precision) for mass — Tri6's quadratic shape functions make
        /// `Ni·Nj` a degree-4 polynomial, which the element's own 3-point (degree-2) stiffness rule
        /// under-integrates.
        fn mass_rule() -> Vec<(f64, f64, f64)> {
            gauss_tri(7)
        }

        fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
            shape_tri6(xi, eta).1.to_vec()
        }

        fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
            let (n, dn) = shape_tri6(xi, eta);
            (n.to_vec(), dn.to_vec())
        }
    }

    impl Element for Tri6Lst {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }

        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty]
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 12)
        }

        fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
        }

        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let coords = plane_coords(ctx);
            Some(plane_mass(&coords, &Self::mass_rule(), Self::shape_full, self.density, self.thickness, 6))
        }

        fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            Some(plane_geometric_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, u_element, 6))
        }
    }
    // #endregion 🔖️Tri6Lst

    // #region 🔖️Quad4
    /// ⬜️ 4-node bilinear quadrilateral — DOFs `[Tx, Ty]` per node, 2x2 Gauss-quad integration.
    pub struct Quad4 {
        pub id: String,
        pub nodes: [String; 4],
        pub e: f64,
        pub nu: f64,
        pub thickness: f64,
        pub kind: PlaneKind,
        pub density: f64,
    }

    impl Quad4 {
        fn rule(&self) -> Vec<(f64, f64, f64)> {
            gauss_quad(2)
        }

        fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
            shape_quad4(xi, eta).1.to_vec()
        }

        fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
            let (n, dn) = shape_quad4(xi, eta);
            (n.to_vec(), dn.to_vec())
        }
    }

    impl Element for Quad4 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }

        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty]
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 8)
        }

        fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
        }

        /// 🏋️ Consistent bilinear mass — the same 2x2 rule as stiffness under-integrates the biquadratic
        /// `Ni·Nj` product for a non-rectangular quad, so mass uses the fuller 3x3 rule instead.
        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let coords = plane_coords(ctx);
            Some(plane_mass(&coords, &gauss_quad(3), Self::shape_full, self.density, self.thickness, 4))
        }

        fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            Some(plane_geometric_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, u_element, 4))
        }
    }
    // #endregion 🔖️Quad4

    // #region 🔖️Quad8
    /// ⬜️ 8-node serendipity quadratic quadrilateral — DOFs `[Tx, Ty]` per node, 3x3 (deliberately
    /// full-integrated, per standard FEM practice for serendipity elements) Gauss-quad integration.
    pub struct Quad8 {
        pub id: String,
        pub nodes: [String; 8],
        pub e: f64,
        pub nu: f64,
        pub thickness: f64,
        pub kind: PlaneKind,
        pub density: f64,
    }

    impl Quad8 {
        fn rule(&self) -> Vec<(f64, f64, f64)> {
            gauss_quad(3)
        }

        fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
            shape_quad8(xi, eta).1.to_vec()
        }

        fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
            let (n, dn) = shape_quad8(xi, eta);
            (n.to_vec(), dn.to_vec())
        }
    }

    impl Element for Quad8 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }

        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty]
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 16)
        }

        fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
        }

        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let coords = plane_coords(ctx);
            Some(plane_mass(&coords, &self.rule(), Self::shape_full, self.density, self.thickness, 8))
        }

        fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
            let coords = plane_coords(ctx);
            let d = self.kind.d_matrix(self.e, self.nu);
            Some(plane_geometric_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, u_element, 8))
        }
    }
    // #endregion 🔖️Quad8
    // #endregion 🔖️Continuum

    // #region 🔖️PlateDkt
    /// 🧮️ Per-edge Batoz DKT geometric coefficients `a,b,c,d,e` (cross-checked against Batoz, Bathe & Ho
    /// (1980) via the JuliaFEM `FEMPlates.jl` reference implementation — `e_k` is a DISTINCT coefficient
    /// from `b_k`, used only in `Hy`'s `βx`-columns; it is NOT the `f_k` appearing in some other DKT
    /// write-ups' `Hx`, which this formulation doesn't need).
    struct DktEdge {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
    }

    fn dkt_edge(pi: [f64; 2], pj: [f64; 2]) -> DktEdge {
        let x_ij = pi[0] - pj[0];
        let y_ij = pi[1] - pj[1];
        let l2 = x_ij * x_ij + y_ij * y_ij;
        DktEdge { a: -x_ij / l2, b: 0.75 * x_ij * y_ij / l2, c: (0.25 * x_ij * x_ij - 0.5 * y_ij * y_ij) / l2, d: -y_ij / l2, e: (0.25 * y_ij * y_ij - 0.5 * x_ij * x_ij) / l2 }
    }

    /// 🧱️ Bending constitutive matrix `(E t³)/(12(1-ν²)) [[1,ν,0],[ν,1,0],[0,0,(1-ν)/2]]`, shared by
    /// `PlateDkt` and (via `crate::elements2d::d_matrix_plate`) `elements3d::ShellFacet3`'s bending part.
    pub(crate) fn d_matrix_plate(e: f64, nu: f64, thickness: f64) -> MatD {
        let factor = e * thickness.powi(3) / (12.0 * (1.0 - nu * nu));
        let mut d = MatD::zeros(3, 3);
        d.set(0, 0, factor);
        d.set(0, 1, factor * nu);
        d.set(1, 0, factor * nu);
        d.set(1, 1, factor);
        d.set(2, 2, factor * (1.0 - nu) / 2.0);
        d
    }

    /// 🧮️ Batoz DKT curvature-displacement B-matrix (3x9) at parametric `(xi, eta)` on a flat triangle
    /// with physical `coords`. Dof order `[w1,Rx1,Ry1,w2,Rx2,Ry2,w3,Rx3,Ry3]`, where `Rx = ∂w/∂y` and
    /// `Ry = -∂w/∂x` (the physically standard rotation-about-local-axis convention — see the module docs
    /// on `PlateDkt`). Builds Batoz's `Hx`/`Hy` shape combinations over the standard mid-edge quadratic
    /// "bubble" functions `N4=4L2L3, N5=4L3L1, N6=4L1L2` and the QUADRATIC Tri6 corner functions
    /// `Ni=Li(2Li-1)` (cross-checked against the JuliaFEM `FEMPlates.jl` reference implementation of
    /// Batoz, Bathe & Ho 1980). Batoz's own `Hx`/`Hy` columns land directly on our `(Rx,Ry)` dof slots
    /// with NO permutation — empirically confirmed via the patch test below — but the curvature signs
    /// `κx=-∂Hx/∂x, κy=-∂Hy/∂y, κxy=-(∂Hx/∂y+∂Hy/∂x)` carry an overall minus relative to the raw `Hx`/`Hy`
    /// derivatives (this formulation's `Hx`/`Hy` represent the NEGATIVE of the physical rotation fields).
    /// Shared by `PlateDkt` and `elements3d::ShellFacet3`.
    pub(crate) fn dkt_b_matrix(coords: &[[f64; 2]; 3], xi: f64, eta: f64) -> MatD {
        let edge4 = dkt_edge(coords[1], coords[2]);
        let edge5 = dkt_edge(coords[2], coords[0]);
        let edge6 = dkt_edge(coords[0], coords[1]);

        let (_, dn_param) = shape_tri3(xi, eta);
        let (_, _, d_n_xy) = jacobian_2d(coords, &dn_param);
        let gx = [d_n_xy[0][0], d_n_xy[1][0], d_n_xy[2][0]];
        let gy = [d_n_xy[0][1], d_n_xy[1][1], d_n_xy[2][1]];

        let (l1, l2v, l3) = (1.0 - xi - eta, xi, eta);
        let dn4dx = 4.0 * (gx[1] * l3 + l2v * gx[2]);
        let dn4dy = 4.0 * (gy[1] * l3 + l2v * gy[2]);
        let dn5dx = 4.0 * (gx[2] * l1 + l3 * gx[0]);
        let dn5dy = 4.0 * (gy[2] * l1 + l3 * gy[0]);
        let dn6dx = 4.0 * (gx[0] * l2v + l1 * gx[1]);
        let dn6dy = 4.0 * (gy[0] * l2v + l1 * gy[1]);

        // The Hx3/Hx6/Hx9 and Hy2/Hy5/Hy8 "direct" terms use the QUADRATIC Tri6 corner shape functions
        // `Ni = Li*(2Li-1)` (matching `formulation::shape_tri6`'s convention), NOT the linear area
        // coordinates `Li` themselves — `dNi/dx = (4Li-1) * dLi/dx`.
        let dn1qdx = (4.0 * l1 - 1.0) * gx[0];
        let dn1qdy = (4.0 * l1 - 1.0) * gy[0];
        let dn2qdx = (4.0 * l2v - 1.0) * gx[1];
        let dn2qdy = (4.0 * l2v - 1.0) * gy[1];
        let dn3qdx = (4.0 * l3 - 1.0) * gx[2];
        let dn3qdy = (4.0 * l3 - 1.0) * gy[2];

        // Batoz-order (w1,βx1,βy1,w2,βx2,βy2,w3,βx3,βy3) partial derivatives of the Hx/Hy shape combinations.
        let dhx_dx = [
            1.5 * (edge6.a * dn6dx - edge5.a * dn5dx),
            edge5.b * dn5dx + edge6.b * dn6dx,
            dn1qdx - edge5.c * dn5dx - edge6.c * dn6dx,
            1.5 * (edge4.a * dn4dx - edge6.a * dn6dx),
            edge6.b * dn6dx + edge4.b * dn4dx,
            dn2qdx - edge6.c * dn6dx - edge4.c * dn4dx,
            1.5 * (edge5.a * dn5dx - edge4.a * dn4dx),
            edge4.b * dn4dx + edge5.b * dn5dx,
            dn3qdx - edge4.c * dn4dx - edge5.c * dn5dx,
        ];
        let dhx_dy = [
            1.5 * (edge6.a * dn6dy - edge5.a * dn5dy),
            edge5.b * dn5dy + edge6.b * dn6dy,
            dn1qdy - edge5.c * dn5dy - edge6.c * dn6dy,
            1.5 * (edge4.a * dn4dy - edge6.a * dn6dy),
            edge6.b * dn6dy + edge4.b * dn4dy,
            dn2qdy - edge6.c * dn6dy - edge4.c * dn4dy,
            1.5 * (edge5.a * dn5dy - edge4.a * dn4dy),
            edge4.b * dn4dy + edge5.b * dn5dy,
            dn3qdy - edge4.c * dn4dy - edge5.c * dn5dy,
        ];
        let dhy_dy = [
            1.5 * (edge6.d * dn6dy - edge5.d * dn5dy),
            -dn1qdy + edge5.e * dn5dy + edge6.e * dn6dy,
            -edge5.b * dn5dy - edge6.b * dn6dy,
            1.5 * (edge4.d * dn4dy - edge6.d * dn6dy),
            -dn2qdy + edge4.e * dn4dy + edge6.e * dn6dy,
            -edge4.b * dn4dy - edge6.b * dn6dy,
            1.5 * (edge5.d * dn5dy - edge4.d * dn4dy),
            -dn3qdy + edge4.e * dn4dy + edge5.e * dn5dy,
            -edge4.b * dn4dy - edge5.b * dn5dy,
        ];
        let dhy_dx = [
            1.5 * (edge6.d * dn6dx - edge5.d * dn5dx),
            -dn1qdx + edge5.e * dn5dx + edge6.e * dn6dx,
            -edge5.b * dn5dx - edge6.b * dn6dx,
            1.5 * (edge4.d * dn4dx - edge6.d * dn6dx),
            -dn2qdx + edge4.e * dn4dx + edge6.e * dn6dx,
            -edge4.b * dn4dx - edge6.b * dn6dx,
            1.5 * (edge5.d * dn5dx - edge4.d * dn4dx),
            -dn3qdx + edge4.e * dn4dx + edge5.e * dn5dx,
            -edge4.b * dn4dx - edge5.b * dn5dx,
        ];

        // Batoz's Hx/Hy columns land directly on our (w,Rx,Ry) triple with no permutation, but with an
        // overall sign flip (see the doc comment above): κx=-∂Hx/∂x, κy=-∂Hy/∂y, κxy=-(∂Hx/∂y+∂Hy/∂x).
        let mut b = MatD::zeros(3, 9);
        for i in 0..3 {
            let (bw, bbx, bby) = (3 * i, 3 * i + 1, 3 * i + 2);
            b.set(0, bw, -dhx_dx[bw]);
            b.set(0, bbx, -dhx_dx[bbx]);
            b.set(0, bby, -dhx_dx[bby]);
            b.set(1, bw, -dhy_dy[bw]);
            b.set(1, bbx, -dhy_dy[bbx]);
            b.set(1, bby, -dhy_dy[bby]);
            b.set(2, bw, -dhx_dy[bw] - dhy_dx[bw]);
            b.set(2, bbx, -dhx_dy[bbx] - dhy_dx[bbx]);
            b.set(2, bby, -dhx_dy[bby] - dhy_dx[bby]);
        }
        b
    }

    /// 🧊️ Batoz Discrete Kirchhoff Triangle (DKT) — 3-node thin-plate bending element, DOFs `[Tz,Rx,Ry]`
    /// per node (`Rx = ∂w/∂y`, `Ry = -∂w/∂x`, the physically standard rotation-about-axis convention: a
    /// positive rotation about the local x-axis tilts the plate normal the same way a positive `∂w/∂y`
    /// slope does). 3-point Gauss-tri integration of the (non-constant, unlike CST) curvature field. See
    /// Batoz, Bathe & Ho (1980) "A study of three-node triangular plate bending elements".
    ///
    /// 🌀️ Reports NO `geometric_stiffness` (stays the trait default `None`) — a pure bending element
    /// carries no membrane stress state to destabilize its own transverse deflection; plate/shell
    /// buckling under in-plane compression needs the membrane-bending coupling `elements3d::ShellFacet3`
    /// provides, not `PlateDkt` alone.
    pub struct PlateDkt {
        pub id: String,
        pub nodes: [String; 3],
        pub e: f64,
        pub nu: f64,
        pub thickness: f64,
        pub density: f64,
    }

    impl PlateDkt {
        fn coords(ctx: &ElementContext) -> [[f64; 2]; 3] {
            [[ctx.positions[0][0], ctx.positions[0][1]], [ctx.positions[1][0], ctx.positions[1][1]], [ctx.positions[2][0], ctx.positions[2][1]]]
        }
    }

    impl Element for PlateDkt {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }

        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tz, Dof::Rx, Dof::Ry]
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let coords = Self::coords(ctx);
            let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
            let d = d_matrix_plate(self.e, self.nu, self.thickness);
            let mut ke = MatD::zeros(9, 9);
            for (xi, eta, w) in gauss_tri(3) {
                let b = dkt_b_matrix(&coords, xi, eta);
                ke.add_triple_product(&b, &d, w * det_j);
            }
            ke
        }

        fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let coords = Self::coords(ctx);
            let d = d_matrix_plate(self.e, self.nu, self.thickness);
            let gauss = gauss_tri(3)
                .into_iter()
                .map(|(xi, eta, _)| {
                    let b = dkt_b_matrix(&coords, xi, eta);
                    let kappa = b.mul_vec(u_local);
                    let m = d.mul_vec(&kappa);
                    PlateMoments { mx: m.get(0), my: m.get(1), mxy: m.get(2) }
                })
                .collect();
            ElementResult::Plate { gauss }
        }

        /// 🏋️ Lumped translational mass `ρtA/3` on each node's `Tz` only — zero rotary inertia. DKT has
        /// no independent transverse-displacement interpolation to derive a consistent mass from (its
        /// curvature field comes from `w`+rotations jointly), so lumping the plate's own weight evenly
        /// across its 3 corners is the standard practical simplification (Cook, Malkus, Plesha & Witt).
        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let coords = Self::coords(ctx);
            let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
            let area = 0.5 * det_j;
            let share = self.density * self.thickness * area / 3.0;
            let mut m = MatD::zeros(9, 9);
            for i in 0..3 {
                m.set(3 * i, 3 * i, share);
            }
            Some(m)
        }
    }
    // #endregion 🔖️PlateDkt

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{solve_linear_static, Model, NodalLoad, Node, Support};

        /// 🪢️ Headless (no document layer) axial elongation check: δ = FL/EA, N = F.
        #[test]
        fn bar2_axial_matches_hand_calc() {
            let (e, area, l, p) = (200e9, 0.001, 2.0, 5000.0);
            let model = Model {
                nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
                elements: vec![Box::new(Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 })],
                // A single bar only resists motion along its own axis, so `b`'s transverse (Ty) DOF must
                // also be restrained here — otherwise it's a mechanism (zero stiffness, singular system).
                supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
                nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p }],
                member_loads: vec![],
            };
            let result = solve_linear_static(&model).expect("solves");
            let expected = p * l / (e * area);
            let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
            assert!((b.values[Dof::Tx.index()] - expected).abs() / expected < 1e-9);
            let ElementResult::Bar { n } = result.elements[0].1 else { panic!("expected bar") };
            assert!((n - p).abs() < 1e-6);
        }

        /// 🏗️ Headless cantilever tip-load check: δ = PL³/3EI, θ = PL²/2EI — the classic beam-theory
        /// benchmark, exercised here directly against `fem_core::Model` (no document layer involved).
        #[test]
        fn beam_eb2_cantilever_matches_hand_calc() {
            let (e, iy, area, l, p) = (200e9, 1e-5, 0.01, 2.0, 1000.0);
            let model = Model {
                nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
                elements: vec![Box::new(BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 })],
                supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
                nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -p }],
                member_loads: vec![],
            };
            let result = solve_linear_static(&model).expect("solves");
            let expected_deflection = p * l.powi(3) / (3.0 * e * iy);
            let expected_rotation = p * l.powi(2) / (2.0 * e * iy);
            let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
            assert!((b.values[Dof::Ty.index()].abs() - expected_deflection).abs() / expected_deflection < 1e-6);
            assert!((b.values[Dof::Rz.index()].abs() - expected_rotation).abs() / expected_rotation < 1e-6);
        }

        /// 🌀️ Rigid-body test: a pure translation (no relative deformation) must produce zero internal
        /// force — `Ke * rigid_translation ≈ 0`. Catches sign/assembly bugs that a single load case might not.
        #[test]
        fn beam_eb2_rigid_translation_gives_zero_force() {
            let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
            let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            let ke = beam.stiffness_global(&ctx);
            let rigid = VecD::from_vec(vec![3.0, 4.0, 0.0, 3.0, 4.0, 0.0]);
            let f = ke.mul_vec(&rigid);
            for i in 0..6 {
                assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
            }
        }

        /// 🏋️ `Bar2::mass` matches the hand-derived isotropic `m = ρAL/6` block form directly.
        #[test]
        fn bar2_mass_matches_hand_calc() {
            let (density, area, l) = (7850.0, 0.001, 2.0);
            let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e: 200e9, area, density };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            let m = bar.mass(&ctx).expect("bar2 reports mass");
            let expected = density * area * l / 6.0;
            assert!((m.get(0, 0) - 2.0 * expected).abs() < 1e-9);
            assert!((m.get(1, 1) - 2.0 * expected).abs() < 1e-9);
            assert!((m.get(0, 2) - expected).abs() < 1e-9);
            assert!((m.get(1, 3) - expected).abs() < 1e-9);
            assert!((m.get(0, 1)).abs() < 1e-12, "no coupling between Tx and Ty");
        }

        /// ⚖️ Consistent-mass physical sanity check: the sum of ALL entries in a pure-translational
        /// submatrix (no rotational DOFs involved) must equal the element's total mass `ρAL` — a
        /// consequence of the shape functions partitioning unity.
        #[test]
        fn bar2_mass_total_equals_rho_a_l() {
            let (density, area, l) = (7850.0, 0.001, 2.0);
            let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e: 200e9, area, density };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            let m = bar.mass(&ctx).expect("bar2 reports mass");
            let sum_tx: f64 = [0, 2].iter().flat_map(|&r| [0, 2].iter().map(move |&c| (r, c))).map(|(r, c)| m.get(r, c)).sum();
            assert!((sum_tx - density * area * l).abs() / (density * area * l) < 1e-9);
        }

        /// 🏋️ `BeamEb2::mass`'s axial 2x2 submatrix sums to the total member mass `ρAL` (same identity as
        /// `Bar2`'s, since the axial DOFs carry no rotational coupling) — checked on a horizontal member so
        /// global == local (rotation is identity) and hand-derived indices apply directly.
        #[test]
        fn beam_eb2_mass_axial_block_sums_to_total_mass() {
            let (e, iy, area, l, density) = (200e9, 1e-5, 0.01, 2.0, 7850.0);
            let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            let m = beam.mass(&ctx).expect("beam_eb2 reports mass");
            let sum_axial = m.get(0, 0) + m.get(0, 3) + m.get(3, 0) + m.get(3, 3);
            let expected = density * area * l;
            assert!((sum_axial - expected).abs() / expected < 1e-9);
        }

        /// 🌀️ Geometric stiffness must vanish under a pure rigid translation, same as ordinary stiffness —
        /// a non-zero axial force alone shouldn't invent a force from rigid motion.
        #[test]
        fn beam_eb2_geometric_stiffness_rigid_translation_gives_zero_force() {
            let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
            let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            // A pure translation along +x produces a nonzero axial force n = EA/L * dx; geometric
            // stiffness only touches the bending block, so a pure translation still gives zero force there.
            let u = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
            let kg = beam.geometric_stiffness(&ctx, &u).expect("beam_eb2 reports geometric stiffness");
            let rigid = VecD::from_vec(vec![3.0, 4.0, 0.0, 3.0, 4.0, 0.0]);
            let f = kg.mul_vec(&rigid);
            for i in 0..6 {
                assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }

        /// 🌀️ Geometric stiffness is symmetric and scales linearly with the recovered axial force.
        #[test]
        fn beam_eb2_geometric_stiffness_is_symmetric_and_scales_with_axial_force() {
            let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
            let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            let u1 = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
            let u2 = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.002, 0.0, 0.0]);
            let kg1 = beam.geometric_stiffness(&ctx, &u1).unwrap();
            let kg2 = beam.geometric_stiffness(&ctx, &u2).unwrap();
            for r in 0..6 {
                for c in 0..6 {
                    assert!((kg1.get(r, c) - kg1.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                    assert!((kg2.get(r, c) - 2.0 * kg1.get(r, c)).abs() < 1e-6, "Kg should scale linearly with axial force at ({r},{c})");
                }
            }
        }

        /// 🌬️ `Bar2::equivalent_nodal_loads` splits a global UDL `wL/2` exactly evenly at both nodes.
        #[test]
        fn bar2_equivalent_nodal_loads_matches_wl_over_2() {
            let (e, area, l) = (200e9, 0.001, 2.0);
            let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            let udl = MemberUdl { wx: 100.0, wy: -50.0, wz: 0.0 };
            let f = bar.equivalent_nodal_loads(&ctx, &udl).expect("bar2 reports equivalent nodal loads");
            let half = l / 2.0;
            assert!((f.get(0) - udl.wx * half).abs() < 1e-9);
            assert!((f.get(1) - udl.wy * half).abs() < 1e-9);
            assert!((f.get(2) - udl.wx * half).abs() < 1e-9);
            assert!((f.get(3) - udl.wy * half).abs() < 1e-9);
        }

        /// 🌀️ `Bar2::geometric_stiffness`: zero under rigid translation, symmetric, and destabilizes only
        /// the direction PERPENDICULAR to the bar's own axis (an axially-aligned bar with axial force `n`
        /// should have ZERO transverse stiffness contribution along its own axis).
        #[test]
        fn bar2_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
            let (e, area, l) = (200e9, 0.001, 2.0);
            let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            let u = VecD::from_vec(vec![0.0, 0.0, 0.001, 0.0]);
            let kg = bar.geometric_stiffness(&ctx, &u).expect("bar2 reports geometric stiffness");
            for r in 0..4 {
                for c in 0..4 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                }
            }
            let rigid = VecD::from_vec(vec![3.0, 4.0, 3.0, 4.0]);
            let f = kg.mul_vec(&rigid);
            for i in 0..4 {
                assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
            // Axial member here runs along global X, so `Kg`'s axial (Tx) rows/columns must be zero.
            for i in [0usize, 2] {
                for j in 0..4 {
                    assert!(kg.get(i, j).abs() < 1e-6, "Kg({i},{j}) should be zero along the bar's own axis");
                }
            }
        }
    }
    // #endregion 🔖️Tests

    // #region 🔖️ContinuumTests
    #[cfg(test)]
    mod continuum_tests {
        use super::*;
        use crate::{solve_linear_static, Model, NodalLoad, Node, Support};

        /// 📐️ Builds a node-major `[u_i,v_i]` displacement vector by sampling the linear field
        /// `u = a.0 + a.1*x + a.2*y`, `v = b.0 + b.1*x + b.2*y` at every node coordinate — the standard
        /// FEM patch-test input, guaranteed to be reproduced EXACTLY by any complete element basis.
        fn linear_field_u_local(coords: &[[f64; 2]], a: (f64, f64, f64), b: (f64, f64, f64)) -> VecD {
            let mut v = Vec::with_capacity(coords.len() * 2);
            for &[x, y] in coords {
                v.push(a.0 + a.1 * x + a.2 * y);
                v.push(b.0 + b.1 * x + b.2 * y);
            }
            VecD::from_vec(v)
        }

        fn rigid_translation_u_local(n_nodes: usize, dx: f64, dy: f64) -> VecD {
            let mut v = Vec::with_capacity(n_nodes * 2);
            for _ in 0..n_nodes {
                v.push(dx);
                v.push(dy);
            }
            VecD::from_vec(v)
        }

        fn assert_plane_gauss_matches(gauss: &[PlaneStress], expected: (f64, f64, f64), tol: f64) {
            for gp in gauss {
                assert!((gp.sxx - expected.0).abs() < tol, "sxx {} vs {}", gp.sxx, expected.0);
                assert!((gp.syy - expected.1).abs() < tol, "syy {} vs {}", gp.syy, expected.1);
                assert!((gp.sxy - expected.2).abs() < tol, "sxy {} vs {}", gp.sxy, expected.2);
            }
        }

        fn assert_rigid_body_gives_zero_force(ke: &MatD, u_local: &VecD) {
            let f = ke.mul_vec(u_local);
            for i in 0..f.len() {
                assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
            }
        }

        // Shared "test material" — small-magnitude E keeps expected stresses O(1) so the 1e-8 absolute
        // patch-test tolerance is meaningful relative to f64 precision, not swamped by it.
        const E: f64 = 1000.0;
        const NU: f64 = 0.25;
        const A: (f64, f64, f64) = (0.01, 0.003, 0.0021);
        const B: (f64, f64, f64) = (-0.02, 0.0012, 0.0027);

        fn expected_stress(kind: PlaneKind) -> (f64, f64, f64) {
            let d = match kind {
                PlaneKind::Stress => d_matrix_plane_stress(E, NU),
                PlaneKind::Strain => d_matrix_plane_strain(E, NU),
            };
            let strain = VecD::from_vec(vec![A.1, B.2, A.2 + B.1]);
            let sigma = d.mul_vec(&strain);
            (sigma.get(0), sigma.get(1), sigma.get(2))
        }

        fn ctx_of(coords: &[[f64; 2]]) -> ElementContext {
            ElementContext { positions: coords.iter().map(|&[x, y]| [x, y, 0.0]).collect() }
        }

        #[test]
        fn tri3_cst_patch_test_reproduces_linear_field() {
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
            let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = linear_field_u_local(&coords, A, B);
            let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
            assert_eq!(gauss.len(), 1);
            assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
        }

        #[test]
        fn tri3_cst_rigid_translation_gives_zero_force() {
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
            let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let ke = el.stiffness_global(&ctx);
            assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(3, 1.5, -2.3));
        }

        #[test]
        fn tri6_lst_patch_test_reproduces_linear_field() {
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
            let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = linear_field_u_local(&coords, A, B);
            let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
            assert_eq!(gauss.len(), 3);
            assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
        }

        #[test]
        fn tri6_lst_rigid_translation_gives_zero_force() {
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
            let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let ke = el.stiffness_global(&ctx);
            assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(6, 1.5, -2.3));
        }

        #[test]
        fn quad4_patch_test_reproduces_linear_field() {
            let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
            let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Strain, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = linear_field_u_local(&coords, A, B);
            let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
            assert_eq!(gauss.len(), 4);
            assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Strain), 1e-8);
        }

        #[test]
        fn quad4_rigid_translation_gives_zero_force() {
            let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
            let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Strain, density: 0.0 };
            let ctx = ctx_of(&coords);
            let ke = el.stiffness_global(&ctx);
            assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(4, 1.5, -2.3));
        }

        #[test]
        fn quad8_patch_test_reproduces_linear_field() {
            let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
            let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = linear_field_u_local(&coords, A, B);
            let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
            assert_eq!(gauss.len(), 9, "quad8 must use the full 3x3 rule, not 2x2");
            assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
        }

        #[test]
        fn quad8_rigid_translation_gives_zero_force() {
            let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
            let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let ke = el.stiffness_global(&ctx);
            assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(8, 1.5, -2.3));
        }

        /// 🌀️ Cook's membrane: the classic tapered/skewed cantilever panel, meshed on a 4x4 grid of
        /// `Quad4` elements via bilinear blending of the four corner points. A coarse-mesh sanity check
        /// (not a fine-mesh convergence study) — the tip deflection must be positive and finite.
        #[test]
        fn quad4_cooks_membrane_tip_deflection_is_positive_and_finite() {
            let n = 4usize;
            let (p00, p10, p11, p01) = ((0.0, 0.0), (48.0, 44.0), (48.0, 60.0), (0.0, 44.0));
            let blend = |r: f64, s: f64| {
                let x = (1.0 - r) * (1.0 - s) * p00.0 + r * (1.0 - s) * p10.0 + r * s * p11.0 + (1.0 - r) * s * p01.0;
                let y = (1.0 - r) * (1.0 - s) * p00.1 + r * (1.0 - s) * p10.1 + r * s * p11.1 + (1.0 - r) * s * p01.1;
                (x, y)
            };
            let node_id = |i: usize, j: usize| format!("n{i}_{j}");

            let mut nodes = Vec::new();
            for i in 0..=n {
                for j in 0..=n {
                    let (x, y) = blend(i as f64 / n as f64, j as f64 / n as f64);
                    nodes.push(Node { id: node_id(i, j), pos: [x, y, 0.0] });
                }
            }
            let mut elements: Vec<Box<dyn Element>> = Vec::new();
            for i in 0..n {
                for j in 0..n {
                    elements.push(Box::new(Quad4 { id: format!("e{i}_{j}"), nodes: [node_id(i, j), node_id(i + 1, j), node_id(i + 1, j + 1), node_id(i, j + 1)], e: 1.0, nu: 1.0 / 3.0, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 }));
                }
            }
            let supports = (0..=n).map(|j| Support { node_id: node_id(0, j), fixed: vec![Dof::Tx, Dof::Ty] }).collect();
            let per_node = 1.0 / (n as f64 + 1.0);
            let nodal_loads = (0..=n).map(|j| NodalLoad { node_id: node_id(n, j), dof: Dof::Ty, value: per_node }).collect();

            let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
            let result = solve_linear_static(&model).expect("cook's membrane mesh solves");
            let tip: f64 = (0..=n).map(|j| result.displacements.iter().find(|d| d.node_id == node_id(n, j)).unwrap().values[Dof::Ty.index()]).sum::<f64>() / (n as f64 + 1.0);
            assert!(tip > 0.0 && tip.is_finite(), "tip deflection = {tip}");
        }

        /// ⚖️ Consistent-mass physical sanity check (same identity `bar2_mass_total_equals_rho_a_l` uses):
        /// the sum of the pure-`Tx` submatrix must equal the element's total mass `ρtA`.
        #[test]
        fn tri3_cst_mass_total_equals_rho_t_area() {
            let (density, thickness) = (7850.0, 0.02);
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
            let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness, kind: PlaneKind::Stress, density };
            let ctx = ctx_of(&coords);
            let m = el.mass(&ctx).expect("tri3cst reports mass");
            let area = triangle_signed_area(&coords).abs();
            let sum_tx: f64 = (0..3).flat_map(|r| (0..3).map(move |c| (2 * r, 2 * c))).map(|(r, c)| m.get(r, c)).sum();
            let expected = density * thickness * area;
            assert!((sum_tx - expected).abs() / expected < 1e-9, "sum={sum_tx} expected={expected}");
        }

        fn triangle_signed_area(coords: &[[f64; 2]]) -> f64 {
            0.5 * ((coords[1][0] - coords[0][0]) * (coords[2][1] - coords[0][1]) - (coords[2][0] - coords[0][0]) * (coords[1][1] - coords[0][1]))
        }

        #[test]
        fn quad4_mass_total_equals_rho_t_area() {
            let (density, thickness) = (2400.0, 0.15);
            let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
            let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness, kind: PlaneKind::Strain, density };
            let ctx = ctx_of(&coords);
            let m = el.mass(&ctx).expect("quad4 reports mass");
            // Shoelace area of the (convex) quad, split as two triangles from vertex 0.
            let area = triangle_signed_area(&[coords[0], coords[1], coords[2]]).abs() + triangle_signed_area(&[coords[0], coords[2], coords[3]]).abs();
            let sum_tx: f64 = (0..4).flat_map(|r| (0..4).map(move |c| (2 * r, 2 * c))).map(|(r, c)| m.get(r, c)).sum();
            let expected = density * thickness * area;
            assert!((sum_tx - expected).abs() / expected < 1e-6, "sum={sum_tx} expected={expected}");
        }

        /// 🌀️ `Tri3Cst::geometric_stiffness` must vanish under a pure rigid translation (zero stress ⇒
        /// zero `Kg`, same reasoning `beam_eb2_geometric_stiffness_rigid_translation_gives_zero_force` uses)
        /// and be symmetric under a genuinely deforming field.
        #[test]
        fn tri3_cst_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
            let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = linear_field_u_local(&coords, A, B);
            let kg = el.geometric_stiffness(&ctx, &u).expect("tri3cst reports geometric stiffness");
            for r in 0..6 {
                for c in 0..6 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                }
            }
            let kg_rigid = el.geometric_stiffness(&ctx, &rigid_translation_u_local(3, 1.5, -2.3)).unwrap();
            let f = kg_rigid.mul_vec(&rigid_translation_u_local(3, 0.4, 0.6));
            for i in 0..6 {
                assert!(f.get(i).abs() < 1e-9, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }

        /// 🌀️ `Quad4::geometric_stiffness` must vanish under a pure rigid translation and be symmetric —
        /// the last `Quad4` method not already exercised by `quad4_mass_total_equals_rho_t_area`/the patch
        /// and rigid-translation stiffness tests above.
        #[test]
        fn quad4_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
            let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
            let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Strain, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = linear_field_u_local(&coords, A, B);
            let kg = el.geometric_stiffness(&ctx, &u).expect("quad4 reports geometric stiffness");
            for r in 0..8 {
                for c in 0..8 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                }
            }
            let kg_rigid = el.geometric_stiffness(&ctx, &rigid_translation_u_local(4, 1.5, -2.3)).unwrap();
            let f = kg_rigid.mul_vec(&rigid_translation_u_local(4, 0.4, 0.6));
            for i in 0..8 {
                assert!(f.get(i).abs() < 1e-9, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }

        /// ⚖️ `Tri6Lst::mass` total (same partition-of-unity identity `tri3_cst_mass_total_equals_rho_t_area`
        /// uses) — `Tri6Lst`'s `mass`/`mass_rule`/`shape_full` are otherwise never exercised.
        #[test]
        fn tri6_lst_mass_total_equals_rho_t_area() {
            let (density, thickness) = (7850.0, 0.02);
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
            let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness, kind: PlaneKind::Stress, density };
            let ctx = ctx_of(&coords);
            let m = el.mass(&ctx).expect("tri6lst reports mass");
            let area = triangle_signed_area(&[coords[0], coords[1], coords[2]]).abs();
            let sum_tx: f64 = (0..6).flat_map(|r| (0..6).map(move |c| (2 * r, 2 * c))).map(|(r, c)| m.get(r, c)).sum();
            let expected = density * thickness * area;
            assert!((sum_tx - expected).abs() / expected < 1e-6, "sum={sum_tx} expected={expected}");
        }

        /// 🌀️ `Tri6Lst::geometric_stiffness` must vanish under a pure rigid translation and be symmetric.
        #[test]
        fn tri6_lst_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
            let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = linear_field_u_local(&coords, A, B);
            let kg = el.geometric_stiffness(&ctx, &u).expect("tri6lst reports geometric stiffness");
            for r in 0..12 {
                for c in 0..12 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                }
            }
            let kg_rigid = el.geometric_stiffness(&ctx, &rigid_translation_u_local(6, 1.5, -2.3)).unwrap();
            let f = kg_rigid.mul_vec(&rigid_translation_u_local(6, 0.4, 0.6));
            for i in 0..12 {
                assert!(f.get(i).abs() < 1e-9, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }

        /// ⚖️ `Quad8::mass` total (same identity as `quad4_mass_total_equals_rho_t_area`) — `Quad8`'s
        /// `mass`/`shape_full` are otherwise never exercised.
        #[test]
        fn quad8_mass_total_equals_rho_t_area() {
            let (density, thickness) = (2400.0, 0.15);
            let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
            let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness, kind: PlaneKind::Stress, density };
            let ctx = ctx_of(&coords);
            let m = el.mass(&ctx).expect("quad8 reports mass");
            let area = triangle_signed_area(&[coords[0], coords[1], coords[2]]).abs() + triangle_signed_area(&[coords[0], coords[2], coords[3]]).abs();
            let sum_tx: f64 = (0..8).flat_map(|r| (0..8).map(move |c| (2 * r, 2 * c))).map(|(r, c)| m.get(r, c)).sum();
            let expected = density * thickness * area;
            assert!((sum_tx - expected).abs() / expected < 1e-6, "sum={sum_tx} expected={expected}");
        }

        /// 🌀️ `Quad8::geometric_stiffness` must vanish under a pure rigid translation and be symmetric.
        #[test]
        fn quad8_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
            let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
            let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = linear_field_u_local(&coords, A, B);
            let kg = el.geometric_stiffness(&ctx, &u).expect("quad8 reports geometric stiffness");
            for r in 0..16 {
                for c in 0..16 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                }
            }
            let kg_rigid = el.geometric_stiffness(&ctx, &rigid_translation_u_local(8, 1.5, -2.3)).unwrap();
            let f = kg_rigid.mul_vec(&rigid_translation_u_local(8, 0.4, 0.6));
            for i in 0..16 {
                assert!(f.get(i).abs() < 1e-9, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }

        /// 🔌️ `Tri3Cst`/`Tri6Lst`/`Quad8` used as `Box<dyn Element>` inside a solved `Model` — unlike every
        /// other test in this module (which calls their methods directly), this exercises `id`/`node_ids`/
        /// `dofs_per_node` via the SAME dynamic-dispatch assembly path `solve_linear_static` uses for every
        /// element kind, on three disjoint single-element-type patches sharing one solve.
        #[test]
        fn continuum_elements_solve_correctly_via_dyn_dispatch() {
            let p = 1000.0;
            let mut nodes = vec![Node { id: "t3_a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "t3_b".into(), pos: [2.0, 0.0, 0.0] }, Node { id: "t3_c".into(), pos: [0.0, 2.0, 0.0] }];
            let mut elements: Vec<Box<dyn Element>> = vec![Box::new(Tri3Cst { id: "t3".into(), nodes: ["t3_a".into(), "t3_b".into(), "t3_c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 })];
            let mut supports = vec![Support { node_id: "t3_a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "t3_b".into(), fixed: vec![Dof::Tx, Dof::Ty] }];
            let mut nodal_loads = vec![NodalLoad { node_id: "t3_c".into(), dof: Dof::Tx, value: p }];

            let tri6_ids = ["t6_n0", "t6_n1", "t6_n2", "t6_n01", "t6_n12", "t6_n20"];
            let tri6_coords: [[f64; 2]; 6] = [[10.0, 0.0], [12.0, 0.0], [10.0, 2.0], [11.0, 0.0], [11.0, 1.0], [10.0, 1.0]];
            for i in 0..6 {
                nodes.push(Node { id: tri6_ids[i].into(), pos: [tri6_coords[i][0], tri6_coords[i][1], 0.0] });
            }
            elements.push(Box::new(Tri6Lst { id: "t6".into(), nodes: std::array::from_fn(|i| tri6_ids[i].to_string()), e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 }));
            for &id in &tri6_ids[..5] {
                supports.push(Support { node_id: id.into(), fixed: vec![Dof::Tx, Dof::Ty] });
            }
            nodal_loads.push(NodalLoad { node_id: "t6_n20".into(), dof: Dof::Tx, value: p });

            let quad8_ids = ["q8_c0", "q8_c1", "q8_c2", "q8_c3", "q8_m01", "q8_m12", "q8_m23", "q8_m30"];
            let quad8_coords: [[f64; 2]; 8] = [[20.0, 0.0], [22.0, 0.0], [22.0, 2.0], [20.0, 2.0], [21.0, 0.0], [22.0, 1.0], [21.0, 2.0], [20.0, 1.0]];
            for i in 0..8 {
                nodes.push(Node { id: quad8_ids[i].into(), pos: [quad8_coords[i][0], quad8_coords[i][1], 0.0] });
            }
            elements.push(Box::new(Quad8 { id: "q8".into(), nodes: std::array::from_fn(|i| quad8_ids[i].to_string()), e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 }));
            for &id in &quad8_ids[..7] {
                supports.push(Support { node_id: id.into(), fixed: vec![Dof::Tx, Dof::Ty] });
            }
            nodal_loads.push(NodalLoad { node_id: "q8_m30".into(), dof: Dof::Tx, value: p });

            let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
            let result = solve_linear_static(&model).expect("mixed continuum patches solve");

            assert_eq!(result.elements.len(), 3);
            for (free_node, element_id) in [("t3_c", "t3"), ("t6_n20", "t6"), ("q8_m30", "q8")] {
                let d = result.displacements.iter().find(|d| d.node_id == free_node).unwrap();
                assert!(d.values[Dof::Tx.index()] > 0.0 && d.values[Dof::Tx.index()].is_finite(), "{free_node}: {}", d.values[Dof::Tx.index()]);
                assert!(result.elements.iter().any(|(id, _)| id == element_id), "missing element result for {element_id}");
            }
        }
    }
    // #endregion 🔖️ContinuumTests

    // #region 🔖️PlateTests
    #[cfg(test)]
    mod plate_tests {
        use super::*;
        use crate::{solve_linear_static, Model, NodalLoad, Node, Support};

        const E: f64 = 1000.0;
        const NU: f64 = 0.25;
        const THICKNESS: f64 = 1.0;
        // Small constant curvatures so the resulting moments stay O(1), matching `continuum_tests`'s
        // rationale for keeping the absolute patch-test tolerance meaningful.
        const KX: f64 = 0.004;
        const KY: f64 = -0.0025;
        const KXY: f64 = 0.0017;

        fn ctx_of(coords: &[[f64; 2]; 3]) -> ElementContext {
            ElementContext { positions: coords.iter().map(|&[x, y]| [x, y, 0.0]).collect() }
        }

        /// 📐️ Constant-curvature field `w = 0.5*(kx*x² + ky*y² + 2*kxy*x*y)` with matching nodal rotations
        /// `Rx = ∂w/∂y = ky*y + kxy*x`, `Ry = -∂w/∂x = -(kx*x + kxy*y)` — the DKT patch-test input.
        fn constant_curvature_u_local(coords: &[[f64; 2]; 3]) -> VecD {
            let mut v = Vec::with_capacity(9);
            for &[x, y] in coords {
                v.push(0.5 * (KX * x * x + KY * y * y + 2.0 * KXY * x * y));
                v.push(KY * y + KXY * x);
                v.push(-(KX * x + KXY * y));
            }
            VecD::from_vec(v)
        }

        #[test]
        fn plate_dkt_patch_test_reproduces_constant_curvature() {
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
            let el = PlateDkt { id: "p".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
            let ctx = ctx_of(&coords);
            let u = constant_curvature_u_local(&coords);
            let d = d_matrix_plate(E, NU, THICKNESS);
            let kappa = VecD::from_vec(vec![KX, KY, 2.0 * KXY]);
            let expected = d.mul_vec(&kappa);
            let ElementResult::Plate { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plate result") };
            assert_eq!(gauss.len(), 3);
            for gp in &gauss {
                let scale = expected.get(0).abs().max(expected.get(1).abs()).max(expected.get(2).abs()).max(1.0);
                assert!((gp.mx - expected.get(0)).abs() / scale < 1e-4, "mx {} vs {}", gp.mx, expected.get(0));
                assert!((gp.my - expected.get(1)).abs() / scale < 1e-4, "my {} vs {}", gp.my, expected.get(1));
                assert!((gp.mxy - expected.get(2)).abs() / scale < 1e-4, "mxy {} vs {}", gp.mxy, expected.get(2));
            }
        }

        #[test]
        fn plate_dkt_rigid_translation_gives_zero_force() {
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
            let el = PlateDkt { id: "p".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
            let ctx = ctx_of(&coords);
            let ke = el.stiffness_global(&ctx);
            let rigid = VecD::from_vec(vec![0.7, 0.0, 0.0, 0.7, 0.0, 0.0, 0.7, 0.0, 0.0]);
            let f = ke.mul_vec(&rigid);
            for i in 0..9 {
                assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
            }
        }

        /// 🏋️ `PlateDkt::mass` lumps `ρtA/3` onto each node's `Tz` only — zero rotary inertia, zero
        /// coupling to `Rx`/`Ry` — `mass` is otherwise never exercised (`stiffness_global`/`recover` are
        /// covered by the patch/rigid-translation/simply-supported tests above and below).
        #[test]
        fn plate_dkt_mass_lumps_rho_t_area_over_3_onto_each_tz_only() {
            let (density, thickness) = (2500.0, 0.02);
            let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
            let el = PlateDkt { id: "p".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness, density };
            let ctx = ctx_of(&coords);
            let m = el.mass(&ctx).expect("plate_dkt reports mass");

            let area = 0.5 * ((coords[1][0] - coords[0][0]) * (coords[2][1] - coords[0][1]) - (coords[2][0] - coords[0][0]) * (coords[1][1] - coords[0][1])).abs();
            let expected_share = density * thickness * area / 3.0;
            for i in 0..3 {
                assert!((m.get(3 * i, 3 * i) - expected_share).abs() / expected_share < 1e-9, "node {i} Tz mass");
            }
            for r in 0..9 {
                for c in 0..9 {
                    if r != c {
                        assert!(m.get(r, c).abs() < 1e-12, "unexpected coupling at ({r},{c})");
                    }
                }
            }
            for i in 0..3 {
                assert!(m.get(3 * i + 1, 3 * i + 1).abs() < 1e-12, "node {i} Rx should carry no mass");
                assert!(m.get(3 * i + 2, 3 * i + 2).abs() < 1e-12, "node {i} Ry should carry no mass");
            }
        }

        /// 🏗️ Simply-supported square plate (side `a`) under a uniform pressure `q`, meshed as a coarse
        /// 2x2 grid (8 `PlateDkt` triangles), `Tz=0` at every boundary node (rotations free everywhere),
        /// load lumped `q*Area_i/3` to each triangle's 3 nodes — checked against the classical thin-plate
        /// centerpoint deflection `w = 0.00406*q*a⁴/D` within an order-of-magnitude (coarse mesh, crude
        /// load lumping, so this is a sanity check, not a convergence study).
        #[test]
        fn plate_dkt_simply_supported_square_center_deflection_right_order_of_magnitude() {
            let (e, nu, t, a) = (2e11, 0.3, 0.01, 2.0);
            let q = 1000.0;
            let n = 2usize;
            let dx = a / n as f64;
            let node_id = |i: usize, j: usize| format!("n{i}_{j}");

            let mut nodes = Vec::new();
            for i in 0..=n {
                for j in 0..=n {
                    nodes.push(Node { id: node_id(i, j), pos: [dx * i as f64, dx * j as f64, 0.0] });
                }
            }

            let mut elements: Vec<Box<dyn Element>> = Vec::new();
            for i in 0..n {
                for j in 0..n {
                    // Each grid cell split into 2 triangles along the (i,j)-(i+1,j+1) diagonal.
                    elements.push(Box::new(PlateDkt { id: format!("t{i}_{j}a"), nodes: [node_id(i, j), node_id(i + 1, j), node_id(i + 1, j + 1)], e, nu, thickness: t, density: 0.0 }));
                    elements.push(Box::new(PlateDkt { id: format!("t{i}_{j}b"), nodes: [node_id(i, j), node_id(i + 1, j + 1), node_id(i, j + 1)], e, nu, thickness: t, density: 0.0 }));
                }
            }

            let supports = (0..=n).flat_map(|i| (0..=n).map(move |j| (i, j))).filter(|&(i, j)| i == 0 || i == n || j == 0 || j == n).map(|(i, j)| Support { node_id: node_id(i, j), fixed: vec![Dof::Tz] }).collect();

            // Lump `q*Area/3` per triangle onto its 3 nodes, summed across all triangles sharing a node.
            let mut lumped: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
            for i in 0..n {
                for j in 0..n {
                    let area = 0.5 * dx * dx;
                    let share = q * area / 3.0;
                    for id in [node_id(i, j), node_id(i + 1, j), node_id(i + 1, j + 1)] {
                        *lumped.entry(id).or_insert(0.0) += share;
                    }
                    for id in [node_id(i, j), node_id(i + 1, j + 1), node_id(i, j + 1)] {
                        *lumped.entry(id).or_insert(0.0) += share;
                    }
                }
            }
            let nodal_loads = lumped.into_iter().map(|(node_id, value)| NodalLoad { node_id, dof: Dof::Tz, value: -value }).collect();

            let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
            let result = solve_linear_static(&model).expect("ss plate mesh solves");
            let center = result.displacements.iter().find(|d| d.node_id == node_id(n / 2, n / 2)).unwrap();
            let w_center = -center.values[Dof::Tz.index()];

            let d = e * t.powi(3) / (12.0 * (1.0 - nu * nu));
            let expected = 0.00406 * q * a.powi(4) / d;
            assert!(w_center.is_finite() && w_center > 0.0, "center deflection = {w_center}");
            let ratio = w_center / expected;
            assert!(ratio > 0.5 && ratio < 2.0, "deflection ratio {ratio} (actual {w_center} vs analytical {expected}) out of order-of-magnitude range");
        }
    }
    // #endregion 🔖️PlateTests
}

pub mod elements3d {
    //! 🧊️ 3D structural elements: axial `Bar3` truss, Euler-Bernoulli `Frame3` frame member (with
    //! torsion and member-UDL support), the `Tet4`/`Hex8` solid continuum elements, and `ShellFacet3`
    //! (flat facet shell: CST membrane + DKT bending + drilling stabilization).

    use crate::formulation::{b_matrix_plane, d_matrix_plane_stress, gauss_tri, jacobian_2d, shape_tri3};
    use crate::{BeamStation, Dof, Element, ElementContext, ElementResult, MemberUdl, ShellState, SolidStress};
    use math::algebra::{vec3d_cross, vec3d_length, vec3d_normalize, vec3d_sub, Mat3d, MatD, VecD};

    // #region 🔖️Bar3
    /// 🪵️ Two-node 3D axial truss element — carries only translational DOFs, stiffness `k = EA/L`
    /// projected onto the member's unit direction.
    pub struct Bar3 {
        pub id: String,
        pub node_a: String,
        pub node_b: String,
        pub e: f64,
        pub a: f64,
        pub density: f64,
    }

    impl Element for Bar3 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            vec![self.node_a.clone(), self.node_b.clone()]
        }

        fn dofs_per_node(&self) -> &[Dof] {
            const DOFS: [Dof; 3] = [Dof::Tx, Dof::Ty, Dof::Tz];
            &DOFS
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
            let l = vec3d_length(d);
            let c = vec3d_normalize(d);
            let k = self.e * self.a / l;
            let mut ke = MatD::zeros(6, 6);
            for i in 0..3 {
                for j in 0..3 {
                    let v = k * c[i] * c[j];
                    ke.set(i, j, v);
                    ke.set(i, j + 3, -v);
                    ke.set(i + 3, j, -v);
                    ke.set(i + 3, j + 3, v);
                }
            }
            ke
        }

        fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
            let l = vec3d_length(d);
            let c = vec3d_normalize(d);
            let k = self.e * self.a / l;
            let du = [u_elem.get(3) - u_elem.get(0), u_elem.get(4) - u_elem.get(1), u_elem.get(5) - u_elem.get(2)];
            let n = k * (c[0] * du[0] + c[1] * du[1] + c[2] * du[2]);
            ElementResult::Bar { n }
        }

        /// 🏋️ Isotropic mass — same pattern as `Bar2` but with 3x3 identity blocks, no preferred direction.
        /// `m = ρAL/6`, block `(node_i, node_j) = (2m if i==j else m) * I3`.
        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
            let l = vec3d_length(d);
            let m = self.density * self.a * l / 6.0;
            let mut out = MatD::zeros(6, 6);
            for i in 0..3 {
                out.set(i, i, 2.0 * m);
                out.set(i + 3, i + 3, 2.0 * m);
                out.set(i, i + 3, m);
                out.set(i + 3, i, m);
            }
            Some(out)
        }

        /// 🌬️ Consistent end-load `wL/2` at each node from a global member UDL `(wx,wy,wz)` — same
        /// exact-split reasoning as `elements2d::Bar2::equivalent_nodal_loads`.
        fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
            let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
            let l = vec3d_length(d);
            let half = l / 2.0;
            Some(VecD::from_vec(vec![udl.wx * half, udl.wy * half, udl.wz * half, udl.wx * half, udl.wy * half, udl.wz * half]))
        }

        /// 🌀️ 3D truss geometric stiffness under axial force `n` (tension-positive, `recover`'s convention):
        /// `N/L·(I₃ − ccᵀ)` per 3x3 node block — the 3D analogue of `elements2d::Bar2::geometric_stiffness`.
        fn geometric_stiffness(&self, ctx: &ElementContext, u_elem: &VecD) -> Option<MatD> {
            let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
            let l = vec3d_length(d);
            let c = vec3d_normalize(d);
            let k = self.e * self.a / l;
            let du = [u_elem.get(3) - u_elem.get(0), u_elem.get(4) - u_elem.get(1), u_elem.get(5) - u_elem.get(2)];
            let n = k * (c[0] * du[0] + c[1] * du[1] + c[2] * du[2]);
            let coeff = n / l;
            let mut kg = MatD::zeros(6, 6);
            for i in 0..3 {
                for j in 0..3 {
                    let identity = if i == j { 1.0 } else { 0.0 };
                    let v = coeff * (identity - c[i] * c[j]);
                    kg.set(i, j, v);
                    kg.set(i, j + 3, -v);
                    kg.set(i + 3, j, -v);
                    kg.set(i + 3, j + 3, v);
                }
            }
            Some(kg)
        }
    }
    // #endregion 🔖️Bar3

    // #region 🔖️Frame3
    /// 🧮️ Places a 4x4 bending block into `k` at the given DOF indices (used for both the y- and z-bending
    /// planes, which are decoupled from each other and from axial/torsion).
    fn set_bend_block(k: &mut MatD, idx: [usize; 4], block: [[f64; 4]; 4]) {
        for (bi, &gi) in idx.iter().enumerate() {
            for (bj, &gj) in idx.iter().enumerate() {
                k.set(gi, gj, block[bi][bj]);
            }
        }
    }

    /// 🏗️ Two-node 3D Euler-Bernoulli frame element with torsion — full 6-DOF-per-node member. Local x
    /// runs node-a to node-b; local y/z are built from a reference "up" vector and rotated by `roll`
    /// (radians) about local x. `stiffness_global`/`recover` rotate the decoupled axial/torsion/biaxial
    /// bending local stiffness into global coordinates via the block-diagonal transform `T`.
    pub struct Frame3 {
        pub id: String,
        pub node_a: String,
        pub node_b: String,
        pub e: f64,
        pub g: f64,
        pub a: f64,
        pub iy: f64,
        pub iz: f64,
        pub j: f64,
        pub roll: f64,
        pub density: f64,
    }

    impl Frame3 {
        /// 🧭️ Builds the member length, local 12x12 stiffness, and the 12x12 global<->local block-diagonal
        /// rotation `T` (four `R^T` 3x3 blocks) shared by `stiffness_global` and `recover`.
        fn local_system(&self, ctx: &ElementContext) -> (f64, MatD, MatD) {
            let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
            let l = vec3d_length(d);
            let cx = vec3d_normalize(d);
            let reference = if cx[2].abs() > 0.99 { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] };
            let y_unrot = vec3d_normalize(vec3d_cross(reference, cx));
            let z_unrot = vec3d_cross(cx, y_unrot);
            let (sin_r, cos_r) = self.roll.sin_cos();
            let local_y = [y_unrot[0] * cos_r + z_unrot[0] * sin_r, y_unrot[1] * cos_r + z_unrot[1] * sin_r, y_unrot[2] * cos_r + z_unrot[2] * sin_r];
            let local_z = [z_unrot[0] * cos_r - y_unrot[0] * sin_r, z_unrot[1] * cos_r - y_unrot[1] * sin_r, z_unrot[2] * cos_r - y_unrot[2] * sin_r];
            let rt = Mat3d::from_axes(cx, local_y, local_z).transpose();
            let mut t = MatD::zeros(12, 12);
            for offset in [0usize, 3, 6, 9] {
                for row in 0..3 {
                    for col in 0..3 {
                        t.set(offset + row, offset + col, rt.cols[col][row]);
                    }
                }
            }
            (l, self.local_stiffness(l), t)
        }

        /// 🧮️ Decoupled local 12x12 stiffness: axial, torsion, and biaxial (y/z) Euler-Bernoulli bending.
        fn local_stiffness(&self, l: f64) -> MatD {
            let mut k = MatD::zeros(12, 12);
            let l2 = l * l;
            let ax = self.e * self.a / l;
            k.set(0, 0, ax);
            k.set(0, 6, -ax);
            k.set(6, 0, -ax);
            k.set(6, 6, ax);
            let tor = self.g * self.j / l;
            k.set(3, 3, tor);
            k.set(3, 9, -tor);
            k.set(9, 3, -tor);
            k.set(9, 9, tor);
            let bz = self.e * self.iz / l;
            set_bend_block(
                &mut k,
                [1, 5, 7, 11],
                [[12.0 * bz / l2, 6.0 * bz / l, -12.0 * bz / l2, 6.0 * bz / l], [6.0 * bz / l, 4.0 * bz, -6.0 * bz / l, 2.0 * bz], [-12.0 * bz / l2, -6.0 * bz / l, 12.0 * bz / l2, -6.0 * bz / l], [6.0 * bz / l, 2.0 * bz, -6.0 * bz / l, 4.0 * bz]],
            );
            let by = self.e * self.iy / l;
            set_bend_block(
                &mut k,
                [2, 4, 8, 10],
                [[12.0 * by / l2, -6.0 * by / l, -12.0 * by / l2, -6.0 * by / l], [-6.0 * by / l, 4.0 * by, 6.0 * by / l, 2.0 * by], [-12.0 * by / l2, 6.0 * by / l, 12.0 * by / l2, 6.0 * by / l], [-6.0 * by / l, 2.0 * by, 6.0 * by / l, 4.0 * by]],
            );
            k
        }

        /// 🏋️ Local 12x12 consistent mass: axial `ρAL/6*[[2,1],[1,2]]` at `(0,6)`, torsion `ρJL/6*[[2,1],[1,2]]`
        /// at `(3,9)` — a simplified polar-inertia proxy (not rigorously exact rotary inertia, but the
        /// accepted simplification at this scope — see `mass`'s doc), and both bending planes ([1,5,7,11]
        /// z-plane, [2,4,8,10] y-plane) using the same 156/22L/54/-13L consistent-beam-mass pattern.
        fn local_mass(&self, l: f64) -> MatD {
            let mut m = MatD::zeros(12, 12);
            let axial = self.density * self.a * l / 6.0;
            m.set(0, 0, 2.0 * axial);
            m.set(0, 6, axial);
            m.set(6, 0, axial);
            m.set(6, 6, 2.0 * axial);

            let torsion = self.density * self.j * l / 6.0;
            m.set(3, 3, 2.0 * torsion);
            m.set(3, 9, torsion);
            m.set(9, 3, torsion);
            m.set(9, 9, 2.0 * torsion);

            let l2 = l * l;
            let factor = self.density * self.a * l / 420.0;
            let block = [[156.0, 22.0 * l, 54.0, -13.0 * l], [22.0 * l, 4.0 * l2, 13.0 * l, -3.0 * l2], [54.0, 13.0 * l, 156.0, -22.0 * l], [-13.0 * l, -3.0 * l2, -22.0 * l, 4.0 * l2]];
            for (bi, &gi) in [1usize, 5, 7, 11].iter().enumerate() {
                for (bj, &gj) in [1usize, 5, 7, 11].iter().enumerate() {
                    m.set(gi, gj, factor * block[bi][bj]);
                }
            }
            for (bi, &gi) in [2usize, 4, 8, 10].iter().enumerate() {
                for (bj, &gj) in [2usize, 4, 8, 10].iter().enumerate() {
                    m.set(gi, gj, factor * block[bi][bj]);
                }
            }
            m
        }

        /// 🌀️ Local 12x12 geometric stiffness under axial force `n` (tension-positive, matches `recover`'s
        /// convention), applied independently to both bending planes via the same `Kg_bend` beam-column
        /// formula `beam_local_geometric_stiffness` in `elements2d` uses.
        fn local_geometric_stiffness(&self, l: f64, n: f64) -> MatD {
            let mut kg = MatD::zeros(12, 12);
            let l2 = l * l;
            let coeff = n / l;
            let block = [[6.0 / 5.0, l / 10.0, -6.0 / 5.0, l / 10.0], [l / 10.0, 2.0 * l2 / 15.0, -l / 10.0, -l2 / 30.0], [-6.0 / 5.0, -l / 10.0, 6.0 / 5.0, -l / 10.0], [l / 10.0, -l2 / 30.0, -l / 10.0, 2.0 * l2 / 15.0]];
            for (bi, &gi) in [1usize, 5, 7, 11].iter().enumerate() {
                for (bj, &gj) in [1usize, 5, 7, 11].iter().enumerate() {
                    kg.set(gi, gj, coeff * block[bi][bj]);
                }
            }
            for (bi, &gi) in [2usize, 4, 8, 10].iter().enumerate() {
                for (bj, &gj) in [2usize, 4, 8, 10].iter().enumerate() {
                    kg.set(gi, gj, coeff * block[bi][bj]);
                }
            }
            kg
        }
    }

    /// 🌬️ Rotates a GLOBAL member UDL `(wx,wy,wz)` into LOCAL `(wx_l,wy_l,wz_l)` via the same 12x12
    /// global->local block-diagonal rotation `T` used for stiffness/displacement (`local_system`'s `t`).
    fn local_udl_components(t: &MatD, udl: &MemberUdl) -> (f64, f64, f64) {
        let global_w = VecD::from_vec(vec![udl.wx, udl.wy, udl.wz, 0.0, 0.0, 0.0, udl.wx, udl.wy, udl.wz, 0.0, 0.0, 0.0]);
        let local_w = t.mul_vec(&global_w);
        (local_w.get(0), local_w.get(1), local_w.get(2))
    }

    /// 🌬️ Local 12-vector fixed-end load for a member UDL, dof order `[u,v,w,θx,θy,θz]` per node.
    /// The z-bending plane (`v`,`θz`, indices 1/5/7/11, using `iz`) uses the standard beam fixed-end-load
    /// formula (identical in form to `elements2d`'s `beam_local_udl`). The y-bending plane (`w`,`θy`,
    /// indices 2/4/8/10, using `iy`) carries the same off-diagonal sign flip `local_stiffness`'s `by`
    /// block has relative to its `bz` block — hand-verified against a cantilever-under-UDL benchmark
    /// (base moment `wL²/2`, base shear `wL`, zero at the tip) in `solid_tests`.
    fn local_udl(l: f64, t: &MatD, udl: &MemberUdl) -> VecD {
        let (wx_l, wy_l, wz_l) = local_udl_components(t, udl);
        let l2 = l * l;
        let mut f = VecD::zeros(12);
        f.set(0, wx_l * l / 2.0);
        f.set(6, wx_l * l / 2.0);
        f.set(1, wy_l * l / 2.0);
        f.set(5, wy_l * l2 / 12.0);
        f.set(7, wy_l * l / 2.0);
        f.set(11, -wy_l * l2 / 12.0);
        f.set(2, wz_l * l / 2.0);
        f.set(4, -wz_l * l2 / 12.0);
        f.set(8, wz_l * l / 2.0);
        f.set(10, wz_l * l2 / 12.0);
        f
    }

    impl Element for Frame3 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            vec![self.node_a.clone(), self.node_b.clone()]
        }

        fn dofs_per_node(&self) -> &[Dof] {
            const DOFS: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];
            &DOFS
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let (_l, k_local, t) = self.local_system(ctx);
            t.transpose().matmul(&k_local).matmul(&t)
        }

        fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
            let (l, _k_local, t) = self.local_system(ctx);
            let f_local = local_udl(l, &t, udl);
            Some(t.transpose().mul_vec(&f_local))
        }

        fn recover(&self, ctx: &ElementContext, u_elem: &VecD, udl: Option<&MemberUdl>) -> ElementResult {
            let (l, k_local, t) = self.local_system(ctx);
            let u_loc = t.mul_vec(u_elem);
            let f_udl_local = udl.map(|u| local_udl(l, &t, u)).unwrap_or_else(|| VecD::zeros(12));
            let f = k_local.mul_vec(&u_loc).sub(&f_udl_local);
            let n = -f.get(0);
            let v1 = f.get(2);
            let m1 = f.get(4);
            let wz_l = udl.map(|u| local_udl_components(&t, u).2).unwrap_or(0.0);
            let stations = (0..11)
                .map(|i| {
                    let x = l * (i as f64) / 10.0;
                    BeamStation { x, n, v: v1 + wz_l * x, m: m1 + v1 * x + wz_l * x * x / 2.0 }
                })
                .collect();
            ElementResult::Beam { stations }
        }

        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let (l, _k_local, t) = self.local_system(ctx);
            let m_local = self.local_mass(l);
            Some(t.transpose().matmul(&m_local).matmul(&t))
        }

        /// 🌀️ Buckling geometric stiffness from the member's own axial force under `u_element` — same
        /// sign convention as `recover`'s `n` (tension-positive): `n = -k_local.mul_vec(u_loc).get(0)`.
        fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
            let (l, k_local, t) = self.local_system(ctx);
            let u_loc = t.mul_vec(u_element);
            let f = k_local.mul_vec(&u_loc);
            let n = -f.get(0);
            let kg_local = self.local_geometric_stiffness(l, n);
            Some(t.transpose().matmul(&kg_local).matmul(&t))
        }
    }
    // #endregion 🔖️Frame3

    // #region 🔖️Solid
    /// 🧮️ Isotropic 3D solid-mechanics constitutive matrix (6x6), strain order `[εxx,εyy,εzz,γxy,γyz,γxz]`.
    fn d_matrix_solid(e: f64, nu: f64) -> MatD {
        let s = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
        let mut d = MatD::zeros(6, 6);
        d.set(0, 0, s * (1.0 - nu));
        d.set(0, 1, s * nu);
        d.set(0, 2, s * nu);
        d.set(1, 0, s * nu);
        d.set(1, 1, s * (1.0 - nu));
        d.set(1, 2, s * nu);
        d.set(2, 0, s * nu);
        d.set(2, 1, s * nu);
        d.set(2, 2, s * (1.0 - nu));
        let g = s * (1.0 - 2.0 * nu) / 2.0;
        d.set(3, 3, g);
        d.set(4, 4, g);
        d.set(5, 5, g);
        d
    }

    /// 🧮️ Standard solid-mechanics B-matrix (6x3n) from per-node physical shape-function gradients —
    /// shared by `Tet4` (constant gradients, one row of blocks) and `Hex8` (per-Gauss-point gradients).
    fn solid_b_matrix(grads: &[[f64; 3]]) -> MatD {
        let mut b = MatD::zeros(6, grads.len() * 3);
        for (i, g) in grads.iter().enumerate() {
            let (bx, by, bz) = (g[0], g[1], g[2]);
            let c = i * 3;
            b.set(0, c, bx);
            b.set(1, c + 1, by);
            b.set(2, c + 2, bz);
            b.set(3, c, by);
            b.set(3, c + 1, bx);
            b.set(4, c + 1, bz);
            b.set(4, c + 2, by);
            b.set(5, c, bz);
            b.set(5, c + 2, bx);
        }
        b
    }

    /// 🧮️ Von Mises equivalent stress from the full 3D stress state.
    fn von_mises_solid(sxx: f64, syy: f64, szz: f64, sxy: f64, syz: f64, sxz: f64) -> f64 {
        (0.5 * ((sxx - syy).powi(2) + (syy - szz).powi(2) + (szz - sxx).powi(2) + 6.0 * (sxy * sxy + syz * syz + sxz * sxz))).sqrt()
    }
    // #endregion 🔖️Solid

    // #region 🔖️Tet4
    /// 🧊️ Four-node linear tetrahedron — constant-strain solid element, DOFs `[Tx,Ty,Tz]` per node.
    /// Exact under a single "integration point" (no quadrature loop needed: a linear tet has constant
    /// strain over its volume).
    pub struct Tet4 {
        pub id: String,
        pub nodes: [String; 4],
        pub e: f64,
        pub nu: f64,
        pub density: f64,
    }

    impl Tet4 {
        /// 🧭️ Signed volume via the scalar triple product of edge vectors from node 0.
        fn volume(ctx: &ElementContext) -> f64 {
            let p = &ctx.positions;
            let e1 = vec3d_sub(p[1], p[0]);
            let e2 = vec3d_sub(p[2], p[0]);
            let e3 = vec3d_sub(p[3], p[0]);
            let cross = vec3d_cross(e1, e2);
            (cross[0] * e3[0] + cross[1] * e3[1] + cross[2] * e3[2]).abs() / 6.0
        }

        /// 🧭️ Constant per-node shape-function gradients `[∂Li/∂x, ∂Li/∂y, ∂Li/∂z]`. `Li(x,y,z) = a+bx+cy+dz`
        /// with `Li(node_j) = δij` for all j — solving `R·[a,b,c,d]ᵀ = e_i` per node (`R`'s row j is
        /// `[1,xj,yj,zj]`) gives node i's coefficients directly, gradient in components 1..4.
        fn gradients(ctx: &ElementContext) -> [[f64; 3]; 4] {
            let p = &ctx.positions;
            let mut r = MatD::zeros(4, 4);
            for (j, pj) in p.iter().enumerate() {
                r.set(j, 0, 1.0);
                r.set(j, 1, pj[0]);
                r.set(j, 2, pj[1]);
                r.set(j, 3, pj[2]);
            }
            let mut grads = [[0.0; 3]; 4];
            for (i, slot) in grads.iter_mut().enumerate() {
                let mut e = VecD::zeros(4);
                e.set(i, 1.0);
                let coeffs = r.lu_solve(&e).expect("non-degenerate tet4");
                *slot = [coeffs.get(1), coeffs.get(2), coeffs.get(3)];
            }
            grads
        }
    }

    impl Element for Tet4 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }

        fn dofs_per_node(&self) -> &[Dof] {
            const DOFS: [Dof; 3] = [Dof::Tx, Dof::Ty, Dof::Tz];
            &DOFS
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let v = Self::volume(ctx);
            let grads = Self::gradients(ctx);
            let b = solid_b_matrix(&grads);
            let d = d_matrix_solid(self.e, self.nu);
            let mut ke = MatD::zeros(12, 12);
            ke.add_triple_product(&b, &d, v);
            ke
        }

        fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let grads = Self::gradients(ctx);
            let b = solid_b_matrix(&grads);
            let d = d_matrix_solid(self.e, self.nu);
            let strain = b.mul_vec(u_elem);
            let stress = d.mul_vec(&strain);
            let (sxx, syy, szz, sxy, syz, sxz) = (stress.get(0), stress.get(1), stress.get(2), stress.get(3), stress.get(4), stress.get(5));
            let von_mises = von_mises_solid(sxx, syy, szz, sxy, syz, sxz);
            ElementResult::Solid { gauss: vec![SolidStress { sxx, syy, szz, sxy, syz, sxz, von_mises }] }
        }

        /// 🏋️ Consistent tet mass `ρV/20 * (2 on the diagonal, 1 off-diagonal)` per direction — the
        /// standard closed-form linear-tetrahedron consistent mass (Cook, Malkus, Plesha & Witt), exact
        /// since `Ni=Li` are the tet's own barycentric coordinates.
        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let v = Self::volume(ctx);
            let mut m = MatD::zeros(12, 12);
            for i in 0..4 {
                for j in 0..4 {
                    let scalar = self.density * v / 20.0 * if i == j { 2.0 } else { 1.0 };
                    for a in 0..3 {
                        m.set(3 * i + a, 3 * j + a, scalar);
                    }
                }
            }
            Some(m)
        }

        /// 🌀️ Initial-stress geometric stiffness `Kg = V·Gᵀ·(σ̂⊗I₃)·G` from the element's own (constant)
        /// stress state under `u_elem` — the 3D analogue of `elements2d::plane_geometric_stiffness`,
        /// `σ̂` the full 3x3 stress tensor built from the recovered `[sxx,syy,szz,sxy,syz,sxz]`.
        fn geometric_stiffness(&self, ctx: &ElementContext, u_elem: &VecD) -> Option<MatD> {
            let v = Self::volume(ctx);
            let grads = Self::gradients(ctx);
            let b = solid_b_matrix(&grads);
            let d = d_matrix_solid(self.e, self.nu);
            let strain = b.mul_vec(u_elem);
            let stress = d.mul_vec(&strain);
            let (sxx, syy, szz, sxy, syz, sxz) = (stress.get(0), stress.get(1), stress.get(2), stress.get(3), stress.get(4), stress.get(5));
            let mut kg = MatD::zeros(12, 12);
            for i in 0..4 {
                let gi = grads[i];
                for j in 0..4 {
                    let gj = grads[j];
                    let s = gi[0] * (sxx * gj[0] + sxy * gj[1] + sxz * gj[2]) + gi[1] * (sxy * gj[0] + syy * gj[1] + syz * gj[2]) + gi[2] * (sxz * gj[0] + syz * gj[1] + szz * gj[2]);
                    let val = s * v;
                    for a in 0..3 {
                        kg.add_at(3 * i + a, 3 * j + a, val);
                    }
                }
            }
            Some(kg)
        }
    }
    // #endregion 🔖️Tet4

    // #region 🔖️Hex8
    /// 🧭️ Reference-cube corner sign vectors `(ξi,ηi,ζi)`, node order: bottom face (ζ=-1) CCW from
    /// `(-1,-1,-1)` [0-3], top face (ζ=1) CCW from `(-1,-1,1)` [4-7] — node `i+4` sits above node `i`.
    const HEX8_CORNERS: [[f64; 3]; 8] = [[-1.0, -1.0, -1.0], [1.0, -1.0, -1.0], [1.0, 1.0, -1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0], [1.0, -1.0, 1.0], [1.0, 1.0, 1.0], [-1.0, 1.0, 1.0]];

    /// 🧭️ 2x2x2 Gauss points (`±1/√3`, weight 1 each — 8 points, tensor product of the 1D 2-point rule).
    fn hex8_gauss_points() -> [([f64; 3], f64); 8] {
        let g = 1.0 / 3.0_f64.sqrt();
        let mut pts = [([0.0; 3], 1.0); 8];
        let mut idx = 0;
        for &xi in &[-g, g] {
            for &eta in &[-g, g] {
                for &zeta in &[-g, g] {
                    pts[idx] = ([xi, eta, zeta], 1.0);
                    idx += 1;
                }
            }
        }
        pts
    }

    /// 🧭️ Per-node trilinear shape values `Ni = 0.125*(1+ξξi)(1+ηηi)(1+ζζi)` at one point — shared by
    /// `mass`'s `Nᵀ·N` (the stiffness/recover Gauss loop only needed `hex8_param_derivs`, not values).
    fn hex8_shape(xi: f64, eta: f64, zeta: f64) -> [f64; 8] {
        let mut n = [0.0; 8];
        for (i, c) in HEX8_CORNERS.iter().enumerate() {
            n[i] = 0.125 * (1.0 + xi * c[0]) * (1.0 + eta * c[1]) * (1.0 + zeta * c[2]);
        }
        n
    }

    /// 🧭️ Per-node parametric shape-function derivatives `[∂Ni/∂ξ, ∂Ni/∂η, ∂Ni/∂ζ]` at one Gauss point.
    fn hex8_param_derivs(xi: f64, eta: f64, zeta: f64) -> [[f64; 3]; 8] {
        let mut out = [[0.0; 3]; 8];
        for (i, c) in HEX8_CORNERS.iter().enumerate() {
            let (xi_i, eta_i, zeta_i) = (c[0], c[1], c[2]);
            out[i] = [0.125 * xi_i * (1.0 + eta * eta_i) * (1.0 + zeta * zeta_i), 0.125 * eta_i * (1.0 + xi * xi_i) * (1.0 + zeta * zeta_i), 0.125 * zeta_i * (1.0 + xi * xi_i) * (1.0 + eta * eta_i)];
        }
        out
    }

    /// 🧭️ 3x3 determinant via cofactor expansion (Jacobians are always 3x3, no need for general-`n` logic).
    fn mat3_det(j: &MatD) -> f64 {
        j.get(0, 0) * (j.get(1, 1) * j.get(2, 2) - j.get(1, 2) * j.get(2, 1)) - j.get(0, 1) * (j.get(1, 0) * j.get(2, 2) - j.get(1, 2) * j.get(2, 0)) + j.get(0, 2) * (j.get(1, 0) * j.get(2, 1) - j.get(1, 1) * j.get(2, 0))
    }

    /// 🧊️ Eight-node trilinear hexahedron ("brick") — DOFs `[Tx,Ty,Tz]` per node, 2x2x2 Gauss integration.
    pub struct Hex8 {
        pub id: String,
        pub nodes: [String; 8],
        pub e: f64,
        pub nu: f64,
        pub density: f64,
    }

    impl Hex8 {
        /// 🧭️ Jacobian `J[a][b] = Σi ∂Ni/∂param_a · coord_i[b]`, its determinant, and the physical
        /// shape-function gradients `∂Ni/∂[x,y,z] = J⁻¹ · ∂Ni/∂[ξ,η,ζ]` (solved via `lu_solve`, one
        /// right-hand side per node, rather than a hand-derived closed-form 3x3 inverse).
        fn gradients_at(ctx: &ElementContext, xi: f64, eta: f64, zeta: f64) -> (f64, [[f64; 3]; 8]) {
            let param = hex8_param_derivs(xi, eta, zeta);
            let mut j = MatD::zeros(3, 3);
            for (i, pd) in param.iter().enumerate() {
                for a in 0..3 {
                    for b in 0..3 {
                        j.add_at(a, b, pd[a] * ctx.positions[i][b]);
                    }
                }
            }
            let det_j = mat3_det(&j);
            let mut grads = [[0.0; 3]; 8];
            for (i, slot) in grads.iter_mut().enumerate() {
                let rhs = VecD::from_vec(param[i].to_vec());
                let phys = j.lu_solve(&rhs).expect("non-degenerate hex8");
                *slot = [phys.get(0), phys.get(1), phys.get(2)];
            }
            (det_j, grads)
        }
    }

    impl Element for Hex8 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }

        fn dofs_per_node(&self) -> &[Dof] {
            const DOFS: [Dof; 3] = [Dof::Tx, Dof::Ty, Dof::Tz];
            &DOFS
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let d = d_matrix_solid(self.e, self.nu);
            let mut ke = MatD::zeros(24, 24);
            for (p, weight) in hex8_gauss_points() {
                let (det_j, grads) = Self::gradients_at(ctx, p[0], p[1], p[2]);
                let b = solid_b_matrix(&grads);
                ke.add_triple_product(&b, &d, det_j * weight);
            }
            ke
        }

        fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let d = d_matrix_solid(self.e, self.nu);
            let gauss = hex8_gauss_points()
                .iter()
                .map(|(p, _)| {
                    let (_det_j, grads) = Self::gradients_at(ctx, p[0], p[1], p[2]);
                    let b = solid_b_matrix(&grads);
                    let strain = b.mul_vec(u_elem);
                    let stress = d.mul_vec(&strain);
                    let (sxx, syy, szz, sxy, syz, sxz) = (stress.get(0), stress.get(1), stress.get(2), stress.get(3), stress.get(4), stress.get(5));
                    let von_mises = von_mises_solid(sxx, syy, szz, sxy, syz, sxz);
                    SolidStress { sxx, syy, szz, sxy, syz, sxz, von_mises }
                })
                .collect();
            ElementResult::Solid { gauss }
        }

        /// 🏋️ Consistent trilinear mass `ρ∫Nᵀ·N·dV` over the same 2x2x2 Gauss rule as stiffness — exact,
        /// since `Ni·Nj` (biquadratic-per-axis) is within that rule's precision.
        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let mut m = MatD::zeros(24, 24);
            for (p, weight) in hex8_gauss_points() {
                let (det_j, _) = Self::gradients_at(ctx, p[0], p[1], p[2]);
                let n_vals = hex8_shape(p[0], p[1], p[2]);
                let scale = self.density * det_j * weight;
                for i in 0..8 {
                    for j in 0..8 {
                        let v = n_vals[i] * n_vals[j] * scale;
                        for a in 0..3 {
                            m.add_at(3 * i + a, 3 * j + a, v);
                        }
                    }
                }
            }
            Some(m)
        }

        /// 🌀️ Initial-stress geometric stiffness, same `Gᵀ(σ̂⊗I₃)G` pattern as `Tet4::geometric_stiffness`
        /// but Gauss-integrated over the element's own 2x2x2 rule (stress varies point-to-point).
        fn geometric_stiffness(&self, ctx: &ElementContext, u_elem: &VecD) -> Option<MatD> {
            let d = d_matrix_solid(self.e, self.nu);
            let mut kg = MatD::zeros(24, 24);
            for (p, weight) in hex8_gauss_points() {
                let (det_j, grads) = Self::gradients_at(ctx, p[0], p[1], p[2]);
                let b = solid_b_matrix(&grads);
                let strain = b.mul_vec(u_elem);
                let stress = d.mul_vec(&strain);
                let (sxx, syy, szz, sxy, syz, sxz) = (stress.get(0), stress.get(1), stress.get(2), stress.get(3), stress.get(4), stress.get(5));
                let scale = det_j * weight;
                for i in 0..8 {
                    let gi = grads[i];
                    for j in 0..8 {
                        let gj = grads[j];
                        let s = gi[0] * (sxx * gj[0] + sxy * gj[1] + sxz * gj[2]) + gi[1] * (sxy * gj[0] + syy * gj[1] + syz * gj[2]) + gi[2] * (sxz * gj[0] + syz * gj[1] + szz * gj[2]);
                        let val = s * scale;
                        for a in 0..3 {
                            kg.add_at(3 * i + a, 3 * j + a, val);
                        }
                    }
                }
            }
            Some(kg)
        }
    }
    // #endregion 🔖️Hex8

    // #region 🔖️ShellFacet3
    /// 🧭️ Local in-plane axes for a flat triangular facet, built directly from 3 non-collinear 3D
    /// points (no roll angle or reference-vector edge case needed — 3 points unambiguously define a
    /// plane, unlike `Frame3::local_system`'s 1D-member case): local x along edge `p0->p1`, local z the
    /// facet normal (`cross(p1-p0, p2-p0)`, right-hand rule), local y completing the right-handed frame.
    fn shell_local_axes(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3]) -> Mat3d {
        let d1 = vec3d_sub(p1, p0);
        let d2 = vec3d_sub(p2, p0);
        let local_x = vec3d_normalize(d1);
        let local_z = vec3d_normalize(vec3d_cross(d1, d2));
        let local_y = vec3d_cross(local_z, local_x);
        Mat3d::from_axes(local_x, local_y, local_z)
    }

    /// 🧭️ 18x18 global<->local block-diagonal rotation `T` — six `R^T` 3x3 blocks (one per node's
    /// translation triple `[Tx,Ty,Tz]`, one per node's rotation triple `[Rx,Ry,Rz]`), the same pattern
    /// `Frame3::local_system` uses for its 12x12 `T`, extended to 3 nodes x 2 triples.
    fn shell_transform(r: &Mat3d) -> MatD {
        let rt = r.transpose();
        let mut t = MatD::zeros(18, 18);
        for offset in [0usize, 3, 6, 9, 12, 15] {
            for row in 0..3 {
                for col in 0..3 {
                    t.set(offset + row, offset + col, rt.cols[col][row]);
                }
            }
        }
        t
    }

    /// 🐚️ Flat facet shell — 3-node, 6-DOF-per-node (`[Tx,Ty,Tz,Rx,Ry,Rz]`) element combining an in-plane
    /// `Tri3Cst`-style CST membrane, `PlateDkt`-style DKT bending, and a small artificial "drilling"
    /// stiffness on the local `Rz` (in-plane rotation) DOF — flat shells have no natural stiffness
    /// resisting drilling rotation, so a small diagonal stabilization avoids a singular system where
    /// coplanar/near-coplanar facets meet. Membrane and bending are exactly decoupled at this (flat,
    /// linear) scope, so the local 18x18 stiffness is block-diagonal by construction.
    pub struct ShellFacet3 {
        pub id: String,
        pub nodes: [String; 3],
        pub e: f64,
        pub nu: f64,
        pub thickness: f64,
        pub density: f64,
    }

    /// 🎯️ Small dimensionless drilling-stabilization factor — standard "just enough to avoid
    /// singularity, small enough not to distort real behavior" scaling on `k_drill = α·E·t·Area`.
    const SHELL_DRILL_ALPHA: f64 = 1e-3;

    impl ShellFacet3 {
        /// 🧭️ Local in-plane 2D triangle coordinates (`p0_local=(0,0)`, `p1_local` on the local x-axis,
        /// `p2_local` completing the triangle) plus the local-axes rotation `Mat3d` shared by
        /// `local_stiffness`'s membrane/bending quadrature and `shell_transform`'s `T`.
        fn local_coords(ctx: &ElementContext) -> ([[f64; 2]; 3], Mat3d) {
            let (p0, p1, p2) = (ctx.positions[0], ctx.positions[1], ctx.positions[2]);
            let r = shell_local_axes(p0, p1, p2);
            let (local_x, local_y) = (r.cols[0], r.cols[1]);
            let d1 = vec3d_sub(p1, p0);
            let d2 = vec3d_sub(p2, p0);
            let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
            let coords = [[0.0, 0.0], [dot(d1, local_x), 0.0], [dot(d2, local_x), dot(d2, local_y)]];
            (coords, r)
        }

        /// 🧮️ Local 18x18 stiffness, dof order `[Tx1,Ty1,Tz1,Rx1,Ry1,Rz1, ... x3]`: 6x6 CST membrane
        /// block at each node's `(Tx,Ty)` indices, 9x9 DKT bending block at each node's `(Tz,Rx,Ry)`
        /// indices, drilling diagonal at each node's `Rz` index — all cross-terms zero by construction.
        fn local_stiffness(&self, coords: &[[f64; 2]; 3]) -> MatD {
            let mut k = MatD::zeros(18, 18);

            let d_mem = d_matrix_plane_stress(self.e, self.nu);
            let mut k_mem = MatD::zeros(6, 6);
            for (xi, eta, w) in gauss_tri(1) {
                let (_, dn) = shape_tri3(xi, eta);
                let (_, det_j, d_n_xy) = jacobian_2d(coords, &dn);
                let b = b_matrix_plane(&d_n_xy);
                k_mem.add_triple_product(&b, &d_mem, w * det_j * self.thickness);
            }
            let mem_idx = [0usize, 1, 6, 7, 12, 13];
            for (i, &gi) in mem_idx.iter().enumerate() {
                for (j, &gj) in mem_idx.iter().enumerate() {
                    k.set(gi, gj, k_mem.get(i, j));
                }
            }

            let (_, det_j, _) = jacobian_2d(coords, &shape_tri3(0.0, 0.0).1);
            let d_bend = crate::elements2d::d_matrix_plate(self.e, self.nu, self.thickness);
            let mut k_bend = MatD::zeros(9, 9);
            for (xi, eta, w) in gauss_tri(3) {
                let b = crate::elements2d::dkt_b_matrix(coords, xi, eta);
                k_bend.add_triple_product(&b, &d_bend, w * det_j);
            }
            let bend_idx = [2usize, 3, 4, 8, 9, 10, 14, 15, 16];
            for (i, &gi) in bend_idx.iter().enumerate() {
                for (j, &gj) in bend_idx.iter().enumerate() {
                    k.set(gi, gj, k_bend.get(i, j));
                }
            }

            let area = 0.5 * det_j;
            let k_drill = SHELL_DRILL_ALPHA * self.e * self.thickness * area;
            for i in 0..3 {
                k.set(6 * i + 5, 6 * i + 5, k_drill);
            }

            k
        }
    }

    impl Element for ShellFacet3 {
        fn id(&self) -> &str {
            &self.id
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }

        fn dofs_per_node(&self) -> &[Dof] {
            const DOFS: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];
            &DOFS
        }

        fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
            let (coords, r) = Self::local_coords(ctx);
            let t = shell_transform(&r);
            let k_local = self.local_stiffness(&coords);
            t.transpose().matmul(&k_local).matmul(&t)
        }

        fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
            let (coords, r) = Self::local_coords(ctx);
            let t = shell_transform(&r);
            let u_loc = t.mul_vec(u_elem);

            let mem_idx = [0usize, 1, 6, 7, 12, 13];
            let u_mem = VecD::from_vec(mem_idx.iter().map(|&i| u_loc.get(i)).collect());
            let d_mem = d_matrix_plane_stress(self.e, self.nu);
            let (_, dn) = shape_tri3(1.0 / 3.0, 1.0 / 3.0);
            let (_, _, d_n_xy) = jacobian_2d(&coords, &dn);
            let b_mem = b_matrix_plane(&d_n_xy);
            let eps = b_mem.mul_vec(&u_mem);
            let sigma = d_mem.mul_vec(&eps);
            let (nxx, nyy, nxy) = (sigma.get(0) * self.thickness, sigma.get(1) * self.thickness, sigma.get(2) * self.thickness);

            let bend_idx = [2usize, 3, 4, 8, 9, 10, 14, 15, 16];
            let u_bend = VecD::from_vec(bend_idx.iter().map(|&i| u_loc.get(i)).collect());
            let d_bend = crate::elements2d::d_matrix_plate(self.e, self.nu, self.thickness);
            let b_bend = crate::elements2d::dkt_b_matrix(&coords, 1.0 / 3.0, 1.0 / 3.0);
            let kappa = b_bend.mul_vec(&u_bend);
            let m = d_bend.mul_vec(&kappa);
            let (mxx, myy, mxy) = (m.get(0), m.get(1), m.get(2));

            let t_th = self.thickness;
            let surface = |sign: f64| {
                let sxx = nxx / t_th + sign * 6.0 * mxx / (t_th * t_th);
                let syy = nyy / t_th + sign * 6.0 * myy / (t_th * t_th);
                let sxy = nxy / t_th + sign * 6.0 * mxy / (t_th * t_th);
                (sxx * sxx - sxx * syy + syy * syy + 3.0 * sxy * sxy).sqrt()
            };
            let von_mises_top = surface(1.0);
            let von_mises_bottom = surface(-1.0);

            ElementResult::Shell { gauss: vec![ShellState { nxx, nyy, nxy, mxx, myy, mxy, von_mises_top, von_mises_bottom }] }
        }

        /// 🏋️ Lumped translational mass `ρtA/3` on each node's `[Tx,Ty,Tz]` — diagonal and isotropic
        /// (equal in all 3 local translation directions), so it needs no local->global rotation, unlike
        /// `local_stiffness`. Zero rotational inertia, same lumping rationale as `PlateDkt::mass`.
        fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
            let (coords, _) = Self::local_coords(ctx);
            let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
            let area = 0.5 * det_j;
            let share = self.density * self.thickness * area / 3.0;
            let mut m = MatD::zeros(18, 18);
            for i in 0..3 {
                for a in 0..3 {
                    m.set(6 * i + a, 6 * i + a, share);
                }
            }
            Some(m)
        }

        /// 🌀️ Geometric stiffness from the facet's own (constant) CST membrane forces `Nxx,Nyy,Nxy`
        /// acting on the LINEAR CST-interpolated out-of-plane `w` gradient (the standard flat-facet
        /// simplification — the DKT bending field's rotation-driven curvature correction is neglected
        /// for this coupling, following common practice for flat shell buckling), local `Tz` dof per
        /// node using the SAME constant gradient `local_stiffness`'s membrane block computes.
        fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
            let (coords, r) = Self::local_coords(ctx);
            let t = shell_transform(&r);
            let u_loc = t.mul_vec(u_element);

            let mem_idx = [0usize, 1, 6, 7, 12, 13];
            let u_mem = VecD::from_vec(mem_idx.iter().map(|&i| u_loc.get(i)).collect());
            let d_mem = d_matrix_plane_stress(self.e, self.nu);
            let (_, dn) = shape_tri3(1.0 / 3.0, 1.0 / 3.0);
            let (_, det_j, d_n_xy) = jacobian_2d(&coords, &dn);
            let b_mem = b_matrix_plane(&d_n_xy);
            let eps = b_mem.mul_vec(&u_mem);
            let sigma = d_mem.mul_vec(&eps);
            let (nxx, nyy, nxy) = (sigma.get(0) * self.thickness, sigma.get(1) * self.thickness, sigma.get(2) * self.thickness);

            let area = 0.5 * det_j;
            let w_idx = [2usize, 8, 14];
            let mut kg_local = MatD::zeros(18, 18);
            for i in 0..3 {
                let (gix, giy) = (d_n_xy[i][0], d_n_xy[i][1]);
                for j in 0..3 {
                    let (gjx, gjy) = (d_n_xy[j][0], d_n_xy[j][1]);
                    let s = gix * nxx * gjx + gix * nxy * gjy + giy * nxy * gjx + giy * nyy * gjy;
                    kg_local.add_at(w_idx[i], w_idx[j], s * area);
                }
            }
            Some(t.transpose().matmul(&kg_local).matmul(&t))
        }
    }
    // #endregion 🔖️ShellFacet3

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{solve_linear_static, Model, NodalLoad, Node, Support};

        /// 🪵️ Headless axial elongation check along an arbitrary (non-axis-aligned) 3D direction.
        #[test]
        fn bar3_axial_matches_hand_calc_on_skew_member() {
            // A free 3D joint needs ≥3 non-coplanar bars to be determinate (see `truss_fixture` below),
            // so `b` gets two extra fixed-node bars (to `d` and `c`) besides the member under test (`e1`).
            // Loading exactly along e1's own axis (0.6,0.8,0) makes e1 carry the full load by equilibrium
            // (hand-solved: N_e1 = p, N_bd = N_bc = 0) — a clean, unambiguous check on a genuinely skew direction.
            let (e, a) = (200e9, 0.001);
            let l = 5.0;
            let p = 2000.0;
            let model = Model {
                nodes: vec![
                    Node { id: "a".into(), pos: [0.0, 0.0, 0.0] },
                    Node { id: "b".into(), pos: [3.0, 4.0, 0.0] }, // length 5 from a, direction (0.6, 0.8, 0.0)
                    Node { id: "c".into(), pos: [3.0, 4.0, 2.0] },
                    Node { id: "d".into(), pos: [3.0, 0.0, 0.0] },
                ],
                elements: vec![
                    Box::new(Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 }),
                    Box::new(Bar3 { id: "bc".into(), node_a: "b".into(), node_b: "c".into(), e, a, density: 0.0 }),
                    Box::new(Bar3 { id: "bd".into(), node_a: "b".into(), node_b: "d".into(), e, a, density: 0.0 }),
                ],
                supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }, Support { node_id: "c".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }, Support { node_id: "d".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }],
                nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p * 0.6 }, NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: p * 0.8 }],
                member_loads: vec![],
            };
            let result = solve_linear_static(&model).expect("solves");
            let n_e1 = result
                .elements
                .iter()
                .find(|(id, _)| id == "e1")
                .map(|(_, r)| match r {
                    ElementResult::Bar { n } => *n,
                    _ => panic!("expected bar"),
                })
                .unwrap();
            assert!((n_e1 - p).abs() / p < 1e-6, "axial force {n_e1} vs expected {p}");
            let expected_elongation = p * l / (e * a);
            let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
            let actual_elongation = b.values[Dof::Tx.index()] * 0.6 + b.values[Dof::Ty.index()] * 0.8;
            assert!((actual_elongation - expected_elongation).abs() / expected_elongation < 1e-6);
        }

        /// 🌀️ Rigid-body test: a pure 3D translation must produce zero internal force on a `Frame3`.
        #[test]
        fn frame3_rigid_translation_gives_zero_force() {
            let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
            let ke = frame.stiffness_global(&ctx);
            let rigid = VecD::from_vec(vec![1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
            let f = ke.mul_vec(&rigid);
            for i in 0..12 {
                assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
            }
        }

        /// 🌬️ Cantilever `Frame3` under a member UDL, checked against the classical cantilever-under-UDL
        /// formulas: base moment `wL²/2`, base shear `wL`, ~0 at the free tip. The member runs along
        /// global X with `roll: 0.0`, for which `local_system`'s reference-vector logic aligns local y/z
        /// with global Y/Z exactly — so a UDL in global `wz` lands directly in the local z-bending plane
        /// that `recover` already reports via `v1 = f.get(2)`/`m1 = f.get(4)`.
        #[test]
        fn frame3_udl_cantilever_matches_hand_calc() {
            let (e, g, a, iy, iz, j) = (200e9, 80e9, 0.01, 8e-5, 8e-5, 1e-6);
            let l = 4.0;
            let w = 1000.0;
            let model = Model {
                nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
                elements: vec![Box::new(Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, g, a, iy, iz, j, roll: 0.0, density: 0.0 })],
                supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }],
                nodal_loads: vec![],
                member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: 0.0, wz: -w })],
            };
            let result = solve_linear_static(&model).expect("solves");
            let (_, e1_result) = result.elements.iter().find(|(id, _)| id == "e1").unwrap();
            let stations = match e1_result {
                ElementResult::Beam { stations } => stations,
                _ => panic!("expected beam"),
            };
            let base = stations.first().unwrap();
            let tip = stations.last().unwrap();
            let expected_m = w * l * l / 2.0;
            let expected_v = w * l;
            assert!((base.m.abs() - expected_m).abs() / expected_m < 1e-6, "base moment {} vs expected {}", base.m, expected_m);
            assert!((base.v.abs() - expected_v).abs() / expected_v < 1e-6, "base shear {} vs expected {}", base.v, expected_v);
            assert!(tip.m.abs() < expected_m * 1e-6, "tip moment {} should be ~0", tip.m);
            assert!(tip.v.abs() < expected_v * 1e-6, "tip shear {} should be ~0", tip.v);
        }

        /// 🏋️ `Bar3::mass` matches the hand-derived isotropic `m = ρAL/6` block form (3x3 identity blocks).
        #[test]
        fn bar3_mass_matches_hand_calc() {
            let (density, a, l) = (7850.0, 0.001, 5.0);
            let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 200e9, a, density };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
            let m = bar.mass(&ctx).expect("bar3 reports mass");
            let expected = density * a * l / 6.0;
            for i in 0..3 {
                assert!((m.get(i, i) - 2.0 * expected).abs() < 1e-9);
                assert!((m.get(i + 3, i + 3) - 2.0 * expected).abs() < 1e-9);
                assert!((m.get(i, i + 3) - expected).abs() < 1e-9);
            }
            assert!(m.get(0, 1).abs() < 1e-12, "no coupling across directions");
        }

        /// ⚖️ Sum of ALL entries of `Bar3::mass` (a pure translational, no-rotation element) equals the
        /// total member mass `ρAL` — same partition-of-unity identity as `Bar2`'s.
        #[test]
        fn bar3_mass_total_equals_rho_a_l() {
            let (density, a, l) = (7850.0, 0.001, 5.0);
            let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 200e9, a, density };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
            let m = bar.mass(&ctx).expect("bar3 reports mass");
            let mut sum_x = 0.0;
            for &r in &[0usize, 3] {
                for &c in &[0usize, 3] {
                    sum_x += m.get(r, c);
                }
            }
            assert!((sum_x - density * a * l).abs() / (density * a * l) < 1e-9);
        }

        /// 🏋️ `Frame3::mass`'s axial and torsion 2x2 submatrices each sum to their own hand-derived total
        /// (`ρAL` axial, `ρJL` torsion) — checked on a member along global X with `roll: 0.0`, for which
        /// `local_system` aligns local axes with global ones exactly (rotation is identity).
        #[test]
        fn frame3_mass_axial_and_torsion_blocks_sum_to_total() {
            let (e, g, a, iy, iz, j, density) = (200e9, 80e9, 0.01, 8e-5, 8e-5, 1e-6, 7850.0);
            let l = 4.0;
            let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, g, a, iy, iz, j, roll: 0.0, density };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
            let m = frame.mass(&ctx).expect("frame3 reports mass");
            let sum_axial = m.get(0, 0) + m.get(0, 6) + m.get(6, 0) + m.get(6, 6);
            assert!((sum_axial - density * a * l).abs() / (density * a * l) < 1e-9);
            let sum_torsion = m.get(3, 3) + m.get(3, 9) + m.get(9, 3) + m.get(9, 9);
            assert!((sum_torsion - density * j * l).abs() / (density * j * l) < 1e-9);
        }

        /// 🌀️ `Frame3` geometric stiffness must vanish under a pure rigid translation.
        #[test]
        fn frame3_geometric_stiffness_rigid_translation_gives_zero_force() {
            let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
            let mut u = VecD::zeros(12);
            u.set(6, 0.001);
            let kg = frame.geometric_stiffness(&ctx, &u).expect("frame3 reports geometric stiffness");
            let rigid = VecD::from_vec(vec![1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
            let f = kg.mul_vec(&rigid);
            for i in 0..12 {
                assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }

        /// 🌀️ `Frame3` geometric stiffness is symmetric.
        #[test]
        fn frame3_geometric_stiffness_is_symmetric() {
            let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
            let mut u = VecD::zeros(12);
            u.set(6, 0.001);
            let kg = frame.geometric_stiffness(&ctx, &u).unwrap();
            for r in 0..12 {
                for c in 0..12 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                }
            }
        }

        /// 🌬️ `Bar3::equivalent_nodal_loads` splits a global UDL `wL/2` exactly evenly at both nodes —
        /// the 3D analogue of `elements2d::bar2_equivalent_nodal_loads_matches_wl_over_2`.
        #[test]
        fn bar3_equivalent_nodal_loads_matches_wl_over_2() {
            let (e, a, l) = (200e9, 0.001, 5.0);
            let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
            let udl = MemberUdl { wx: 100.0, wy: -50.0, wz: 20.0 };
            let f = bar.equivalent_nodal_loads(&ctx, &udl).expect("bar3 reports equivalent nodal loads");
            let half = l / 2.0;
            assert!((f.get(0) - udl.wx * half).abs() < 1e-9);
            assert!((f.get(1) - udl.wy * half).abs() < 1e-9);
            assert!((f.get(2) - udl.wz * half).abs() < 1e-9);
            assert!((f.get(3) - udl.wx * half).abs() < 1e-9);
            assert!((f.get(4) - udl.wy * half).abs() < 1e-9);
            assert!((f.get(5) - udl.wz * half).abs() < 1e-9);
        }

        /// 🌀️ `Bar3::geometric_stiffness`: zero under rigid translation, symmetric, and (same reasoning
        /// as `elements2d::bar2_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric`)
        /// zero along the bar's own axis.
        #[test]
        fn bar3_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
            let (e, a) = (200e9, 0.001);
            let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 };
            let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
            let u = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
            let kg = bar.geometric_stiffness(&ctx, &u).expect("bar3 reports geometric stiffness");
            for r in 0..6 {
                for c in 0..6 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                }
            }
            let rigid = VecD::from_vec(vec![3.0, 4.0, 1.0, 3.0, 4.0, 1.0]);
            let f = kg.mul_vec(&rigid);
            for i in 0..6 {
                assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }
    }
    // #endregion 🔖️Tests

    // #region 🔖️SolidTests
    #[cfg(test)]
    mod solid_tests {
        use super::*;
        use crate::{solve_linear_static, Model, NodalLoad, Node, Support};

        /// 🧮️ Linear displacement field `u=ux·x+uy·y+uz·z` (and analogous `v`,`w`) shared by the Tet4/Hex8
        /// patch tests — its gradient (hence strain) is constant everywhere, so a direct
        /// `stiffness_global`/`recover` call can be checked against the closed-form `σ=Dε` exactly.
        struct LinearField {
            ux: f64,
            uy: f64,
            uz: f64,
            vx: f64,
            vy: f64,
            vz: f64,
            wx: f64,
            wy: f64,
            wz: f64,
        }

        impl LinearField {
            fn sample() -> Self {
                Self { ux: 0.0011, uy: 0.0007, uz: -0.0004, vx: -0.0006, vy: 0.0012, vz: 0.0003, wx: 0.0002, wy: -0.0005, wz: 0.0009 }
            }

            fn displacement_at(&self, p: [f64; 3]) -> [f64; 3] {
                [self.ux * p[0] + self.uy * p[1] + self.uz * p[2], self.vx * p[0] + self.vy * p[1] + self.vz * p[2], self.wx * p[0] + self.wy * p[1] + self.wz * p[2]]
            }

            /// 🧮️ Analytical `σ = Dε` for this field's (constant) strain, hand-expanded independently of
            /// `d_matrix_solid` as a cross-check of the whole B/D recovery pipeline.
            fn expected_stress(&self, e: f64, nu: f64) -> SolidStress {
                let (exx, eyy, ezz) = (self.ux, self.vy, self.wz);
                let (gxy, gyz, gxz) = (self.uy + self.vx, self.vz + self.wy, self.uz + self.wx);
                let s = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
                let sxx = s * ((1.0 - nu) * exx + nu * eyy + nu * ezz);
                let syy = s * (nu * exx + (1.0 - nu) * eyy + nu * ezz);
                let szz = s * (nu * exx + nu * eyy + (1.0 - nu) * ezz);
                let g = s * (1.0 - 2.0 * nu) / 2.0;
                let (sxy, syz, sxz) = (g * gxy, g * gyz, g * gxz);
                let von_mises = von_mises_solid(sxx, syy, szz, sxy, syz, sxz);
                SolidStress { sxx, syy, szz, sxy, syz, sxz, von_mises }
            }

            fn nodal_vector(&self, positions: &[[f64; 3]]) -> VecD {
                let mut data = Vec::with_capacity(positions.len() * 3);
                for &p in positions {
                    data.extend_from_slice(&self.displacement_at(p));
                }
                VecD::from_vec(data)
            }
        }

        /// 🔍️ Component-wise relative comparison (scaled by `max(|expected|, 1.0)` so a near-zero
        /// expected component doesn't demand an absurdly tight absolute match).
        fn assert_stress_close(actual: &SolidStress, expected: &SolidStress, rel_tol: f64) {
            let check = |name: &str, a: f64, ex: f64| {
                let scale = ex.abs().max(1.0);
                assert!((a - ex).abs() / scale < rel_tol, "{name}: {a} vs expected {ex}");
            };
            check("sxx", actual.sxx, expected.sxx);
            check("syy", actual.syy, expected.syy);
            check("szz", actual.szz, expected.szz);
            check("sxy", actual.sxy, expected.sxy);
            check("syz", actual.syz, expected.syz);
            check("sxz", actual.sxz, expected.sxz);
        }

        // #region 🔖️Tet4
        fn skew_tet_positions() -> [[f64; 3]; 4] {
            [[0.0, 0.0, 0.0], [1.0, 0.1, 0.05], [0.2, 1.0, 0.1], [0.1, 0.15, 1.0]]
        }

        /// 🧮️ Constant-strain patch test: an exact linear field imposed at every node must recover the
        /// exact analytical `σ=Dε` at the (single, exact) integration point.
        #[test]
        fn tet4_patch_test_recovers_exact_constant_stress() {
            let (e, nu) = (200e9, 0.3);
            let positions = skew_tet_positions();
            let field = LinearField::sample();
            let ctx = ElementContext { positions: positions.to_vec() };
            let tet = Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density: 0.0 };
            let ke = tet.stiffness_global(&ctx);
            assert_eq!(ke.rows, 12);
            let u = field.nodal_vector(&positions);
            let result = tet.recover(&ctx, &u, None);
            let ElementResult::Solid { gauss } = result else { panic!("expected solid") };
            assert_eq!(gauss.len(), 1);
            assert_stress_close(&gauss[0], &field.expected_stress(e, nu), 1e-6);
        }

        /// 🌀️ Rigid-body test: a pure translation of all 4 nodes must produce zero internal force.
        #[test]
        fn tet4_rigid_translation_gives_zero_force() {
            let (e, nu) = (200e9, 0.3);
            let positions = skew_tet_positions();
            let ctx = ElementContext { positions: positions.to_vec() };
            let tet = Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density: 0.0 };
            let ke = tet.stiffness_global(&ctx);
            let rigid = VecD::from_vec((0..4).flat_map(|_| [1.0, 2.0, 3.0]).collect());
            let f = ke.mul_vec(&rigid);
            for i in 0..12 {
                assert!(f.get(i).abs() < 1e-3, "rigid-body force[{i}] = {}", f.get(i));
            }
        }

        fn tet_volume(positions: &[[f64; 3]; 4]) -> f64 {
            let e1 = [positions[1][0] - positions[0][0], positions[1][1] - positions[0][1], positions[1][2] - positions[0][2]];
            let e2 = [positions[2][0] - positions[0][0], positions[2][1] - positions[0][1], positions[2][2] - positions[0][2]];
            let e3 = [positions[3][0] - positions[0][0], positions[3][1] - positions[0][1], positions[3][2] - positions[0][2]];
            let cross = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]];
            (cross[0] * e3[0] + cross[1] * e3[1] + cross[2] * e3[2]).abs() / 6.0
        }

        /// ⚖️ `Tet4::mass`'s total (the pure-`Tx` submatrix's sum) equals `ρV` — same partition-of-unity
        /// identity as `Bar3`'s.
        #[test]
        fn tet4_mass_total_equals_rho_v() {
            let (density, e, nu) = (2400.0, 200e9, 0.3);
            let positions = skew_tet_positions();
            let tet = Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density };
            let ctx = ElementContext { positions: positions.to_vec() };
            let m = tet.mass(&ctx).expect("tet4 reports mass");
            let sum_tx: f64 = (0..4).flat_map(|r| (0..4).map(move |c| (3 * r, 3 * c))).map(|(r, c)| m.get(r, c)).sum();
            let expected = density * tet_volume(&positions);
            assert!((sum_tx - expected).abs() / expected < 1e-9, "sum={sum_tx} expected={expected}");
        }

        /// ⚖️ A single `Tet4` under self-weight only: the vertical reaction sum must equal `ρVg` — the
        /// same strong equilibrium check `analyses`'s beam self-weight test uses, now exercised on a
        /// continuum solid element (only possible once `Tet4::mass` exists).
        #[test]
        fn tet4_self_weight_matches_total_mass_times_gravity() {
            let (density, e, nu, g) = (2400.0, 30e9, 0.2, 9.81);
            let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
            let nodes: Vec<Node> = (0..4).map(|i| Node { id: format!("n{i}"), pos: positions[i] }).collect();
            let model = crate::analyses::AnalysisModel {
                nodes,
                elements: vec![Box::new(Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density })],
                supports: vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }, Support { node_id: "n1".into(), fixed: vec![Dof::Ty, Dof::Tz] }, Support { node_id: "n2".into(), fixed: vec![Dof::Tz] }],
            };
            let case = crate::analyses::LoadCase { id: "self_weight".into(), nodal_loads: vec![], member_loads: vec![], self_weight: true };
            let results = crate::analyses::solve_multi_case(&model, &[case], &[], [0.0, 0.0, -g]).expect("solves");
            let result = results.get("self_weight").unwrap();
            let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
            let expected = density * tet_volume(&positions) * g;
            assert!((total_tz_reaction - expected).abs() / expected < 1e-9, "reaction sum {total_tz_reaction} vs expected {expected}");
        }

        /// 🌀️ `Tet4::geometric_stiffness`: zero under rigid translation and symmetric.
        #[test]
        fn tet4_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
            let (e, nu) = (200e9, 0.3);
            let positions = skew_tet_positions();
            let field = LinearField::sample();
            let ctx = ElementContext { positions: positions.to_vec() };
            let tet = Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density: 0.0 };
            let u = field.nodal_vector(&positions);
            let kg = tet.geometric_stiffness(&ctx, &u).expect("tet4 reports geometric stiffness");
            for r in 0..12 {
                for c in 0..12 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-6, "Kg not symmetric at ({r},{c})");
                }
            }
            let rigid = VecD::from_vec((0..4).flat_map(|_| [1.0, 2.0, 3.0]).collect());
            let f = kg.mul_vec(&rigid);
            for i in 0..12 {
                assert!(f.get(i).abs() < 1e-3, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }
        // #endregion 🔖️Tet4

        // #region 🔖️Hex8
        fn skew_hex_positions() -> [[f64; 3]; 8] {
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.05, 1.0, 0.02], [-0.03, 0.98, 0.01], [0.02, 0.01, 1.0], [1.02, -0.01, 0.97], [0.99, 1.03, 1.05], [0.01, 1.0, 0.98]]
        }

        /// 🧮️ Constant-strain patch test, same field as `Tet4`'s — checked at all 8 Gauss points (a
        /// skewed-but-non-degenerate hex still reproduces an exact linear field everywhere, a fundamental
        /// isoparametric-interpolation property, not something specific to parallelepiped geometry).
        #[test]
        fn hex8_patch_test_recovers_exact_constant_stress() {
            let (e, nu) = (200e9, 0.3);
            let positions = skew_hex_positions();
            let field = LinearField::sample();
            let ctx = ElementContext { positions: positions.to_vec() };
            let nodes: [String; 8] = std::array::from_fn(|i| format!("n{i}"));
            let hex = Hex8 { id: "h1".into(), nodes, e, nu, density: 0.0 };
            let ke = hex.stiffness_global(&ctx);
            assert_eq!(ke.rows, 24);
            let u = field.nodal_vector(&positions);
            let result = hex.recover(&ctx, &u, None);
            let ElementResult::Solid { gauss } = result else { panic!("expected solid") };
            assert_eq!(gauss.len(), 8);
            let expected = field.expected_stress(e, nu);
            for g in &gauss {
                assert_stress_close(g, &expected, 1e-6);
            }
        }

        /// 🌀️ Rigid-body test: a pure translation of all 8 nodes must produce zero internal force.
        #[test]
        fn hex8_rigid_translation_gives_zero_force() {
            let (e, nu) = (200e9, 0.3);
            let positions = skew_hex_positions();
            let ctx = ElementContext { positions: positions.to_vec() };
            let nodes: [String; 8] = std::array::from_fn(|i| format!("n{i}"));
            let hex = Hex8 { id: "h1".into(), nodes, e, nu, density: 0.0 };
            let ke = hex.stiffness_global(&ctx);
            let rigid = VecD::from_vec((0..8).flat_map(|_| [1.0, 2.0, 3.0]).collect());
            let f = ke.mul_vec(&rigid);
            for i in 0..24 {
                assert!(f.get(i).abs() < 1e-3, "rigid-body force[{i}] = {}", f.get(i));
            }
        }

        /// 🏗️ Coarse hex-meshed cantilever (4 elements along the span) vs classical beam theory
        /// `δ = PL³/3EI` — a sanity check on assembly/BC wiring, not on element accuracy (low-order hex
        /// without incompatible modes is known to lock somewhat stiff in bending), so the tolerance is
        /// wide: just confirm the deflection is negative (toward the load), finite, and the right order
        /// of magnitude.
        #[test]
        fn hex8_meshed_cantilever_deflection_is_right_order_of_magnitude() {
            let (e, nu) = (200e9, 0.3);
            let (b, h, l, nx) = (0.2, 0.3, 4.0, 4usize);
            let dx = l / nx as f64;
            let corner_id = |ix: usize, iy: usize, iz: usize| format!("n{ix}_{iy}_{iz}");
            let corners = [(0usize, 0usize), (1, 0), (1, 1), (0, 1)];

            let mut nodes = Vec::new();
            for ix in 0..=nx {
                let x = dx * ix as f64;
                for &(iy, iz) in &corners {
                    let y = if iy == 0 { 0.0 } else { b };
                    let z = if iz == 0 { 0.0 } else { h };
                    nodes.push(Node { id: corner_id(ix, iy, iz), pos: [x, y, z] });
                }
            }

            let mut elements: Vec<Box<dyn Element>> = Vec::new();
            for ix in 0..nx {
                elements.push(Box::new(Hex8 {
                    id: format!("hex{ix}"),
                    nodes: [corner_id(ix, 0, 0), corner_id(ix + 1, 0, 0), corner_id(ix + 1, 1, 0), corner_id(ix, 1, 0), corner_id(ix, 0, 1), corner_id(ix + 1, 0, 1), corner_id(ix + 1, 1, 1), corner_id(ix, 1, 1)],
                    e,
                    nu,
                    density: 0.0,
                }));
            }

            let supports = corners.iter().map(|&(iy, iz)| Support { node_id: corner_id(0, iy, iz), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }).collect();
            let p_total = 1e4;
            let nodal_loads = corners.iter().map(|&(iy, iz)| NodalLoad { node_id: corner_id(nx, iy, iz), dof: Dof::Tz, value: -p_total / 4.0 }).collect();

            let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
            let result = solve_linear_static(&model).expect("solves");

            let tip_dz: f64 = corners
                .iter()
                .map(|&(iy, iz)| {
                    let id = corner_id(nx, iy, iz);
                    result.displacements.iter().find(|d| d.node_id == id).unwrap().values[Dof::Tz.index()]
                })
                .sum::<f64>()
                / corners.len() as f64;

            let i_area = b * h.powi(3) / 12.0;
            let expected = p_total * l.powi(3) / (3.0 * e * i_area);
            assert!(tip_dz.is_finite());
            assert!(tip_dz < 0.0, "tip should deflect toward -Z, got {tip_dz}");
            let ratio = tip_dz.abs() / expected;
            assert!(ratio > 0.02 && ratio < 3.0, "deflection ratio {ratio} (actual {tip_dz} vs beam-theory {expected}) out of order-of-magnitude range");
        }

        /// ⚖️ `Hex8::mass`'s total (pure-`Tx` submatrix sum) equals `ρV` on the UNIT cube (skewed hex
        /// positions make an independent volume oracle fiddly — the axis-aligned unit cube's volume is
        /// trivially `1.0`, isolating the mass identity from any volume-computation risk).
        #[test]
        fn hex8_mass_total_equals_rho_v() {
            let density = 2400.0;
            let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
            let nodes: [String; 8] = std::array::from_fn(|i| format!("n{i}"));
            let hex = Hex8 { id: "h1".into(), nodes, e: 200e9, nu: 0.3, density };
            let ctx = ElementContext { positions: positions.to_vec() };
            let m = hex.mass(&ctx).expect("hex8 reports mass");
            let sum_tx: f64 = (0..8).flat_map(|r| (0..8).map(move |c| (3 * r, 3 * c))).map(|(r, c)| m.get(r, c)).sum();
            assert!((sum_tx - density).abs() / density < 1e-9, "sum={sum_tx} expected={density}");
        }

        /// 🌀️ `Hex8::geometric_stiffness`: zero under rigid translation and symmetric.
        #[test]
        fn hex8_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
            let (e, nu) = (200e9, 0.3);
            let positions = skew_hex_positions();
            let field = LinearField::sample();
            let ctx = ElementContext { positions: positions.to_vec() };
            let nodes: [String; 8] = std::array::from_fn(|i| format!("n{i}"));
            let hex = Hex8 { id: "h1".into(), nodes, e, nu, density: 0.0 };
            let u = field.nodal_vector(&positions);
            let kg = hex.geometric_stiffness(&ctx, &u).expect("hex8 reports geometric stiffness");
            for r in 0..24 {
                for c in 0..24 {
                    assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-6, "Kg not symmetric at ({r},{c})");
                }
            }
            let rigid = VecD::from_vec((0..8).flat_map(|_| [1.0, 2.0, 3.0]).collect());
            let f = kg.mul_vec(&rigid);
            for i in 0..24 {
                assert!(f.get(i).abs() < 1e-3, "rigid-body geometric force[{i}] = {}", f.get(i));
            }
        }
        // #endregion 🔖️Hex8
    }
    // #endregion 🔖️SolidTests

    // #region 🔖️ShellTests
    #[cfg(test)]
    mod shell_tests {
        use super::*;
        use crate::{solve_linear_static, Model, NodalLoad, Node, Support};

        const E: f64 = 1000.0;
        const NU: f64 = 0.25;
        const THICKNESS: f64 = 1.0;

        // Same small-magnitude membrane field as `elements2d::continuum_tests` (keeps expected forces
        // O(1) relative to the absolute patch-test tolerance) plus a small constant-curvature bending field.
        const MU: (f64, f64, f64) = (0.01, 0.003, 0.0021);
        const MV: (f64, f64, f64) = (-0.02, 0.0012, 0.0027);
        const KX: f64 = 0.004;
        const KY: f64 = -0.0025;
        const KXY: f64 = 0.0017;

        /// 📐️ A triangle placed so `p1-p0` lies exactly on global X and `p2` has `z=0` — the local shell
        /// frame (`local_x=normalize(p1-p0)`, `local_z=facet normal`) then coincides EXACTLY with global
        /// `(X,Y,Z)`, so local == global coordinates and the combined membrane+bending patch-test input
        /// can be built directly in global `(x,y)` without any local-frame bookkeeping.
        fn aligned_triangle_positions() -> [[f64; 3]; 3] {
            [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.2, 1.8, 0.0]]
        }

        #[test]
        fn shell_facet3_patch_test_reproduces_linear_membrane_and_constant_curvature() {
            let positions = aligned_triangle_positions();
            let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
            let ctx = ElementContext { positions: positions.to_vec() };

            let mut u = Vec::with_capacity(18);
            for &[x, y, _] in &positions {
                u.push(MU.0 + MU.1 * x + MU.2 * y); // Tx
                u.push(MV.0 + MV.1 * x + MV.2 * y); // Ty
                u.push(0.5 * (KX * x * x + KY * y * y + 2.0 * KXY * x * y)); // Tz = w
                u.push(KY * y + KXY * x); // Rx = ∂w/∂y
                u.push(-(KX * x + KXY * y)); // Ry = -∂w/∂x
                u.push(0.0); // Rz (drilling) stays zero — not excited by this field
            }
            let u = VecD::from_vec(u);

            let ElementResult::Shell { gauss } = el.recover(&ctx, &u, None) else { panic!("expected shell result") };
            assert_eq!(gauss.len(), 1);
            let state = &gauss[0];

            let d_mem = d_matrix_plane_stress(E, NU);
            let strain = VecD::from_vec(vec![MU.1, MV.2, MU.2 + MV.1]);
            let sigma = d_mem.mul_vec(&strain);
            let (expected_nxx, expected_nyy, expected_nxy) = (sigma.get(0) * THICKNESS, sigma.get(1) * THICKNESS, sigma.get(2) * THICKNESS);

            let d_bend = crate::elements2d::d_matrix_plate(E, NU, THICKNESS);
            let kappa = VecD::from_vec(vec![KX, KY, 2.0 * KXY]);
            let m = d_bend.mul_vec(&kappa);

            let mem_scale = expected_nxx.abs().max(expected_nyy.abs()).max(expected_nxy.abs()).max(1.0);
            assert!((state.nxx - expected_nxx).abs() / mem_scale < 1e-6, "nxx {} vs {}", state.nxx, expected_nxx);
            assert!((state.nyy - expected_nyy).abs() / mem_scale < 1e-6, "nyy {} vs {}", state.nyy, expected_nyy);
            assert!((state.nxy - expected_nxy).abs() / mem_scale < 1e-6, "nxy {} vs {}", state.nxy, expected_nxy);

            let bend_scale = m.get(0).abs().max(m.get(1).abs()).max(m.get(2).abs()).max(1.0);
            assert!((state.mxx - m.get(0)).abs() / bend_scale < 1e-4, "mxx {} vs {}", state.mxx, m.get(0));
            assert!((state.myy - m.get(1)).abs() / bend_scale < 1e-4, "myy {} vs {}", state.myy, m.get(1));
            assert!((state.mxy - m.get(2)).abs() / bend_scale < 1e-4, "mxy {} vs {}", state.mxy, m.get(2));
        }

        /// 🌀️ Rigid-body test: a pure 3D translation (zero rotation, so the drilling DOF is untouched too)
        /// must produce zero internal force on a generic (non-axis-aligned) skew triangle.
        #[test]
        fn shell_facet3_rigid_translation_gives_zero_force() {
            let positions = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.3], [0.5, 1.5, 0.7]];
            let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
            let ctx = ElementContext { positions: positions.to_vec() };
            let ke = el.stiffness_global(&ctx);
            let mut rigid = Vec::with_capacity(18);
            for _ in 0..3 {
                rigid.extend_from_slice(&[1.2, -0.8, 0.5, 0.0, 0.0, 0.0]);
            }
            let rigid = VecD::from_vec(rigid);
            let f = ke.mul_vec(&rigid);
            for i in 0..18 {
                assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
            }
        }

        /// 🏗️ Smoke test: a single flat `ShellFacet3` with one full edge fixed, loaded out-of-plane at the
        /// free node — deflection must be finite, nonzero, and in the same direction as the applied load
        /// (not a precision benchmark, just a physical-sanity check on assembly/BC wiring).
        #[test]
        fn shell_facet3_cantilever_deflects_toward_tip_load() {
            let (e, nu, t) = (200e9, 0.3, 0.01);
            let p = -1000.0;
            let model = Model {
                nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [1.0, 0.0, 0.0] }, Node { id: "c".into(), pos: [0.0, 1.0, 0.0] }],
                elements: vec![Box::new(ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e, nu, thickness: t, density: 0.0 })],
                supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }, Support { node_id: "b".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }],
                nodal_loads: vec![NodalLoad { node_id: "c".into(), dof: Dof::Tz, value: p }],
                member_loads: vec![],
            };
            let result = solve_linear_static(&model).expect("cantilevered shell facet solves");
            let c = result.displacements.iter().find(|d| d.node_id == "c").unwrap();
            let dz = c.values[Dof::Tz.index()];
            assert!(dz.is_finite() && dz < 0.0, "tip deflection {dz} should be finite and negative (toward the -Tz load)");
        }

        /// ⚖️ `ShellFacet3::mass`'s total (pure-`Tx` submatrix sum) equals `ρtA` — same lumped-mass
        /// row-sum identity `PlateDkt`'s translational lump satisfies.
        #[test]
        fn shell_facet3_mass_total_equals_rho_t_area() {
            let (density, thickness) = (7850.0, 0.008);
            let positions = aligned_triangle_positions();
            let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness, density };
            let ctx = ElementContext { positions: positions.to_vec() };
            let m = el.mass(&ctx).expect("shell facet reports mass");
            let sum_tx: f64 = (0..3).flat_map(|r| (0..3).map(move |c| (6 * r, 6 * c))).map(|(r, c)| m.get(r, c)).sum();
            // `aligned_triangle_positions` is `[[0,0,0],[2,0,0],[0.2,1.8,0]]` — shoelace area directly.
            let area = 0.5 * ((positions[1][0] - positions[0][0]) * (positions[2][1] - positions[0][1]) - (positions[2][0] - positions[0][0]) * (positions[1][1] - positions[0][1])).abs();
            let expected = density * thickness * area;
            assert!((sum_tx - expected).abs() / expected < 1e-9, "sum={sum_tx} expected={expected}");
        }

        /// 🌀️ A cantilevered flat shell panel (2 `ShellFacet3` triangles, one edge fully fixed) under
        /// in-plane axial COMPRESSION at the free edge must produce a finite, positive lowest linear-
        /// buckling load factor — possible only now that `ShellFacet3::geometric_stiffness` exists (a
        /// `PlateDkt`-only panel would report no geometric stiffness at all, per its documented `None`).
        #[test]
        fn shell_facet3_membrane_compression_destabilizes_and_tension_stabilizes_out_of_plane_stiffness() {
            let positions = aligned_triangle_positions();
            let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
            let ctx = ElementContext { positions: positions.to_vec() };

            // Uniform uniaxial membrane strain `u = k*x` (zero elsewhere) recovers a constant `Nxx`,
            // compressive for k<0 and tensile for k>0 — same field shape `elements2d::continuum_tests`
            // uses for its patch tests.
            let field = |k: f64| {
                let mut u = Vec::with_capacity(18);
                for &[x, _, _] in &positions {
                    u.extend_from_slice(&[k * x, 0.0, 0.0, 0.0, 0.0, 0.0]);
                }
                VecD::from_vec(u)
            };
            let kg_tension = el.geometric_stiffness(&ctx, &field(1e-4)).expect("shell reports geometric stiffness");
            let kg_compression = el.geometric_stiffness(&ctx, &field(-1e-4)).expect("shell reports geometric stiffness");

            // Node `b`'s local `Tz` sits at global index 8 (node 1 * 6 dof + 2) — the aligned-triangle
            // fixture makes local == global, so this global diagonal entry is directly the out-of-plane
            // stiffness contribution the buckling solver would add for node b.
            let tz_b = 8usize;
            assert!(kg_tension.get(tz_b, tz_b) > 0.0, "tension should STIFFEN out-of-plane bending, got Kg[b,Tz]={}", kg_tension.get(tz_b, tz_b));
            assert!(kg_compression.get(tz_b, tz_b) < 0.0, "compression should DESTABILIZE out-of-plane bending, got Kg[b,Tz]={}", kg_compression.get(tz_b, tz_b));

            for r in 0..18 {
                for c in 0..18 {
                    assert!((kg_compression.get(r, c) - kg_compression.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                }
            }
        }
    }
    // #endregion 🔖️ShellTests
}

pub mod formulation {
    //! 🧭️ Shared element-formulation toolkit: Gauss quadrature rules, isoparametric shape functions and
    //! their parametric derivatives, Jacobians, plane/solid B-matrices, and constitutive D-matrices —
    //! consumed by the continuum/plate/shell elements in `elements2d`/`elements3d`.

    use math::algebra::MatD;

    // #region 🔖️Quadrature
    /// 🎯️ 1D Gauss-Legendre points/weights on `[-1,1]`, `n = 1..=4`.
    pub fn gauss_1d(n: usize) -> Vec<(f64, f64)> {
        match n {
            1 => vec![(0.0, 2.0)],
            2 => {
                let g = 1.0 / 3.0f64.sqrt();
                vec![(-g, 1.0), (g, 1.0)]
            }
            3 => {
                let g = (3.0 / 5.0f64).sqrt();
                vec![(-g, 5.0 / 9.0), (0.0, 8.0 / 9.0), (g, 5.0 / 9.0)]
            }
            4 => {
                let x1 = 0.3399810435848563;
                let x2 = 0.8611363115940526;
                let w1 = 0.6521451548625461;
                let w2 = 0.3478548451374538;
                vec![(-x2, w2), (-x1, w1), (x1, w1), (x2, w2)]
            }
            _ => panic!("gauss_1d: unsupported order {n}, only 1..=4 are implemented"),
        }
    }

    /// 🎯️ Triangle (area-coordinate) Gauss rules on the UNIT triangle (vertices `(0,0),(1,0),(0,1)`,
    /// area 0.5). `n=1`: centroid, weight 0.5. `n=3`: standard 3-point rule (weights sum to 0.5).
    /// `n=7`: 7-point Gauss-Hammer rule (weights sum to 0.5). Points/weights are in the triangle's
    /// PARAMETRIC `(xi, eta)` coords, ready to use directly as `∫f dA ≈ Σ f(xi_i,eta_i) * weight_i`.
    pub fn gauss_tri(n: usize) -> Vec<(f64, f64, f64)> {
        match n {
            1 => vec![(1.0 / 3.0, 1.0 / 3.0, 0.5)],
            3 => vec![(1.0 / 6.0, 1.0 / 6.0, 1.0 / 6.0), (2.0 / 3.0, 1.0 / 6.0, 1.0 / 6.0), (1.0 / 6.0, 2.0 / 3.0, 1.0 / 6.0)],
            7 => {
                let a = (6.0 - 15.0f64.sqrt()) / 21.0;
                let b = (6.0 + 15.0f64.sqrt()) / 21.0;
                let w1 = (155.0 - 15.0f64.sqrt()) / 2400.0;
                let w2 = (155.0 + 15.0f64.sqrt()) / 2400.0;
                vec![(1.0 / 3.0, 1.0 / 3.0, 9.0 / 80.0), (a, a, w1), (1.0 - 2.0 * a, a, w1), (a, 1.0 - 2.0 * a, w1), (b, b, w2), (1.0 - 2.0 * b, b, w2), (b, 1.0 - 2.0 * b, w2)]
            }
            _ => panic!("gauss_tri: unsupported order {n}, only 1, 3, 7 are implemented"),
        }
    }

    /// 🎯️ Tensor-product Gauss rule on the reference square `[-1,1]x[-1,1]`, `n x n` points.
    pub fn gauss_quad(n: usize) -> Vec<(f64, f64, f64)> {
        let rule = gauss_1d(n);
        let mut out = Vec::with_capacity(n * n);
        for &(xi, w_xi) in &rule {
            for &(eta, w_eta) in &rule {
                out.push((xi, eta, w_xi * w_eta));
            }
        }
        out
    }
    // #endregion 🔖️Quadrature

    // #region 🔖️ShapeFunctions
    /// 📐️ Tri3 (linear) shape functions and PARAMETRIC derivatives at `(xi, eta)`:
    /// `N = [1-xi-eta, xi, eta]`.
    pub fn shape_tri3(xi: f64, eta: f64) -> ([f64; 3], [[f64; 2]; 3]) {
        let n = [1.0 - xi - eta, xi, eta];
        let dn = [[-1.0, -1.0], [1.0, 0.0], [0.0, 1.0]];
        (n, dn)
    }

    /// 📐️ Tri6 (quadratic) shape functions. Node order: 3 corners `[n0,n1,n2]` at `(0,0),(1,0),(0,1)`,
    /// then 3 mid-edge nodes `[n01,n12,n20]` where mid-edge `ij` sits at the midpoint between corner
    /// `i` and corner `j` — i.e. the full node order is `[n0,n1,n2,n01,n12,n20]`. `mesh.rs`'s
    /// quadratic-promotion code must number Tri6 nodes to match this exact convention.
    pub fn shape_tri6(xi: f64, eta: f64) -> ([f64; 6], [[f64; 2]; 6]) {
        let l1 = 1.0 - xi - eta;
        let l2 = xi;
        let l3 = eta;
        let n = [l1 * (2.0 * l1 - 1.0), l2 * (2.0 * l2 - 1.0), l3 * (2.0 * l3 - 1.0), 4.0 * l1 * l2, 4.0 * l2 * l3, 4.0 * l3 * l1];
        let dn = [[1.0 - 4.0 * l1, 1.0 - 4.0 * l1], [4.0 * l2 - 1.0, 0.0], [0.0, 4.0 * l3 - 1.0], [4.0 * (l1 - l2), -4.0 * l2], [4.0 * l3, 4.0 * l2], [-4.0 * l3, 4.0 * (l1 - l3)]];
        (n, dn)
    }

    /// 📐️ Quad4 (bilinear) on reference square `[-1,1]^2`, node order counterclockwise from `(-1,-1)`.
    pub fn shape_quad4(xi: f64, eta: f64) -> ([f64; 4], [[f64; 2]; 4]) {
        let n = [0.25 * (1.0 - xi) * (1.0 - eta), 0.25 * (1.0 + xi) * (1.0 - eta), 0.25 * (1.0 + xi) * (1.0 + eta), 0.25 * (1.0 - xi) * (1.0 + eta)];
        let dn = [[-0.25 * (1.0 - eta), -0.25 * (1.0 - xi)], [0.25 * (1.0 - eta), -0.25 * (1.0 + xi)], [0.25 * (1.0 + eta), 0.25 * (1.0 + xi)], [-0.25 * (1.0 + eta), 0.25 * (1.0 - xi)]];
        (n, dn)
    }

    /// 📐️ Quad8 (serendipity quadratic), node order: 4 corners CCW from `(-1,-1)`, then 4 mid-edges in
    /// order (bottom `(0,-1)`, right `(1,0)`, top `(0,1)`, left `(-1,0)`).
    pub fn shape_quad8(xi: f64, eta: f64) -> ([f64; 8], [[f64; 2]; 8]) {
        let n = [
            -0.25 * (1.0 - xi) * (1.0 - eta) * (1.0 + xi + eta),
            0.25 * (1.0 + xi) * (1.0 - eta) * (xi - eta - 1.0),
            0.25 * (1.0 + xi) * (1.0 + eta) * (xi + eta - 1.0),
            0.25 * (1.0 - xi) * (1.0 + eta) * (eta - xi - 1.0),
            0.5 * (1.0 - xi * xi) * (1.0 - eta),
            0.5 * (1.0 + xi) * (1.0 - eta * eta),
            0.5 * (1.0 - xi * xi) * (1.0 + eta),
            0.5 * (1.0 - xi) * (1.0 - eta * eta),
        ];
        let dn = [
            [0.25 * (1.0 - eta) * (2.0 * xi + eta), 0.25 * (1.0 - xi) * (xi + 2.0 * eta)],
            [0.25 * (1.0 - eta) * (2.0 * xi - eta), -0.25 * (1.0 + xi) * (xi - 2.0 * eta)],
            [0.25 * (1.0 + eta) * (2.0 * xi + eta), 0.25 * (1.0 + xi) * (xi + 2.0 * eta)],
            [0.25 * (1.0 + eta) * (2.0 * xi - eta), 0.25 * (1.0 - xi) * (2.0 * eta - xi)],
            [-xi * (1.0 - eta), -0.5 * (1.0 - xi * xi)],
            [0.5 * (1.0 - eta * eta), -eta * (1.0 + xi)],
            [-xi * (1.0 + eta), 0.5 * (1.0 - xi * xi)],
            [-0.5 * (1.0 - eta * eta), -eta * (1.0 - xi)],
        ];
        (n, dn)
    }
    // #endregion 🔖️ShapeFunctions

    // #region 🔖️Jacobian
    /// 🧮️ Jacobian matrix (2x2 as `[[dx/dxi, dx/deta],[dy/dxi, dy/deta]]`), its determinant, and physical
    /// `(x,y)` shape-function derivatives (`dN/dx, dN/dy`) computed from parametric derivatives via
    /// `J^-1`, given nodal `(x,y)` coordinates and the parametric derivatives from a `shape_*` function.
    pub fn jacobian_2d(coords: &[[f64; 2]], d_n_param: &[[f64; 2]]) -> ([[f64; 2]; 2], f64, Vec<[f64; 2]>) {
        let mut dx_dxi = 0.0;
        let mut dx_deta = 0.0;
        let mut dy_dxi = 0.0;
        let mut dy_deta = 0.0;
        for (p, dn) in coords.iter().zip(d_n_param.iter()) {
            dx_dxi += dn[0] * p[0];
            dx_deta += dn[1] * p[0];
            dy_dxi += dn[0] * p[1];
            dy_deta += dn[1] * p[1];
        }
        let j = [[dx_dxi, dx_deta], [dy_dxi, dy_deta]];
        let det_j = dx_dxi * dy_deta - dx_deta * dy_dxi;
        let d_n_xy = d_n_param.iter().map(|dn| [(dy_deta * dn[0] - dy_dxi * dn[1]) / det_j, (-dx_deta * dn[0] + dx_dxi * dn[1]) / det_j]).collect();
        (j, det_j, d_n_xy)
    }
    // #endregion 🔖️Jacobian

    // #region 🔖️BMatrix
    /// 🧮️ Plane-stress/strain B-matrix (3 x 2n) from physical shape derivatives, standard ordering
    /// `[du/dx, dv/dy, du/dy+dv/dx]` rows, node-major `[u_i, v_i]` column order.
    pub fn b_matrix_plane(d_n_xy: &[[f64; 2]]) -> MatD {
        let mut b = MatD::zeros(3, d_n_xy.len() * 2);
        for (i, dn) in d_n_xy.iter().enumerate() {
            let (dx, dy) = (dn[0], dn[1]);
            b.set(0, 2 * i, dx);
            b.set(1, 2 * i + 1, dy);
            b.set(2, 2 * i, dy);
            b.set(2, 2 * i + 1, dx);
        }
        b
    }
    // #endregion 🔖️BMatrix

    // #region 🔖️DMatrix
    /// 🧱️ Plane-stress constitutive matrix (3x3) from `E`, `nu`:
    /// `E/(1-nu^2) * [[1,nu,0],[nu,1,0],[0,0,(1-nu)/2]]`.
    pub fn d_matrix_plane_stress(e: f64, nu: f64) -> MatD {
        let factor = e / (1.0 - nu * nu);
        let mut d = MatD::zeros(3, 3);
        d.set(0, 0, factor);
        d.set(0, 1, factor * nu);
        d.set(1, 0, factor * nu);
        d.set(1, 1, factor);
        d.set(2, 2, factor * (1.0 - nu) / 2.0);
        d
    }

    /// 🧱️ Plane-strain constitutive matrix (3x3):
    /// `E/((1+nu)(1-2nu)) * [[1-nu,nu,0],[nu,1-nu,0],[0,0,(1-2nu)/2]]`.
    pub fn d_matrix_plane_strain(e: f64, nu: f64) -> MatD {
        let factor = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
        let mut d = MatD::zeros(3, 3);
        d.set(0, 0, factor * (1.0 - nu));
        d.set(0, 1, factor * nu);
        d.set(1, 0, factor * nu);
        d.set(1, 1, factor * (1.0 - nu));
        d.set(2, 2, factor * (1.0 - 2.0 * nu) / 2.0);
        d
    }
    // #endregion 🔖️DMatrix

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn gauss_1d_weights_sum_to_two() {
            for n in 1..=4 {
                let sum: f64 = gauss_1d(n).iter().map(|(_, w)| w).sum();
                assert!((sum - 2.0).abs() < 1e-12, "n={n} sum={sum}");
            }
        }

        #[test]
        fn gauss_tri_weights_sum_to_half() {
            for n in [1, 3, 7] {
                let sum: f64 = gauss_tri(n).iter().map(|(_, _, w)| w).sum();
                assert!((sum - 0.5).abs() < 1e-12, "n={n} sum={sum}");
            }
        }

        #[test]
        fn gauss_quad_weights_sum_to_four() {
            for n in 1..=4 {
                let sum: f64 = gauss_quad(n).iter().map(|(_, _, w)| w).sum();
                assert!((sum - 4.0).abs() < 1e-9, "n={n} sum={sum}");
            }
        }

        #[test]
        fn shape_tri3_partition_of_unity_and_node_values() {
            let (n, _) = shape_tri3(0.2, 0.3);
            assert!((n[0] + n[1] + n[2] - 1.0).abs() < 1e-12);
            let nodes = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
            for (i, &(xi, eta)) in nodes.iter().enumerate() {
                let (n, _) = shape_tri3(xi, eta);
                for (j, &nj) in n.iter().enumerate() {
                    let expected = if i == j { 1.0 } else { 0.0 };
                    assert!((nj - expected).abs() < 1e-12, "tri3 node {i} shape {j} = {nj}");
                }
            }
        }

        #[test]
        fn shape_tri6_partition_of_unity_and_node_values() {
            let (n, _) = shape_tri6(0.15, 0.35);
            let sum: f64 = n.iter().sum();
            assert!((sum - 1.0).abs() < 1e-12);
            let nodes = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.5, 0.0), (0.5, 0.5), (0.0, 0.5)];
            for (i, &(xi, eta)) in nodes.iter().enumerate() {
                let (n, _) = shape_tri6(xi, eta);
                for (j, &nj) in n.iter().enumerate() {
                    let expected = if i == j { 1.0 } else { 0.0 };
                    assert!((nj - expected).abs() < 1e-10, "tri6 node {i} shape {j} = {nj}");
                }
            }
        }

        #[test]
        fn shape_quad4_partition_of_unity_and_node_values() {
            let (n, _) = shape_quad4(0.3, -0.4);
            let sum: f64 = n.iter().sum();
            assert!((sum - 1.0).abs() < 1e-12);
            let nodes = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
            for (i, &(xi, eta)) in nodes.iter().enumerate() {
                let (n, _) = shape_quad4(xi, eta);
                for (j, &nj) in n.iter().enumerate() {
                    let expected = if i == j { 1.0 } else { 0.0 };
                    assert!((nj - expected).abs() < 1e-12, "quad4 node {i} shape {j} = {nj}");
                }
            }
        }

        #[test]
        fn shape_quad8_partition_of_unity_and_node_values() {
            let (n, _) = shape_quad8(0.25, -0.6);
            let sum: f64 = n.iter().sum();
            assert!((sum - 1.0).abs() < 1e-10);
            let nodes = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)];
            for (i, &(xi, eta)) in nodes.iter().enumerate() {
                let (n, _) = shape_quad8(xi, eta);
                for (j, &nj) in n.iter().enumerate() {
                    let expected = if i == j { 1.0 } else { 0.0 };
                    assert!((nj - expected).abs() < 1e-10, "quad8 node {i} shape {j} = {nj}");
                }
            }
        }

        #[test]
        fn jacobian_of_axis_aligned_rectangle_is_diagonal() {
            let (lx, ly) = (4.0, 6.0);
            let coords = [[0.0, 0.0], [lx, 0.0], [lx, ly], [0.0, ly]];
            let (_, d_n) = shape_quad4(0.3, -0.2);
            let (j, det_j, _) = jacobian_2d(&coords, &d_n);
            assert!(j[0][1].abs() < 1e-12);
            assert!(j[1][0].abs() < 1e-12);
            assert!((j[0][0] - lx / 2.0).abs() < 1e-12);
            assert!((j[1][1] - ly / 2.0).abs() < 1e-12);
            assert!((det_j - lx * ly / 4.0).abs() < 1e-9);
        }

        #[test]
        #[should_panic(expected = "gauss_1d: unsupported order")]
        fn gauss_1d_panics_on_unsupported_order() {
            gauss_1d(5);
        }

        #[test]
        #[should_panic(expected = "gauss_tri: unsupported order")]
        fn gauss_tri_panics_on_unsupported_order() {
            gauss_tri(2);
        }
    }
    // #endregion 🔖️Tests
}

pub mod mesh {
    //! 🕸️ Meshing: 2D constrained Delaunay triangulation with holes (`PlanarDomain` → `TriMesh2`),
    //! structured quad grids, quadratic promotion, and 3D extrusion (wedge/hex) with tet splitting.
    //! `spade` (constrained Delaunay + Ruppert refinement) is the only external geometry dependency and
    //! NEVER leaks through this module's public API — every public type here is a first-party plain-data
    //! struct/enum of `f64`/`u32` so callers never need to import or know about `spade`.

    use spade::{AngleLimit, ConstrainedDelaunayTriangulation, Point2, RefinementParameters, Triangulation};
    use std::collections::HashMap;

    // #region 🔖️PlanarDomain
    /// 📐️ A planar region to mesh: an outer boundary loop and zero or more hole loops, each a closed
    /// polygon (points NOT repeating the first point at the end), in consistent (either) winding order.
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlanarDomain {
        pub outer: Vec<[f64; 2]>,
        pub holes: Vec<Vec<[f64; 2]>>,
    }

    /// 🕸️ A triangulated mesh: shared node positions plus triangles as index triples into `points`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct TriMesh2 {
        pub points: Vec<[f64; 2]>,
        pub tris: Vec<[u32; 3]>,
    }

    /// ⚙️ Refinement targets for [`triangulate`] — either left at `0.0` to disable that constraint.
    #[derive(Clone, Copy, Debug)]
    pub struct MeshOpts {
        pub max_edge: f64,
        pub min_angle_deg: f64,
    }

    /// ⚠️ Everything that can go wrong building a mesh.
    #[derive(Debug, thiserror::Error)]
    pub enum MeshError {
        #[error("domain has a degenerate outer boundary (fewer than 3 points)")]
        DegenerateDomain,
        #[error("triangulation failed: {0}")]
        TriangulationFailed(String),
    }

    type Cdt = ConstrainedDelaunayTriangulation<Point2<f64>>;

    /// 🧵️ Inserts one closed loop's points as constrained CDT vertices, then constrains consecutive
    /// pairs (wrapping around) so the loop's edges survive triangulation/refinement unbroken-in-shape.
    fn insert_loop(cdt: &mut Cdt, loop_pts: &[[f64; 2]]) -> Result<(), MeshError> {
        let mut handles = Vec::with_capacity(loop_pts.len());
        for p in loop_pts {
            let handle = cdt.insert(Point2::new(p[0], p[1])).map_err(|e| MeshError::TriangulationFailed(format!("{e:?}")))?;
            handles.push(handle);
        }
        for i in 0..handles.len() {
            let a = handles[i];
            let b = handles[(i + 1) % handles.len()];
            if a != b {
                cdt.add_constraint(a, b);
            }
        }
        Ok(())
    }

    /// 🎯️ Ray-casting point-in-polygon test (standard even-odd rule; polygon need not be convex).
    fn point_in_polygon(point: [f64; 2], polygon: &[[f64; 2]]) -> bool {
        if polygon.len() < 3 {
            return false;
        }
        let mut inside = false;
        let mut j = polygon.len() - 1;
        for i in 0..polygon.len() {
            let (xi, yi) = (polygon[i][0], polygon[i][1]);
            let (xj, yj) = (polygon[j][0], polygon[j][1]);
            let crosses = (yi > point[1]) != (yj > point[1]);
            if crosses && point[0] < (xj - xi) * (point[1] - yi) / (yj - yi) + xi {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    /// 🕸️ Constrained Delaunay triangulation of `domain` honoring the outer boundary and hole boundaries
    /// as constrained edges, with holes excluded from the output, optionally refined per `opts`.
    ///
    /// Refinement (Ruppert's algorithm via `spade::RefinementParameters`) targets a minimum triangle
    /// angle (`opts.min_angle_deg`, disabled when `<= 0.0`) and/or a maximum triangle area derived from
    /// `opts.max_edge` via the equilateral-triangle area formula `sqrt(3)/4 * max_edge^2` (disabled when
    /// `<= 0.0`). Triangle inside/outside classification happens AFTER refinement, by a local
    /// point-in-polygon centroid test — `spade`'s own outer-face exclusion is not relied upon, since a
    /// domain with holes has several disjoint constrained loops that classification must handle directly.
    pub fn triangulate(domain: &PlanarDomain, opts: &MeshOpts) -> Result<TriMesh2, MeshError> {
        if domain.outer.len() < 3 {
            return Err(MeshError::DegenerateDomain);
        }

        let mut cdt: Cdt = ConstrainedDelaunayTriangulation::new();
        insert_loop(&mut cdt, &domain.outer)?;
        for hole in &domain.holes {
            if hole.len() < 3 {
                return Err(MeshError::DegenerateDomain);
            }
            insert_loop(&mut cdt, hole)?;
        }

        if opts.max_edge > 0.0 || opts.min_angle_deg > 0.0 {
            let mut params = RefinementParameters::<f64>::new();
            if opts.min_angle_deg > 0.0 {
                params = params.with_angle_limit(AngleLimit::from_deg(opts.min_angle_deg));
            }
            if opts.max_edge > 0.0 {
                let max_area = (3f64.sqrt() / 4.0) * opts.max_edge * opts.max_edge;
                params = params.with_max_allowed_area(max_area);
            }
            cdt.refine(params);
        }

        let mut point_index: HashMap<(u64, u64), u32> = HashMap::new();
        let mut points: Vec<[f64; 2]> = Vec::new();
        let mut tris: Vec<[u32; 3]> = Vec::new();

        for face in cdt.inner_faces() {
            let verts = face.vertices();
            let positions: [[f64; 2]; 3] = std::array::from_fn(|i| {
                let p = verts[i].position();
                [p.x, p.y]
            });
            let centroid = [(positions[0][0] + positions[1][0] + positions[2][0]) / 3.0, (positions[0][1] + positions[1][1] + positions[2][1]) / 3.0];

            let mut outside = !point_in_polygon(centroid, &domain.outer);
            if !outside {
                outside = domain.holes.iter().any(|hole| point_in_polygon(centroid, hole));
            }
            if outside {
                continue;
            }

            let mut idxs = [0u32; 3];
            for k in 0..3 {
                let key = (positions[k][0].to_bits(), positions[k][1].to_bits());
                let idx = *point_index.entry(key).or_insert_with(|| {
                    points.push(positions[k]);
                    (points.len() - 1) as u32
                });
                idxs[k] = idx;
            }
            tris.push(idxs);
        }

        Ok(TriMesh2 { points, tris })
    }
    // #endregion 🔖️PlanarDomain

    // #region 🔖️QuadMesh2
    /// 🔲️ A structured quad mesh: shared node positions plus quads as index quadruples into `points`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct QuadMesh2 {
        pub points: Vec<[f64; 2]>,
        pub quads: Vec<[u32; 4]>,
    }

    /// 🔲️ An `nx` x `ny` structured grid of quads over an axis-aligned rectangle `[x0,x1] x [y0,y1]`,
    /// row-major point numbering, each quad wound `[bottom-left, bottom-right, top-right, top-left]`.
    pub fn quad_grid(x0: f64, y0: f64, x1: f64, y1: f64, nx: usize, ny: usize) -> QuadMesh2 {
        let mut points = Vec::with_capacity((nx + 1) * (ny + 1));
        for j in 0..=ny {
            for i in 0..=nx {
                let x = x0 + (x1 - x0) * (i as f64) / (nx as f64);
                let y = y0 + (y1 - y0) * (j as f64) / (ny as f64);
                points.push([x, y]);
            }
        }
        let index = |i: usize, j: usize| (j * (nx + 1) + i) as u32;
        let mut quads = Vec::with_capacity(nx * ny);
        for j in 0..ny {
            for i in 0..nx {
                quads.push([index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)]);
            }
        }
        QuadMesh2 { points, quads }
    }
    // #endregion 🔖️QuadMesh2

    // #region 🔖️Quadratic
    /// 🔺️ A quadratic-promoted triangle mesh: shared node positions (originals first, then appended
    /// mid-edge points) plus 6-node triangles.
    #[derive(Clone, Debug, PartialEq)]
    pub struct TriMesh2Quadratic {
        pub points: Vec<[f64; 2]>,
        pub tris6: Vec<[u32; 6]>,
    }

    /// 🔗️ Looks up (or creates, welding shared edges to exactly one mid-node) the mid-edge point index
    /// for edge `(a,b)`, keyed by the sorted `(min,max)` index pair.
    fn mid_index(a: u32, b: u32, points: &mut Vec<[f64; 2]>, edge_mid: &mut HashMap<(u32, u32), u32>) -> u32 {
        let key = if a < b { (a, b) } else { (b, a) };
        if let Some(&idx) = edge_mid.get(&key) {
            return idx;
        }
        let pa = points[a as usize];
        let pb = points[b as usize];
        let idx = points.len() as u32;
        points.push([(pa[0] + pb[0]) * 0.5, (pa[1] + pb[1]) * 0.5]);
        edge_mid.insert(key, idx);
        idx
    }

    /// 🔺️ Promotes a linear `TriMesh2` to quadratic by inserting a mid-edge node per unique edge (shared
    /// edges between adjacent triangles get exactly ONE mid-node, deduped by sorted `(min,max)` edge key).
    /// Original points keep their indices unchanged; mid-edge points are appended after them. Each
    /// triangle's 6 node indices follow `[n0,n1,n2, mid(n0,n1), mid(n1,n2), mid(n2,n0)]` — the standard
    /// Tri6 convention (matches `elements2d.rs`'s `shape_tri6` node ordering, documented here since that
    /// function may land concurrently with this module).
    pub fn to_quadratic(mesh: &TriMesh2) -> TriMesh2Quadratic {
        let mut points = mesh.points.clone();
        let mut edge_mid: HashMap<(u32, u32), u32> = HashMap::new();
        let mut tris6 = Vec::with_capacity(mesh.tris.len());

        for tri in &mesh.tris {
            let [n0, n1, n2] = *tri;
            let m01 = mid_index(n0, n1, &mut points, &mut edge_mid);
            let m12 = mid_index(n1, n2, &mut points, &mut edge_mid);
            let m20 = mid_index(n2, n0, &mut points, &mut edge_mid);
            tris6.push([n0, n1, n2, m01, m12, m20]);
        }

        TriMesh2Quadratic { points, tris6 }
    }
    // #endregion 🔖️Quadratic

    // #region 🔖️VolumeMesh
    /// 🧱️ One volumetric cell — a linear wedge/hex prism or a tet, as index tuples into `VolumeMesh::points`.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Cell {
        Wedge6([u32; 6]),
        Hex8([u32; 8]),
        Tet4([u32; 4]),
    }

    /// 🧱️ A volumetric mesh: shared node positions plus cells.
    #[derive(Clone, Debug, PartialEq)]
    pub struct VolumeMesh {
        pub points: Vec<[f64; 3]>,
        pub cells: Vec<Cell>,
    }

    /// 🧱️ Extrudes a flat `TriMesh2` (lying in the z=0 plane) along +z by `height`, split into `layers`
    /// equal-height layers, producing one `Wedge6` per (triangle, layer) — node order
    /// `[bottom0,bottom1,bottom2, top0,top1,top2]` (bottom face matches the triangle's own `[n0,n1,n2]`
    /// winding, top face directly above).
    pub fn extrude_tri_mesh(mesh: &TriMesh2, height: f64, layers: usize) -> VolumeMesh {
        let layers = layers.max(1);
        let n = mesh.points.len();
        let mut points = Vec::with_capacity(n * (layers + 1));
        for l in 0..=layers {
            let z = height * (l as f64) / (layers as f64);
            for p in &mesh.points {
                points.push([p[0], p[1], z]);
            }
        }
        let mut cells = Vec::with_capacity(mesh.tris.len() * layers);
        for l in 0..layers {
            let bottom_off = (l * n) as u32;
            let top_off = ((l + 1) * n) as u32;
            for tri in &mesh.tris {
                let [a, b, c] = *tri;
                cells.push(Cell::Wedge6([bottom_off + a, bottom_off + b, bottom_off + c, top_off + a, top_off + b, top_off + c]));
            }
        }
        VolumeMesh { points, cells }
    }

    /// 🧱️ Extrudes a flat `QuadMesh2` along +z by `height` into `layers` layers of `Hex8` cells — node
    /// order `[bottom0,bottom1,bottom2,bottom3, top0,top1,top2,top3]` matching the quad's own winding.
    pub fn extrude_quad_mesh(mesh: &QuadMesh2, height: f64, layers: usize) -> VolumeMesh {
        let layers = layers.max(1);
        let n = mesh.points.len();
        let mut points = Vec::with_capacity(n * (layers + 1));
        for l in 0..=layers {
            let z = height * (l as f64) / (layers as f64);
            for p in &mesh.points {
                points.push([p[0], p[1], z]);
            }
        }
        let mut cells = Vec::with_capacity(mesh.quads.len() * layers);
        for l in 0..layers {
            let bottom_off = (l * n) as u32;
            let top_off = ((l + 1) * n) as u32;
            for quad in &mesh.quads {
                let [a, b, c, d] = *quad;
                cells.push(Cell::Hex8([bottom_off + a, bottom_off + b, bottom_off + c, bottom_off + d, top_off + a, top_off + b, top_off + c, top_off + d]));
            }
        }
        VolumeMesh { points, cells }
    }

    /// ✂️ Splits a quad face `[a,b,c,d]` (in winding order, so `a`-`c` and `b`-`d` are the two diagonals)
    /// into 2 triangles, choosing the diagonal FROM the corner with the smallest global point index. This
    /// depends only on the face's own 4 global indices (not on cell/apex choice), so two cells sharing a
    /// quad face always agree — the parity-consistency guarantee `split_to_tets` relies on.
    fn split_quad_face(a: u32, b: u32, c: u32, d: u32) -> [[u32; 3]; 2] {
        let min = a.min(b).min(c).min(d);
        if min == a || min == c {
            [[a, b, c], [a, c, d]]
        } else {
            [[a, b, d], [b, c, d]]
        }
    }

    /// 🔺️ Fan-triangulates a convex cell's boundary from `apex` (one of the cell's own vertices): every
    /// quad face is split via [`split_quad_face`], every triangular face passes through as-is, and every
    /// resulting boundary triangle that does NOT already contain `apex` becomes a tet `(apex, t0, t1, t2)`
    /// — the standard star/cone decomposition of a convex polyhedron, valid since convexity guarantees no
    /// overlap/gaps. Faces touching `apex` need no explicit tet: their volume is degenerate (zero) from
    /// `apex`'s own cone and is instead captured as internal faces of tets from adjacent, non-apex faces.
    fn split_cell_to_tets(quad_faces: &[[u32; 4]], tri_faces: &[[u32; 3]], apex: u32) -> Vec<[u32; 4]> {
        let mut tets = Vec::new();
        for &[a, b, c, d] in quad_faces {
            for tri in split_quad_face(a, b, c, d) {
                if !tri.contains(&apex) {
                    tets.push([apex, tri[0], tri[1], tri[2]]);
                }
            }
        }
        for &tri in tri_faces {
            if !tri.contains(&apex) {
                tets.push([apex, tri[0], tri[1], tri[2]]);
            }
        }
        tets
    }

    /// 🔪️ Splits every `Wedge6`/`Hex8` cell into `Tet4` cells (`Wedge6` → 3 tets, `Hex8` → 6 tets), using
    /// the minimum-global-node-index apex + face-diagonal rule so adjacent cells split their SHARED quad
    /// faces identically (see [`split_quad_face`]). `Tet4` cells in the input pass through unchanged.
    pub fn split_to_tets(mesh: &VolumeMesh) -> VolumeMesh {
        let mut cells = Vec::with_capacity(mesh.cells.len());
        for cell in &mesh.cells {
            match cell {
                Cell::Tet4(t) => cells.push(Cell::Tet4(*t)),
                Cell::Wedge6(w) => {
                    let [n0, n1, n2, n3, n4, n5] = *w;
                    let apex = w.iter().copied().min().unwrap();
                    let quads = [[n0, n1, n4, n3], [n1, n2, n5, n4], [n2, n0, n3, n5]];
                    let tris = [[n0, n1, n2], [n3, n4, n5]];
                    for tet in split_cell_to_tets(&quads, &tris, apex) {
                        cells.push(Cell::Tet4(tet));
                    }
                }
                Cell::Hex8(h) => {
                    let [n0, n1, n2, n3, n4, n5, n6, n7] = *h;
                    let apex = h.iter().copied().min().unwrap();
                    let quads = [[n0, n1, n2, n3], [n4, n5, n6, n7], [n0, n1, n5, n4], [n1, n2, n6, n5], [n2, n3, n7, n6], [n3, n0, n4, n7]];
                    for tet in split_cell_to_tets(&quads, &[], apex) {
                        cells.push(Cell::Tet4(tet));
                    }
                }
            }
        }
        VolumeMesh { points: mesh.points.clone(), cells }
    }

    /// 🧭️ The average of `mesh.points` at `idxs` — shared by `boundary_faces`'s per-tet and per-face
    /// centroid computations.
    fn point_centroid(mesh: &VolumeMesh, idxs: &[u32]) -> [f64; 3] {
        let mut c = [0.0; 3];
        for &i in idxs {
            let p = mesh.points[i as usize];
            for k in 0..3 {
                c[k] += p[k];
            }
        }
        let n = idxs.len() as f64;
        [c[0] / n, c[1] / n, c[2] / n]
    }

    /// 🧱️ Every triangular face belonging to EXACTLY ONE `Tet4` cell — the mesh's outer surface (call
    /// [`split_to_tets`] first if `mesh` still has `Wedge6`/`Hex8` cells; those contribute no faces here).
    /// Each returned triangle is independently wound so its `cross(edge0,edge1)` normal points AWAY from
    /// its own tet's centroid (outward) — determined per-tet via a centroid side-test, so the result
    /// doesn't depend on any input node-order convention. Used by `fem_3d`'s solid mesh preview/rendering.
    pub fn boundary_faces(mesh: &VolumeMesh) -> Vec<[u32; 3]> {
        let mut counts: HashMap<[u32; 3], usize> = HashMap::new();
        let mut oriented: HashMap<[u32; 3], [u32; 3]> = HashMap::new();

        for cell in &mesh.cells {
            let Cell::Tet4(t) = cell else { continue };
            let [n0, n1, n2, n3] = *t;
            let tet_centroid = point_centroid(mesh, &[n0, n1, n2, n3]);

            for face in [[n0, n1, n2], [n0, n1, n3], [n0, n2, n3], [n1, n2, n3]] {
                let mut key = face;
                key.sort_unstable();
                *counts.entry(key).or_insert(0) += 1;

                let p = |i: u32| mesh.points[i as usize];
                let (a, b, c) = (p(face[0]), p(face[1]), p(face[2]));
                let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
                let normal = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
                let face_centroid = point_centroid(mesh, &face);
                let to_tet = [tet_centroid[0] - face_centroid[0], tet_centroid[1] - face_centroid[1], tet_centroid[2] - face_centroid[2]];
                let dot = normal[0] * to_tet[0] + normal[1] * to_tet[1] + normal[2] * to_tet[2];
                let outward = if dot > 0.0 { [face[0], face[2], face[1]] } else { face };
                oriented.insert(key, outward);
            }
        }

        counts.into_iter().filter(|(_, count)| *count == 1).map(|(key, _)| oriented[&key]).collect()
    }
    // #endregion 🔖️VolumeMesh

    // #region 🔖️Quality
    /// 📊️ Cheap mesh sanity report — interior angle bounds (2D) and inverted-cell detection (3D).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct QualityReport {
        pub min_angle_deg: f64,
        pub max_angle_deg: f64,
        pub min_jacobian_sign_positive: bool,
        pub element_count: usize,
    }

    /// 📐️ Interior angle at `p`, between the edges to `prev` and `next`, in degrees.
    fn angle_at(prev: [f64; 2], p: [f64; 2], next: [f64; 2]) -> f64 {
        let v1 = [prev[0] - p[0], prev[1] - p[1]];
        let v2 = [next[0] - p[0], next[1] - p[1]];
        let dot = v1[0] * v2[0] + v1[1] * v2[1];
        let n1 = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
        let n2 = (v2[0] * v2[0] + v2[1] * v2[1]).sqrt();
        let cos_a = (dot / (n1 * n2)).clamp(-1.0, 1.0);
        cos_a.acos().to_degrees()
    }

    /// 📊️ Min/max interior angle across all triangles; `min_jacobian_sign_positive` mirrors the 2D
    /// analogue of the 3D check — true iff every triangle's signed area (shoelace, `[n0,n1,n2]` order) is
    /// positive, i.e. consistently wound.
    pub fn tri_mesh_quality(mesh: &TriMesh2) -> QualityReport {
        let mut min_angle = f64::INFINITY;
        let mut max_angle = f64::NEG_INFINITY;
        let mut all_positive = true;
        for tri in &mesh.tris {
            let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
            for i in 0..3 {
                let a = angle_at(p[(i + 2) % 3], p[i], p[(i + 1) % 3]);
                min_angle = min_angle.min(a);
                max_angle = max_angle.max(a);
            }
            let signed_area = 0.5 * ((p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[2][0] - p[0][0]) * (p[1][1] - p[0][1]));
            if signed_area <= 0.0 {
                all_positive = false;
            }
        }
        if mesh.tris.is_empty() {
            min_angle = 0.0;
            max_angle = 0.0;
        }
        QualityReport { min_angle_deg: min_angle, max_angle_deg: max_angle, min_jacobian_sign_positive: all_positive, element_count: mesh.tris.len() }
    }

    /// 🧮️ Signed tet volume via the scalar triple product of edge vectors from `p0`.
    fn tet_signed_volume(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
        let a = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
        let b = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
        let c = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];
        let cross = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
        (cross[0] * c[0] + cross[1] * c[1] + cross[2] * c[2]) / 6.0
    }

    /// 🧮️ Signed volume of one cell, via a FIXED (purely local-index-based, not global-min-based like
    /// [`split_to_tets`]) tet decomposition — so the sign faithfully reflects whether the cell's own
    /// documented local node order (`extrude_tri_mesh`/`extrude_quad_mesh`'s convention) is right-handed,
    /// independent of the cell's global point indices. Verified by hand against a unit right prism and a
    /// unit cube (both give the expected positive volume for correctly-ordered nodes).
    fn cell_signed_volume(points: &[[f64; 3]], cell: &Cell) -> f64 {
        let p = |i: u32| points[i as usize];
        match cell {
            Cell::Tet4([a, b, c, d]) => tet_signed_volume(p(*a), p(*b), p(*c), p(*d)),
            Cell::Wedge6(n) => tet_signed_volume(p(n[0]), p(n[1]), p(n[2]), p(n[3])) + tet_signed_volume(p(n[1]), p(n[2]), p(n[3]), p(n[4])) + tet_signed_volume(p(n[2]), p(n[3]), p(n[4]), p(n[5])),
            Cell::Hex8(n) => {
                tet_signed_volume(p(n[0]), p(n[4]), p(n[5]), p(n[6]))
                    + tet_signed_volume(p(n[0]), p(n[4]), p(n[6]), p(n[7]))
                    + tet_signed_volume(p(n[0]), p(n[1]), p(n[2]), p(n[6]))
                    + tet_signed_volume(p(n[0]), p(n[1]), p(n[6]), p(n[5]))
                    + tet_signed_volume(p(n[0]), p(n[2]), p(n[3]), p(n[7]))
                    + tet_signed_volume(p(n[0]), p(n[2]), p(n[7]), p(n[6]))
            }
        }
    }

    /// 📊️ `min_jacobian_sign_positive` is true iff every cell's signed volume is positive — a negative
    /// signed volume flags inverted/degenerate connectivity. Angle bounds are a 2D-only concept and are
    /// left at `0.0` here.
    pub fn volume_mesh_quality(mesh: &VolumeMesh) -> QualityReport {
        let all_positive = mesh.cells.iter().all(|cell| cell_signed_volume(&mesh.points, cell) > 0.0);
        QualityReport { min_angle_deg: 0.0, max_angle_deg: 0.0, min_jacobian_sign_positive: all_positive, element_count: mesh.cells.len() }
    }
    // #endregion 🔖️Quality

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashSet;

        fn shoelace_area(points: &[[f64; 2]]) -> f64 {
            let mut sum = 0.0;
            for i in 0..points.len() {
                let a = points[i];
                let b = points[(i + 1) % points.len()];
                sum += a[0] * b[1] - b[0] * a[1];
            }
            (sum * 0.5).abs()
        }

        fn tri_area(mesh: &TriMesh2, tri: &[u32; 3]) -> f64 {
            shoelace_area(&[mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]])
        }

        fn total_area(mesh: &TriMesh2) -> f64 {
            mesh.tris.iter().map(|t| tri_area(mesh, t)).sum()
        }

        fn no_refine() -> MeshOpts {
            MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }
        }

        fn square(side: f64) -> Vec<[f64; 2]> {
            vec![[0.0, 0.0], [side, 0.0], [side, side], [0.0, side]]
        }

        #[test]
        fn triangulate_square_area_matches_input() {
            let outer = square(10.0);
            let expected = shoelace_area(&outer);
            let domain = PlanarDomain { outer, holes: vec![] };
            let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
            assert!(!mesh.tris.is_empty());
            assert!((total_area(&mesh) - expected).abs() < 1e-9);
        }

        #[test]
        fn triangulate_respects_hole_area() {
            let outer = square(10.0);
            let hole = vec![[3.0, 3.0], [7.0, 3.0], [7.0, 7.0], [3.0, 7.0]];
            let domain = PlanarDomain { outer, holes: vec![hole.clone()] };
            let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
            let expected = 100.0 - 16.0;
            assert!((total_area(&mesh) - expected).abs() < 1e-6, "area={}", total_area(&mesh));
            for tri in &mesh.tris {
                let p0 = mesh.points[tri[0] as usize];
                let p1 = mesh.points[tri[1] as usize];
                let p2 = mesh.points[tri[2] as usize];
                let centroid = [(p0[0] + p1[0] + p2[0]) / 3.0, (p0[1] + p1[1] + p2[1]) / 3.0];
                assert!(!point_in_polygon(centroid, &hole));
            }
        }

        #[test]
        fn triangulate_honors_constrained_boundary_edges() {
            // L-shape: non-convex outer boundary.
            let outer = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [5.0, 5.0], [5.0, 10.0], [0.0, 10.0]];
            let domain = PlanarDomain { outer: outer.clone(), holes: vec![] };
            let mesh = triangulate(&domain, &no_refine()).expect("triangulates");

            let key = |p: [f64; 2]| (p[0].to_bits(), p[1].to_bits());
            let mut edge_set: HashSet<((u64, u64), (u64, u64))> = HashSet::new();
            for tri in &mesh.tris {
                let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
                for i in 0..3 {
                    let a = key(p[i]);
                    let b = key(p[(i + 1) % 3]);
                    let edge = if a <= b { (a, b) } else { (b, a) };
                    edge_set.insert(edge);
                }
            }

            for i in 0..outer.len() {
                let a = key(outer[i]);
                let b = key(outer[(i + 1) % outer.len()]);
                let edge = if a <= b { (a, b) } else { (b, a) };
                assert!(edge_set.contains(&edge), "boundary edge {i} missing from triangulation");
            }
        }

        #[test]
        fn refined_mesh_respects_min_angle() {
            // A long thin rectangle: all INPUT corners are 90 degrees (refinable), but the single
            // diagonal edge spade's initial CDT picks to fill it naturally produces slivers absent
            // refinement — Ruppert refinement can freely add Steiner points to fix that, unlike a sharp
            // INPUT corner angle (which no amount of edge splitting can widen).
            let outer = vec![[0.0, 0.0], [20.0, 0.0], [20.0, 1.0], [0.0, 1.0]];
            let domain = PlanarDomain { outer, holes: vec![] };
            let opts = MeshOpts { max_edge: 1.0, min_angle_deg: 25.0 };
            let mesh = triangulate(&domain, &opts).expect("triangulates");
            let quality = tri_mesh_quality(&mesh);
            let epsilon = 2.0; // Ruppert refinement guarantees are best-effort/asymptotic, not exact.
            assert!(quality.min_angle_deg >= opts.min_angle_deg - epsilon, "min_angle={}", quality.min_angle_deg);
        }

        #[test]
        fn quad_grid_has_expected_topology() {
            let mesh = quad_grid(0.0, 0.0, 3.0, 2.0, 3, 2);
            assert_eq!(mesh.quads.len(), 6);
            assert_eq!(mesh.points.len(), 12);
            assert_eq!(mesh.points[0], [0.0, 0.0]);
            assert_eq!(mesh.points[3], [3.0, 0.0]);
            assert_eq!(mesh.points[11], [3.0, 2.0]);
            assert_eq!(mesh.quads[0], [0, 1, 5, 4]);
            assert_eq!(mesh.quads[5], [6, 7, 11, 10]);
        }

        #[test]
        fn to_quadratic_welds_shared_edges() {
            let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
            let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
            assert!(mesh.tris.len() >= 2);

            let mut unique_edges: HashSet<(u32, u32)> = HashSet::new();
            for tri in &mesh.tris {
                for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
                    unique_edges.insert(if a < b { (a, b) } else { (b, a) });
                }
            }

            let quadratic = to_quadratic(&mesh);
            let new_points = quadratic.points.len() - mesh.points.len();
            assert_eq!(new_points, unique_edges.len());
        }

        #[test]
        fn extrude_tri_mesh_volume_matches_area_times_height() {
            let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
            let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
            let area = total_area(&mesh);
            let height = 3.0;
            let volume_mesh = extrude_tri_mesh(&mesh, height, 2);
            let tets = split_to_tets(&volume_mesh);
            let total: f64 = tets.cells.iter().map(|c| cell_signed_volume(&tets.points, c).abs()).sum();
            assert!((total - area * height).abs() < 1e-6, "total={} expected={}", total, area * height);
        }

        #[test]
        fn extrude_quad_mesh_volume_matches_area_times_height() {
            let mesh = quad_grid(0.0, 0.0, 4.0, 3.0, 4, 3);
            let area = 12.0;
            let height = 2.5;
            let volume_mesh = extrude_quad_mesh(&mesh, height, 3);
            let tets = split_to_tets(&volume_mesh);
            let total: f64 = tets.cells.iter().map(|c| cell_signed_volume(&tets.points, c).abs()).sum();
            assert!((total - area * height).abs() < 1e-6, "total={} expected={}", total, area * height);
        }

        #[test]
        fn split_to_tets_preserves_volume() {
            let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
            let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
            let wedge_mesh = extrude_tri_mesh(&mesh, 2.0, 2);
            let pre_wedge: f64 = wedge_mesh.cells.iter().map(|c| cell_signed_volume(&wedge_mesh.points, c).abs()).sum();
            let post_wedge = split_to_tets(&wedge_mesh);
            let post_wedge_total: f64 = post_wedge.cells.iter().map(|c| cell_signed_volume(&post_wedge.points, c).abs()).sum();
            assert!((pre_wedge - post_wedge_total).abs() < 1e-9);

            let quad_mesh = quad_grid(0.0, 0.0, 4.0, 4.0, 2, 2);
            let hex_mesh = extrude_quad_mesh(&quad_mesh, 2.0, 2);
            let pre_hex: f64 = hex_mesh.cells.iter().map(|c| cell_signed_volume(&hex_mesh.points, c).abs()).sum();
            let post_hex = split_to_tets(&hex_mesh);
            let post_hex_total: f64 = post_hex.cells.iter().map(|c| cell_signed_volume(&post_hex.points, c).abs()).sum();
            assert!((pre_hex - post_hex_total).abs() < 1e-9);
        }

        #[test]
        fn split_to_tets_shared_faces_are_parity_consistent() {
            // Two Hex8 cells sharing the quad face [1,2,6,5] (cell A's +x face / cell B's -x face).
            let points = vec![
                [0.0, 0.0, 0.0], // 0
                [1.0, 0.0, 0.0], // 1
                [1.0, 1.0, 0.0], // 2
                [0.0, 1.0, 0.0], // 3
                [0.0, 0.0, 1.0], // 4
                [1.0, 0.0, 1.0], // 5
                [1.0, 1.0, 1.0], // 6
                [0.0, 1.0, 1.0], // 7
                [2.0, 0.0, 0.0], // 8
                [2.0, 1.0, 0.0], // 9
                [2.0, 0.0, 1.0], // 10
                [2.0, 1.0, 1.0], // 11
            ];
            let cell_a = Cell::Hex8([0, 1, 2, 3, 4, 5, 6, 7]);
            let cell_b = Cell::Hex8([1, 8, 9, 2, 5, 10, 11, 6]);
            let shared_face: HashSet<u32> = [1, 2, 6, 5].into_iter().collect();

            let mesh_a = VolumeMesh { points: points.clone(), cells: vec![cell_a] };
            let mesh_b = VolumeMesh { points: points.clone(), cells: vec![cell_b] };
            let tets_a = split_to_tets(&mesh_a);
            let tets_b = split_to_tets(&mesh_b);

            let face_triangles = |vm: &VolumeMesh| -> HashSet<[u32; 3]> {
                let mut out = HashSet::new();
                for cell in &vm.cells {
                    if let Cell::Tet4(t) = cell {
                        let faces = [[t[0], t[1], t[2]], [t[0], t[1], t[3]], [t[0], t[2], t[3]], [t[1], t[2], t[3]]];
                        for mut f in faces {
                            if f.iter().all(|v| shared_face.contains(v)) {
                                f.sort_unstable();
                                out.insert(f);
                            }
                        }
                    }
                }
                out
            };

            let from_a = face_triangles(&tets_a);
            let from_b = face_triangles(&tets_b);
            assert_eq!(from_a.len(), 2, "expected the shared quad face split into 2 triangles from cell A");
            assert_eq!(from_a, from_b, "shared face must split identically from both cells");
        }

        #[test]
        fn volume_mesh_quality_detects_inverted_cell() {
            let points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
            let good = VolumeMesh { points: points.clone(), cells: vec![Cell::Tet4([0, 1, 2, 3])] };
            assert!(volume_mesh_quality(&good).min_jacobian_sign_positive);

            // Swap two nodes to invert the signed volume.
            let inverted = VolumeMesh { points, cells: vec![Cell::Tet4([1, 0, 2, 3])] };
            assert!(!volume_mesh_quality(&inverted).min_jacobian_sign_positive);
        }

        /// 🧱️ A `side`x`side` square extruded `height` tall, 1 layer, split to tets — `boundary_faces`'s
        /// total triangle area must equal the analytic box surface `2*side² + 4*side*height` (top + bottom
        /// + 4 sides), which also confirms every internal (shared, appears-twice) face was excluded.
        #[test]
        fn boundary_faces_area_matches_extruded_box_surface() {
            let side = 4.0;
            let height = 3.0;
            let domain = PlanarDomain { outer: square(side), holes: vec![] };
            let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
            let volume_mesh = extrude_tri_mesh(&mesh, height, 1);
            let tets = split_to_tets(&volume_mesh);

            let faces = boundary_faces(&tets);
            let tri_area = |f: &[u32; 3]| -> f64 {
                let (a, b, c) = (tets.points[f[0] as usize], tets.points[f[1] as usize], tets.points[f[2] as usize]);
                let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
                let cross = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
                0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
            };
            let total_area: f64 = faces.iter().map(tri_area).sum();
            let expected = 2.0 * side * side + 4.0 * side * height;
            assert!((total_area - expected).abs() < 1e-6, "total={total_area} expected={expected}");

            // Every boundary face must be wound so its normal points away from its own tet's centroid —
            // spot-checked here on the bottom face (z=0, outward normal must have negative z).
            for f in &faces {
                if tets.points[f[0] as usize][2] < 1e-9 && tets.points[f[1] as usize][2] < 1e-9 && tets.points[f[2] as usize][2] < 1e-9 {
                    let (a, b, c) = (tets.points[f[0] as usize], tets.points[f[1] as usize], tets.points[f[2] as usize]);
                    let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                    let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
                    let normal_z = e0[0] * e1[1] - e0[1] * e1[0];
                    assert!(normal_z < 0.0, "bottom face normal should point outward (-z), got normal_z={normal_z}");
                }
            }
        }

        #[test]
        fn triangulate_rejects_degenerate_outer_boundary() {
            let domain = PlanarDomain { outer: vec![[0.0, 0.0], [1.0, 0.0]], holes: vec![] };
            match triangulate(&domain, &no_refine()) {
                Err(MeshError::DegenerateDomain) => {}
                other => panic!("expected DegenerateDomain, got {other:?}"),
            }
        }

        #[test]
        fn triangulate_rejects_degenerate_hole() {
            let domain = PlanarDomain { outer: square(10.0), holes: vec![vec![[3.0, 3.0], [4.0, 4.0]]] };
            match triangulate(&domain, &no_refine()) {
                Err(MeshError::DegenerateDomain) => {}
                other => panic!("expected DegenerateDomain, got {other:?}"),
            }
        }

        #[test]
        fn point_in_polygon_returns_false_for_degenerate_polygon() {
            assert!(!point_in_polygon([0.0, 0.0], &[]));
            assert!(!point_in_polygon([0.0, 0.0], &[[0.0, 0.0], [1.0, 0.0]]));
        }

        /// 📊️ `tri_mesh_quality` flags a clockwise-wound (negative signed area) triangle via
        /// `min_jacobian_sign_positive`, and reports `0.0` angle bounds for an empty mesh instead of the
        /// unhelpful `f64::INFINITY`/`NEG_INFINITY` an empty min/max fold would otherwise leave behind.
        #[test]
        fn tri_mesh_quality_detects_inverted_winding_and_handles_empty_mesh() {
            let ccw = TriMesh2 { points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], tris: vec![[0, 1, 2]] };
            assert!(tri_mesh_quality(&ccw).min_jacobian_sign_positive);

            let cw = TriMesh2 { points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], tris: vec![[0, 2, 1]] };
            assert!(!tri_mesh_quality(&cw).min_jacobian_sign_positive);

            let empty = TriMesh2 { points: vec![], tris: vec![] };
            let quality = tri_mesh_quality(&empty);
            assert_eq!(quality.min_angle_deg, 0.0);
            assert_eq!(quality.max_angle_deg, 0.0);
            assert_eq!(quality.element_count, 0);
        }

        /// 🔺️ A `Cell::Tet4` already present in the input `VolumeMesh` passes through `split_to_tets`
        /// completely unchanged — the only cell kind besides `Wedge6`/`Hex8` `split_to_tets` accepts.
        #[test]
        fn split_to_tets_passes_through_existing_tet4_cells() {
            let points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
            let tet = Cell::Tet4([0, 1, 2, 3]);
            let mesh = VolumeMesh { points, cells: vec![tet] };
            let result = split_to_tets(&mesh);
            assert_eq!(result.cells.len(), 1);
            match result.cells[0] {
                Cell::Tet4(nodes) => assert_eq!(nodes, [0, 1, 2, 3]),
                _ => panic!("expected the Tet4 cell to pass through unchanged"),
            }
        }
    }
    // #endregion 🔖️Tests
}

pub mod sparse {
    //! 🧮️ Sparse linear algebra: COO/CSR/CSC assembly, a left-looking sparse LDLT direct solver, a
    //! Jacobi-preconditioned conjugate-gradient iterative solver, a subspace-iteration eigensolver
    //! (modal/buckling `Kφ=λBφ`) backed by a dense cyclic-Jacobi eigensolver for its small projected
    //! subproblem, and reverse-Cuthill-McKee bandwidth-reduction ordering. No dependency beyond
    //! `math::algebra`'s dense `MatD`/`VecD`, used here as both scratch storage for small
    //! projected problems and as the correctness oracle in this module's tests.

    use math::algebra::{MatD, VecD};
    use std::collections::{BTreeMap, VecDeque};

    // #region 🔖️Coo
    /// 🧱️ Triplet (row, col, value) accumulator for FEM-style assembly — duplicate `(row, col)`
    /// entries are summed lazily by whichever `to_*` conversion reads them.
    pub struct Coo {
        pub n: usize,
        rows: Vec<u32>,
        cols: Vec<u32>,
        vals: Vec<f64>,
    }

    impl Coo {
        pub fn new(n: usize) -> Self {
            Self { n, rows: Vec::new(), cols: Vec::new(), vals: Vec::new() }
        }

        pub fn add(&mut self, row: usize, col: usize, value: f64) {
            self.rows.push(row as u32);
            self.cols.push(col as u32);
            self.vals.push(value);
        }

        /// 🧩️ Scatters a small dense element block (e.g. from `Element::stiffness_global`) at global indices.
        pub fn add_block(&mut self, indices: &[usize], block: &MatD) {
            for (local_row, &global_row) in indices.iter().enumerate() {
                for (local_col, &global_col) in indices.iter().enumerate() {
                    let value = block.get(local_row, local_col);
                    if value != 0.0 {
                        self.add(global_row, global_col, value);
                    }
                }
            }
        }

        fn merge_sorted(mut entries: Vec<(u32, f64)>) -> Vec<(u32, f64)> {
            entries.sort_by_key(|&(k, _)| k);
            let mut merged: Vec<(u32, f64)> = Vec::with_capacity(entries.len());
            for (k, v) in entries {
                if let Some(last) = merged.last_mut() {
                    if last.0 == k {
                        last.1 += v;
                        continue;
                    }
                }
                merged.push((k, v));
            }
            merged
        }

        /// 🧮️ General CSR (both triangles present or not, caller's choice) — used for SpMV.
        pub fn to_csr(&self) -> Csr {
            let n = self.n;
            let mut by_row: Vec<Vec<(u32, f64)>> = vec![Vec::new(); n];
            for i in 0..self.rows.len() {
                by_row[self.rows[i] as usize].push((self.cols[i], self.vals[i]));
            }
            let mut indptr = vec![0u32; n + 1];
            let mut indices = Vec::new();
            let mut vals = Vec::new();
            for row in 0..n {
                let merged = Self::merge_sorted(std::mem::take(&mut by_row[row]));
                indptr[row + 1] = indptr[row] + merged.len() as u32;
                for (c, v) in merged {
                    indices.push(c);
                    vals.push(v);
                }
            }
            Csr { n, indptr, indices, vals }
        }

        /// 🔺️ Keeps only entries where `col >= row` (upper triangle), grouped by the SMALLER index `j`
        /// so that storage-column `j` directly holds `A[j][c]` for every `c >= j` (via symmetry
        /// `A[j][c] = A[c][j]`) — the layout the left-looking LDLT column loop needs without a scan.
        pub fn to_csc_sym_upper(&self) -> CscSym {
            let n = self.n;
            let mut by_col: Vec<Vec<(u32, f64)>> = vec![Vec::new(); n];
            for i in 0..self.rows.len() {
                let row = self.rows[i];
                let col = self.cols[i];
                if col >= row {
                    by_col[row as usize].push((col, self.vals[i]));
                }
            }
            let mut colptr = vec![0u32; n + 1];
            let mut rowind = Vec::new();
            let mut vals = Vec::new();
            for col in 0..n {
                let merged = Self::merge_sorted(std::mem::take(&mut by_col[col]));
                colptr[col + 1] = colptr[col] + merged.len() as u32;
                for (r, v) in merged {
                    rowind.push(r);
                    vals.push(v);
                }
            }
            CscSym { n, colptr, rowind, vals }
        }

        /// 🪞️ Dense form for testing/cross-validation against `MatD::lu_solve`.
        pub fn to_dense(&self) -> MatD {
            let mut m = MatD::zeros(self.n, self.n);
            for i in 0..self.rows.len() {
                m.add_at(self.rows[i] as usize, self.cols[i] as usize, self.vals[i]);
            }
            m
        }
    }
    // #endregion 🔖️Coo

    // #region 🔖️Csr
    /// 🧮️ General compressed-sparse-row matrix — used for SpMV (PCG, residual checks).
    pub struct Csr {
        pub n: usize,
        indptr: Vec<u32>,
        indices: Vec<u32>,
        vals: Vec<f64>,
    }

    impl Csr {
        pub fn mul_vec(&self, x: &VecD) -> VecD {
            let mut out = VecD::zeros(self.n);
            for row in 0..self.n {
                let start = self.indptr[row] as usize;
                let end = self.indptr[row + 1] as usize;
                let mut sum = 0.0;
                for idx in start..end {
                    sum += self.vals[idx] * x.get(self.indices[idx] as usize);
                }
                out.set(row, sum);
            }
            out
        }

        pub fn diag(&self) -> VecD {
            let mut out = VecD::zeros(self.n);
            for row in 0..self.n {
                let start = self.indptr[row] as usize;
                let end = self.indptr[row + 1] as usize;
                for idx in start..end {
                    if self.indices[idx] as usize == row {
                        out.set(row, self.vals[idx]);
                    }
                }
            }
            out
        }
    }
    // #endregion 🔖️Csr

    // #region 🔖️CscSym
    /// 🔺️ Symmetric matrix, upper-triangle entries only (`col >= row`), grouped by the smaller index —
    /// storage-column `j` holds `A[j][c]` for every `c >= j`, the LDLT input format.
    pub struct CscSym {
        pub n: usize,
        colptr: Vec<u32>,
        rowind: Vec<u32>,
        vals: Vec<f64>,
    }

    impl CscSym {
        /// 🔍️ Reads `(row, col)` — storage-column is the smaller index, stored-row the larger.
        pub fn get(&self, row: usize, col: usize) -> f64 {
            let (lo, hi) = if row <= col { (row, col) } else { (col, row) };
            let start = self.colptr[lo] as usize;
            let end = self.colptr[lo + 1] as usize;
            for idx in start..end {
                if self.rowind[idx] as usize == hi {
                    return self.vals[idx];
                }
            }
            0.0
        }

        /// 🪟️ Mirrors into a full general CSR (for SpMV/PCG/residual use).
        pub fn to_csr_full(&self) -> Csr {
            let mut coo = Coo::new(self.n);
            for col in 0..self.n {
                let start = self.colptr[col] as usize;
                let end = self.colptr[col + 1] as usize;
                for idx in start..end {
                    let row = self.rowind[idx] as usize;
                    let value = self.vals[idx];
                    coo.add(row, col, value);
                    if row != col {
                        coo.add(col, row, value);
                    }
                }
            }
            coo.to_csr()
        }
    }
    // #endregion 🔖️CscSym

    // #region 🔖️Ldlt
    /// ⚠️ Everything that can go wrong factoring a `CscSym`.
    #[derive(Debug)]
    pub enum SparseError {
        ZeroPivot { column: usize },
        DimensionMismatch,
    }

    /// 🧊️ A sparse left-looking LDLT factorization (unit lower `L`, diagonal `D`), permutation-agnostic
    /// — a caller applying `rcm_order` reorders the matrix/RHS/solution indices itself before/after
    /// calling into this module.
    #[derive(Debug)]
    pub struct LdltFactor {
        n: usize,
        l_cols: Vec<BTreeMap<u32, f64>>,
        d: Vec<f64>,
    }

    /// 🧮️ Left-looking sparse LDLT: for each column `j`, seeds an accumulator from `A`'s column `j`
    /// (rows `>= j`), then for every earlier column `k` with `L[j][k] != 0` (tracked via each row's
    /// list of contributing earlier columns) subtracts `L[j][k] * L[i][k] * D[k]` at every row `i`
    /// where `L[i][k] != 0` — this is where fill-in appears. Symbolic and numeric phases are combined
    /// in one pass, per Davis's "Direct Methods for Sparse Linear Systems".
    pub fn ldlt_factor(a: &CscSym) -> Result<LdltFactor, SparseError> {
        let n = a.n;
        let mut l_cols: Vec<BTreeMap<u32, f64>> = vec![BTreeMap::new(); n];
        let mut d = vec![0.0; n];
        let mut row_lists: Vec<Vec<usize>> = vec![Vec::new(); n];

        for j in 0..n {
            let mut accum: BTreeMap<usize, f64> = BTreeMap::new();
            let start = a.colptr[j] as usize;
            let end = a.colptr[j + 1] as usize;
            for idx in start..end {
                let row = a.rowind[idx] as usize;
                *accum.entry(row).or_insert(0.0) += a.vals[idx];
            }

            for &k in &row_lists[j] {
                let ljk = *l_cols[k].get(&(j as u32)).unwrap_or(&0.0);
                if ljk == 0.0 {
                    continue;
                }
                let factor = ljk * d[k];
                for (&row_u32, &lik) in l_cols[k].iter() {
                    let row = row_u32 as usize;
                    if row >= j {
                        *accum.entry(row).or_insert(0.0) -= factor * lik;
                    }
                }
            }

            let djj = *accum.get(&j).unwrap_or(&0.0);
            if djj.abs() < 1e-12 {
                return Err(SparseError::ZeroPivot { column: j });
            }
            d[j] = djj;

            for (&row, &value) in accum.iter() {
                if row > j && value != 0.0 {
                    l_cols[j].insert(row as u32, value / djj);
                    row_lists[row].push(j);
                }
            }
        }

        Ok(LdltFactor { n, l_cols, d })
    }

    impl LdltFactor {
        /// 🧭️ Forward (`Ly=b`) → diagonal (`z=y/D`) → backward (`Lᵀx=z`) substitution, column-oriented
        /// so no separate row-major structure of `L` is needed.
        pub fn solve(&self, b: &VecD) -> VecD {
            let n = self.n;
            let mut y = b.0.clone();
            for j in 0..n {
                let yj = y[j];
                if yj == 0.0 {
                    continue;
                }
                for (&row, &lij) in self.l_cols[j].iter() {
                    y[row as usize] -= lij * yj;
                }
            }
            for (j, value) in y.iter_mut().enumerate().take(n) {
                *value /= self.d[j];
            }
            for j in (0..n).rev() {
                let mut sum = y[j];
                for (&row, &lij) in self.l_cols[j].iter() {
                    sum -= lij * y[row as usize];
                }
                y[j] = sum;
            }
            VecD::from_vec(y)
        }

        pub fn solve_many(&self, b: &MatD) -> MatD {
            let mut out = MatD::zeros(b.rows, b.cols);
            for col in 0..b.cols {
                let rhs = VecD::from_vec((0..b.rows).map(|row| b.get(row, col)).collect());
                let x = self.solve(&rhs);
                for row in 0..b.rows {
                    out.set(row, col, x.get(row));
                }
            }
            out
        }

        /// 🔢️ Count of `D[j] < 0` — a Sturm-sequence inertia count, used later for eigenvalue-count checks.
        pub fn negative_pivot_count(&self) -> usize {
            self.d.iter().filter(|&&value| value < 0.0).count()
        }
    }
    // #endregion 🔖️Ldlt

    // #region 🔖️Pcg
    /// 📈️ Convergence outcome of a `pcg` call.
    #[derive(Debug, Clone, Copy)]
    pub struct PcgStats {
        pub iterations: usize,
        pub residual_norm: f64,
        pub converged: bool,
    }

    /// ➰️ Jacobi-preconditioned conjugate gradient — mutates `x0` in place, converges when
    /// `‖r‖ / ‖b‖ < tol_rel` or `max_iter` is reached.
    pub fn pcg(a: &Csr, b: &VecD, x0: &mut VecD, tol_rel: f64, max_iter: usize) -> PcgStats {
        let n = a.n;
        let diag = a.diag();
        let precondition = |r: &VecD| -> VecD {
            let mut z = VecD::zeros(n);
            for i in 0..n {
                let d = diag.get(i);
                z.set(i, if d.abs() > 1e-300 { r.get(i) / d } else { r.get(i) });
            }
            z
        };

        let b_norm = b.norm2().max(1e-300);
        let mut r = b.sub(&a.mul_vec(x0));
        let mut residual_norm = r.norm2() / b_norm;
        if residual_norm < tol_rel {
            return PcgStats { iterations: 0, residual_norm, converged: true };
        }

        let mut z = precondition(&r);
        let mut p = z.clone();
        let mut rz_old = r.dot(&z);
        let mut iterations = 0;

        for iter in 0..max_iter {
            iterations = iter + 1;
            let ap = a.mul_vec(&p);
            let pap = p.dot(&ap);
            if pap.abs() < 1e-300 {
                break;
            }
            let alpha = rz_old / pap;
            for i in 0..n {
                x0.set(i, x0.get(i) + alpha * p.get(i));
            }
            r = r.sub(&ap.scale(alpha));
            residual_norm = r.norm2() / b_norm;
            if residual_norm < tol_rel {
                return PcgStats { iterations, residual_norm, converged: true };
            }
            z = precondition(&r);
            let rz_new = r.dot(&z);
            let beta = rz_new / rz_old;
            p = z.add(&p.scale(beta));
            rz_old = rz_new;
        }

        PcgStats { iterations, residual_norm, converged: false }
    }
    // #endregion 🔖️Pcg

    // #region 🔖️DenseEigen
    /// 🎯️ Cyclic Jacobi eigenvalue algorithm for a small dense symmetric matrix — returns eigenvalues
    /// (ascending) and the matching eigenvectors as columns of the returned `MatD`. Used internally to
    /// solve the small (`p×p`, `p ≤ ~40`) projected eigenproblem inside `subspace_iteration`.
    fn dense_symmetric_eigen_jacobi(a: &MatD) -> (Vec<f64>, MatD) {
        let n = a.rows;
        let mut m = a.clone();
        let mut v = MatD::identity(n);
        if n == 0 {
            return (Vec::new(), v);
        }

        for _sweep in 0..100 {
            let mut off_sq = 0.0;
            for p in 0..n {
                for q in (p + 1)..n {
                    off_sq += m.get(p, q) * m.get(p, q);
                }
            }
            if off_sq.sqrt() < 1e-12 * (frobenius_norm(&m) + 1.0) {
                break;
            }

            for p in 0..n {
                for q in (p + 1)..n {
                    let apq = m.get(p, q);
                    if apq.abs() < 1e-300 {
                        continue;
                    }
                    let app = m.get(p, p);
                    let aqq = m.get(q, q);
                    let theta = (aqq - app) / (2.0 * apq);
                    let t = if theta >= 0.0 { 1.0 / (theta + (theta * theta + 1.0).sqrt()) } else { -1.0 / (-theta + (theta * theta + 1.0).sqrt()) };
                    let c = 1.0 / (t * t + 1.0).sqrt();
                    let s = t * c;

                    m.set(p, p, app - t * apq);
                    m.set(q, q, aqq + t * apq);
                    m.set(p, q, 0.0);
                    m.set(q, p, 0.0);

                    for i in 0..n {
                        if i == p || i == q {
                            continue;
                        }
                        let aip = m.get(i, p);
                        let aiq = m.get(i, q);
                        let new_aip = c * aip - s * aiq;
                        let new_aiq = s * aip + c * aiq;
                        m.set(i, p, new_aip);
                        m.set(p, i, new_aip);
                        m.set(i, q, new_aiq);
                        m.set(q, i, new_aiq);
                    }

                    for i in 0..n {
                        let vip = v.get(i, p);
                        let viq = v.get(i, q);
                        v.set(i, p, c * vip - s * viq);
                        v.set(i, q, s * vip + c * viq);
                    }
                }
            }
        }

        let raw_vals: Vec<f64> = (0..n).map(|i| m.get(i, i)).collect();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a_idx, &b_idx| raw_vals[a_idx].partial_cmp(&raw_vals[b_idx]).unwrap());
        let mut vals = vec![0.0; n];
        let mut vecs = MatD::zeros(n, n);
        for (new_idx, &old_idx) in order.iter().enumerate() {
            vals[new_idx] = raw_vals[old_idx];
            for row in 0..n {
                vecs.set(row, new_idx, v.get(row, old_idx));
            }
        }
        (vals, vecs)
    }

    fn frobenius_norm(m: &MatD) -> f64 {
        let mut sum = 0.0;
        for row in 0..m.rows {
            for col in 0..m.cols {
                sum += m.get(row, col) * m.get(row, col);
            }
        }
        sum.sqrt()
    }

    /// 🪜️ Lower-triangular Cholesky `A = L Lᵀ` of a small dense SPD matrix (Cholesky-Banachiewicz).
    fn cholesky_lower(a: &MatD) -> MatD {
        let n = a.rows;
        let mut l = MatD::zeros(n, n);
        for i in 0..n {
            for j in 0..=i {
                let mut sum = a.get(i, j);
                for k in 0..j {
                    sum -= l.get(i, k) * l.get(j, k);
                }
                if i == j {
                    l.set(i, j, sum.max(1e-300).sqrt());
                } else {
                    l.set(i, j, sum / l.get(j, j));
                }
            }
        }
        l
    }

    /// 🔁️ Inverse of a lower-triangular matrix via forward substitution, one identity column at a time.
    fn invert_lower_triangular(l: &MatD) -> MatD {
        let n = l.rows;
        let mut inv = MatD::zeros(n, n);
        for col in 0..n {
            let mut x = vec![0.0; n];
            for i in 0..n {
                let mut sum = if i == col { 1.0 } else { 0.0 };
                for k in 0..i {
                    sum -= l.get(i, k) * x[k];
                }
                x[i] = sum / l.get(i, i);
            }
            for i in 0..n {
                inv.set(i, col, x[i]);
            }
        }
        inv
    }

    fn symmetrize(a: &MatD) -> MatD {
        let n = a.rows;
        let mut out = MatD::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                out.set(i, j, 0.5 * (a.get(i, j) + a.get(j, i)));
            }
        }
        out
    }
    // #endregion 🔖️DenseEigen

    // #region 🔖️SubspaceIteration
    /// 📐️ The lowest `p` eigenpairs of a generalized eigenproblem, ascending by value.
    pub struct EigenPairs {
        pub values: Vec<f64>,
        pub vectors: Vec<VecD>,
    }

    fn mat_col(m: &MatD, col: usize) -> VecD {
        VecD::from_vec((0..m.rows).map(|row| m.get(row, col)).collect())
    }

    fn set_col(m: &mut MatD, col: usize, v: &VecD) {
        for row in 0..m.rows {
            m.set(row, col, v.get(row));
        }
    }

    fn apply_b(b: &Csr, m: &MatD) -> MatD {
        let mut out = MatD::zeros(m.rows, m.cols);
        for col in 0..m.cols {
            let bv = b.mul_vec(&mat_col(m, col));
            set_col(&mut out, col, &bv);
        }
        out
    }

    /// 🎯️ Finds the lowest `p` eigenpairs of `K x = λ B x`, given `K`'s LDLT factorization and `B` as a
    /// `Csr` (mass matrix, or `−Kg` for buckling — sign convention is the caller's responsibility).
    /// Bathe-style subspace iteration: B-orthonormalize the current subspace via modified Gram-Schmidt,
    /// solve `K Y = B X`, project both `K` and `B` onto `Y` (using `Yᵀ K Y = Yᵀ (K Y) = Yᵀ (B X)` so the
    /// raw `K` operator is never needed — only its factorization), solve the small dense generalized
    /// eigenproblem via a Cholesky-of-`B_proj` transform to a standard eigenproblem, rotate the subspace
    /// by the recovered eigenvectors, and repeat until the lowest `p` eigenvalues stop changing.
    pub fn subspace_iteration(k_factor: &LdltFactor, b: &Csr, n: usize, p: usize, max_iter: usize) -> EigenPairs {
        let m = (p + 8).max(2 * p).min(n).max(1);
        let mut x = MatD::zeros(n, m);
        for j in 0..m {
            x.set(j, j, 1.0);
            if j + 1 < n {
                x.add_at(j + 1, j, 0.3);
            }
            if j >= 1 {
                x.add_at(j - 1, j, 0.3);
            }
        }

        let mut prev_theta: Vec<f64> = vec![f64::INFINITY; p];
        let mut final_theta: Vec<f64> = Vec::new();
        let mut final_x = x.clone();

        for _iter in 0..max_iter {
            let bx = apply_b(b, &x);
            let mut cols: Vec<VecD> = (0..m).map(|j| mat_col(&x, j)).collect();
            let mut bcols: Vec<VecD> = (0..m).map(|j| mat_col(&bx, j)).collect();
            for j in 0..m {
                for k in 0..j {
                    let coeff = cols[j].dot(&bcols[k]);
                    cols[j] = cols[j].sub(&cols[k].scale(coeff));
                    bcols[j] = bcols[j].sub(&bcols[k].scale(coeff));
                }
                let norm = cols[j].dot(&bcols[j]).max(1e-300).sqrt();
                cols[j] = cols[j].scale(1.0 / norm);
                bcols[j] = bcols[j].scale(1.0 / norm);
            }
            for j in 0..m {
                set_col(&mut x, j, &cols[j]);
            }

            let rhs = apply_b(b, &x);
            let y = k_factor.solve_many(&rhs);

            let k_proj = symmetrize(&y.transpose().matmul(&rhs));
            let by = apply_b(b, &y);
            let b_proj = symmetrize(&y.transpose().matmul(&by));

            let l = cholesky_lower(&b_proj);
            let l_inv = invert_lower_triangular(&l);
            let a_hat = symmetrize(&l_inv.matmul(&k_proj).matmul(&l_inv.transpose()));
            let (theta, w) = dense_symmetric_eigen_jacobi(&a_hat);
            let z = l_inv.transpose().matmul(&w);

            let x_new = y.matmul(&z);

            let current_p: Vec<f64> = theta.iter().take(p).cloned().collect();
            let converged = current_p.iter().zip(prev_theta.iter()).all(|(&cur, &prev)| if prev.is_infinite() { false } else { ((cur - prev) / prev.abs().max(1e-12)).abs() < 1e-6 });

            prev_theta = current_p;
            final_theta = theta;
            final_x = x_new.clone();
            x = x_new;

            if converged {
                break;
            }
        }

        let values: Vec<f64> = final_theta.into_iter().take(p).collect();
        let vectors: Vec<VecD> = (0..p.min(final_x.cols)).map(|j| mat_col(&final_x, j)).collect();
        EigenPairs { values, vectors }
    }
    // #endregion 🔖️SubspaceIteration

    // #region 🔖️Rcm
    fn bfs_distances(start: usize, adjacency: &[Vec<usize>]) -> Vec<i64> {
        let n = adjacency.len();
        let mut dist = vec![-1i64; n];
        dist[start] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(u) = queue.pop_front() {
            for &v in &adjacency[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        dist
    }

    fn farthest_node(start: usize, adjacency: &[Vec<usize>]) -> usize {
        let dist = bfs_distances(start, adjacency);
        (0..adjacency.len()).filter(|&i| dist[i] >= 0).max_by_key(|&i| dist[i]).unwrap_or(start)
    }

    /// 🧭️ George-Liu pseudo-peripheral heuristic: BFS to the farthest node, repeat twice more.
    fn pseudo_peripheral(start: usize, adjacency: &[Vec<usize>]) -> usize {
        let a = farthest_node(start, adjacency);
        let b = farthest_node(a, adjacency);
        farthest_node(b, adjacency)
    }

    /// 🌀️ Reverse Cuthill-McKee ordering of an adjacency list (undirected graph: `adjacency[i]` =
    /// neighbors of node `i`). Returns a permutation `perm` such that `perm[new_index] = old_index`.
    /// Disconnected graphs are processed one component at a time, in order of first unvisited node;
    /// each component is seeded from its pseudo-peripheral node, BFS'd with each level's nodes emitted
    /// sorted by ascending degree, and the whole resulting order is reversed at the end.
    pub fn rcm_order(adjacency: &[Vec<usize>]) -> Vec<usize> {
        let n = adjacency.len();
        let mut visited = vec![false; n];
        let mut order: Vec<usize> = Vec::with_capacity(n);

        for seed in 0..n {
            if visited[seed] {
                continue;
            }
            let root = pseudo_peripheral(seed, adjacency);
            visited[root] = true;
            let mut queue = VecDeque::new();
            queue.push_back(root);
            order.push(root);
            while let Some(u) = queue.pop_front() {
                let mut neighbors: Vec<usize> = adjacency[u].iter().copied().filter(|&v| !visited[v]).collect();
                neighbors.sort_by_key(|&v| adjacency[v].len());
                for v in neighbors {
                    if !visited[v] {
                        visited[v] = true;
                        order.push(v);
                        queue.push_back(v);
                    }
                }
            }
        }

        order.reverse();
        order
    }
    // #endregion 🔖️Rcm

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn graph_laplacian_plus_identity(n: usize, edges: &[(usize, usize)]) -> Coo {
            let mut degree = vec![0usize; n];
            for &(u, v) in edges {
                degree[u] += 1;
                degree[v] += 1;
            }
            let mut coo = Coo::new(n);
            for i in 0..n {
                coo.add(i, i, degree[i] as f64 + 1.0);
            }
            for &(u, v) in edges {
                coo.add(u, v, -1.0);
                coo.add(v, u, -1.0);
            }
            coo
        }

        #[test]
        fn ldlt_matches_dense_lu_on_random_spd() {
            let n = 8;
            let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
            let coo = graph_laplacian_plus_identity(n, &edges);
            let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
            let x_expected = VecD::from_vec((0..n).map(|i| (i as f64) * 0.5 + 1.0).collect());
            let dense = coo.to_dense();
            let b = dense.mul_vec(&x_expected);
            let x_ldlt = factor.solve(&b);
            let x_lu = dense.lu_solve(&b).expect("dense solvable");
            for i in 0..n {
                assert!((x_ldlt.get(i) - x_expected.get(i)).abs() < 1e-8);
                assert!((x_ldlt.get(i) - x_lu.get(i)).abs() < 1e-8);
            }
        }

        #[test]
        fn ldlt_matches_dense_lu_on_1d_laplacian() {
            let n = 20;
            let mut coo = Coo::new(n);
            for i in 0..n {
                coo.add(i, i, 2.0);
                if i + 1 < n {
                    coo.add(i, i + 1, -1.0);
                    coo.add(i + 1, i, -1.0);
                }
            }
            let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
            let x_expected = VecD::from_vec((0..n).map(|i| ((i % 5) as f64) - 1.5).collect());
            let dense = coo.to_dense();
            let b = dense.mul_vec(&x_expected);
            let x_ldlt = factor.solve(&b);
            let x_lu = dense.lu_solve(&b).expect("dense solvable");
            for i in 0..n {
                assert!((x_ldlt.get(i) - x_lu.get(i)).abs() < 1e-8);
            }
        }

        #[test]
        fn ldlt_solve_many_matches_solve_per_column() {
            let n = 8;
            let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
            let coo = graph_laplacian_plus_identity(n, &edges);
            let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
            let mut rhs = MatD::zeros(n, 3);
            for r in 0..n {
                rhs.set(r, 0, r as f64 + 1.0);
                rhs.set(r, 1, (n - r) as f64);
                rhs.set(r, 2, if r % 2 == 0 { 1.0 } else { -1.0 });
            }
            let combined = factor.solve_many(&rhs);
            for c in 0..3 {
                let col = VecD::from_vec((0..n).map(|r| rhs.get(r, c)).collect());
                let single = factor.solve(&col);
                for r in 0..n {
                    assert!((combined.get(r, c) - single.get(r)).abs() < 1e-12);
                }
            }
        }

        #[test]
        fn ldlt_reports_zero_pivot_on_singular_matrix() {
            let n = 5;
            let edges = [(0, 1), (1, 2), (2, 3)];
            let mut degree = vec![0usize; n];
            for &(u, v) in &edges {
                degree[u] += 1;
                degree[v] += 1;
            }
            let mut coo = Coo::new(n);
            for i in 0..4 {
                coo.add(i, i, degree[i] as f64 + 1.0);
            }
            for &(u, v) in &edges {
                coo.add(u, v, -1.0);
                coo.add(v, u, -1.0);
            }
            match ldlt_factor(&coo.to_csc_sym_upper()) {
                Err(SparseError::ZeroPivot { column }) => assert_eq!(column, 4),
                other => panic!("expected zero pivot error, got {other:?}"),
            }
        }

        #[test]
        fn pcg_matches_ldlt_and_dense_lu() {
            let n = 8;
            let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
            let coo = graph_laplacian_plus_identity(n, &edges);
            let dense = coo.to_dense();
            let x_expected = VecD::from_vec((0..n).map(|i| (i as f64) * 0.3 - 1.0).collect());
            let b = dense.mul_vec(&x_expected);
            let csr = coo.to_csr();
            let mut x0 = VecD::zeros(n);
            let stats = pcg(&csr, &b, &mut x0, 1e-10, 500);
            assert!(stats.converged);
            let lu = dense.lu_solve(&b).expect("dense solvable");
            for i in 0..n {
                assert!((x0.get(i) - lu.get(i)).abs() < 1e-6);
            }
        }

        #[test]
        fn rcm_reduces_bandwidth_on_scattered_path_graph() {
            let shuffle = [9usize, 0, 8, 1, 7, 2, 6, 3, 5, 4];
            let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); 10];
            let mut edges = Vec::new();
            for i in 0..9 {
                let (u, v) = (shuffle[i], shuffle[i + 1]);
                adjacency[u].push(v);
                adjacency[v].push(u);
                edges.push((u, v));
            }
            let bandwidth = |index_of: &dyn Fn(usize) -> usize| -> usize { edges.iter().map(|&(u, v)| (index_of(u) as i64 - index_of(v) as i64).unsigned_abs() as usize).max().unwrap() };
            let before = bandwidth(&|x| x);
            let perm = rcm_order(&adjacency);
            let mut new_index = vec![0usize; 10];
            for (new_idx, &old_idx) in perm.iter().enumerate() {
                new_index[old_idx] = new_idx;
            }
            let after = bandwidth(&|x| new_index[x]);
            assert!(after <= before);
        }

        #[test]
        fn dense_symmetric_eigen_jacobi_matches_known_eigenvalues() {
            let mut a = MatD::zeros(3, 3);
            a.set(0, 0, 3.0);
            a.set(1, 1, 1.0);
            a.set(2, 2, 2.0);
            let (vals, _vecs) = dense_symmetric_eigen_jacobi(&a);
            assert!((vals[0] - 1.0).abs() < 1e-9);
            assert!((vals[1] - 2.0).abs() < 1e-9);
            assert!((vals[2] - 3.0).abs() < 1e-9);
        }

        #[test]
        fn subspace_iteration_matches_diagonal_analytic_case() {
            let n = 10;
            let mut k_coo = Coo::new(n);
            let mut b_coo = Coo::new(n);
            for i in 0..n {
                k_coo.add(i, i, (i + 1) as f64);
                b_coo.add(i, i, 1.0);
            }
            let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
            let b_csr = b_coo.to_csr();
            let pairs = subspace_iteration(&k_factor, &b_csr, n, 4, 30);
            let expected = [1.0, 2.0, 3.0, 4.0];
            for i in 0..4 {
                assert!((pairs.values[i] - expected[i]).abs() / expected[i] < 1e-4, "eigenvalue {} = {} expected {}", i, pairs.values[i], expected[i]);
            }
        }

        #[test]
        fn subspace_iteration_matches_dense_jacobi_on_small_nondiagonal_case() {
            let n = 7;
            let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 0), (0, 3)];
            let k_coo = graph_laplacian_plus_identity(n, &edges);
            let dense_k = k_coo.to_dense();
            let (dense_vals, _) = dense_symmetric_eigen_jacobi(&dense_k);

            let mut b_coo = Coo::new(n);
            for i in 0..n {
                b_coo.add(i, i, 1.0);
            }
            let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
            let b_csr = b_coo.to_csr();
            let pairs = subspace_iteration(&k_factor, &b_csr, n, 3, 30);

            for i in 0..3 {
                assert!((pairs.values[i] - dense_vals[i]).abs() / dense_vals[i].abs().max(1e-9) < 1e-3);
            }
        }

        /// 🔍️ `CscSym::get` reads back every entry of a symmetric matrix (both `row<=col` and `row>col`
        /// orderings resolve to the same stored upper-triangle slot) and returns `0.0` for an absent entry;
        /// `to_csr_full` mirrors the SAME matrix into a full (both triangles materialized) `Csr`.
        #[test]
        fn csc_sym_get_and_to_csr_full_match_dense() {
            let mut coo = Coo::new(3);
            coo.add(0, 0, 4.0);
            coo.add(1, 1, 5.0);
            coo.add(2, 2, 6.0);
            coo.add(0, 1, 2.0);
            coo.add(1, 0, 2.0);
            coo.add(1, 2, 3.0);
            coo.add(2, 1, 3.0);
            let dense = coo.to_dense();
            let csc = coo.to_csc_sym_upper();

            for r in 0..3 {
                for c in 0..3 {
                    assert!((csc.get(r, c) - dense.get(r, c)).abs() < 1e-12, "get({r},{c}) = {} vs dense {}", csc.get(r, c), dense.get(r, c));
                }
            }
            assert_eq!(csc.get(0, 2), 0.0, "no (0,2) entry was ever added");

            let full = csc.to_csr_full();
            let x = VecD::from_vec(vec![1.0, 2.0, 3.0]);
            let expected = dense.mul_vec(&x);
            let actual = full.mul_vec(&x);
            for i in 0..3 {
                assert!((actual.get(i) - expected.get(i)).abs() < 1e-9, "mul_vec[{i}] = {} vs {}", actual.get(i), expected.get(i));
            }
        }

        /// 🔢️ `negative_pivot_count` counts `D[j] < 0` — a diagonal (already-factored-trivially) indefinite
        /// matrix with one negative entry must report exactly one negative pivot.
        #[test]
        fn negative_pivot_count_counts_negative_diagonal_entries() {
            let mut coo = Coo::new(3);
            coo.add(0, 0, 1.0);
            coo.add(1, 1, -2.0);
            coo.add(2, 2, 3.0);
            let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("diagonal matrix factors trivially");
            assert_eq!(factor.negative_pivot_count(), 1);
        }

        /// ⏱️ `pcg` returns immediately (zero iterations, `converged: true`) when the initial guess `x0`
        /// already satisfies the residual tolerance.
        #[test]
        fn pcg_converges_immediately_when_initial_guess_is_already_exact() {
            let mut coo = Coo::new(3);
            coo.add(0, 0, 2.0);
            coo.add(1, 1, 3.0);
            coo.add(2, 2, 4.0);
            let csr = coo.to_csr();
            let mut x0 = VecD::from_vec(vec![1.0, 2.0, 3.0]);
            let b = csr.mul_vec(&x0);
            let stats = pcg(&csr, &b, &mut x0, 1e-8, 100);
            assert_eq!(stats.iterations, 0);
            assert!(stats.converged);
        }

        /// ⏱️ `pcg` with `max_iter: 0` never enters its iteration loop and reports `converged: false`.
        #[test]
        fn pcg_reports_not_converged_when_max_iter_is_zero() {
            let mut coo = Coo::new(3);
            coo.add(0, 0, 2.0);
            coo.add(1, 1, 3.0);
            coo.add(2, 2, 4.0);
            let csr = coo.to_csr();
            let b = VecD::from_vec(vec![1.0, 1.0, 1.0]);
            let mut x0 = VecD::zeros(3);
            let stats = pcg(&csr, &b, &mut x0, 1e-12, 0);
            assert_eq!(stats.iterations, 0);
            assert!(!stats.converged);
        }

        /// ⏱️ `pcg` against an all-zero operator has zero search-direction curvature (`pᵀAp = 0`) on its
        /// very first step, hitting the early `break` guard against dividing by zero — reported as
        /// `converged: false` after exactly 1 iteration.
        #[test]
        fn pcg_breaks_on_zero_curvature_direction() {
            let coo = Coo::new(3); // no entries added: A is the zero operator
            let csr = coo.to_csr();
            let b = VecD::from_vec(vec![1.0, 1.0, 1.0]);
            let mut x0 = VecD::zeros(3);
            let stats = pcg(&csr, &b, &mut x0, 1e-12, 50);
            assert_eq!(stats.iterations, 1);
            assert!(!stats.converged);
        }

        /// 🎯️ `dense_symmetric_eigen_jacobi` on a 0x0 matrix returns empty eigenvalues/eigenvectors instead
        /// of looping — the degenerate size `subspace_iteration`'s own `.max(1)` guard against normally
        /// avoids, but the helper itself must still handle directly.
        #[test]
        fn dense_symmetric_eigen_jacobi_handles_zero_size_matrix() {
            let a = MatD::zeros(0, 0);
            let (vals, vecs) = dense_symmetric_eigen_jacobi(&a);
            assert!(vals.is_empty());
            assert_eq!(vecs.rows, 0);
            assert_eq!(vecs.cols, 0);
        }
    }
    // #endregion 🔖️Tests
}

pub use elements2d::{Bar2, BeamEb2};
pub use elements3d::{Bar3, Frame3};

use math::algebra::{MatD, VecD};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;

// #region 🔖️Dof
/// 🧭️ Nodal degree of freedom kind, shared by 2D (Tx, Ty, Rz) and 3D (all six) models.
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
// #endregion 🔖️Dof

// #region 🔖️Model
/// 📍️ A structural node: a stable id and a global position (2D models keep `pos[2] == 0`).
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub pos: [f64; 3],
}

/// 🔒️ A support: the subset of a node's active DOFs restrained to zero displacement.
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

/// 📦️ Resolved geometry handed to an element when it builds its stiffness/loads/results —
/// node positions in the same order as `Element::node_ids`.
pub struct ElementContext {
    pub positions: Vec<[f64; 3]>,
}

/// 🔩️ One finite element: contributes a global-coordinate stiffness matrix, optional equivalent
/// nodal loads for a member UDL, and recovers internal forces from the solved displacement vector.
/// Every vector this trait produces or consumes is node-major, DOF-minor ordered, matching
/// `node_ids()` paired with `dofs_per_node()`.
pub trait Element {
    fn id(&self) -> &str;
    fn node_ids(&self) -> Vec<String>;
    fn dofs_per_node(&self) -> &[Dof];
    fn stiffness_global(&self, ctx: &ElementContext) -> MatD;
    /// 🌬️ Fixed-end nodal loads equivalent to a per-unit-length `MemberUdl` in GLOBAL coordinates.
    /// `None` (the default) means this element doesn't support member UDLs — meaningful for 2-node
    /// line members (`Bar2`/`Bar3`/`BeamEb2`/`Frame3`); continuum/plate/shell elements have no per-unit-
    /// length member concept, so distributed loading on them is the document layer's job (translated
    /// into ordinary nodal loads from an `Area`/pressure load — see `fem_2d`/`fem_3d`'s bridges).
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
    /// 🌀️ Geometric ("stress") stiffness in GLOBAL coordinates for the element's current axial/stress
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

/// 🔍️ Element trait objects aren't `Debug`, so print element ids/count instead — this is what lets
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
// #endregion 🔖️Model

// #region 🔖️Results
/// 📐️ Per-node displacement, indexed by `Dof::index()`; inactive DOFs stay `0.0`.
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

/// 📊️ A station along a beam's length: internal axial/shear/moment at `x` from the start node.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BeamStation {
    pub x: f64,
    pub n: f64,
    pub v: f64,
    pub m: f64,
}

/// 🧮️ In-plane stress state at one Gauss point of a plane-stress/plane-strain continuum element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaneStress {
    pub sxx: f64,
    pub syy: f64,
    pub sxy: f64,
    pub von_mises: f64,
}

/// 🧊️ Full 3D stress state at one Gauss point of a solid element.
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

/// 🧊️ Bending moments per unit width at one Gauss point of a plate element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlateMoments {
    pub mx: f64,
    pub my: f64,
    pub mxy: f64,
}

/// 🐚️ Membrane forces + bending moments per unit width at one Gauss point of a facet shell element.
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

/// 📤️ Element-kind-specific internal-force recovery. Only Gauss-point values live here — nodal
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

/// ✅️ Global sanity checks on the solved system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolutionChecks {
    pub residual_norm: f64,
    pub reaction_sum: [f64; 6],
}

/// 📈️ The full result of a linear-static solve.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticResult {
    pub displacements: Vec<NodeDisplacement>,
    pub reactions: Vec<NodeReaction>,
    pub elements: Vec<(String, ElementResult)>,
    pub checks: SolutionChecks,
}
// #endregion 🔖️Results

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

// #region 🔖️DofMap
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

/// 🔢️ Numbers each node's active DOFs (the union of `dofs_per_node()` over elements touching it),
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
// #endregion 🔖️DofMap

// #region 🔖️Validate
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
// #endregion 🔖️Validate

// #region 🔖️Assembly
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
// #endregion 🔖️Assembly

// #region 🔖️Solve
/// 🧮️ Assembles and solves the model for linear-static equilibrium `Ku = F`, partitioned by
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
// #endregion 🔖️Solve

// #region 🔖️Tests
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

    /// 🔍️ Duplicate node ids are rejected the same way `analyses::validate` rejects them.
    #[test]
    fn duplicate_node_id_is_rejected() {
        let mut model = two_spring_model();
        model.nodes.push(Node { id: "n1".into(), pos: [5.0, 0.0, 0.0] });
        assert_eq!(solve_linear_static(&model), Err(FemError::DuplicateNodeId("n1".into())));
    }

    /// 🔍️ `Model`'s hand-rolled `Debug` (trait objects aren't `Debug`) must print element ids, not panic.
    #[test]
    fn model_debug_fmt_prints_element_ids_not_trait_objects() {
        let model = two_spring_model();
        let printed = format!("{model:?}");
        assert!(printed.contains("e1"), "expected element id \"e1\" in {printed}");
        assert!(printed.contains("n1") && printed.contains("n2"), "expected node ids in {printed}");
    }

    /// 🌬️ A member UDL on an element that doesn't override `equivalent_nodal_loads` (the trait default,
    /// `None`) is silently a no-op — same displacement as solving with no member load at all.
    #[test]
    fn member_udl_on_element_without_udl_support_is_a_no_op() {
        let mut model = two_spring_model();
        model.member_loads.push(("e1".into(), MemberUdl { wx: 123.0, wy: 456.0, wz: 0.0 }));
        let with_udl = solve_linear_static(&model).expect("solves");
        let without_udl = solve_linear_static(&two_spring_model()).expect("solves");
        for (a, b) in with_udl.displacements.iter().zip(without_udl.displacements.iter()) {
            for k in 0..6 {
                assert!((a.values[k] - b.values[k]).abs() < 1e-12, "dof {k}: {} vs {}", a.values[k], b.values[k]);
            }
        }
    }
}
// #endregion 🔖️Tests
