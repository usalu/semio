//! ✍️ Writer plugin — declarative writer app bundled as a hot-swappable WASM component.

mod grammar;

use grammar::tokenize_language;
use semio_framework_plugin::{
    build_text_editor_scene, ui_declarative_sections_to_tree, ui_text, App,
    CommandDescriptor, PluginApp, PluginBundle, TextEditorScene, UiNode, UiSectionNode,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, create_default_layout,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const WRITER_PLAY_APP_ID: &str = "writer-play";
const WRITER_PLAY_CONTROLLER_ID: &str = "writer-play";
const WRITER_PLAY_SURFACE_ID: &str = "writer.play";
const WRITER_PLAY_BODY_MAIN: &str = "writer.play.main";
const WRITER_PLAY_BODY_HIERARCHY: &str = "writer.play.hierarchy";
const WRITER_PLAY_BODY_CATALOGUE: &str = "writer.play.catalogue";
const WRITER_PLAY_BODY_INSPECTION: &str = "writer.play.inspection";
const WRITER_PLAY_WINDOW_KIND: &str = "writer-main";
const WRITER_DOCUMENT_SCHEMA: &str = "writer.document";

const JACK_EXAMPLE_JSON: &str = include_str!("../../example/jack.writer.json");
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterCamera {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "default_zoom")]
    zoom: f64,
}

fn default_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterDocument {
    schema: String,
    id: String,
    language_id: String,
    #[serde(default = "default_uri")]
    uri: String,
    #[serde(default)]
    text: String,
    #[serde(default = "default_camera")]
    camera: WriterCamera,
}

fn default_uri() -> String {
    "writer://empty".into()
}

fn default_camera() -> WriterCamera {
    WriterCamera {
        x: 0.0,
        y: 0.0,
        zoom: 1.0,
    }
}

fn empty_writer_document() -> WriterDocument {
    WriterDocument {
        schema: WRITER_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        language_id: "plaintext".into(),
        uri: "writer://empty".into(),
        text: String::new(),
        camera: default_camera(),
    }
}

fn apply_writer_edit(mut document: WriterDocument, op: &Value) -> WriterDocument {
    let Some(op_name) = op.get("op").and_then(|value| value.as_str()) else {
        return document;
    };
    match op_name {
        "setDocument" => {
            if let Ok(next) = serde_json::from_value(
                op.get("document")
                    .cloned()
                    .unwrap_or(Value::Null),
            ) {
                document = next;
            }
        }
        "setText" => {
            if let Some(text) = op.get("text").and_then(|value| value.as_str()) {
                document.text = text.into();
            }
        }
        _ => {}
    }
    document
}
//#endregion 🔖Document

//#region 🔖JackAst
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JackAstNode {
    id: String,
    kind: String,
    label: String,
    start: usize,
    end: usize,
    #[serde(default)]
    children: Vec<JackAstNode>,
}

fn jack_ast_tree_icon(kind: &str) -> Option<&'static str> {
    match kind {
        "query" => Some("file-code"),
        "match" | "create" | "merge" => Some("git-branch"),
        "where" => Some("filter"),
        "return" => Some("corner-down-left"),
        "pattern" | "patternNode" => Some("box"),
        "edge" => Some("arrow-right"),
        "var" => Some("variable"),
        "label" | "property" => Some("tag"),
        "string" => Some("quote"),
        "number" | "bool" | "null" => Some("hash"),
        "error" => Some("alert-circle"),
        _ => None,
    }
}

fn jack_ast_node(kind: &str, start: usize, end: usize, source: &str, children: Vec<JackAstNode>, label: Option<&str>) -> JackAstNode {
    let slice = source.get(start..end).unwrap_or("").trim();
    JackAstNode {
        id: format!("jack-{start}-{end}"),
        kind: kind.into(),
        label: label.unwrap_or(slice).into(),
        start,
        end,
        children,
    }
}

fn parse_jack_ast(text: &str) -> JackAstNode {
    let upper = text.to_uppercase();
    let mut clauses = Vec::new();
    for keyword in ["MATCH", "CREATE", "MERGE", "WHERE", "RETURN"] {
        if let Some(index) = upper.find(keyword) {
            let end = upper[index..]
                .find(|c: char| c == '\n' && keyword != "WHERE")
                .map(|offset| index + offset)
                .unwrap_or(text.len());
            let kind = keyword.to_lowercase();
            clauses.push(jack_ast_node(
                &kind,
                index,
                end.max(index + keyword.len()),
                text,
                Vec::new(),
                Some(keyword),
            ));
        }
    }
    if clauses.is_empty() {
        return jack_ast_node("error", 0, text.len(), text, Vec::new(), Some("(empty query)"));
    }
    jack_ast_node("query", 0, text.len(), text, clauses, Some("query"))
}

fn jack_ast_to_tree_item(node: &JackAstNode) -> UiTreeItemNode {
    let children: Vec<UiTreeItemNode> = node.children.iter().map(jack_ast_to_tree_item).collect();
    UiTreeItemNode {
        id: node.id.clone(),
        label: node.label.clone(),
        description: Some(node.kind.clone()),
        icon_id: jack_ast_tree_icon(&node.kind).map(str::to_string),
        selected: None,
        default_open: Some(matches!(node.kind.as_str(), "query" | "match" | "pattern" | "return")),
        command: Some(play_cmd(
            WRITER_PLAY_CONTROLLER_ID,
            "selectAstNode",
            Some(json!({ "id": node.id, "start": node.start, "end": node.end })),
        )),
        draggable: None,
        drag_data: None,
        items: if children.is_empty() { None } else { Some(children) },
        control: None,
        is_hidden: None,
    }
}
//#endregion 🔖JackAst

//#region 🔖Panels
fn play_cmd(controller_id: &str, command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: controller_id.into(),
        command: command.into(),
        args,
    }
}

fn selection_from_view(view_state: &ViewState) -> Vec<String> {
    view_state
        .selection_json
        .as_ref()
        .and_then(|json| serde_json::from_str::<Value>(json).ok())
        .and_then(|value| {
            value
                .as_array()
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| item.as_str().map(str::to_string))
                        .collect()
                })
        })
        .unwrap_or_default()
}

fn render_hierarchy_panel(document: &WriterDocument, view_state: &ViewState) -> UiNode {
    if document.language_id != "jack" {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "writer-hierarchy".into(),
            label: Some("Document".into()),
            default_open: Some(true),
            children: vec![
                ui_text(document.id.clone()),
                ui_text(document.language_id.clone()),
            ],
        }]);
    }
    let root = parse_jack_ast(&document.text);
    let items = if root.kind == "error" {
        vec![jack_ast_to_tree_item(&root)]
    } else {
        root.children.iter().map(jack_ast_to_tree_item).collect()
    };
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "writer-play-hierarchy.ast".into(),
            label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
            default_open: Some(true),
            items: if items.is_empty() {
                vec![UiTreeItemNode {
                    id: "writer-play-hierarchy.empty".into(),
                    label: "(empty query)".into(),
                    description: None,
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    command: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                }]
            } else {
                items
            },
        }],
        selected_ids: Some(selection_from_view(view_state)),
        highlighted_ids: None,
        selection_change: Some(play_cmd(
            WRITER_PLAY_CONTROLLER_ID,
            "setAstSelection",
            None,
        )),
    })
}

fn render_catalogue_panel() -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "writer-catalogue".into(),
        label: Some("Language".into()),
        default_open: Some(true),
        children: vec![ui_text("jack — Cypher-inspired trinity query language")],
    }])
}

fn render_inspection_panel(document: &WriterDocument) -> UiNode {
    ui_declarative_sections_to_tree(&[
        UiSectionNode {
            id: "writer-inspector.document".into(),
            label: Some("Document".into()),
            default_open: Some(true),
            children: vec![
                ui_text(format!("Schema: {}", document.schema)),
                ui_text(format!("Id: {}", document.id)),
                ui_text(format!("Language: {}", document.language_id)),
                ui_text(format!("Uri: {}", document.uri)),
                ui_text(format!("Lines: {}", document.text.lines().count())),
            ],
        },
        UiSectionNode {
            id: "writer-inspector.camera".into(),
            label: Some("Camera".into()),
            default_open: Some(false),
            children: vec![
                ui_text(format!("x: {}", document.camera.x)),
                ui_text(format!("y: {}", document.camera.y)),
                ui_text(format!("zoom: {}", document.camera.zoom)),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Scene
fn render_main_scene(document: &WriterDocument, view_state: &ViewState) -> UiNode {
    let selection_json = view_state.selection_json.clone().or_else(|| {
        view_state
            .panel_json
            .as_ref()
            .and_then(|json| serde_json::from_str::<Value>(json).ok())
            .and_then(|value| value.get("editorSelection").cloned())
            .map(|value| value.to_string())
    });
    let tokens = tokenize_language(&document.text, &document.language_id);
    let tokens_json = serde_json::to_string(&tokens).ok();
    build_text_editor_scene(
        WRITER_PLAY_SURFACE_ID,
        WRITER_PLAY_CONTROLLER_ID,
        TextEditorScene {
            buffer: document.text.clone(),
            language: Some(document.language_id.clone()),
            selection_json,
            tokens_json,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
        },
    )
}
//#endregion 🔖Scene

//#region 🔖WriterApp
struct WriterApp;

impl PluginApp for WriterApp {
    fn app_id(&self) -> &str {
        WRITER_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&empty_writer_document()).expect("writer document json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut document: WriterDocument =
            serde_json::from_str(document_json).unwrap_or_else(|_| empty_writer_document());
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        document = parsed;
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
            }
            "setDocumentJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(parsed) = serde_json::from_str(json_text) {
                        document = parsed;
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
            }
            "setText" => {
                if let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) {
                    document.text = text.into();
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        document.camera = parsed;
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
            }
            "selectAstNode" | "setEditorSelection" => {
                return Vec::new();
            }
            "setAstSelection" | "setAstHover" | "setEditorHover" | "formatDocument" | "lintDocument"
            | "toggleLineNumbers" | "setEditorSetting" => {}
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let document: WriterDocument =
            serde_json::from_str(document_json).unwrap_or_else(|_| empty_writer_document());
        match body_key {
            WRITER_PLAY_BODY_MAIN => render_main_scene(&document, view_state),
            WRITER_PLAY_BODY_HIERARCHY => render_hierarchy_panel(&document, view_state),
            WRITER_PLAY_BODY_CATALOGUE => render_catalogue_panel(),
            WRITER_PLAY_BODY_INSPECTION => render_inspection_panel(&document),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖WriterApp

//#region 🔖Manifest
fn create_writer_app() -> App {
    App::from_builder(
        App::builder(WRITER_PLAY_APP_ID, "Writer")
            .icon_id("writer")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(WRITER_PLAY_WINDOW_KIND, "Jack", WRITER_PLAY_BODY_MAIN)
            .default_layout(create_default_layout(
                &[WRITER_PLAY_WINDOW_KIND.into()],
                "row",
                Some(&[100.0]),
                Some(&["Jack".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                WRITER_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                WRITER_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                WRITER_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("empty", "Empty", serde_json::to_string(&empty_writer_document()).unwrap())
    .example("jack", "Jack", JACK_EXAMPLE_JSON)
    .program("writer", "Writer", "text.document")
}

fn writer_bundle() -> PluginBundle {
    PluginBundle::new("writer", "Writer", "0.1.0").register_app(create_writer_app(), || Box::new(WriterApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(writer_bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_text_editor_scene() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let node = app.render(WRITER_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn renders_hierarchy_tree_for_jack() {
        let app = WriterApp;
        let document = JACK_EXAMPLE_JSON.to_string();
        let node = app.render(WRITER_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("MATCH"));
    }

    #[test]
    fn renders_catalogue_panel() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let node = app.render(WRITER_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("jack"));
    }

    #[test]
    fn set_text_command_updates_document() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let ops = app.handle_command(
            "setText",
            Some(&json!({ "text": "MATCH (a) RETURN a" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        assert!(ops[0].contains("MATCH"));
    }
}
//#endregion 🧪Tests
