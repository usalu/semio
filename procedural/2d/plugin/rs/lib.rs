//! 🎲 Procedural 2D plugin — procedural flow play app bundled as a hot-swappable WASM component.

use flow_core::{FlowFixture, FlowHost, Widget};
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, Canvas2dScene, CommandDescriptor, PluginApp,
    PluginBundle, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const PROCEDURAL2D_PLAY_APP_ID: &str = "procedural2d-play";
const PROCEDURAL2D_PLAY_SURFACE_MAIN: &str = "procedural2d.play.main";
const PROCEDURAL2D_PLAY_SURFACE_PREVIEW: &str = "procedural2d.play.preview";
const PROCEDURAL2D_PLAY_BODY_MAIN: &str = "procedural2d.play.main";
const PROCEDURAL2D_PLAY_BODY_PREVIEW: &str = "procedural2d.play.preview";
const PROCEDURAL2D_PLAY_BODY_HIERARCHY: &str = "procedural2d.play.hierarchy";
const PROCEDURAL2D_PLAY_BODY_CATALOGUE: &str = "procedural2d.play.catalogue";
const PROCEDURAL2D_PLAY_BODY_INSPECTION: &str = "procedural2d.play.inspection";
const PROCEDURAL2D_PLAY_WINDOW_MAIN: &str = "procedural2d-main";
const PROCEDURAL2D_PLAY_WINDOW_PREVIEW: &str = "procedural2d-preview";
const DEFAULT_PROCEDURAL2D_FIXTURE_JSON: &str = include_str!("../../example/default.procedural2d.json");
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural2dPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default = "default_show_mode")]
    show_mode: String,
    #[serde(default)]
    eval_outputs_json: String,
    #[serde(default)]
    undo_stack: Vec<FlowFixture>,
    #[serde(default)]
    redo_stack: Vec<FlowFixture>,
}

fn default_show_mode() -> String {
    "preview".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural2dPlayEnvelope {
    fixture: FlowFixture,
    #[serde(default)]
    runtime: Procedural2dPlayRuntime,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_fixture() -> FlowFixture {
    serde_json::from_str(DEFAULT_PROCEDURAL2D_FIXTURE_JSON).unwrap_or_default()
}

fn default_envelope() -> Procedural2dPlayEnvelope {
    Procedural2dPlayEnvelope {
        fixture: default_fixture(),
        runtime: Procedural2dPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> Procedural2dPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Procedural2dPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn procedural2d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PROCEDURAL2D_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn host_from_envelope(envelope: &Procedural2dPlayEnvelope) -> FlowHost {
    FlowHost::from_fixture(envelope.fixture.clone())
}

fn push_undo(play: &mut Procedural2dPlayEnvelope) {
    play.runtime.undo_stack.push(play.fixture.clone());
    if play.runtime.undo_stack.len() > 32 {
        play.runtime.undo_stack.remove(0);
    }
    play.runtime.redo_stack.clear();
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}

fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputStepper { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn eval_preview_layers(play: &Procedural2dPlayEnvelope, preview: bool) -> String {
    let mut host = host_from_envelope(play);
    let eval_json = if play.runtime.eval_outputs_json.is_empty() {
        host.evaluate().unwrap_or_default()
    } else {
        host.apply_eval_outputs_json(&play.runtime.eval_outputs_json);
        play.runtime.eval_outputs_json.clone()
    };
    let offset = if preview { 240.0 } else { 0.0 };
    let mode = play.runtime.show_mode.as_str();
    let mut layers = vec![json!({
        "id": if preview { "procedural2d-preview.flow" } else { "procedural2d-main.flow" },
        "kind": "rect",
        "name": format!("Mode: {mode}"),
        "x": offset,
        "y": 0.0,
        "width": 180.0,
        "height": 72.0,
    })];
    if let Ok(outputs) = serde_json::from_str::<Value>(&eval_json) {
        if let Some(preview_value) = outputs.get("preview").or_else(|| outputs.get("outputs")) {
            layers.push(json!({
                "id": if preview { "procedural2d-preview.eval" } else { "procedural2d-main.eval" },
                "kind": "text",
                "name": "Eval",
                "x": offset + 24.0,
                "y": 96.0,
                "width": 220.0,
                "height": 120.0,
                "text": preview_value.to_string().chars().take(120).collect::<String>(),
            }));
        }
    }
    for widget in &play.fixture.widgets {
        let id = widget_id(widget).to_string();
        if play.runtime.selected_ids.is_empty() || play.runtime.selected_ids.iter().any(|selected| selected == &id) {
            layers.push(json!({
                "id": format!("widget-{id}"),
                "kind": "node",
                "name": id,
                "x": offset + 48.0,
                "y": 240.0,
                "width": 96.0,
                "height": 48.0,
            }));
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn tree_item(id: impl Into<String>, label: impl Into<String>, command: Option<CommandDescriptor>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        command,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_hierarchy_tree(play: &Procedural2dPlayEnvelope) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = play
        .fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            tree_item(
                format!("procedural2d-play-hierarchy.widget.{id}"),
                id.clone(),
                Some(procedural2d_cmd("setSelection", Some(json!({ "ids": [id] })))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "procedural2d-play-hierarchy.widgets".into(),
            label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
            default_open: Some(true),
            items: if widget_items.is_empty() {
                vec![tree_item("procedural2d-play-hierarchy.empty", "(none)", None)]
            } else {
                widget_items
            },
        }],
        selected_ids: Some(
            play.runtime
                .selected_ids
                .iter()
                .map(|id| format!("procedural2d-play-hierarchy.widget.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(procedural2d_cmd("setSelection", None)),
    })
}

fn build_catalogue_tree() -> UiNode {
    let mode_items = ["preview", "generate", "wire"]
        .iter()
        .map(|mode| {
            tree_item(
                format!("procedural2d-play-catalogue.mode.{mode}"),
                format!("Show {mode}"),
                Some(procedural2d_cmd("setShowMode", Some(json!({ "value": mode })))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "procedural2d-play-catalogue.modes".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items: mode_items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(play: &Procedural2dPlayEnvelope) -> UiNode {
    if play.runtime.selected_ids.is_empty() {
        return ui_stack_vertical(vec![
            ui_text("Schema: flow.fixture"),
            ui_text(format!("Widgets: {}", play.fixture.widgets.len())),
            ui_text(format!("Show mode: {}", play.runtime.show_mode)),
        ]);
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "procedural2d-play-inspector.selection".into(),
        label: "Selection".into(),
        default_open: Some(true),
        fields: vec![ui_inspector_readonly_field(
            "procedural2d-play-inspector.ids",
            "Ids",
            play.runtime.selected_ids.join(", "),
        )],
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_canvas(play: &Procedural2dPlayEnvelope) -> UiNode {
    build_canvas_2d_scene(
        PROCEDURAL2D_PLAY_SURFACE_MAIN,
        PROCEDURAL2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: play.fixture.camera.x,
            camera_y: play.fixture.camera.y,
            zoom: play.fixture.camera.zoom,
            layers_json: eval_preview_layers(play, false),
        },
    )
}

fn render_preview_canvas(play: &Procedural2dPlayEnvelope) -> UiNode {
    build_canvas_2d_scene(
        PROCEDURAL2D_PLAY_SURFACE_PREVIEW,
        PROCEDURAL2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: play.fixture.camera.x,
            camera_y: play.fixture.camera.y,
            zoom: play.fixture.camera.zoom,
            layers_json: eval_preview_layers(play, true),
        },
    )
}
//#endregion 🔖Render

//#region 🔖Procedural2dPlayApp
struct Procedural2dPlayApp;

impl PluginApp for Procedural2dPlayApp {
    fn app_id(&self) -> &str {
        PROCEDURAL2D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("procedural2d envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<Procedural2dPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                play.runtime.selected_ids = selection_ids(args);
                return vec![set_document_op(&play)];
            }
            "setShowMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    play.runtime.show_mode = mode.into();
                    return vec![set_document_op(&play)];
                }
            }
            "generate" => {
                push_undo(&mut play);
                let mut host = host_from_envelope(&play);
                play.runtime.eval_outputs_json = host.evaluate().unwrap_or_default();
                play.runtime.show_mode = "generate".into();
                return vec![set_document_op(&play)];
            }
            "setEvalOutputs" => {
                if let Some(outputs) = args.and_then(|value| value.get("outputs")) {
                    play.runtime.eval_outputs_json = outputs.to_string();
                    return vec![set_document_op(&play)];
                }
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    play.runtime.eval_outputs_json = json_text.into();
                    return vec![set_document_op(&play)];
                }
            }
            "undo" => {
                if let Some(previous) = play.runtime.undo_stack.pop() {
                    play.runtime.redo_stack.push(play.fixture.clone());
                    play.fixture = previous;
                    return vec![set_document_op(&play)];
                }
            }
            "redo" => {
                if let Some(next) = play.runtime.redo_stack.pop() {
                    play.runtime.undo_stack.push(play.fixture.clone());
                    play.fixture = next;
                    return vec![set_document_op(&play)];
                }
            }
            "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "canvasWheel" => {}
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            PROCEDURAL2D_PLAY_BODY_MAIN => render_main_canvas(&play),
            PROCEDURAL2D_PLAY_BODY_PREVIEW => render_preview_canvas(&play),
            PROCEDURAL2D_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&play),
            PROCEDURAL2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            PROCEDURAL2D_PLAY_BODY_INSPECTION => build_inspector_tree(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖Procedural2dPlayApp

//#region 🔖AppFactory
fn create_procedural2d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL2D_PLAY_APP_ID, "Procedural 2D")
            .icon_id("procedural2d")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_MAIN, "Main", PROCEDURAL2D_PLAY_BODY_MAIN)
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_PREVIEW, "Preview", PROCEDURAL2D_PLAY_BODY_PREVIEW)
            .default_layout(create_default_layout(
                &[PROCEDURAL2D_PLAY_WINDOW_MAIN.into(), PROCEDURAL2D_PLAY_WINDOW_PREVIEW.into()],
                "row",
                Some(&[55.0, 45.0]),
                Some(&["Main".into(), "Preview".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                PROCEDURAL2D_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                PROCEDURAL2D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                PROCEDURAL2D_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("default", "Default", serde_json::to_string(&default_envelope()).unwrap())
    .program("procedural2d", "Procedural 2D", "layout")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("procedural2d", "Procedural 2D", "0.1.0")
        .register_app(create_procedural2d_app(), || Box::new(Procedural2dPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_main_canvas_scene() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_preview_canvas_scene() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_PREVIEW, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn hierarchy_lists_widgets() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("procedural2d-play-hierarchy.widget.rect"));
    }

    #[test]
    fn catalogue_lists_show_modes() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("procedural2d-play-catalogue.mode.preview"));
    }

    #[test]
    fn generate_command_sets_eval_outputs() {
        let mut app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("generate", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Procedural2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.show_mode, "generate");
    }

    #[test]
    fn set_show_mode_updates_runtime() {
        let mut app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("setShowMode", Some(&json!({ "value": "wire" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Procedural2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.show_mode, "wire");
    }
}
//#endregion 🧪Tests
