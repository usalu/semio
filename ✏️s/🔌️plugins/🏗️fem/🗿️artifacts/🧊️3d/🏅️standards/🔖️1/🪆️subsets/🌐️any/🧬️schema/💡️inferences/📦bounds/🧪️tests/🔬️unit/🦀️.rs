use super::*;

#[test]
fn inference_default_law() {
    let snapshot = Fem3dSnapshot::default();
    assert_eq!(compute_fem3d_bounds(&snapshot), Fem3dBounds::default());
    std::mem::forget(snapshot);
}
