//! Mutation unit smoke — vocabulary kinds + from_snapshot round-trip.

use crate::standards::v1::subsets::any::schema::mutations::{KINDS, Din4108Mutation};
use crate::Din4108Snapshot;

#[semio_framework_async_macros::async_test]
async fn kinds_are_unique() {
    let mut seen = std::collections::BTreeSet::new();
    for kind in KINDS {
        assert!(seen.insert(*kind), "duplicate {kind}");
    }
    assert_eq!(seen.len(), KINDS.len());
}

#[semio_framework_async_macros::async_test]
async fn from_snapshot_no_ops_on_identical() {
    let snap = Din4108Snapshot::default();
    let ops = Din4108Mutation::from_snapshot(&snap, &snap);
    assert!(ops.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn from_snapshot_emits_usage_change() {
    let base = Din4108Snapshot::default();
    let mut target = base.clone();
    target.usage = "nonResidential".into();
    let ops = Din4108Mutation::from_snapshot(&base, &target);
    assert!(ops.iter().any(|m| matches!(m, Din4108Mutation::ChangeUsage(_))));
}
