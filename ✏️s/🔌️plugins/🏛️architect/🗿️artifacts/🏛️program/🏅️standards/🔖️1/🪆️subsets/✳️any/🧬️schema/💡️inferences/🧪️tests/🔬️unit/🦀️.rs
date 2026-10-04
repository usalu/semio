use super::*;

//#region 🧪️InferenceLaws
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
