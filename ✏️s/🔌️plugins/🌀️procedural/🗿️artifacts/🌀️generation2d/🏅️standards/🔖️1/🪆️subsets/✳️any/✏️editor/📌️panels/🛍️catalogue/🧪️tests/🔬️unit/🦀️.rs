use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, render as render_body, render_with_view};
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

const MODES_SECTION: &str = "procedural2d-play-catalogue.modes";

#[semio_framework_async_macros::async_test]
async fn generation2d_labels_resolve_native_english_by_default() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE).await;
    close(app);
    assert!(json.contains("\"Sources\""));
    assert!(json.contains("\"Components\""));
    assert!(json.contains("\"Sinks\""));
    assert!(json.contains("\"Show mode\""));
    assert!(!json.contains("Quellen"));
}

/// ⚖️ LAW (b): the show-mode section authors itself CLOSED, so it stamps its extent and materialises
/// nothing until the reader opens it — the lazy half of the windowed tree.
#[semio_framework_async_macros::async_test]
async fn the_closed_show_mode_section_stamps_its_total_with_no_rows() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE).await;
    close(app);
    let section = section_of(&json, MODES_SECTION);
    assert_eq!(section.0, 3, "the closed section still states its three modes: {json}");
    assert_eq!(section.2, 0, "a closed section materialises nothing: {json}");
    assert!(!json.contains("procedural2d-play-catalogue.mode.preview"), "a closed section carries no rows: {json}");
}

/// ⚖️ LAW (c): opening the section through a host window request materialises exactly the requested
/// slice, keyed by the rows' own ids.
#[semio_framework_async_macros::async_test]
async fn opening_the_show_mode_section_materialises_its_rows() {
    let mut app = app().await;
    let view = ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: GENERATION2D_PLAY_BODY_CATALOGUE.into(), node_key: MODES_SECTION.into(), open: Some(true), offset: 1, rows: 2 }],
        ..Default::default()
    };
    let json = render_with_view(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE, &view).await;
    close(app);
    let section = section_of(&json, MODES_SECTION);
    assert_eq!(section.0, 3, "the total stays the whole mode roster: {json}");
    assert_eq!(section.1, 1, "the stamped offset is the requested one: {json}");
    assert_eq!(section.2, 2, "exactly the requested rows are materialised: {json}");
    assert!(!json.contains("procedural2d-play-catalogue.mode.preview"), "entry 0 is outside the window: {json}");
    assert!(json.contains("procedural2d-play-catalogue.mode.generate"), "entry 1 is inside the window: {json}");
    assert!(json.contains("procedural2d-play-catalogue.mode.wire"), "entry 2 is inside the window: {json}");
}

/// ⚖️ LAW (a)/(d): every catalogue section stamps its own total and none of them summarises a
/// remainder with a `+n`.
#[semio_framework_async_macros::async_test]
async fn every_catalogue_section_stamps_its_total_and_none_continues() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE).await;
    close(app);
    for (key, total) in [("procedural2d-play-catalogue.sources", 2), ("procedural2d-play-catalogue.components", 3), ("procedural2d-play-catalogue.sinks", 2), (MODES_SECTION, 3)] {
        assert_eq!(section_of(&json, key).0, total, "{key} stamps its own total: {json}");
    }
    assert!(!json.contains(".more\""), "a windowed catalogue has no continuation row: {json}");
    assert!(!json.contains(r#""label":"+"#), "a windowed catalogue publishes no `+n` label: {json}");
}

/// 🔎️ `(total, offset, materialised rows)` of one section, read off the rendered body by its key.
fn section_of(json: &str, key: &str) -> (u64, u64, usize) {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("catalogue projection json");
    let node = walk(&projection, key).unwrap_or_else(|| panic!("{key} is not in the rendered catalogue: {json}"));
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{key} stamps no window: {json}"));
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        node["children"].as_array().map(Vec::len).unwrap_or_default(),
    )
}
