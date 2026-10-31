use super::*;

#[test]
fn inference_default_law() {
    assert_eq!(compute_fem2d_bounds(&Fem2dSnapshot::default()), Fem2dBounds::default());
}
