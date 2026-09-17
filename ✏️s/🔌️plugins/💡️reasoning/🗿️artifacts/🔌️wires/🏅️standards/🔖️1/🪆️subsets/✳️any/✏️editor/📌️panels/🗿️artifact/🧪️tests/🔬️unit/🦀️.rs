use super::*;
use crate::editor::wires::unit_tests::context::{metabolism_app, render as render_body};
use crate::editor::wires::WIRES_PLAY_BODY_ARTIFACT as APP_BODY_ARTIFACT;
use dsl::DslValue;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

#[semio_framework_async_macros::async_test]
async fn document_has_identities_section() {
    let mut app = metabolism_app().await;
    let json = render_body(&mut app, APP_BODY_ARTIFACT).await;
    assert!(json.contains("wires-play-document.identities"));
    assert!(json.contains("Metabolism"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
/// 🪟️ A board an order of magnitude past one viewport — the shape a real reasoning document reaches
/// and the one this flat tree used to hard-fail on past the fixed child cap.
fn oversized(identities: usize, relationships: usize) -> WiresSnapshot {
    let rows: Vec<DslValue> =
        (0..identities).map(|index| DslValue::object([("nodeId".into(), DslValue::String(format!("identity-{index}"))), ("label".into(), DslValue::String(format!("Identity {index}")))])).collect();
    let edges: Vec<DslValue> = (0..relationships).map(|index| DslValue::object([("id".into(), DslValue::String(format!("edge-{index}")))])).collect();
    let mut document = crate::empty_wires_snapshot();
    document.wires_fixture = DslValue::object([
        ("schema".into(), DslValue::String(crate::MINDMAP_WIRES_SCHEMA.into())),
        ("identities".into(), DslValue::Array(rows)),
        ("relationships".into(), DslValue::Array(Vec::new())),
    ]);
    document.content = crate::wires_content_child_with_owner(Vec::new(), edges);
    document
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(document: &WiresSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view_state = ViewModel { tree_windows: requests, ..Default::default() };
    let labels = semio_framework_plugin::resolve_labels::<WiresLabels>(&ViewModel::default());
    let node = render(document, labels, &TreeWindows::for_body(&view_state, WIRES_PLAY_BODY_ARTIFACT)).expect("wires document tree assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("wires document tree projection")
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: WIRES_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

/// 🪟️ Law (a): both sections stamp their FULL extent and the first paint materialises at most one
/// viewport of rows — no `+N` row and no truncation anywhere.
#[test]
fn an_oversized_board_stamps_both_totals_and_never_a_continuation_row() {
    let document = oversized(300, 180);
    let json = window_body(&document, vec![request("wires-play-document.relationships", Some(true), 0, 4)]);
    assert!(json.contains("\"total\":300"), "the identities section stamps its full extent: {json}");
    assert!(json.contains("\"total\":180"), "the relationships section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"key\":\"identity-").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): the relationships section is authored CLOSED, and a section the host closes behaves the
/// same — the total is stamped, no child is built.
#[test]
fn a_closed_section_stamps_its_total_and_materialises_no_children() {
    let document = oversized(300, 180);
    let cold = window_body(&document, Vec::new());
    assert!(cold.contains("\"total\":180"), "the authored-closed relationships section still stamps its extent: {cold}");
    assert!(!cold.contains("\"key\":\"edge-"), "an authored-closed section materialises no rows: {cold}");
    let json = window_body(&document, vec![request("wires-play-document.identities", Some(false), 0, 0)]);
    assert!(json.contains("\"total\":300"), "a host-closed section still stamps its extent: {json}");
    assert!(!json.contains("\"key\":\"identity-"), "a host-closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the BARE identity
/// id the canvas hit-tests against.
#[test]
fn a_host_window_materialises_exactly_its_slice() {
    let json = window_body(&oversized(300, 0), vec![request("wires-play-document.identities", Some(true), 120, 10)]);
    assert!(json.contains("\"offset\":120"), "the section reports its offset: {json}");
    for index in 120..130 {
        assert!(json.contains(&format!("\"key\":\"identity-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"key\":\"identity-119\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"key\":\"identity-130\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d): the tree binds the "graph" domain ONCE and every row carries only its granularity — the
/// per-row `interactionSelect` `{domainId, merge, method, targets}` argument map is gone.
#[test]
fn pick_rows_carry_granularity_while_the_tree_carries_the_one_interaction_select() {
    let json = window_body(&oversized(4, 3), vec![request("wires-play-document.relationships", Some(true), 0, 3)]);
    assert!(json.contains("\"interactionDomain\":\"graph\""), "the tree binds the graph domain: {json}");
    assert_eq!(json.matches("interactionSelect").count(), 1, "exactly one tree-level interactionSelect binding: {json}");
    assert!(json.contains("\"granularity\":\"node\""), "identity rows are pick targets: {json}");
    assert!(json.contains("\"granularity\":\"edge\""), "relationship rows are pick targets: {json}");
    assert!(!json.contains("\"targets\""), "no per-row target argument map survives: {json}");
}
//#endregion 🪟️WindowLaws
