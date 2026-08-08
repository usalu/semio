//! 🎵️ FEM 2D artifact engine — modal/buckling analysis bridge (was the old engine crate's
//! `ModalBuckling` region).

use crate::artifacts::fem2d::engine::meshing::build_nodes_and_elements;
use crate::artifacts::fem2d::engine::Fem2dError;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemLoad};
use crate::model::{Dof, Element, MemberUdl, Node, Support};
use std::collections::HashMap;

/// 🔢️ Node-major, active-DOF-filtered ordering matching `crate::analyses::ModalResult`/
/// `BucklingResult`'s documented shape-vector layout — a small local reimplementation (mirrors
/// `analyses::build_dof_map`, which isn't `pub`, following the same precedent that module's own doc
/// comment sets for `lib.rs`'s private `build_dof_map`) used to unpack a raw mode-shape `VecD` back
/// into per-node `[f64;6]` values.
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
pub fn fem2d_modal(doc: &Fem2dSnapshot) -> Result<crate::analyses::ModalResult, Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    crate::analyses::modal(&model, doc.analysis.modal_count).map_err(Fem2dError::from)
}

/// 🌉️ Richer modal entry point: solves the same modal analysis as `fem2d_modal` but also unpacks mode
/// `mode_index`'s shape `VecD` into a friendly per-node `[f64;6]` displacement map (see `mode_dof_order`),
/// ready to feed the same deformed-shape rendering the results window already uses for static results.
/// Returns `(frequency_hz, node_id -> displacement values)`.
pub fn fem2d_modal_mode_values(doc: &Fem2dSnapshot, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    let result = crate::analyses::modal(&model, doc.analysis.modal_count)?;
    let freq = *result.frequencies_hz.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((freq, values))
}

/// 🧩️ `buckling_inputs`'s resolved `(nodes, elements, supports, load case)` quadruple.
type BucklingInputs = (Vec<Node>, Vec<Box<dyn Element>>, Vec<Support>, crate::analyses::LoadCase);

/// 🌉️ Shared buckling-case resolution for `fem2d_buckling`/`fem2d_buckling_mode_values`: builds the
/// geometry plus the ONE named `case_id`'s `analyses::LoadCase`, mirroring `fem2d_solve_all`'s
/// per-case load translation (nodal/member-UDL/area loads), erroring `"load case not found: {case_id}"`
/// if `case_id` isn't in `doc.load_cases`.
fn buckling_inputs(doc: &Fem2dSnapshot, case_id: &str) -> Result<BucklingInputs, Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let member_node_ids: std::collections::HashSet<String> = doc.nodes.iter().map(|n| n.id.clone()).collect();
    let nodes: Vec<Node> = nodes.into_iter().filter(|n| member_node_ids.contains(&n.id)).collect();
    let elements: Vec<Box<dyn Element>> = elements.into_iter().take(doc.elements.len()).collect();
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let load_case = doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;

    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in &load_case.loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(crate::model::NodalLoad { node_id: node_id.clone(), dof: (*dof).into(), value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, .. } => member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 })),
            FemLoad::Area { .. } => {}
        }
    }
    let case = crate::analyses::LoadCase { id: load_case.id.clone(), nodal_loads, member_loads, self_weight: load_case.self_weight };
    Ok((nodes, elements, supports, case))
}

/// 🏛️ Linear buckling: lowest `doc.analysis.buckling_count` load factors/mode shapes for `case_id`.
pub fn fem2d_buckling(doc: &Fem2dSnapshot, case_id: &str) -> Result<crate::analyses::BucklingResult, Fem2dError> {
    let (nodes, elements, supports, case) = buckling_inputs(doc, case_id)?;
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    crate::analyses::buckling(&model, &case, doc.analysis.buckling_count).map_err(Fem2dError::from)
}

/// 🌉️ Richer buckling entry point: mirrors `fem2d_modal_mode_values` — solves the same buckling
/// analysis as `fem2d_buckling` but also unpacks mode `mode_index`'s shape into a per-node
/// displacement map. Returns `(load_factor, node_id -> displacement values)`.
pub fn fem2d_buckling_mode_values(doc: &Fem2dSnapshot, case_id: &str, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem2dError> {
    let (nodes, elements, supports, case) = buckling_inputs(doc, case_id)?;
    let order = mode_dof_order(&nodes, &elements);
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    let result = crate::analyses::buckling(&model, &case, doc.analysis.buckling_count)?;
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
mod tests {
    use super::*;
    use crate::artifacts::fem2d::{FemAnalysisSettings, FemDof, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};

    // #region 🔖️Fixtures
    fn simply_supported_beam_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![crate::artifacts::fem2d::FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    /// 🟩️ A 4x2m rectangular region (steel, 0.02m thick, 1m mesh) whose 4 corners are pre-placed as
    /// document nodes.
    fn rectangle_region_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    #[test]
    fn fem2d_modal_returns_requested_mode_count() {
        let doc = rectangle_region_doc();
        let result = fem2d_modal(&doc).expect("modal solves");
        assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
        for w in result.frequencies_hz.windows(2) {
            assert!(w[0] <= w[1], "frequencies should be ascending: {:?}", result.frequencies_hz);
        }
        for &f in &result.frequencies_hz {
            assert!(f.is_finite() && f >= 0.0, "frequency should be finite and non-negative: {f}");
        }
    }

    #[test]
    fn fem2d_modal_mode_values_returns_node_displacements() {
        let doc = simply_supported_beam_doc();
        let (freq, values) = fem2d_modal_mode_values(&doc, 0).expect("modal mode values solves");
        assert!(freq.is_finite() && freq >= 0.0);
        assert!(values.contains_key("n1"));
        assert!(values.contains_key("n2"));
    }

    #[test]
    fn fem2d_buckling_returns_requested_mode_count() {
        let doc = simply_supported_beam_doc();
        let result = fem2d_buckling(&doc, "dead").expect("buckling solves");
        assert_eq!(result.factors.len(), doc.analysis.buckling_count);
        for &f in &result.factors {
            assert!(f.is_finite(), "buckling factor should be finite: {f}");
        }
    }

    #[test]
    fn fem2d_buckling_mode_values_returns_node_displacements() {
        let doc = simply_supported_beam_doc();
        let (factor, values) = fem2d_buckling_mode_values(&doc, "dead", 0).expect("buckling mode values solves");
        assert!(factor.is_finite());
        assert!(values.contains_key("n1"));
        assert!(values.contains_key("n2"));
    }

    #[test]
    fn fem2d_buckling_unknown_case_errors() {
        let doc = simply_supported_beam_doc();
        let err = fem2d_buckling(&doc, "missing").err().expect("expected error");
        assert!(err.to_string().contains("load case not found"), "unexpected error: {err}");
    }
}
// #endregion 🧪️Tests
