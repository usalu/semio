use super::*;
use crate::editor::wires::testkit::{metabolism_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn empty_selection_shows_document_summary() {
    let mut app = metabolism_app().await;
    let json = render_body(&mut app, WIRES_PLAY_BODY_PROPERTIES).await;
    assert!(json.contains("Schema:"));
    assert!(json.contains("Board nodes:"));
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
    // 📜️ The `.wires` fixture is handcrafted in `crate::dsl`'s DSL — parse it,
    // then hydrate this crate's JSON-facing extension from its `wires_fixture` value, the same
    // shape `from_fixture_json` has always expected.
    let document = crate::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
    let json = dsl::os_pack::json::to_string(&crate::schema::dsl_to_json(&document.wires_fixture));
    let ext = DefaultWiresExtension::from_fixture_json(&json).expect("metabolism fixture");
    assert_eq!(ext.topics.len(), 7);
    assert_eq!(ext.relationships.len(), 9);
    assert_eq!(ext.relationship_kind_label(8), Some("is"));
    assert!(ext.validate_identity_set(&[1, 2, 3]).is_ok());
}
