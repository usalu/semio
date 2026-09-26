use crate::standards::v1::subsets::any::schema::mutations::KINDS;

#[semio_framework_async_macros::async_test]
async fn kinds_are_unique_and_non_empty() {
    assert!(!KINDS.is_empty());
    let mut seen = std::collections::BTreeSet::new();
    for kind in KINDS {
        assert!(seen.insert(*kind), "duplicate {kind}");
    }
}
