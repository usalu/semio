use super::*;

/// 🪪️ `artifact_kind().schema` IS `SHOOTING_DOCUMENT_SCHEMA`: a document kind has ONE schema identity — the hub's codec rows, document-open targets and genesis, the MCP workspace
/// store and host-media contributions all key on it (ticket 26/09/23 W4: a distinct "media schema" left the package without a
/// codec owner, so the trusted catalog refused it). The former media string stays declared as `source_format`.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_names_the_store_schema() {
    assert_eq!(artifact_kind().schema, SHOOTING_DOCUMENT_SCHEMA);
    assert_eq!(artifact_kind().source_format, "shooting.scene");
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
