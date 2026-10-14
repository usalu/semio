use super::*;

#[test]
fn fem_initial_viewport_has_a_valid_explicit_pose() {
    assert!(INITIAL.validate().is_ok());
    assert_eq!(INITIAL.position, [24.0, -20.0, 14.0]);
    assert_eq!(INITIAL.target, [6.0, 3.0, 3.0]);
}
