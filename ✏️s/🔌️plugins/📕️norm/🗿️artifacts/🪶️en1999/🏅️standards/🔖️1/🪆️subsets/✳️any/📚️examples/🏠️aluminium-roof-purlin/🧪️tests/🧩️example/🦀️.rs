//! 🔬️ Example smoke.

use crate::aluminium_roof_purlin;

#[test]
fn example_snapshot_is_realistic() {
    let s = aluminium_roof_purlin::snapshot();
    assert_eq!(s.members[0].id, "purlin-1");
}
