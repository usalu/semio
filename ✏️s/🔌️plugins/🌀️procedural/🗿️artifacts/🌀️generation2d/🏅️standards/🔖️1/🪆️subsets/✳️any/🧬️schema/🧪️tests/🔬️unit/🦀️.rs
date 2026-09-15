use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead;

#[test]
fn default_snapshot_parses_the_bundled_example() {
    let snapshot = Generation2dSnapshotRead::new(default_snapshot());
    assert!(!snapshot.host_document.widgets.is_empty());
}
