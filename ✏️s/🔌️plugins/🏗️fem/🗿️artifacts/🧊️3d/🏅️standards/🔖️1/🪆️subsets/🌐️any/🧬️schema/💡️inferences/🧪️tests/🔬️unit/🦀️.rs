use super::*;
use protocol::Inference;

#[test]
fn inference_default_law() {
    assert_eq!(Fem3dInference::infer(&Fem3dSnapshot::default()).expect("valid materialized inference fixture"), Fem3dInference::default());
}
