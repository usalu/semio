//! 🔬️ Updated for hierarchical EN 1999 subject.

use crate::En1999Snapshot;
use crate::mutations::En1999Mutation;

#[test]
fn default_snapshot_has_members() {
    let s = En1999Snapshot::default();
    assert!(!s.members.is_empty());
    assert!(!s.materials.is_empty());
}

