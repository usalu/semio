//! 🔬️ Outline inference for hierarchical subject.

use crate::En1999Snapshot;
use crate::standards::v1::subsets::any::schema::inferences::outline::En1999Outline;

#[test]
fn outline_counts_entities() {
    let o = En1999Outline::compute(&En1999Snapshot::default());
    assert!(o.member_count >= 1);
    assert!(o.section_count >= 1);
    assert_eq!(o.annex, "de");
}
