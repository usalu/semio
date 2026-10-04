use super::*;
use crate::FemNode;
use protocol::Inference;

//#region 🧸️Fixtures
fn sample_snapshot() -> Fem3dSnapshot {
    Fem3dSnapshot { nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 4.0, y: 0.0, z: 0.0 }, FemNode { id: "n3".into(), x: 4.0, y: 3.0, z: 2.0 }], ..Default::default() }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = sample_snapshot();
    assert_eq!(Fem3dInference::infer(&snapshot).expect("valid materialized inference fixture"), Fem3dInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[test]
fn inference_default_law() {
    assert_eq!(Fem3dInference::infer(&Fem3dSnapshot::default()).expect("valid materialized inference fixture"), Fem3dInference::default());
}

#[test]
fn bounds_matches_node_extent() {
    let snapshot = sample_snapshot();
    let inferred = Fem3dInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inferred.bounds.node_count, 3);
    assert_eq!(inferred.bounds.bounding_box.min, [0.0, 0.0, 0.0]);
    assert_eq!(inferred.bounds.bounding_box.max, [4.0, 3.0, 2.0]);
}
//#endregion 🧪️InferenceLaws
