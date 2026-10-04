#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    let snapshot = <crate::LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text).expect("concrete forest fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::LowpolyInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::LowpolyInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::LowpolyInference::infer(&crate::LowpolySnapshot::default()).expect("valid materialized inference fixture"), crate::standards::v1::subsets::any::schema::inferences::LowpolyInference::default(),);
}
//#endregion 🧪️InferenceLaws
