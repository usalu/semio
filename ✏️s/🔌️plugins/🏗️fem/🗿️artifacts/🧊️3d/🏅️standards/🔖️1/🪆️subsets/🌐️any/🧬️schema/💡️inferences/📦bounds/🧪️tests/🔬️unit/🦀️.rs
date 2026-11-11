use super::*;

#[test]
fn inference_default_law() {
    assert_eq!(compute_fem3d_bounds(&Fem3dSnapshot::default()), Fem3dBounds::default());
}
