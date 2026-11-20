use super::*;
use protocol::Inference;

#[test]
fn inference_default_law() {
    let snapshot = Fem3dSnapshot::default();
    assert_eq!(Fem3dInference::infer(&snapshot).expect("valid materialized inference fixture"), Fem3dInference::default());
    std::mem::forget(snapshot);
}
