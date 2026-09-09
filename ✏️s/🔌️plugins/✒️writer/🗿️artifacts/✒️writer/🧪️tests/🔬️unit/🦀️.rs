use super::*;

#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_matching_the_store_schema() {
    assert_eq!(artifact_kind().schema, WRITER_DOCUMENT_SCHEMA);
    assert_eq!(WRITER_DOCUMENT_SCHEMA, "writer.document");
}

#[semio_framework_async_macros::async_test]
async fn default_camera_is_centered_and_unzoomed() {
    assert_eq!(WriterCamera::default(), WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 });
}

#[semio_framework_async_macros::async_test]
async fn writer_child_restore_projection_accepts_the_exact_owned_document() {
    let snapshot = WriterSnapshot::default();
    let projection = store::ChildRestoreProjection::from_snapshot(&snapshot).expect("canonical Writer document child");
    assert_eq!(projection.len(), 1);
    assert!(projection.admits_member("document", &snapshot.document.target));
    assert_eq!(snapshot.document.child_id, snapshot.document.target.artifact_id);
}

#[semio_framework_async_macros::async_test]
async fn child_local_text_fixture_proves_bounded_identity_isolation_aba_and_wire_omission() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚖️writer-child-local-text-law.json")).expect("language-neutral writer child fixture");
    let cases = fixture["cases"].as_array().expect("fixture cases");
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(cases.len(), fixture["maximumCases"].as_u64().expect("bounded maximum") as usize);
    assert_eq!(cases.len(), 4);

    for case in cases {
        let law = case["law"].as_str().expect("law");
        let first = case["first"].as_str().expect("first");
        let second = case["second"].as_str().expect("second");
        let expected = case["expected"].as_str().expect("expected");
        match law {
            "cloneIdentity" => {
                let snapshot = writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "identity", "plaintext", "writer://identity", first);
                let retained = writer_text_owner(&snapshot);
                let cloned = snapshot.clone();
                let cloned_owner = writer_text_owner(&cloned);
                assert!(Arc::ptr_eq(&retained, &cloned_owner));
                assert_eq!(&*cloned_owner, expected);
                assert_eq!(Arc::strong_count(&retained), 4);
            }
            "instanceIsolation" => {
                let mut left = document_child_handle("collision", "", "plaintext");
                let mut right = left.clone();
                attach_writer_document_text(&mut left, first);
                attach_writer_document_text(&mut right, second);
                assert_eq!(format!("{}|{}", writer_text_for_handle(&left), writer_text_for_handle(&right)), expected);
            }
            "abaIsolation" => {
                let mut stale = document_child_handle("aba", "", "plaintext");
                attach_writer_document_text(&mut stale, first);
                let mut reused_identity = document_child_handle("aba", "", "plaintext");
                assert_eq!(stale.child_id, reused_identity.child_id);
                attach_writer_document_text(&mut reused_identity, second);
                assert_eq!(format!("{}|{}", writer_text_for_handle(&stale), writer_text_for_handle(&reused_identity)), expected);
            }
            "wireOmission" => {
                let handle = document_child_handle_with_text("wire", first, "plaintext");
                let wire: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&handle)).expect("third-party JSON oracle reads handle");
                assert!(wire.get("localText").is_none());
                let decoded: WriterDocumentChild = dsl::os_pack::json::from_json_str(&wire.to_string()).expect("owned handle decoder reads oracle JSON");
                assert_eq!(writer_text_for_handle(&decoded), expected);
                assert_eq!(writer_text_for_handle(&handle), first);
            }
            other => panic!("unexpected writer child law {other}"),
        }
    }
}
