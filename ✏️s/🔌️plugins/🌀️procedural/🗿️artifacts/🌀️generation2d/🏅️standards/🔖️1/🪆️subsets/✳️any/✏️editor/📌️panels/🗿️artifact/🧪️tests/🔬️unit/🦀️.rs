use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, render as render_body, render_with_view, snapshot_read};
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

#[semio_framework_async_macros::async_test]
async fn document_lists_widgets() {
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION2D_PLAY_BODY_ARTIFACT).await;
    let fixture_widgets: Vec<String> = snapshot_read(&app).host_snapshot.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    close(app);
    let first = fixture_widgets.first().expect("default fixture has at least one widget");
    assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
}

#[test]
fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(GENERATION2D_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
/// 🔎️ `(total, offset, materialised rows, row keys)` of one container, read off the rendered body.
fn container_of(json: &str, key: &str) -> (u64, u64, usize, Vec<String>) {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("document projection json");
    let node = walk(&projection, key).unwrap_or_else(|| panic!("{key} is not in the rendered document: {json}"));
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{key} stamps no window: {json}"));
    let rows: Vec<String> = node["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect();
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        rows.len(),
        rows,
    )
}

fn widget_window(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> ViewModel {
    ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: GENERATION2D_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }],
        ..Default::default()
    }
}

/// ⚖️ LAW (a)/(d): the widgets section states the document's full extent, materialises at most that
/// slice, and never summarises a remainder as a `+n`.
#[semio_framework_async_macros::async_test]
async fn the_widgets_section_stamps_the_documents_total() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_ARTIFACT).await;
    let widgets = snapshot_read(&app).host_snapshot.widgets.len();
    close(app);
    let (total, offset, rows, _) = container_of(&json, GENERATION2D_PLAY_DOCUMENT_WIDGETS);
    assert!(widgets > 0, "the default fixture carries widgets");
    assert_eq!(total as usize, widgets, "the section stamps the whole widget count: {json}");
    assert_eq!(offset, 0, "a first paint starts at zero: {json}");
    assert!(rows <= widgets, "the section materialises at most its slice: {json}");
    assert!(!json.contains(".more\""), "a windowed document has no continuation row: {json}");
    assert!(!json.contains(r#""label":"+"#), "a windowed document publishes no `+n` label: {json}");
}

/// ⚖️ LAW (b): a closed section states its extent and materialises nothing.
#[semio_framework_async_macros::async_test]
async fn a_closed_widgets_section_stamps_its_total_with_no_rows() {
    let mut app = app().await;
    let view = widget_window(GENERATION2D_PLAY_DOCUMENT_WIDGETS, Some(false), 0, 0);
    let json = render_with_view(&mut app, GENERATION2D_PLAY_BODY_ARTIFACT, &view).await;
    let widgets = snapshot_read(&app).host_snapshot.widgets.len();
    close(app);
    let (total, _, rows, _) = container_of(&json, GENERATION2D_PLAY_DOCUMENT_WIDGETS);
    assert_eq!(total as usize, widgets, "a closed section still states its extent: {json}");
    assert_eq!(rows, 0, "a closed section materialises nothing: {json}");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)`, keyed by the raw
/// widget id the `graph` domain targets.
#[semio_framework_async_macros::async_test]
async fn a_widgets_window_request_materialises_exactly_its_slice() {
    let mut app = app().await;
    let ids: Vec<String> = snapshot_read(&app).host_snapshot.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    assert!(ids.len() >= 2, "the default fixture carries at least two widgets: {ids:?}");
    let view = widget_window(GENERATION2D_PLAY_DOCUMENT_WIDGETS, Some(true), 1, 1);
    let json = render_with_view(&mut app, GENERATION2D_PLAY_BODY_ARTIFACT, &view).await;
    close(app);
    let (total, offset, _, rows) = container_of(&json, GENERATION2D_PLAY_DOCUMENT_WIDGETS);
    assert_eq!(total as usize, ids.len(), "the total stays the whole document: {json}");
    assert_eq!(offset, 1, "the stamped offset is the requested one: {json}");
    assert_eq!(rows, vec![ids[1].clone()], "exactly entry 1 is materialised, keyed by its raw id: {json}");
}

/// ⚖️ LAW (d): widget rows pick through the tree's ONE domain binding — they declare a granularity
/// and carry no per-row activate binding of their own.
#[semio_framework_async_macros::async_test]
async fn widget_rows_declare_their_granularity_and_carry_no_row_binding() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_ARTIFACT).await;
    close(app);
    let projection: serde_json::Value = serde_json::from_str(&json).expect("document projection json");
    assert_eq!(projection["component"]["interactionDomain"].as_str(), Some("graph"), "{json}");
    assert_eq!(projection["bindings"].as_array().cloned().unwrap_or_default().iter().filter(|binding| binding["trigger"] == "activate").count(), 1, "exactly one tree-level interactionSelect: {json}");
    let section = projection["children"].as_array().cloned().unwrap_or_default().into_iter().find(|child| child["key"].as_str() == Some(GENERATION2D_PLAY_DOCUMENT_WIDGETS)).expect("the widgets section");
    let rows = section["children"].as_array().cloned().unwrap_or_default();
    assert!(!rows.is_empty(), "the widgets section materialised rows: {json}");
    for row in rows {
        assert_eq!(row["component"]["granularity"].as_str(), Some("node"), "{row}");
        assert!(row["bindings"].as_array().map(|bindings| bindings.is_empty()).unwrap_or(true), "a pick row carries no binding of its own: {row}");
    }
}
//#endregion 🪟️WindowLaws
