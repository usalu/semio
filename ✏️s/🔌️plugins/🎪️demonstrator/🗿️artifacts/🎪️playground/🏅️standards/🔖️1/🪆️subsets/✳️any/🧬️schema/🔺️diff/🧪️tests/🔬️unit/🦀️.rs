use crate::standards::v1::subsets::any::schema::diff::*;

#[test]
fn empty_diff_is_a_no_operation() {
    let base = crate::standards::v1::subsets::any::schema::empty_playground_snapshot();
    let diff = PlaygroundDiff::default();
    assert_eq!(protocol::apply_diff_cold(&diff, &base).expect("valid mutation diff"), base);
}
