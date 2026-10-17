use semio_framework_test_fixtures::fixture_path;

#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = std::fs::read_to_string(fixture_path!("../../🖼️assets/🗣️.dsl.semio")).expect("../../🖼️assets/🗣️.dsl.semio").leak();
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = std::fs::read_to_string(fixture_path!("../../🖼️assets/🗣️.dsl.semio")).expect("../../🖼️assets/🗣️.dsl.semio").leak();
    let snapshot = <semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("concrete forest fixture parses");
    let inference = semio_s_artifact_lowpoly_lowpoly_inferences::LowpolyInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, semio_s_artifact_lowpoly_lowpoly_inferences::LowpolyInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(semio_s_artifact_lowpoly_lowpoly_inferences::LowpolyInference::infer(&semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot::default()).expect("valid materialized inference fixture"), semio_s_artifact_lowpoly_lowpoly_inferences::LowpolyInference::default(),);
}
//#endregion 🧪️InferenceLaws
