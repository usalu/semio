use super::*;
use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::terminology::lowpoly_play_labels;
use crate::editor::lowpoly::unit_tests::context::{app, render as render_body};
use crate::{LowpolySnapshot, LowpolyTransform};
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

#[semio_framework_async_macros::async_test]
async fn layers_panel_lists_the_base_layer() {
    let mut a = app().await;
    let json = render_body(&mut a, LOWPOLY_PLAY_BODY_LAYERS).await;
    assert!(json.contains("lowpoly-layer:0"));
}

//#region 🪟️WindowLaws
/// 🪟️ A paint stack an order of magnitude past one viewport, on an object with no mesh handle — the
/// layers panel reads `paint_layers` alone, so the stack is all this law needs.
fn oversized(layers: usize) -> (LowpolySnapshot, LowpolyConfig) {
    let mut snapshot = LowpolySnapshot::default();
    snapshot.objects.push(crate::LowpolyObject {
        id: "obj-0".into(),
        name: "Object 0".into(),
        transform: LowpolyTransform::default(),
        smooth_shading: false,
        mesh: None,
        paint_layers: (0..layers).map(|index| LowpolyPaintLayer { name: format!("Layer {index}"), visible: true, opacity: 1.0, blend_mode: "normal".into(), pixels: Vec::new() }).collect(),
    });
    (snapshot, LowpolyConfig { active_object_id: "obj-0".into(), ..Default::default() })
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(snapshot: &LowpolySnapshot, config: &LowpolyConfig, requests: Vec<TreeWindowRequest>) -> String {
    let view_state = ViewModel { tree_windows: requests, ..Default::default() };
    let labels = lowpoly_play_labels(&ViewModel::default());
    let node = render(LowpolyView { snapshot, config }, labels, &TreeWindows::for_body(&view_state, LOWPOLY_PLAY_BODY_LAYERS)).expect("lowpoly layers tree assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("lowpoly layers tree projection")
}

fn request(open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: LOWPOLY_PLAY_BODY_LAYERS.into(), node_key: "lowpoly-play-layers.paint".into(), open, offset, rows }
}

/// 🪟️ Law (a): the paint section stamps its full extent and materialises at most one viewport — no
/// `+N`, and nothing truncated away.
#[test]
fn an_oversized_paint_stack_stamps_its_total_and_never_a_continuation_row() {
    let (snapshot, config) = oversized(300);
    let json = window_body(&snapshot, &config, Vec::new());
    assert!(json.contains("\"total\":300"), "the paint section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"key\":\"lowpoly-layer:").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a section the host closed stamps its total and materialises nothing.
#[test]
fn a_closed_paint_section_stamps_its_total_and_materialises_no_children() {
    let (snapshot, config) = oversized(300);
    let json = window_body(&snapshot, &config, vec![request(Some(false), 0, 0)]);
    assert!(json.contains("\"total\":300"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("\"key\":\"lowpoly-layer:"), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the layer's own
/// stack index — never renumbered per window.
#[test]
fn a_host_window_materialises_exactly_its_slice() {
    let (snapshot, config) = oversized(300);
    let json = window_body(&snapshot, &config, vec![request(Some(true), 120, 10)]);
    assert!(json.contains("\"offset\":120"), "the section reports its offset: {json}");
    for index in 120..130 {
        assert!(json.contains(&format!("\"key\":\"lowpoly-layer:{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"key\":\"lowpoly-layer:119\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"key\":\"lowpoly-layer:130\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d) for an UNBOUND tree: a paint layer is not a mesh pick target, so this tree declares no
/// interaction domain, stamps no granularity, and every row keeps its own `setActivePaintLayer` action.
#[test]
fn the_layer_tree_declares_no_domain_and_keeps_per_row_actions() {
    let (snapshot, config) = oversized(4);
    let json = window_body(&snapshot, &config, Vec::new());
    assert!(!json.contains("interactionDomain"), "the layer stack binds no domain: {json}");
    assert!(!json.contains("granularity"), "an unbound tree stamps no pick granularity: {json}");
    assert!(json.contains("setActivePaintLayer"), "every layer row keeps its own action: {json}");
}
//#endregion 🪟️WindowLaws
