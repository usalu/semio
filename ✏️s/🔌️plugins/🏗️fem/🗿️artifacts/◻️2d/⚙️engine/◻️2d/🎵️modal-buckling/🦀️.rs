//! 🌳️ artifact tree).
//! 🎵️ FEM 2D module engine — modal/buckling analysis bridge (pure FE algorithm, moved out of the

use crate::fem2d_engine::meshing::build_nodes_and_elements;

use crate::fem2d_engine::meshing::translate_loads;

use crate::fem2d_engine::meshing::GRAVITY_G;
use crate::fem2d_engine::Fem2dError;
use semio_s_fem_model::Dof;
use semio_s_fem_model::Element;
use semio_s_fem_static::Elements;
use semio_s_fem_model::Node;
use semio_s_fem_model::Support;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use std::collections::HashMap;

/// 🔢️ Node-major, active-DOF-filtered ordering matching `semio_s_fem_analyses_model::ModalResult`/
/// `BucklingResult`'s documented shape-vector layout — a small local reimplementation (mirrors
/// `analyses::build_dof_map`, which isn't `pub`, following the same precedent that module's own doc
/// comment sets for `lib.rs`'s private `build_dof_map`) used to unpack a raw mode-shape `VecD` back
/// into per-node `[f64;6]` values.
fn mode_dof_order(nodes: &[Node], elements: &[Elements]) -> Vec<(String, Dof)> {
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
pub fn fem2d_modal(doc: &Fem2dSnapshot) -> Result<semio_s_fem_analyses_model::ModalResult, Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = semio_s_fem_analyses_model::AnalysisModel { nodes, elements, supports };
    semio_s_fem_analyses::modal(&model, doc.analysis.modal_count).map_err(Fem2dError::from)
}

/// 🌉️ Richer modal entry point: solves the same modal analysis as `fem2d_modal` but also unpacks mode
/// `mode_index`'s shape `VecD` into a friendly per-node `[f64;6]` displacement map (see `mode_dof_order`),
/// ready to feed the same deformed-shape rendering the results window already uses for static results.
/// Returns `(frequency_hz, node_id -> displacement values)`.
pub fn fem2d_modal_mode_values(doc: &Fem2dSnapshot, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = semio_s_fem_analyses_model::AnalysisModel { nodes, elements, supports };
    let result = semio_s_fem_analyses::modal(&model, doc.analysis.modal_count)?;
    let freq = *result.frequencies_hz.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((freq, values))
}

/// 🧩️ `buckling_inputs`'s resolved `(nodes, elements, supports, load case)` quadruple.
type BucklingInputs = (Vec<Node>, Vec<Elements>, Vec<Support>, semio_s_fem_analyses_model::LoadCase);

/// 🌉️ Shared buckling-case resolution for `fem2d_buckling`/`fem2d_buckling_mode_values`: builds the
/// geometry plus the ONE named `case_id`'s `analyses::LoadCase`, mirroring `fem2d_solve_all`'s
/// per-case load translation (nodal/member-UDL/area loads), erroring `"load case not found: {case_id}"`
/// if `case_id` isn't in `doc.load_cases`.
fn buckling_inputs(doc: &Fem2dSnapshot, case_id: &str) -> Result<BucklingInputs, Fem2dError> {
    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let load_case = doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;

    let (nodal_loads, member_loads) = translate_loads(&load_case.loads, &regions)?;
    let case = semio_s_fem_analyses_model::LoadCase { id: load_case.id.clone(), nodal_loads, member_loads, self_weight: load_case.self_weight };
    Ok((nodes, elements, supports, case))
}

/// 🏛️ Linear buckling: lowest `doc.analysis.buckling_count` load factors/mode shapes for `case_id`.
pub fn fem2d_buckling(doc: &Fem2dSnapshot, case_id: &str) -> Result<semio_s_fem_analyses_model::BucklingResult, Fem2dError> {
    let (nodes, elements, supports, case) = buckling_inputs(doc, case_id)?;
    let model = semio_s_fem_analyses_model::AnalysisModel { nodes, elements, supports };
    semio_s_fem_analyses::buckling(&model, &case, doc.analysis.buckling_count, [0.0, -GRAVITY_G, 0.0]).map_err(Fem2dError::from)
}

/// 🌉️ Richer buckling entry point: mirrors `fem2d_modal_mode_values` — solves the same buckling
/// analysis as `fem2d_buckling` but also unpacks mode `mode_index`'s shape into a per-node
/// displacement map. Returns `(load_factor, node_id -> displacement values)`.
pub fn fem2d_buckling_mode_values(doc: &Fem2dSnapshot, case_id: &str, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem2dError> {
    let (nodes, elements, supports, case) = buckling_inputs(doc, case_id)?;
    let order = mode_dof_order(&nodes, &elements);
    let model = semio_s_fem_analyses_model::AnalysisModel { nodes, elements, supports };
    let result = semio_s_fem_analyses::buckling(&model, &case, doc.analysis.buckling_count, [0.0, -GRAVITY_G, 0.0])?;
    let factor = *result.factors.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((factor, values))
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
