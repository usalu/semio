#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use crate::standards::v1::subsets::any::schema::inferences::ProgramInference;
use crate::ProgramSnapshot;
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
