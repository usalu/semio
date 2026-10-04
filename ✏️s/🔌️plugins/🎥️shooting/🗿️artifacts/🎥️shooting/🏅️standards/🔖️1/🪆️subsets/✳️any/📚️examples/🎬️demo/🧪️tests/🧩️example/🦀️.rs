#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
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
