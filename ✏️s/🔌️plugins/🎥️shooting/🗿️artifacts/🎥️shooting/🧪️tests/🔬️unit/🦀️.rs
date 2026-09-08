
use super::*;

/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("shooting.scene") is deliberately NOT
/// `SHOOTING_DOCUMENT_SCHEMA` ("shooting.shooting") — the former names the artifact kind in the OS
/// media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
/// merge them.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
    assert_eq!(artifact_kind().schema, "shooting.scene");
    assert_eq!(SHOOTING_DOCUMENT_SCHEMA, "shooting.shooting");
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_has_no_entities() {
    let snapshot = empty_shooting_snapshot();
    assert!(snapshot.assets.is_empty() && snapshot.shots.is_empty() && snapshot.saved_cameras.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn emblem_materialization_is_owned_by_the_exact_snapshot_child() {
    let mut first = empty_shooting_snapshot();
    shooting_set_emblem_from_base64(&mut first, Some("AQID"));
    let first_handle = first.emblem.as_ref().expect("materialized emblem child");
    let mut second = empty_shooting_snapshot();
    second.emblem = Some(store::ArtifactChild::new(first_handle.child_id.clone(), first_handle.target.clone()));

    assert_eq!(shooting_emblem_bytes(&first), Some(vec![1, 2, 3]));
    assert_eq!(shooting_emblem_bytes(&second), None, "a matching wire identity cannot read another snapshot's instance-owned payload");
}
