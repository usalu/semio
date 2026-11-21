#[test]
fn observed_schema_fields_are_visible_without_write_controls() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔒️read-only/🔣️.json")).unwrap();
    let value = semio_framework_pack_json::from_json_str(&fixture["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let provider = DslSnapshotDetailsProvider::<NativeLazySnapshot>::from_value_and_schema(value, &fixture["schema"].to_string());
    for entry in fixture["paths"].as_array().unwrap() {
        let path: Vec<_> = entry["segments"].as_array().unwrap().iter().map(|segment| match segment {
            serde_json::Value::String(key) => SnapshotDetailPathSegment::Key(key.clone()),
            _ => SnapshotDetailPathSegment::Index(segment.as_u64().unwrap() as usize),
        }).collect();
        let capabilities = provider.item_capabilities(&path);
        assert_eq!(capabilities.edit, entry["edit"].as_bool().unwrap(), "{path:?}");
        if let Some(expected) = entry["insert"].as_bool() {
            assert_eq!(provider.allows_collection_insert(&path), expected, "{path:?}");
        }
        if let Some(expected) = entry["reorder"].as_bool() {
            assert_eq!(capabilities.reorder, expected, "{path:?}");
        }
    }
    let source = [SnapshotDetailPathSegment::Key("source".into())];
    let available: Vec<_> = provider.missing_property_templates(&source).into_iter().map(|(key, _)| key).collect();
    assert_eq!(available, serde_json::from_value::<Vec<String>>(fixture["addable"].clone()).unwrap());

    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let rendered = render_snapshot_details_provider_revisioned(&provider, semio_ui_contract::UiPublicationRevision(1),
        Locale::En, "test.read-only#editor", &TreeWindows::unhosted()).unwrap();
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(rendered), |rendered| {
        let header = path_id(&[SnapshotDetailPathSegment::Key("declaredVersion".into())]);
        let readonly = find_key(rendered, &format!("{header}-read-only")).unwrap();
        let semio_ui_contract::Component::Text(text) = &readonly.component else { panic!("observed version is display text") };
        assert!(text.packed_payload().contains("1.6"));
        assert!(!contains_key(rendered, &format!("{header}-text")));
    });
}
