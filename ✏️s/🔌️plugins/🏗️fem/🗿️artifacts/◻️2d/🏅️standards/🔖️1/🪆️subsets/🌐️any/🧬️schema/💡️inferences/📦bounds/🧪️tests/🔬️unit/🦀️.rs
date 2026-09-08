
use super::*;
use crate::{FemElement, FemNode};

//#region 🧸️Fixtures
fn sample_snapshot() -> Fem2dSnapshot {
    Fem2dSnapshot {
        nodes: vec![FemNode { id: "n1".into(), x: -2.0, y: 1.0 }, FemNode { id: "n2".into(), x: 5.0, y: 1.0 }, FemNode { id: "n3".into(), x: 5.0, y: 7.5 }],
        elements: vec![FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }],
        ..Default::default()
    }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = sample_snapshot();
    assert_eq!(compute_fem2d_bounds(&snapshot), compute_fem2d_bounds(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(compute_fem2d_bounds(&Fem2dSnapshot::default()), Fem2dBounds::default());
}

#[test]
fn bounds_matches_hand_built_node_extent() {
    let bounds = compute_fem2d_bounds(&sample_snapshot());
    assert_eq!(bounds.bounding_box.min, [-2.0, 1.0]);
    assert_eq!(bounds.bounding_box.max, [5.0, 7.5]);
    assert_eq!(bounds.node_count, 3);
    assert_eq!(bounds.element_count, 1);
}
//#endregion 🧪️InferenceLaws
