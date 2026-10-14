#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_layout_layout_model::inferences::LayoutInference;
    use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
    use protocol::Inference;

    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <LayoutSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo asset parses as a layout snapshot");
    let inference = LayoutInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, LayoutInference::infer(&snapshot).expect("valid materialized inference fixture"));

    let expected_nodes = (snapshot.parent_pages.len() + snapshot.spreads.len() + snapshot.pages.len()) as u32;
    assert_eq!(inference.topology.node_count, expected_nodes);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_layout_layout_model::inferences::LayoutInference;

    assert_eq!(LayoutInference::default().topology.node_count, 0);
    assert!(LayoutInference::default().topology.cycle_free);
}
//#endregion 💡️InferenceLaws
