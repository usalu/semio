
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_vcs_snapshot();
    assert_eq!(snapshot.schema, crate::VCS_DOCUMENT_SCHEMA);
    assert_eq!(snapshot.status, "new");
}
