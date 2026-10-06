//! EN 1999 mutation binary unit smoke.

use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

#[test]
fn from_snapshot_emits_seven_bulk_mutations() {
    let empty = En1999Snapshot::empty();
    let target = En1999Snapshot::compliant_roof_purlin();
    let raised = En1999Mutation::from_snapshot(&empty, &target);
    assert!(raised.len() >= 5, "expected hierarchical replace mutations, got {}", raised.len());
}
