use super::*;
use semio_s_fem_analyses_assembly::AssemblyJobConstruction;
use semio_s_fem_analyses_model::MOUNTED_OWNER_PAGE_BYTES;
use semio_s_fem_analyses_model::reserve_exact_owner_page;
use semio_s_fem_elements2d::Bar2;
use semio_s_fem_elements2d::BeamEb2;
use semio_s_fem_test_support::close_job;
use semio_s_fem_test_support::pages_bytes;
use semio_s_fem_test_support::seen_of;
use semio_s_fem_test_support::Seen;
use semio_s_fem_test_support::TEST_GRANT;
use semio_s_fem_static::solve_linear_static;
use semio_s_fem_static::Model;
use semio_s_fem_model::PlaneStress;
use semio_s_fem_model::ShellState;
use semio_s_fem_model::SolidStress;

fn cantilever_analysis_model(e: f64, area: f64, iy: f64, l: f64, density: f64) -> (AnalysisModel, Vec<LoadCase>) {
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    };
    let cases = vec![LoadCase { id: "tip_load".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -1000.0 }], member_loads: vec![], self_weight: false }];
    (model, cases)
}

include!("🥓️static-solve-and-rejection-laws/🦀️.rs");

include!("🥩️member-load-scalar-average-and-job-graph-laws/🦀️.rs");

/// 🧮️ The physical solver retains both same-named sources and exact superposition.
#[test]
fn result_source_namespaces_preserve_same_named_case_and_combination() {
    use semio_s_fem_analyses_model::{AnalysisModel, Combination, LoadCase, ResultSource};
    use semio_s_fem_model::{Dof, NodalLoad, Node, Support};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🏷️result-source/🧪️tests/🧫️fixtures/🔣️.json")).expect("independent fixture parser");
    let beam = &fixture["cantilever"];
    let scalar = |key: &str| beam[key].as_f64().expect("scalar");
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [scalar("length"), 0.0, 0.0] }],
        elements: vec![semio_s_fem_elements2d::BeamEb2 { id: "e".into(), start: "a".into(), end: "b".into(), e: scalar("e"), area: scalar("area"), iy: scalar("iy"), density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    };
    let case = LoadCase { id: "same".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: scalar("force") }], member_loads: vec![], self_weight: false };
    let combination = Combination { id: "same".into(), terms: vec![("same".into(), scalar("factor"))] };
    let results = solve_multi_case(&model, &[case], &[combination], [0.0; 3]).expect("solve");
    assert_eq!(results.len(), fixture["expected"]["count"].as_u64().expect("count") as usize);
    for (source, expected) in [(ResultSource::LoadCase("same".into()), "loadCaseTy"), (ResultSource::Combination("same".into()), "combinationTy")] {
        let result = results.get(&source).expect("typed result");
        let actual = result.displacements.iter().find(|row| row.node_id == "b").expect("tip").values[Dof::Ty.index()];
        assert!((actual - fixture["expected"][expected].as_f64().expect("oracle")).abs() < 1e-12);
    }
    println!("[DEBUG] FEM physical solver returns two independent same-named typed result sources and their exact scaled displacement");
}
