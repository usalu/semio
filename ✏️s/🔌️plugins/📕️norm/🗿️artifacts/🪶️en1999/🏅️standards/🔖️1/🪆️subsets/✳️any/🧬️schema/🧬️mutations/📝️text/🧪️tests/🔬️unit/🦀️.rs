//! 🔬️ Updated for hierarchical EN 1999 subject.

use crate::En1999Snapshot;
use crate::mutations::En1999Mutation;

#[test]
fn default_snapshot_has_members() {
    let s = En1999Snapshot::default();
    assert!(!s.members.is_empty());
    assert!(!s.materials.is_empty());
}

#[test]
fn from_snapshot_bulk_replace_count() {
    let empty = En1999Snapshot::empty();
    let target = En1999Snapshot::compliant_roof_purlin();
    let raised = En1999Mutation::from_snapshot(&empty, &target);
    assert!(raised.len() >= 5, "expected hierarchical replace mutations, got {}", raised.len());
    let noop = En1999Mutation::from_snapshot(&target, &target);
    assert!(noop.is_empty());
}
