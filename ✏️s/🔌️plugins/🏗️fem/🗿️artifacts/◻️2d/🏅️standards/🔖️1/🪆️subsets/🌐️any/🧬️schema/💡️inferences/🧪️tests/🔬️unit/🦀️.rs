use super::*;
use protocol::Inference;

#[test]
fn inference_default_law() {
    assert_eq!(Fem2dInference::infer(&Fem2dSnapshot::default()).expect("valid materialized inference fixture"), Fem2dInference::default());
}
