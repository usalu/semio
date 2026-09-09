use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = crate::standards::v1::subsets::any::schema::empty_vcs_snapshot();
    let diff = VcsDiff::default();
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
}
