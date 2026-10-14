//! 🔬️ Updated for hierarchical EN 1999 subject.

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::En1999Mutation;

#[test]
fn default_snapshot_has_members() {
    let s = En1999Snapshot::default();
    assert!(!s.members.is_empty());
    assert!(!s.materials.is_empty());
}

