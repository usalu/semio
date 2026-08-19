#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::layout::LayoutSnapshot;
    use crate::artifacts::layout::standards::v1::subsets::any::schema::inferences::LayoutInference;
    use protocol::Inference;

    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo asset parses as a layout snapshot");
    let inference = LayoutInference::infer(&snapshot);
    assert_eq!(inference, LayoutInference::infer(&snapshot));

    let expected_nodes = (snapshot.parent_pages.len() + snapshot.spreads.len() + snapshot.pages.len()) as u32;
    assert_eq!(inference.topology.node_count, expected_nodes);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::layout::standards::v1::subsets::any::schema::inferences::LayoutInference;

    assert_eq!(LayoutInference::default().topology.node_count, 0);
    assert!(LayoutInference::default().topology.cycle_free);
}
//#endregion 💡️InferenceLaws
