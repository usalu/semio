use super::*;
use crate::editor::process3d::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_ARTIFACT));
}

#[semio_framework_async_macros::async_test]
async fn document_panel_lists_stock_and_steps() {
    let mut app = context::app();
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_ARTIFACT);
    assert!(rendered.contains("process3d-play-document.stock"));
    assert!(rendered.contains("process3d-play-document.steps"));
}

/// 🎞️ `render` must list every `step_payloads` entry, in timeline order — the authoritative
/// record since wave 4, not the unresolvable `steps` child handle.
#[semio_framework_async_macros::async_test]
async fn document_panel_lists_every_step_payload_in_order() {
    use crate::{process_working_scene_to_snapshot, ProcessMeasure, ProcessStep, ProcessWorkingScene, Stock, Workshop};
    let scene = ProcessWorkingScene {
        stock: Stock::default(),
        steps: vec![
            ProcessStep { id: "step-rip".into(), label: "Rip Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: Default::default(), pose: Default::default() } },
            ProcessStep { id: "step-bore".into(), label: "Bore Hole".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.01, depth: 0.02, pose: Default::default() } },
            ProcessStep { id: "step-dowel".into(), label: "Attach Dowel".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: Default::default(), pose: Default::default() } },
        ],
    };
    let fixture = process_working_scene_to_snapshot(&scene, Workshop::default(), None);
    let labels = crate::editor::process3d::terminology::process3d_labels(&semio_framework_plugin::ViewModel::default());
    let node = render(&fixture, labels, &semio_framework_plugin::TreeWindows::unhosted()).expect("document tree renders");
    let rendered = serde_json::to_string(&node).expect("render json");
    let rip_index = rendered.find("step-rip").expect("step-rip present");
    let bore_index = rendered.find("step-bore").expect("step-bore present");
    let dowel_index = rendered.find("step-dowel").expect("step-dowel present");
    assert!(rip_index < bore_index && bore_index < dowel_index, "expected steps in timeline order: {rendered}");
}

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

/// 🔎️ `(total, offset, row keys)` of one container, read off a projected body.
fn container_of(json: &str, key: &str) -> (u64, u64, Vec<String>) {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("document projection json");
    let node = walk(&projection, key).unwrap_or_else(|| panic!("{key} is not in the rendered document: {json}"));
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{key} stamps no window: {json}"));
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        node["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect(),
    )
}

/// 🌾️ A 180-step timeline — an order of magnitude past any viewport.
fn oversized_document() -> Process3dSnapshot {
    use crate::{process_working_scene_to_snapshot, ProcessMeasure, ProcessStep, ProcessWorkingScene, Stock, Workshop};
    let scene = ProcessWorkingScene {
        stock: Stock::default(),
        steps: (0..180)
            .map(|index| ProcessStep {
                id: format!("step-{index:03}"),
                label: format!("Step {index}"),
                enabled: true,
                origin: None,
                measure: ProcessMeasure::Drill { radius: 0.01, depth: 0.02, pose: Default::default() },
            })
            .collect(),
    };
    process_working_scene_to_snapshot(&scene, Workshop::default(), None)
}

fn project(document: &Process3dSnapshot, windows: &semio_framework_plugin::TreeWindows<'_>) -> String {
    let labels = crate::editor::process3d::terminology::process3d_labels(&ViewModel::default());
    let node = render(document, labels, windows).expect("document tree renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("document projection")
}

fn window_view(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> ViewModel {
    ViewModel { tree_windows: vec![TreeWindowRequest { body_key: PROCESS_3D_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }], ..Default::default() }
}

/// ⚖️ LAW (a)/(d): an oversized timeline states its full extent, materialises at most its slice, and
/// never summarises a remainder as a `+n`.
#[semio_framework_async_macros::async_test]
async fn an_oversized_timeline_stamps_every_sections_total() {
    let document = oversized_document();
    let json = project(&document, &semio_framework_plugin::TreeWindows::unhosted());
    let (stock_total, _, stock_rows) = container_of(&json, PROCESS_3D_PLAY_DOCUMENT_STOCK);
    assert_eq!(stock_total, 1, "the stock section stamps its single row: {json}");
    assert_eq!(stock_rows.len(), 1, "the stock row is always materialised: {json}");
    let (steps_total, steps_offset, steps_rows) = container_of(&json, PROCESS_3D_PLAY_DOCUMENT_STEPS);
    assert_eq!(steps_total, 180, "the steps section stamps the whole timeline: {json}");
    assert_eq!(steps_offset, 0, "a first paint starts at zero");
    assert!(steps_rows.len() <= 180, "the steps section materialises at most its slice");
    assert!(!json.contains(".more\""), "a windowed timeline has no continuation row");
    assert!(!json.contains(r#""label":"+"#), "a windowed timeline publishes no `+n` label");
}

/// ⚖️ LAW (b): a closed section states its extent and materialises nothing.
#[semio_framework_async_macros::async_test]
async fn a_closed_steps_section_stamps_its_total_with_no_rows() {
    let document = oversized_document();
    let view = window_view(PROCESS_3D_PLAY_DOCUMENT_STEPS, Some(false), 0, 0);
    let json = project(&document, &semio_framework_plugin::TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_ARTIFACT));
    let (total, offset, rows) = container_of(&json, PROCESS_3D_PLAY_DOCUMENT_STEPS);
    assert_eq!(total, 180, "a closed section still states its extent: {json}");
    assert_eq!(offset, 0);
    assert!(rows.is_empty(), "a closed section materialises nothing: {json}");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)`, keyed by the raw
/// step id the `geometry` domain targets — and each of those rows keeps BOTH its own row actions.
#[semio_framework_async_macros::async_test]
async fn a_steps_window_request_materialises_exactly_its_slice_with_its_row_actions() {
    let document = oversized_document();
    let view = window_view(PROCESS_3D_PLAY_DOCUMENT_STEPS, Some(true), 100, 6);
    let json = project(&document, &semio_framework_plugin::TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_ARTIFACT));
    let (total, offset, rows) = container_of(&json, PROCESS_3D_PLAY_DOCUMENT_STEPS);
    assert_eq!(total, 180);
    assert_eq!(offset, 100, "the stamped offset is the requested one: {json}");
    assert_eq!(rows, (100..106).map(|index| format!("step-{index:03}")).collect::<Vec<_>>(), "exactly entries [100, 106): {json}");
    assert!(json.contains("setStepEnabled"), "a materialised step keeps its enable row action: {json}");
    assert!(json.contains("removeStep"), "a materialised step keeps its remove row action: {json}");
}

/// ⚖️ LAW (d): step and stock rows pick through the tree's ONE domain binding — they declare their
/// `granularity` and carry no per-row activate BINDING (their row actions are a separate channel).
#[semio_framework_async_macros::async_test]
async fn document_rows_declare_their_granularity_and_pick_through_the_tree() {
    let document = oversized_document();
    let json = project(&document, &semio_framework_plugin::TreeWindows::unhosted());
    let projection: serde_json::Value = serde_json::from_str(&json).expect("document projection json");
    assert_eq!(projection["component"]["interactionDomain"].as_str(), Some(PROCESS3D_INTERACTION_DOMAIN), "{json}");
    assert_eq!(projection["bindings"].as_array().cloned().unwrap_or_default().iter().filter(|binding| binding["trigger"] == "activate").count(), 1, "exactly one tree-level interactionSelect: {json}");
    for section in projection["children"].as_array().cloned().unwrap_or_default() {
        for row in section["children"].as_array().cloned().unwrap_or_default() {
            assert_eq!(row["component"]["granularity"].as_str(), Some(PROCESS3D_GRANULARITY_OBJECT), "{row}");
            assert!(row["bindings"].as_array().map(|bindings| bindings.is_empty()).unwrap_or(true), "a pick row carries no binding of its own: {row}");
        }
    }
}
//#endregion 🪟️WindowLaws
