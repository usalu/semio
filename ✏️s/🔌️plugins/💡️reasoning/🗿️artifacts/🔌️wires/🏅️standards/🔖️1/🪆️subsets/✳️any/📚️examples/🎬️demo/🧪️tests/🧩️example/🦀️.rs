/// ⚖️ LAW: the committed assets ARE the curated demo — the play pane loads them itself (`setActiveExample` composes the board
/// child from the bundled content the parent names). The parent carries the seven metabolism identities, the board child the
/// seven topics and the nine relationships between them.
#[semio_framework_async_macros::async_test]
async fn the_assets_carry_the_whole_metabolism_graph() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::WiresSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo parent parses");
    let pack = crate::genesis_wires_child_pack(&snapshot, crate::WIRES_CONTENT_SLOT, &snapshot.content.child_id).expect("the demo names its bundled board");
    let content = <crate::SemioGraphSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("board pack");
    let composed = crate::wires_composed(&snapshot, &content);
    let nodes = crate::schema::fixture_nodes(&composed.board);
    assert_eq!(nodes.len(), 7, "the demo board carries every metabolism topic");
    assert_eq!(crate::schema::fixture_edges(&composed.board).len(), 9, "the demo board carries every relationship between those topics");
    assert_eq!(crate::schema::wires_identities(&composed.fixture).len(), 7);
    assert_eq!(nodes[0].get("text").and_then(|value| value.as_str()), Some("Metabolism"), "the first topic names the example");
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::WiresSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&snapshot).expect("parent-only inference");
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&snapshot).expect("parent-only inference"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&crate::empty_wires_snapshot()).expect("parent-only inference"), crate::standards::v1::subsets::any::schema::inferences::WiresInference::default());
}
//#endregion 🧪️InferenceLaws
