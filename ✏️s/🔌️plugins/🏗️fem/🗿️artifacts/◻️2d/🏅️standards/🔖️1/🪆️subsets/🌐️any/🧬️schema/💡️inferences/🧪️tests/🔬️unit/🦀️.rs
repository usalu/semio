
use super::*;
use crate::FemNode;
use protocol::Inference;

//#region 🧸️Fixtures
fn sample_snapshot() -> Fem2dSnapshot {
    Fem2dSnapshot { nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 4.0, y: 0.0 }, FemNode { id: "n3".into(), x: 4.0, y: 3.0 }], ..Default::default() }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = sample_snapshot();
    assert_eq!(Fem2dInference::infer(&snapshot), Fem2dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(Fem2dInference::infer(&Fem2dSnapshot::default()), Fem2dInference::default());
}

#[test]
fn bounds_matches_node_extent() {
    let snapshot = sample_snapshot();
    let inferred = Fem2dInference::infer(&snapshot);
    assert_eq!(inferred.bounds.node_count, 3);
    assert_eq!(inferred.bounds.bounding_box.min, [0.0, 0.0]);
    assert_eq!(inferred.bounds.bounding_box.max, [4.0, 3.0]);
}
//#endregion 🧪️InferenceLaws
