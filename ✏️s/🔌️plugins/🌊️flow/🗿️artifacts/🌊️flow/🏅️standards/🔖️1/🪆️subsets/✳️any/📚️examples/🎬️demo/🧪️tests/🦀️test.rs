#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::flow::FlowSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::flow::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::flow::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::flow::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&crate::artifacts::flow::FlowSnapshot::default()),
        crate::artifacts::flow::standards::v1::subsets::any::schema::inferences::FlowInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
