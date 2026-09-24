use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, render as render_body, render_with_view};
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

const MODES_SECTION: &str = "procedural2d-play-catalogue.modes";
const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";

#[semio_framework_async_macros::async_test]
async fn generation2d_labels_resolve_native_english_by_default() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE).await;
    close(app);
    assert!(json.contains("\"Components\""));
    assert!(json.contains("\"Show mode\""));
    assert!(json.contains("\"Inputs\""));
    assert!(json.contains("\"Outputs\""));
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
    let modes = node_of(&json, MODES_SECTION);
    for row in modes["children"].as_array().expect("mode rows") {
        let rendered = serde_json::to_string(row).expect("mode row");
        assert!(!rendered.contains(FLOW_WIDGET_DRAG_MIME), "show mode stays a click, not a widget drag: {rendered}");
    }
}

/// ⚖️ Every palette group stamps the total from `flow_palette_catalogue_sections`, and the show-mode
/// section still states its three modes. Nothing summarises a remainder with `+n`.
#[semio_framework_async_macros::async_test]
async fn every_catalogue_section_stamps_its_total_and_none_continues() {
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    assert!(sections.iter().any(|section| section.id == "inputs"), "inputs is a palette section");
    assert!(sections.iter().any(|section| section.id == "outputs"), "outputs is a palette section");
    assert!(sections.iter().any(|section| section.items.iter().any(|item| item.kind == "neuron")), "operator sections are neurons");
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE).await;
    close(app);
    for section in &sections {
        let key = format!("procedural2d-play-catalogue.{}", section.id);
        assert_eq!(section_of(&json, &key).0, section.items.len() as u64, "{key} stamps its palette total: {json}");
    }
    assert_eq!(section_of(&json, MODES_SECTION).0, 3, "show mode still states three modes: {json}");
    assert_eq!(section_of(&json, MODES_SECTION).2, 0, "show mode stays closed: {json}");
    assert!(!json.contains(".more\""), "a windowed catalogue has no continuation row: {json}");
    assert!(!json.contains(r#""label":"+"#), "a windowed catalogue publishes no `+n` label: {json}");
}

/// 🖱️ Opened input and output windows drag `application/x-flow-widget`, including format and action.
/// The inputs window is requested explicitly: which group opens by default is the FIRST registered
/// section, and the process-wide flow registry this test binary shares may hold operator sections
/// that sort before it.
#[semio_framework_async_macros::async_test]
async fn component_rows_drag_the_flow_widget_descriptor() {
    let inputs_json = {
        let input_section = semio_framework_os_flow::flow_palette_catalogue_sections().into_iter().find(|section| section.id == "inputs").expect("inputs section");
        let mut app = app().await;
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: GENERATION2D_PLAY_BODY_CATALOGUE.into(), node_key: format!("{GENERATION2D_PLAY_CATALOGUE_SECTION}{}procedural2d-play-catalogue.inputs", semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR), open: Some(true), offset: 0, rows: input_section.items.len() as u32 }],
            ..Default::default()
        };
        let json = render_with_view(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE, &view).await;
        close(app);
        json
    };
    let inputs = node_of(&inputs_json, "procedural2d-play-catalogue.inputs")["children"].as_array().cloned().unwrap_or_default();
    let slider = inputs.iter().find(|row| row["key"].as_str() == Some("procedural2d-play-catalogue.inputSlider")).expect("slider row");
    let slider_row = serde_json::to_string(slider).expect("slider");
    assert!(slider_row.contains("inputSlider"), "{slider_row}");
    assert!(slider_row.contains(FLOW_WIDGET_DRAG_MIME), "{slider_row}");
    assert!(slider_row.contains(r#""draggable":true"#), "{slider_row}");

    let outputs = semio_framework_os_flow::flow_palette_catalogue_sections().into_iter().find(|section| section.id == "outputs").expect("outputs section");
    let mut app = app().await;
    let view = ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: GENERATION2D_PLAY_BODY_CATALOGUE.into(), node_key: format!("{GENERATION2D_PLAY_CATALOGUE_SECTION}{}procedural2d-play-catalogue.outputs", semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR), open: Some(true), offset: 0, rows: outputs.items.len() as u32 }],
        ..Default::default()
    };
    let json = render_with_view(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE, &view).await;
    close(app);
    let output_rows = node_of(&json, "procedural2d-play-catalogue.outputs")["children"].as_array().cloned().unwrap_or_default();
    assert_eq!(output_rows.len(), outputs.items.len(), "every output row in the window is materialised: {json}");
    let mut saw_format = false;
    let mut saw_action = false;
    for row in &output_rows {
        let rendered = serde_json::to_string(row).expect("output row");
        assert!(rendered.contains(FLOW_WIDGET_DRAG_MIME), "component row drag: {rendered}");
        assert!(rendered.contains(r#""draggable":true"#), "{rendered}");
        saw_format |= rendered.contains("svg");
        saw_action |= rendered.contains("log");
    }
    assert!(saw_format, "an export row drag carries format: {json}");
    assert!(saw_action, "an action row drag carries action: {json}");
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    if let Some((section, index, neuron_kind)) = sections.iter().find_map(|section| {
        section.items.iter().position(|item| item.kind == "neuron").map(|index| (section, index, section.items[index].neuron_kind.clone().unwrap_or_default()))
    }) {
        let mut neuron_app = crate::editor::generation2d::unit_tests::context::app().await;
        let node_key = format!("{GENERATION2D_PLAY_CATALOGUE_SECTION}{}procedural2d-play-catalogue.{}", semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR, section.id);
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: GENERATION2D_PLAY_BODY_CATALOGUE.into(), node_key, open: Some(true), offset: index as u32, rows: 1 }],
            ..Default::default()
        };
        let json = render_with_view(&mut neuron_app, GENERATION2D_PLAY_BODY_CATALOGUE, &view).await;
        close(neuron_app);
        let rows = node_of(&json, &format!("procedural2d-play-catalogue.{}", section.id))["children"].as_array().cloned().unwrap_or_default();
        assert_eq!(rows.len(), 1, "the neuron window materialises the requested row: {json}");
        let row = serde_json::to_string(&rows[0]).expect("neuron row");
        assert!(row.contains(&neuron_kind), "{row}");
        assert!(row.contains("neuronKind"), "{row}");
        assert!(row.contains(FLOW_WIDGET_DRAG_MIME), "{row}");
    }

}

/// 🔎️ `(total, offset, materialised rows)` of one section, read off the rendered body by its key.
fn section_of(json: &str, key: &str) -> (u64, u64, usize) {
    let node = node_of(json, key);
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{key} stamps no window: {json}"));
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        node["children"].as_array().map(Vec::len).unwrap_or_default(),
    )
}

fn node_of(json: &str, key: &str) -> serde_json::Value {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("catalogue projection json");
    walk(&projection, key).unwrap_or_else(|| panic!("{key} is not in the rendered catalogue: {json}"))
}
