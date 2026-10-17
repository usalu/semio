#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = runtime_fixture_text("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use crate::standards::v1::subsets::any::schema::inferences::ProgramInference;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = ProgramSnapshot::default();
    assert_eq!(ProgramInference::infer(&snapshot).expect("valid materialized inference fixture"), ProgramInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(ProgramInference::infer(&ProgramSnapshot::default()).expect("valid materialized inference fixture"), ProgramInference::default());
}
//#endregion 🧪️InferenceLaws

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
