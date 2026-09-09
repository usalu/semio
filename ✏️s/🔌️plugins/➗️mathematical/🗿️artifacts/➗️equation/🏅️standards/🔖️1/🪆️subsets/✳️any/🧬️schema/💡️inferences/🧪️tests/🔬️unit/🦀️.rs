use super::*;
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = EquationSnapshot::default();
    assert_eq!(EquationInference::infer(&snapshot), EquationInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(EquationInference::infer(&EquationSnapshot::default()), EquationInference::default());
}

#[semio_framework_async_macros::async_test]
async fn default_graph_diamond_is_cycle_free_with_two_roots() {
    // 🔷 The default graph is a diamond: a->b, a->c, b->d, c->d — acyclic, `a` is the only root.
    let inferred = EquationInference::infer(&EquationSnapshot::default());
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
    // 🔎️ `EquationExprSnapshot::default()` is the integer literal `0` — the zero polynomial has no
    // isolated real roots, an empty `Vec`, never a panic (see `🌱roots`'s own scope tests).
    let inferred = EquationInference::infer(&EquationSnapshot::default());
    assert!(inferred.roots.is_empty());
}
//#endregion 🧪️InferenceLaws
