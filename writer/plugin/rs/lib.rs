//! ✍️ Writer plugin — declarative writer app bundled as a hot-swappable WASM component.

mod grammar {
// #region grammar
//! ✍️ Lightweight grammar tokenization for writer plugin scenes.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrammarToken {
    pub class: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
struct GrammarRule {
    pattern: regex::Regex,
    class: &'static str,
}

fn jack_rules() -> Vec<GrammarRule> {
    vec![
        GrammarRule {
            pattern: regex::Regex::new(r"(?i)\b(MATCH|WHERE|RETURN|CREATE|DELETE|SET|MERGE|AND|OR)\b").expect("jack keyword"),
            class: "keyword",
        },
        GrammarRule {
            pattern: regex::Regex::new(r#"'[^']*'|"[^"]*""#).expect("jack string"),
            class: "string",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b\d+(?:\.\d+)?\b").expect("jack number"),
            class: "number",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"->|!=|[:=.,\[\]()-]").expect("jack operator"),
            class: "operator",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b[A-Za-z_][A-Za-z0-9_]*\b").expect("jack ident"),
            class: "ident",
        },
    ]
}

fn wire_rules() -> Vec<GrammarRule> {
    vec![
        GrammarRule {
            pattern: regex::Regex::new(r"->").expect("wire keyword"),
            class: "keyword",
        },
        GrammarRule {
            pattern: regex::Regex::new(r#"'[^']*'|"[^"]*""#).expect("wire string"),
            class: "string",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b\d+(?:\.\d+)?\b").expect("wire number"),
            class: "number",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"[:@{}.,\[\]-]").expect("wire operator"),
            class: "operator",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b[A-Za-z_][A-Za-z0-9_.-]*\b").expect("wire ident"),
            class: "ident",
        },
    ]
}

/** @emoji 🎨 Tokenizes source text for a supported writer language id. */
pub fn tokenize_language(text: &str, language_id: &str) -> Vec<GrammarToken> {
    let rules = match language_id {
        "jack" => jack_rules(),
        "wire" => wire_rules(),
        _ => return Vec::new(),
    };
    let mut occupied = vec![false; text.len()];
    let mut tokens = Vec::new();
    for rule in rules {
        for capture in rule.pattern.find_iter(text) {
            let start = capture.start();
            let end = capture.end();
            if occupied[start..end].iter().any(|filled| *filled) {
                continue;
            }
            for slot in &mut occupied[start..end] {
                *slot = true;
            }
            tokens.push(GrammarToken {
                class: rule.class.into(),
                start,
                end,
            });
        }
    }
    tokens.sort_by_key(|token| (token.start, std::cmp::Reverse(token.end)));
    tokens
}
// #endregion grammar
}


use grammar::tokenize_language;
use trinity_jack::{complete, example_graph, format as jack_format, lint, semantic_tokens, Diagnostic};
use semio_framework_plugin::{SurfaceKind, PanelGroup, 
    build_text_editor_scene, ui_declarative_sections_to_tree, ui_text, App,
    CommandDescriptor, PluginApp, PluginBundle, TextEditorScene, UiNode, UiSectionNode,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
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
const WRITER_PLAY_BODY_DOCUMENT: &str = "writer.play.document";
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterEditorSelection {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterEditorSettings {
    #[serde(default)]
    show_line_numbers: bool,
    #[serde(default = "default_font_px")]
    font_px: u32,
    #[serde(default = "default_line_height")]
    line_height: u32,
    #[serde(default = "default_tab_size")]
    tab_size: u32,
}

fn default_font_px() -> u32 {
    14
}

fn default_line_height() -> u32 {
    20
}

fn default_tab_size() -> u32 {
    2
}

impl Default for WriterEditorSettings {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            font_px: default_font_px(),
            line_height: default_line_height(),
            tab_size: default_tab_size(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterPlayRuntime {
    #[serde(default)]
    selected_ast_ids: Vec<String>,
    #[serde(default)]
    editor_selection: Option<WriterEditorSelection>,
    #[serde(default)]
    format_signal: u32,
    #[serde(default)]
    lint_signal: u32,
    #[serde(default)]
    revision: u32,
    #[serde(default)]
    editor_settings: WriterEditorSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterPlayEnvelope {
    #[serde(flatten)]
    document: WriterDocument,
    #[serde(default)]
    runtime: WriterPlayRuntime,
    #[serde(default)]
    undo_stack: Vec<WriterDocument>,
    #[serde(default)]
    redo_stack: Vec<WriterDocument>,
}

fn parse_envelope(document_json: &str) -> WriterPlayEnvelope {
    if let Ok(envelope) = serde_json::from_str::<WriterPlayEnvelope>(document_json) {
        return envelope;
    }
    let document: WriterDocument = serde_json::from_str(document_json).unwrap_or_else(|_| empty_writer_document());
    WriterPlayEnvelope {
        document,
        runtime: WriterPlayRuntime::default(),
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
    }
}

fn set_document_op(envelope: &WriterPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn push_undo_writer(play: &mut WriterPlayEnvelope) {
    play.undo_stack.push(play.document.clone());
    if play.undo_stack.len() > 32 {
        play.undo_stack.remove(0);
    }
    play.redo_stack.clear();
}

fn jack_ast_node_for_selection(root: &JackAstNode, start: usize, end: usize) -> Option<&JackAstNode> {
    if start > end {
        return None;
    }
    fn visit<'a>(node: &'a JackAstNode, start: usize, end: usize) -> Option<&'a JackAstNode> {
        if start >= node.start && end <= node.end {
            let deeper = node
                .children
                .iter()
                .find_map(|child| visit(child, start, end));
            Some(deeper.unwrap_or(node))
        } else {
            None
        }
    }
    visit(root, start, end)
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
        hover_command: None,
        unhover_command: None,
        actions: None,
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

fn render_document_panel(document: &WriterDocument, runtime: &WriterPlayRuntime) -> UiNode {
    if document.language_id != "jack" {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "writer-document".into(),
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
            id: "writer-play-document.ast".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            items: if items.is_empty() {
                vec![UiTreeItemNode {
                    id: "writer-play-document.empty".into(),
                    label: "(empty query)".into(),
                    description: None,
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    command: None,
                    hover_command: None,
                    unhover_command: None,
                    actions: None,
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
        selected_ids: Some(runtime.selected_ast_ids.clone()),
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

//#region 🔖JackEditor
fn identifier_bounds_at(text: &str, offset: usize) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut index = offset.min(bytes.len());
    while index > 0 && (bytes[index - 1] as char).is_ascii_alphanumeric() || bytes[index - 1] == b'_' {
        index -= 1;
    }
    let start = index;
    while index < bytes.len() && (bytes[index] as char).is_ascii_alphanumeric() || bytes[index] == b'_' {
        index += 1;
    }
    if start == index {
        return None;
    }
    Some((start, index))
}

fn jack_occurrences_json(text: &str, cursor: usize) -> Option<String> {
    let (start, end) = identifier_bounds_at(text, cursor)?;
    let needle = &text[start..end];
    if needle.is_empty() {
        return None;
    }
    let mut ranges = Vec::new();
    let mut scan = 0usize;
    while let Some(found) = text[scan..].find(needle) {
        let at = scan + found;
        let next_end = at + needle.len();
        if identifier_bounds_at(text, at) == Some((at, next_end)) {
            ranges.push(json!({ "start": at, "end": next_end }));
        }
        scan = at + needle.len();
    }
    Some(
        json!({
            "selection": serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into()),
            "hover": serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into()),
        })
        .to_string(),
    )
}

/// 🪞 Canonical jack format when possible, else a whitespace-only normalization for other languages.
fn format_writer_text(text: &str, language_id: &str) -> String {
    if language_id == "jack" {
        if let Ok(formatted) = jack_format(text) {
            return formatted;
        }
    }
    let mut normalized: String = text
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    if !text.is_empty() && !normalized.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}

fn jack_completions_json(text: &str, cursor: usize) -> Option<String> {
    let graph = example_graph();
    let items: Vec<Value> = complete(&graph, text, cursor)
        .into_iter()
        .map(|item| json!({ "label": item.label, "detail": item.detail }))
        .collect();
    serde_json::to_string(&items).ok()
}
//#endregion 🔖JackEditor

//#region 🔖Scene
fn render_main_scene(document: &WriterDocument, runtime: &WriterPlayRuntime) -> UiNode {
    let selection_json = runtime.editor_selection.as_ref().map(|selection| {
        json!({ "start": selection.start, "end": selection.end }).to_string()
    });
    let tokens_json = if document.language_id == "jack" {
        serde_json::to_string(&semantic_tokens(&document.text)).ok()
    } else {
        let tokens = tokenize_language(&document.text, &document.language_id);
        serde_json::to_string(&tokens).ok()
    };
    let diagnostics_json = if document.language_id == "jack" {
        let graph = example_graph();
        let diagnostics: Vec<serde_json::Value> = lint(&graph, &document.text)
            .into_iter()
            .map(|diag: Diagnostic| {
                json!({
                    "start": diag.start,
                    "end": diag.end,
                    "severity": diag.severity,
                    "message": diag.message
                })
            })
            .collect();
        Some(serde_json::to_string(&diagnostics).unwrap_or_else(|_| "[]".into()))
    } else if runtime.lint_signal > 0 {
        Some(json!([{
            "start": 0,
            "end": document.text.len().max(1),
            "severity": "info",
            "message": format!("Lint pass #{}", runtime.lint_signal)
        }]).to_string())
    } else {
        None
    };
    let cursor = runtime.editor_selection.as_ref().map(|selection| selection.end).unwrap_or(0);
    let occurrences_json = (document.language_id == "jack").then(|| jack_occurrences_json(&document.text, cursor)).flatten();
    let completions_json = if document.language_id == "jack" {
        jack_completions_json(&document.text, cursor)
    } else if runtime.format_signal > 0 {
        Some(json!([{ "label": "format", "detail": format!("pass #{}", runtime.format_signal) }]).to_string())
    } else {
        None
    };
    build_text_editor_scene(
        WRITER_PLAY_SURFACE_ID,
        WRITER_PLAY_CONTROLLER_ID,
        TextEditorScene {
            buffer: document.text.clone(),
            language: Some(document.language_id.clone()),
            selection_json,
            tokens_json,
            diagnostics_json,
            completions_json,
            occurrences_json,
            overlays_json: runtime
                .editor_settings
                .show_line_numbers
                .then(|| json!({ "lineNumbers": true }).to_string()),
            settings_json: Some(serde_json::to_string(&runtime.editor_settings).unwrap_or_else(|_| "{}".into())),
            camera_json: Some(
                json!({ "x": document.camera.x, "y": document.camera.y, "zoom": document.camera.zoom }).to_string(),
            ),
            ..TextEditorScene::base(String::new(), None, None)
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
        serde_json::to_string(&WriterPlayEnvelope {
            document: empty_writer_document(),
            runtime: WriterPlayRuntime::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
        .expect("writer document json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        match command {
            "textEdit" | "setDocument" => {
                if let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) {
                    push_undo_writer(&mut play);
                    play.document.text = text.into();
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<WriterPlayEnvelope>(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                    if let Ok(parsed) = serde_json::from_value::<WriterDocument>(next.clone()) {
                        play.document = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setDocumentJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(parsed) = serde_json::from_str(json_text) {
                        push_undo_writer(&mut play);
                        play.document = parsed;
                        play.runtime.revision += 1;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setText" => {
                if let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) {
                    push_undo_writer(&mut play);
                    play.document.text = text.into();
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        play.document.camera = parsed;
                        play.runtime.revision += 1;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "formatDocument" => {
                let formatted = format_writer_text(&play.document.text, &play.document.language_id);
                if formatted != play.document.text {
                    push_undo_writer(&mut play);
                    play.document.text = formatted;
                }
                play.runtime.format_signal += 1;
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "requestCompletions" => {
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "commitRename" => {
                if let (Some(start), Some(end), Some(text)) = (
                    args.and_then(|value| value.get("start")).and_then(|value| value.as_u64()),
                    args.and_then(|value| value.get("end")).and_then(|value| value.as_u64()),
                    args.and_then(|value| value.get("text")).and_then(|value| value.as_str()),
                ) {
                    let start = start as usize;
                    let end = end as usize;
                    push_undo_writer(&mut play);
                    let mut next = play.document.text.clone();
                    if start <= end && end <= next.len() {
                        next.replace_range(start..end, text);
                        play.document.text = next;
                        play.runtime.revision += 1;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "lintDocument" => {
                play.runtime.lint_signal += 1;
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "textSelect" | "selectAstNode" => {
                if command == "textSelect" {
                    let start = args.and_then(|value| value.get("start")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                    let end = args.and_then(|value| value.get("end")).and_then(|value| value.as_u64()).unwrap_or(start as u64) as usize;
                    play.runtime.editor_selection = Some(WriterEditorSelection { start, end });
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
                let start = args.and_then(|value| value.get("start")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                let end = args.and_then(|value| value.get("end")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                play.runtime.selected_ast_ids = if id.is_empty() { Vec::new() } else { vec![id.into()] };
                play.runtime.editor_selection = Some(WriterEditorSelection { start, end });
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "setEditorSelection" => {
                let start = args.and_then(|value| value.get("start")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                let end = args.and_then(|value| value.get("end")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                play.runtime.editor_selection = Some(WriterEditorSelection { start, end });
                if play.document.language_id == "jack" {
                    let root = parse_jack_ast(&play.document.text);
                    play.runtime.selected_ast_ids = jack_ast_node_for_selection(&root, start, end)
                        .map(|node| vec![node.id.clone()])
                        .unwrap_or_default();
                } else {
                    play.runtime.selected_ast_ids.clear();
                }
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "setAstSelection" => {
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| value.as_array())
                    .map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                play.runtime.selected_ast_ids = ids.clone();
                if let Some(id) = ids.first() {
                    if play.document.language_id == "jack" {
                        let root = parse_jack_ast(&play.document.text);
                        let selection = root
                            .children
                            .iter()
                            .chain(std::iter::once(&root))
                            .find(|node| node.id == *id)
                            .map(|node| WriterEditorSelection {
                                start: node.start,
                                end: node.end,
                            });
                        play.runtime.editor_selection = selection;
                    }
                }
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "setAstHover" | "setEditorHover" => {
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "toggleLineNumbers" => {
                play.runtime.editor_settings.show_line_numbers = !play.runtime.editor_settings.show_line_numbers;
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "setEditorSetting" => {
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                match field {
                    "fontPx" => {
                        if let Some(px) = value.and_then(|v| v.as_u64()) {
                            play.runtime.editor_settings.font_px = px as u32;
                        }
                    }
                    "lineHeight" => {
                        if let Some(px) = value.and_then(|v| v.as_u64()) {
                            play.runtime.editor_settings.line_height = px as u32;
                        }
                    }
                    "tabSize" => {
                        if let Some(px) = value.and_then(|v| v.as_u64()) {
                            play.runtime.editor_settings.tab_size = px.max(1) as u32;
                        }
                    }
                    _ => return Vec::new(),
                }
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "undo" => {
                if let Some(previous) = play.undo_stack.pop() {
                    play.redo_stack.push(play.document.clone());
                    play.document = previous;
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            "redo" => {
                if let Some(next) = play.redo_stack.pop() {
                    play.undo_stack.push(play.document.clone());
                    play.document = next;
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            WRITER_PLAY_BODY_MAIN => render_main_scene(&play.document, &play.runtime),
            WRITER_PLAY_BODY_DOCUMENT => render_document_panel(&play.document, &play.runtime),
            WRITER_PLAY_BODY_CATALOGUE => render_catalogue_panel(),
            WRITER_PLAY_BODY_INSPECTION => render_inspection_panel(&play.document),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖WriterApp

//#region 🔖Manifest
fn create_writer_app() -> App {
    App::from_builder(
        App::builder(WRITER_PLAY_APP_ID, "Writer").document(["semio", "writer"])
            .icon_id("writer")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(WRITER_PLAY_WINDOW_KIND, "Jack", WRITER_PLAY_BODY_MAIN, SurfaceKind::TextEditor)
            .default_layout(create_default_layout(
                &[WRITER_PLAY_WINDOW_KIND.into()],
                "row",
                Some(&[100.0]),
                Some(&["Jack".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                WRITER_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                WRITER_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
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

semio_framework_plugin::plugin_exports!(writer_bundle);
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
    fn renders_document_tree_for_jack() {
        let app = WriterApp;
        let document = JACK_EXAMPLE_JSON.to_string();
        let node = app.render(WRITER_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
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
    fn format_document_reformats_jack_query() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&WriterPlayEnvelope {
            document: WriterDocument {
                schema: WRITER_DOCUMENT_SCHEMA.into(),
                id: "jack".into(),
                language_id: "jack".into(),
                uri: "writer://jack".into(),
                text: "MATCH (a:Piece)   WHERE a.name='core' RETURN a.name".into(),
                camera: default_camera(),
            },
            runtime: WriterPlayRuntime::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
        .unwrap();
        let ops = app.handle_command_patch_ops("formatDocument", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let envelope: WriterPlayEnvelope = serde_json::from_str(
            &serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string(),
        )
        .unwrap();
        assert!(envelope.document.text.contains('\n'));
        assert_eq!(envelope.runtime.format_signal, 1);
    }

    #[test]
    fn jack_completions_use_example_fixture() {
        let json = jack_completions_json("RETURN a.", 9).unwrap_or_default();
        assert!(!json.is_empty());
    }

    #[test]
    fn set_text_command_updates_document() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let ops = app.handle_command_patch_ops(
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
