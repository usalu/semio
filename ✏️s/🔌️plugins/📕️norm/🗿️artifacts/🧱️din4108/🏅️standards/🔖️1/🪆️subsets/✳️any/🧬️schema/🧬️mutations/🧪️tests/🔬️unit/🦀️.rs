//! Mutation unit smoke — vocabulary kinds.

use crate::standards::v1::subsets::any::schema::mutations::{KINDS,Din4108Mutation};

use crate::Din4108Snapshot;

#[semio_framework_async_macros::async_test]
async fn kinds_are_unique() {
    let mut seen = std::collections::BTreeSet::new();
    for kind in KINDS {
        assert!(seen.insert(*kind), "duplicate {kind}");
    }
    assert_eq!(seen.len(), KINDS.len());
}

