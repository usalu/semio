
use super::*;

#[semio_framework_async_macros::async_test]
async fn standard_mounts_exactly_one_subset() {
    assert_eq!(standard().subsets.len(), 1);
}
