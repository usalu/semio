#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::EquationInference;
    use crate::EquationSnapshot;
    use protocol::Inference;

    let snapshot = EquationSnapshot::default();
    let inference = EquationInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, EquationInference::infer(&snapshot).expect("valid materialized inference fixture"));
    assert_eq!(inference.topology.node_count, snapshot.graph.nodes.len() as u32);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::EquationInference;
    use crate::EquationSnapshot;
    use protocol::Inference;

    assert_eq!(EquationInference::infer(&EquationSnapshot::default()).expect("valid materialized inference fixture"), EquationInference::default());
}
//#endregion 💡️InferenceLaws
