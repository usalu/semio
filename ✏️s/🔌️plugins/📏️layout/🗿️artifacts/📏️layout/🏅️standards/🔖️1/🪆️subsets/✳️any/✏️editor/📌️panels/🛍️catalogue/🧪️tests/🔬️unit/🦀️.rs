use super::*;
use semio_s_artifact_layout_layout_editor_commands::NATIVE_PLACEMENTS;
use semio_s_artifact_layout_layout_editor::unit_tests::context::{layout_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_frame_kinds() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("layout-catalogue.rect"));
    assert!(json.contains("Text Frame"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_items_are_draggable() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains(LAYOUT_CATALOGUE_DRAG_MIME));
    assert!(json.contains("\"draggable\":true"));
    assert!(json.contains("layout-catalogue.page"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_CATALOGUE));
}

#[test]
fn catalogue_roster_offers_every_native_artifact() {
    for (kind, _, _) in NATIVE_PLACEMENTS {
        assert!(LAYOUT_CATALOGUE_ROSTER.iter().any(|(candidate, _)| candidate == kind), "{kind} is placeable but missing from the catalogue");
    }
}

#[test]
fn catalogue_declares_the_neutral_kind_witness_and_drop_payload() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../🧪️tests/🧫️fixtures/🛍️canvas-catalogue/🔣️.json" )).expect("neutral catalogue producer");
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case| case["action"] == "canvasDrop" && case["kind"].is_string()) {
        let kind = case["kind"].as_str().unwrap();
        let item = catalogue_tree_item(kind, Label::data(kind), "square").expect("catalogue row");
        let semio_plugin::Component::TreeItem(props) = &item.component else { panic!("catalogue row component") };
        let entries = props.drag_data.as_ref().expect("catalogue drag data").iter().map(|(mime, raw)| (mime.as_str(), raw.as_str())).collect::<Vec<_>>();
        let kind_mime = format!("{}{}", fixture["kindMimePrefix"].as_str().unwrap(), kind);
        assert_eq!(entries, vec![(fixture["mime"].as_str().unwrap(), case["args"]["dragData"].as_str().unwrap()), (kind_mime.as_str(), "")]);
    }
}

#[test]
fn oversized_drag_rows_keep_the_catalogue_and_normal_source_transfer_available() {
    fn find_row<'a>(node: &'a semio_plugin::BuiltNode, key: &str) -> Option<&'a semio_plugin::BuiltNode> {
        if node.key.as_str() == key { return Some(node); }
        node.children.iter().find_map(|child| find_row(child, key))
    }

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../🧪️tests/🧫️fixtures/🛍️canvas-catalogue/linked-asset.json")).unwrap();
    let expected = &fixture["expected"]["drag"];
    let document_id = expected["sourceDocumentId"].as_str().unwrap();
    let source = ImageLink {
        id: expected["sourceLinkId"].as_str().unwrap().into(), path: "chart-preview.png".into(),
        hash: String::new(), width: 16, height: 16, dpi: 72, color_profile: None, state: None,
        proxy_data_url: None, artifact_kind: "s.stdio.png".into(),
        artifact_ref: fixture["source"]["artifactRef"].as_str().unwrap().into(),
    };
    let boundary = &fixture["catalogueBoundary"];
    assert_eq!(UiText::default().capacity(), boundary["maximumTextBytes"].as_u64().unwrap() as usize);
    let mut links = vec![source.clone()];
    for link in boundary["links"].as_array().unwrap() {
        let id = link["character"].as_str().unwrap().repeat(link["count"].as_u64().unwrap() as usize);
        links.push(ImageLink { id, path: link["path"].as_str().unwrap().into(), ..source.clone() });
    }
    let labels = crate::terminology::layout_labels(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native));
    let panel = render(&links, Some(document_id), labels, &TreeWindows::unhosted()).unwrap();
    let valid = find_row(&panel, &format!("layout-catalogue.link.{}", source.id)).unwrap();
    assert!(!valid.disabled);
    let semio_plugin::Component::TreeItem(valid) = &valid.component else { panic!("source row") };
    let raw = valid.drag_data.as_ref().unwrap().iter().find_map(|(key, raw)| (key.as_str() == LAYOUT_CATALOGUE_DRAG_MIME).then_some(raw)).unwrap();
    assert_eq!(&serde_json::from_str::<serde_json::Value>(raw.as_str()).unwrap(), expected);
    assert!(find_row(&panel, "layout-catalogue.rect").is_some());

    for (index, fixture) in boundary["links"].as_array().unwrap().iter().enumerate() {
        let position = index + 1;
        let link = &links[position];
        let key = if fixture["rowKeyFits"].as_bool().unwrap() { format!("layout-catalogue.link.{}", link.id) } else { format!("layout-catalogue.long-link.{position}") };
        let row = find_row(&panel, &key).unwrap();
        assert!(!row.disabled);
        assert!(serde_json::to_string(row).unwrap().contains(&link.path));
        let semio_plugin::Component::TreeItem(props) = &row.component else { panic!("source row") };
        assert_ne!(props.draggable, Some(true));
        assert!(props.drag_data.is_none());
    }
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(panel)).unwrap();

    let panel = render(std::slice::from_ref(&source), None, labels, &TreeWindows::unhosted()).unwrap();
    let row = find_row(&panel, &format!("layout-catalogue.link.{}", source.id)).unwrap();
    assert!(!row.disabled);
    let semio_plugin::Component::TreeItem(props) = &row.component else { panic!("unbound source row") };
    assert!(props.drag_data.is_none());
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(panel)).unwrap();
}
