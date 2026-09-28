#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::forms_steps;
    use protocol::Inference;

    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = crate::document_dsl::parse_dsl(text).expect("demo asset parses as a forms snapshot");
    let inference = crate::standards::v1::subsets::any::schema::inferences::FormsInference::infer(&snapshot);
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::FormsInference::infer(&snapshot));

    let expected_nodes: u32 = forms_steps(&snapshot).iter().map(|step| 1 + step.blocks.len() as u32).sum();
    assert_eq!(inference.topology.node_count, expected_nodes);
    assert_eq!(inference.topology.topo_order.len() as u32, expected_nodes);
    assert!(inference.topology.cycle_free, "the demo document has no cyclic block conditions");
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::FormsInference;
    use crate::FormsSnapshot;
    use protocol::Inference;

    assert_eq!(FormsInference::infer(&FormsSnapshot::default()), FormsInference::default());
}
//#endregion 💡️InferenceLaws
