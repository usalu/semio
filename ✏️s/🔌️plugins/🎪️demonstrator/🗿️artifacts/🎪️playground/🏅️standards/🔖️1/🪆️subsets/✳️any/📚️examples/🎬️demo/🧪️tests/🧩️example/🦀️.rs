#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::PlaygroundInference;
    use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
    use protocol::Inference;

    let snapshot = PlaygroundSnapshot::default();
    assert_eq!(PlaygroundInference::infer(&snapshot).expect("valid materialized inference fixture"), PlaygroundInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[test]
fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::PlaygroundInference;
    use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
    use protocol::Inference;

    assert_eq!(PlaygroundInference::infer(&PlaygroundSnapshot::default()).expect("valid materialized inference fixture"), PlaygroundInference::default());
}
//#endregion 🧪️InferenceLaws
