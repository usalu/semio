
use super::*;

/// ⚖️ LAW: an empty diff is a no-operation on the snapshot.
#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = crate::empty_shooting_snapshot();
    let diff = ShootingDiff::default();
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
}
