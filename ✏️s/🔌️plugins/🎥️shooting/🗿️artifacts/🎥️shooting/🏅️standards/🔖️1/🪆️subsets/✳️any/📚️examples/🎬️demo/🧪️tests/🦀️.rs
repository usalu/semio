#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::shooting::standards::v1::subsets::any::schema::inferences::ShootingInference;
    use crate::artifacts::shooting::ShootingSnapshot;
    use protocol::Inference;

    let snapshot = ShootingSnapshot::default();
    assert_eq!(ShootingInference::infer(&snapshot), ShootingInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::shooting::standards::v1::subsets::any::schema::inferences::ShootingInference;
    use crate::artifacts::shooting::ShootingSnapshot;
    use protocol::Inference;

    assert_eq!(ShootingInference::infer(&ShootingSnapshot::default()), ShootingInference::default());
}
//#endregion 🧪️InferenceLaws
