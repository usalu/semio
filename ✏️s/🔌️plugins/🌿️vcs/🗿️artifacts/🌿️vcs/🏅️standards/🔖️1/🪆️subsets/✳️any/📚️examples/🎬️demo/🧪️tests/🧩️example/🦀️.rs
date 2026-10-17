#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = runtime_fixture_text("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = runtime_fixture_text("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::VcsSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::VcsInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::VcsInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::standards::v1::subsets::any::schema::inferences::VcsInference::infer(&crate::VcsSnapshot::default()).expect("valid materialized inference fixture"),
        crate::standards::v1::subsets::any::schema::inferences::VcsInference::default(),
    );
}

#[semio_framework_async_macros::async_test]
async fn summary_counts_the_demo_fixtures_tags_and_notes() {
    use protocol::Inference;
    let text = runtime_fixture_text("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::VcsSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::VcsInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference.summary.tag_count, 2);
    assert!(inference.summary.has_notes);
}
//#endregion 🧪️InferenceLaws

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
