#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::mathematical::MathematicalSnapshot;
    use crate::artifacts::mathematical::standards::v1::subsets::any::schema::inferences::MathematicalInference;
    use protocol::Inference;

    let snapshot = MathematicalSnapshot::default();
    let inference = MathematicalInference::infer(&snapshot);
    assert_eq!(inference, MathematicalInference::infer(&snapshot));
    assert_eq!(inference.topology.node_count, crate::artifacts::mathematical::mathematical_graph(&snapshot).nodes.len() as u32);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::mathematical::MathematicalSnapshot;
    use crate::artifacts::mathematical::standards::v1::subsets::any::schema::inferences::MathematicalInference;
    use protocol::Inference;

    assert_eq!(MathematicalInference::infer(&MathematicalSnapshot::default()), MathematicalInference::default());
}
//#endregion 💡️InferenceLaws
