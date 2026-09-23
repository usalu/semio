use super::*;

#[test]
fn transform_utility_id_and_options() {
    assert_eq!(UTILITY_ID, "transform");
    assert_eq!(definition().id, UTILITY_ID);
    let options = options();
    assert!(options.move_axes && options.rotate && options.scale_axes && options.scale_uniform);
}
