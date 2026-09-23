use super::*;
use crate::editor::layout::unit_tests::context::{layout_app, render as render_body};

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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../🧫️fixtures/🛍️canvas-catalogue/🔣️.json" )).expect("neutral catalogue producer");
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case| case["action"] == "canvasDrop" && case["kind"].is_string()) {
        let kind = case["kind"].as_str().unwrap();
        let item = catalogue_tree_item(kind, Label::data(kind), "square").expect("catalogue row");
        let semio_framework_plugin::Component::TreeItem(props) = &item.component else { panic!("catalogue row component") };
        let entries = props.drag_data.as_ref().expect("catalogue drag data").iter().map(|(mime, raw)| (mime.as_str(), raw.as_str())).collect::<Vec<_>>();
        let kind_mime = format!("{}{}", fixture["kindMimePrefix"].as_str().unwrap(), kind);
        assert_eq!(entries, vec![(fixture["mime"].as_str().unwrap(), case["args"]["dragData"].as_str().unwrap()), (kind_mime.as_str(), "")]);
    }
}
