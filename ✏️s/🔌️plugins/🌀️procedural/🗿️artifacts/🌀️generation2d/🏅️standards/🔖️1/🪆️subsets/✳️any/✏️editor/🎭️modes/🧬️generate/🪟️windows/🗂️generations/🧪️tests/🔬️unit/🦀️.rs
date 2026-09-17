use super::*;
use crate::editor::generation2d::commands::add_generation;
use crate::editor::generation2d::unit_tests::context::{app, close, dispatch, render as render_body, render_with_view, snapshot_read, Generation2dApp};
use crate::editor::generation2d::Generation2dCommand;
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

/// 🗂️ The generate-mode window renders BOTH of its sections against an untouched document: the
/// generations list (empty placeholder) and the actions section carrying the add row. The fixture
/// projection serializes keys and components only — action bindings ride on `BuiltNode::on` and are
/// not projected — so the add route is measured by its stable surface key, and its bridge to
/// `addGeneration` by `declared_actions_bridge_to_commands`.
#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_surfaces() {
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION2D_PLAY_BODY_GENERATIONS).await;
    close(app);
    assert!(rendered.contains("procedural2d-play-generate.generations"), "{rendered}");
    assert!(rendered.contains("procedural2d-play-generate.add-generation"), "{rendered}");
    assert!(rendered.contains("Add Generation"), "{rendered}");
}

//#region 🪟️WindowLaws
/// 🗂️ Seeds `count` generations through the SAME `addGeneration` the tree's add row dispatches, and
/// returns their ids in roster order — never a hand-built `GenerationPlayState`.
async fn seed_generations(app: &mut Generation2dApp, count: usize) -> Vec<String> {
    for _ in 0..count {
        dispatch(app, Generation2dCommand::AddGeneration(add_generation::AddGeneration {})).await;
    }
    let ids: Vec<String> = snapshot_read(app).generation.generations.iter().map(|entry| entry.id.clone()).collect();
    assert_eq!(ids.len(), count, "addGeneration must publish one roster entry per dispatch");
    ids
}

/// 🔎️ `(total, offset, materialised row keys)` of the generations container, read off the rendered
/// body exactly the way the host observer reads it.
fn generations_window(json: &str) -> (u64, u64, Vec<String>) {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("generations projection json");
    let node = walk(&projection, GENERATION2D_PLAY_GENERATIONS_SECTION).unwrap_or_else(|| panic!("the generations container is not in the rendered body: {json}"));
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("the generations container stamps no window: {json}"));
    let rows: Vec<String> = node["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect();
    (window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(), window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(), rows)
}

fn generations_view(open: Option<bool>, offset: u32, rows: u32) -> ViewModel {
    ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: GENERATION2D_PLAY_BODY_GENERATIONS.into(), node_key: GENERATION2D_PLAY_GENERATIONS_SECTION.into(), open, offset, rows }],
        ..Default::default()
    }
}

/// ⚖️ LAW (a)/(d): the roster container states its FULL extent, materialises at most that slice, and
/// never summarises a remainder as a `+n` continuation row.
#[semio_framework_async_macros::async_test]
async fn the_generations_container_stamps_the_whole_roster() {
    let mut app = app().await;
    let ids = seed_generations(&mut app, 4).await;
    let body = render_body(&mut app, GENERATION2D_PLAY_BODY_GENERATIONS).await;
    close(app);
    let (total, offset, rows) = generations_window(&body);
    assert_eq!(total as usize, ids.len(), "the container stamps the whole roster: {body}");
    assert_eq!(offset, 0, "a first paint starts at zero: {body}");
    assert!(rows.len() <= ids.len(), "the container materialises at most its slice: {body}");
    assert!(!body.contains(".more\""), "a windowed roster has no continuation row: {body}");
    assert!(!body.contains(r#""label":"+"#), "a windowed roster publishes no `+n` label: {body}");
}

/// ⚖️ LAW (b): a closed container still states its extent and materialises nothing.
#[semio_framework_async_macros::async_test]
async fn a_closed_generations_container_stamps_its_total_with_no_rows() {
    let mut app = app().await;
    let ids = seed_generations(&mut app, 3).await;
    let view = generations_view(Some(false), 0, 0);
    let body = render_with_view(&mut app, GENERATION2D_PLAY_BODY_GENERATIONS, &view).await;
    close(app);
    let (total, _, rows) = generations_window(&body);
    assert_eq!(total as usize, ids.len(), "a closed container still states its extent: {body}");
    assert!(rows.is_empty(), "a closed container materialises nothing: {body}");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)` of the roster.
#[semio_framework_async_macros::async_test]
async fn a_generations_window_request_materialises_exactly_its_slice() {
    let mut app = app().await;
    let ids = seed_generations(&mut app, 4).await;
    let view = generations_view(Some(true), 2, 1);
    let body = render_with_view(&mut app, GENERATION2D_PLAY_BODY_GENERATIONS, &view).await;
    close(app);
    let (total, offset, rows) = generations_window(&body);
    assert_eq!(total as usize, ids.len(), "the total stays the whole roster: {body}");
    assert_eq!(offset, 2, "the stamped offset is the requested one: {body}");
    assert_eq!(rows, vec![format!("{GENERATION2D_PLAY_GENERATE_PREFIX}.generation.{}", ids[2])], "exactly entry 2 is materialised: {body}");
}
//#endregion 🪟️WindowLaws
