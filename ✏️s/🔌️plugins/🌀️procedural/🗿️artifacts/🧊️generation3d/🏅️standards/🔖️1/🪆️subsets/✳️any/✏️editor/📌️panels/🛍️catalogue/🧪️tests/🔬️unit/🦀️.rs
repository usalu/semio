use super::*;
use crate::editor::generation3d::unit_tests::context::{app, render as render_body, render_with_view};
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

#[semio_framework_async_macros::async_test]
async fn generation3d_labels_resolve_native_english_by_default() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("\"Widgets\""));
    assert!(!json.contains("Elemente"));
}

//#region 🪟️WindowLaws
/// 🪟️ One windowed container read off a rendered panel body: its key, the `total` it stamped, the
/// `offset` its slice starts at, and the rows it actually materialised.
#[derive(Clone, Debug, PartialEq)]
struct RenderedWindow {
    key: String,
    total: u64,
    offset: u64,
    rows: usize,
}

fn collect_windows(node: &serde_json::Value, out: &mut Vec<RenderedWindow>) {
    if let Some(window) = node["component"]["window"].as_object() {
        out.push(RenderedWindow {
            key: node["key"].as_str().unwrap_or_default().to_string(),
            total: window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
            offset: window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
            rows: node["children"].as_array().map(Vec::len).unwrap_or_default(),
        });
    }
    for child in node["children"].as_array().cloned().unwrap_or_default().iter() {
        collect_windows(child, out);
    }
}

fn rendered_windows(json: &str) -> Vec<RenderedWindow> {
    let projection: serde_json::Value = serde_json::from_str(json).expect("catalogue projection json");
    let mut windows = Vec::new();
    collect_windows(&projection, &mut windows);
    windows
}

fn group_windows(json: &str) -> Vec<RenderedWindow> {
    rendered_windows(json).into_iter().filter(|window| window.key.starts_with("procedural-play-catalogue.") && window.key != GENERATION_3D_PLAY_CATALOGUE_SECTION).collect()
}

fn catalogue_view(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

fn first_group_key() -> String {
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let section = sections.first().expect("the flow palette registers at least one catalogue section");
    format!("procedural-play-catalogue.{}", section.id)
}

fn first_group_items() -> Vec<String> {
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let section = sections.first().expect("the flow palette registers at least one catalogue section");
    section
        .items
        .iter()
        .map(|item| super::identity::item_key(&item.kind, item.neuron_kind.as_deref(), item.format.as_deref(), item.action.as_deref()))
        .collect()
}

/// ⚖️ LAW (a): every registered operator is ACCOUNTED FOR, not shown. Each catalogue group is one
/// window that stamps its full `total` and materialises at most that slice, so the sum of the group
/// totals is the whole `flow_palette_catalogue_sections()` roster — the reader's scrollbar spans the
/// catalogue even though only a viewport of rows exists. Before this the panel paged and summarised
/// the remainder as an inert `+n` (`📓️audit-user-journey-gaps-2026-09-13.md` §9 item 11).
#[semio_framework_async_macros::async_test]
async fn every_catalogue_group_stamps_the_total_its_window_slices() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let roster: usize = sections.iter().map(|section| section.items.len()).sum();
    assert!(roster > 0, "the flow palette registers at least one operator");
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE).await;
    let groups = group_windows(&json);
    assert_eq!(groups.len(), sections.len(), "every catalogue group is one windowed container: {groups:?}");
    for (group, section) in groups.iter().zip(sections.iter()) {
        assert_eq!(group.total as usize, section.items.len(), "group {} stamps its own total", group.key);
        assert!(group.rows <= section.items.len(), "group {} materialises at most its slice: {group:?}", group.key);
    }
    assert_eq!(groups.iter().map(|group| group.total as usize).sum::<usize>(), roster, "the group totals are the whole registered roster: {groups:?}");
}

/// ⚖️ LAW (b): a closed group costs nothing but still states its extent — `total` stamped, zero
/// children. That is what makes the listing lazy: hundreds of operators can be registered and the
/// first paint still materialises about one viewport.
#[semio_framework_async_macros::async_test]
async fn a_closed_catalogue_group_stamps_its_total_with_no_children() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let key = first_group_key();
    let mut app = app().await;
    let view = catalogue_view(vec![TreeWindowRequest { body_key: GENERATION_3D_PLAY_BODY_CATALOGUE.into(), node_key: key.clone(), open: Some(false), offset: 0, rows: 0 }]);
    let json = render_with_view(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE, &view).await;
    let group = group_windows(&json).into_iter().find(|group| group.key == key).unwrap_or_else(|| panic!("the closed group is still a row: {json}"));
    assert!(group.total > 0, "a closed group still states its extent: {group:?}");
    assert_eq!(group.rows, 0, "a closed group materialises nothing: {group:?}");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)` of that group's
/// entries, keyed by the raw catalogue-item id — the guest never re-derives a page, it answers the
/// slice the viewport asked for.
#[semio_framework_async_macros::async_test]
async fn a_catalogue_window_request_materialises_exactly_its_slice() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let key = first_group_key();
    let items = first_group_items();
    let offset = 2u32.min(items.len().saturating_sub(1) as u32);
    let rows = 3u32.min(items.len().saturating_sub(offset as usize) as u32);
    assert!(rows > 0, "the first catalogue group carries at least one operator");
    let mut app = app().await;
    let view = catalogue_view(vec![TreeWindowRequest { body_key: GENERATION_3D_PLAY_BODY_CATALOGUE.into(), node_key: key.clone(), open: Some(true), offset, rows }]);
    let json = render_with_view(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE, &view).await;
    let projection: serde_json::Value = serde_json::from_str(&json).expect("catalogue projection json");
    let mut found: Option<Vec<String>> = None;
    fn walk(node: &serde_json::Value, key: &str, found: &mut Option<Vec<String>>) {
        if node["key"].as_str() == Some(key) {
            *found = Some(node["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect());
            return;
        }
        for child in node["children"].as_array().cloned().unwrap_or_default().iter() {
            walk(child, key, found);
        }
    }
    walk(&projection, &key, &mut found);
    let rendered = found.unwrap_or_else(|| panic!("the requested group is in the body: {json}"));
    let expected: Vec<String> = items.iter().skip(offset as usize).take(rows as usize).cloned().collect();
    assert_eq!(rendered, expected, "the group materialises exactly its requested slice: {json}");
    let group = group_windows(&json).into_iter().find(|group| group.key == key).expect("the requested group window");
    assert_eq!(group.offset, u64::from(offset), "the stamped offset is the requested one: {group:?}");
    assert_eq!(group.total as usize, items.len(), "the stamped total stays the whole group: {group:?}");
}

/// ⚖️ LAW (d): no `+n`. A windowed catalogue never summarises a remainder, so neither the `.more`
/// key nor a `+`-prefixed label may appear anywhere in the body.
#[semio_framework_async_macros::async_test]
async fn the_catalogue_panel_never_renders_a_continuation_row() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE).await;
    assert!(!json.contains(".more\""), "a windowed catalogue has no continuation row: {json}");
    assert!(!json.contains(r#""label":"+"#), "a windowed catalogue publishes no `+n` label: {json}");
}

/// ⚖️ LAW: every windowed container in this body has a DISTINCT node key. A `TreeWindowRequest` is
/// addressed by `(body_key, node_key)`, so two containers sharing a key would both answer one
/// request and the host could never scroll either independently. The group keys are derived from the
/// registered `CatalogueSection` ids, which is exactly where a future extension could collide with
/// the panel's own `…catalogue.widgets` section — this law is what makes that collision loud.
#[semio_framework_async_macros::async_test]
async fn every_windowed_container_carries_a_distinct_node_key() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE).await;
    let keys: Vec<String> = rendered_windows(&json).into_iter().map(|window| window.key).collect();
    let unique: std::collections::BTreeSet<&String> = keys.iter().collect();
    assert_eq!(unique.len(), keys.len(), "two windowed containers share a node key: {keys:?}");
}
//#endregion 🪟️WindowLaws

/// ⚖️ LAW: the spotlight browses the SAME roster the panel windows. The canvas spotlight reads the
/// app-static `flow_app_catalogue` published once per app instance on the reserved
/// `framework.section.catalogue` surface, so that payload must carry the whole roster — it is the
/// search surface over the catalogue, never a substitute for rows the panel dropped.
#[test]
fn the_spotlight_catalogue_offers_every_registered_operator() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let spotlight = semio_framework_os_flow::flow_app_catalogue();
    let spotlight_rows: usize = spotlight.sections.iter().map(|section| section.items.len()).sum();
    let roster: usize = sections.iter().map(|section| section.items.len()).sum();
    assert_eq!(spotlight_rows, roster, "the spotlight catalogue is the whole roster");
}
