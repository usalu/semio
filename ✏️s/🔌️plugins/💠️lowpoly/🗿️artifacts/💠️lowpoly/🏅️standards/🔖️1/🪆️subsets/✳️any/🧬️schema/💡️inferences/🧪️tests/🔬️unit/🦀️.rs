use super::*;
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = crate::snapshot_from_mesh_json("{}", "o1", "Object 1");
    assert_eq!(LowpolyInference::infer(&snapshot).expect("valid materialized inference fixture"), LowpolyInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(LowpolyInference::infer(&LowpolySnapshot::default()).expect("valid materialized inference fixture"), LowpolyInference::default());
}
//#endregion 🧪️InferenceLaws
