use crate::EquationSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::*;
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = EquationSnapshot::default();
    assert_eq!(EquationInference::infer(&snapshot).expect("valid materialized inference fixture"), EquationInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(EquationInference::infer(&EquationSnapshot::default()).expect("valid materialized inference fixture"), EquationInference::default());
}

#[semio_framework_async_macros::async_test]
async fn default_graph_diamond_is_cycle_free_with_two_roots() {
    let inferred = EquationInference::infer(&EquationSnapshot::default()).expect("valid materialized inference fixture");
    assert!(inferred.topology.cycle_free);
    assert_eq!(inferred.topology.node_count, 4);
    assert_eq!(inferred.topology.depth["a"], 0);
    assert_eq!(inferred.topology.depth["d"], 2);
    let a_index = inferred.topology.topo_order.iter().position(|id| id == "a").unwrap();
    let d_index = inferred.topology.topo_order.iter().position(|id| id == "d").unwrap();
    assert!(a_index < d_index);
}

#[semio_framework_async_macros::async_test]
async fn default_equation_is_the_zero_polynomial_with_no_roots() {
    let inferred = EquationInference::infer(&EquationSnapshot::default()).expect("valid materialized inference fixture");
    assert!(inferred.roots.is_empty());
}
//#endregion 🧪️InferenceLaws
