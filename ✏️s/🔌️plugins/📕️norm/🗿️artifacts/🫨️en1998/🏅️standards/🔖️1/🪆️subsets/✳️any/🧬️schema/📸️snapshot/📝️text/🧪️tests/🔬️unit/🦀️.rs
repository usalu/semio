use super::*;
use crate::En1998Snapshot;

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_nested_site_and_building() {
    let doc = En1998Snapshot::default();
    assert_eq!(doc.annex, "de");
    assert_eq!(doc.site.importance_class, "II");
    assert!(!doc.buildings.is_empty());
    assert!(!doc.buildings[0].systems.is_empty());
}
