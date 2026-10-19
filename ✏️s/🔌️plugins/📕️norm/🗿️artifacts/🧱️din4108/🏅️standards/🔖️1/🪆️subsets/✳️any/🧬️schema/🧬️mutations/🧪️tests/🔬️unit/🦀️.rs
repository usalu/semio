//! 🧬️ Mutation unit smoke — vocabulary kinds.

use crate::{KINDS, Din4108Mutation};

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[semio_framework_async_macros::async_test]
async fn kinds_are_unique() {
    let mut seen = std::collections::BTreeSet::new();
    for kind in KINDS {
        assert!(seen.insert(*kind), "duplicate {kind}");
    }
    assert_eq!(seen.len(), KINDS.len());
}

