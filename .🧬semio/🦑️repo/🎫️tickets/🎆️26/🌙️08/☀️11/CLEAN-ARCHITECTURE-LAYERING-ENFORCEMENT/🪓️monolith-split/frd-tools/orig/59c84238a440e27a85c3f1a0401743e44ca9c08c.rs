#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = runtime_fixture_text("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::ShootingInference;
    use crate::ShootingSnapshot;
    use protocol::Inference;

    let snapshot = ShootingSnapshot::default();
    assert_eq!(ShootingInference::infer(&snapshot).expect("valid materialized inference fixture"), ShootingInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::ShootingInference;
    use crate::ShootingSnapshot;
    use protocol::Inference;

    assert_eq!(ShootingInference::infer(&ShootingSnapshot::default()).expect("valid materialized inference fixture"), ShootingInference::default());
}
//#endregion 🧪️InferenceLaws

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
