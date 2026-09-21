/// ⚖️ LAW: the committed asset IS the curated demo — the play pane loads the asset itself, so an
/// asset that parses to an empty board renders an empty canvas no matter what `setActiveExample`
/// would have built in code. It carries the seven metabolism topics and the nine relationships
/// between them.
#[semio_framework_async_macros::async_test]
async fn primary_asset_carries_the_whole_metabolism_graph() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::WiresSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let board = crate::wires_working_board(&snapshot);
    let nodes = crate::standards::v1::subsets::any::schema::fixture_nodes(&board);
    assert_eq!(nodes.len(), 7, "the demo board carries every metabolism topic");
    assert_eq!(crate::standards::v1::subsets::any::schema::fixture_edges(&board).len(), 9, "the demo board carries every relationship between those topics");
    assert_eq!(nodes[0].get("text").and_then(|value| value.as_str()), Some("Metabolism"), "the first topic names the example");
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::WiresSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&snapshot);
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&crate::empty_wires_snapshot()), crate::standards::v1::subsets::any::schema::inferences::WiresInference::default(),);
}
//#endregion 🧪️InferenceLaws

