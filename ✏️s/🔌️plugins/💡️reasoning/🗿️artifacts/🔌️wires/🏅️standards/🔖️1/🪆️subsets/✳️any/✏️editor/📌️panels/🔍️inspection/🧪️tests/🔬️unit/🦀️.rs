use super::*;
use crate::editor::wires::unit_tests::context::{metabolism_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn empty_selection_shows_document_summary() {
    let mut app = metabolism_app().await;
    let json = render_body(&mut app, WIRES_PLAY_BODY_PROPERTIES).await;
    assert!(json.contains("Schema:"));
    assert!(json.contains("Board nodes:"));
    crate::editor::wires::unit_tests::context::close(app);
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_PROPERTIES));
}

#[semio_framework_async_macros::async_test]
async fn relationship_kind_labels() {
    assert_eq!(RelationshipKind::Owns.label(), "owns");
    assert_eq!(RelationshipKind::Has.label(), "has");
}

#[semio_framework_async_macros::async_test]
async fn fixed_identity_set_validation() {
    let mut ext = DefaultWiresExtension::default();
    ext.allowed_identities.insert(1);
    ext.allowed_identities.insert(2);
    assert!(ext.validate_identity_set(&[1, 2]).is_ok());
    assert!(ext.validate_identity_set(&[1, 3]).is_err());
}

#[semio_framework_async_macros::async_test]
async fn relationship_lookup() {
    let mut ext = DefaultWiresExtension::default();
    ext.relationships.insert(7, RelationshipKind::References);
    assert_eq!(ext.relationship_kind_label(7), Some("references"));
}

#[semio_framework_async_macros::async_test]
async fn topic_lookup_stays_local_to_the_wires_extension() {
    let mut ext = DefaultWiresExtension::default();
    ext.topics.insert(7, "Context".into());
    assert_eq!(ext.topic_label(7), Some("Context"));
}

#[semio_framework_async_macros::async_test]
async fn metabolism_fixture_hydrates_extension() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
    let (_, content) = crate::wires_bundled_contents().iter().find(|(id, _)| id == &document.content.child_id).expect("the demo parent names its bundled board");
    let json = board_snapshot_json_string(&crate::wires_composed(&document, content).identity_snapshot);
    let ext = DefaultWiresExtension::from_host_snapshot_json(&json).expect("metabolism fixture");
    assert_eq!(ext.topics.len(), 7);
    assert_eq!(ext.relationships.len(), 9);
    assert_eq!(ext.relationship_kind_label(8), Some("is"));
    assert!(ext.validate_identity_set(&[1, 2, 3]).is_ok());
}


/// 🏗 DSL txt carrier: serialize then deserialize must restore the snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn txt_dsl_carrier_round_trips_exactly() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    use semio_framework::io_schema::IoPayload;
    let snapshot = crate::empty_wires_snapshot();
    let exported = export::txt::v_utf_8::any::WiresIntoTxt::serialize(&snapshot, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("dsl txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = import::txt::v_utf_8::any::TxtIntoWires::deserialize(&IoPayload::Text(text)).await.expect("dsl txt import");
    assert_eq!(back.value, snapshot);
}
