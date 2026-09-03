#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::equation::standards::v1::subsets::any::schema::inferences::EquationInference;
    use crate::artifacts::equation::EquationSnapshot;
    use protocol::Inference;

    let snapshot = EquationSnapshot::default();
    let inference = EquationInference::infer(&snapshot);
    assert_eq!(inference, EquationInference::infer(&snapshot));
    assert_eq!(inference.topology.node_count, crate::artifacts::equation::equation_graph(&snapshot).nodes.len() as u32);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::equation::standards::v1::subsets::any::schema::inferences::EquationInference;
    use crate::artifacts::equation::EquationSnapshot;
    use protocol::Inference;

    assert_eq!(EquationInference::infer(&EquationSnapshot::default()), EquationInference::default());
}
//#endregion 💡️InferenceLaws
