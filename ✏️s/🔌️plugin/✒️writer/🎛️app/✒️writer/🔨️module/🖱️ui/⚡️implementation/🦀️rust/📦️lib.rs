//! ✍️ Writer app — `DocumentApp` impl, render, manifest (constitutional: ui). B1: the pure-trait
//! flip — `WriterPlayApp` is a unit struct; every former `WriterPlayRuntime` field (selection, hover,
//! camera, editor settings, signals, engagement draft) now lives in `writer_engine::WriterConfig`,
//! written via `writer_op::WriterConfigOperation`s (real `backwards`, no ad hoc `InverseAction`); every
//! action dispatches through the single typed `writer_protocol::WriterCommand` channel via
//! `DocumentApp::handle` — mirrors `shooting_ui::ShootingPlayApp` (the B1 pilot) exactly.

use trinity_jack::{example_graph, lint, semantic_tokens, Diagnostic};
use writer::{WriterCamera, WriterProjection, WRITER_DOCUMENT_SCHEMA};
use writer_engine::{
    apply_jack_rename, dag_jack_example_document, dag_jack_example_json, empty_writer_projection, find_deepest_jack_ast_node_at, format_writer_text, jack_ast_node_by_id,
    jack_ast_node_for_selection, jack_ast_tree_icon, jack_completions_json, jack_editor_placeholders, jack_example_document, jack_example_json, jack_newline_gate_offsets, jack_symbol_at_offset,
    parse_jack_ast, selectable_spans_for_jack, tokenize_language, writer_chapter_payload, JackAstNode, JackSymbolKind, WriterConfig, WriterEditorSelection,
};
use writer_op::{WriterConfigOperation, WriterOperation};
use writer_protocol::WriterCommand;
use semio_framework_plugin::{SurfaceKind, PanelGroup, PanelTabSpec,
    build_text_editor_scene, engagement_token_matches, localized_label_map, strip_engagement_prefix,
    tree_item, ui_declarative_sections_to_tree, ui_text, App,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, ActionDescriptor, AppIo, AppLabelsOverlay, AppLabelsOverlayExt,
    DocumentApp, DocumentView, ConfigView, Emit, IconName, LocaleLabels, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, PanelTreeBuilder, ArtifactKindSpec, TextEditorScene, UiNode, UiPresence, UiSectionNode,
    UiTreeItemNode, WindowEngagement, WindowEngagementInput,
    WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus, WindowMeasure,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, create_default_layout,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::DocumentPack;

//#region 🔖️Constants
pub const WRITER_PLAY_APP_ID: &str = "writer-play";
const WRITER_PLAY_CONTROLLER_ID: &str = "writer-play";
const WRITER_PLAY_SURFACE_ID: &str = "writer.play";
const WRITER_PLAY_BODY_MAIN: &str = "writer.play.main";
const WRITER_PLAY_BODY_DOCUMENT: &str = "writer.play.document";
const WRITER_PLAY_BODY_CATALOGUE: &str = "writer.play.catalogue";
const WRITER_PLAY_BODY_INSPECTION: &str = "writer.play.inspection";
/// 🌳️ Nested children of the document tab — demonstrates the recursive panel-tab tree (stacked tab rows).
const WRITER_PANEL_TAB_DOCUMENT_CONTENT_ID: &str = "framework.panel.document.content";
const WRITER_PANEL_TAB_DOCUMENT_OUTLINE_ID: &str = "framework.panel.document.outline";
const WRITER_PLAY_WINDOW_KIND: &str = "writer-main";
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`'s identical region.
fn is_de_locale(cfg: &WriterConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &WriterConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn jack_ast_to_tree_item(node: &JackAstNode) -> UiTreeItemNode {
    let children: Vec<UiTreeItemNode> = node.children.iter().map(jack_ast_to_tree_item).collect();
    UiTreeItemNode {
        id: node.id.clone(),
        label: node.label.clone(),
        description: Some(node.kind.clone()),
        // 🛟️ `and_then(IconName::from_str)` (not the panicking `IconName::from`) so a jack AST kind
        // whose icon string isn't (yet) in the shared icon catalog just renders with no icon.
        icon_id: jack_ast_tree_icon(&node.kind).and_then(IconName::from_str),
        presence: UiPresence::default(),
        default_open: Some(matches!(node.kind.as_str(), "query" | "match" | "pattern" | "return")),
        action: Some(play_action(
            WRITER_PLAY_CONTROLLER_ID,
            "selectAstNode",
            Some(json!({ "id": node.id, "start": node.start, "end": node.end })),
        )),
        hover_action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "setAstHover", Some(json!({ "id": node.id })))),
        unhover_action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "setAstHover", Some(json!({ "id": Value::Null })))),
        actions: None,
        draggable: None,
        drag_data: None,
        items: if children.is_empty() { None } else { Some(children) },
        control: None,
        dimmed: None,
        menu: None,
    }
}

/// 🐁️ Resolves tree/editor hover cross-highlighting: (highlighted AST id, tree-hover span, hover occurrences).
fn editor_hover_context(document: &WriterProjection, config: &WriterConfig) -> (Option<String>, Option<(usize, usize)>, Vec<(usize, usize)>) {
    if document.language_id != "jack" {
        return (None, None, Vec::new());
    }
    let root = parse_jack_ast(&document.text);
    let tree_span = config.tree_hovered_ast_id.as_ref().and_then(|id| jack_ast_node_by_id(&root, id)).map(|node| (node.start, node.end));
    let editor_hovered_ast_id = config.editor_hover_offset.and_then(|offset| find_deepest_jack_ast_node_at(&root, offset)).map(|node| node.id.clone());
    let highlighted = config.tree_hovered_ast_id.clone().or(editor_hovered_ast_id);
    let hover_occurrences = config
        .editor_hover_offset
        .and_then(|offset| jack_symbol_at_offset(&document.text, offset))
        .filter(|symbol| symbol.kind == JackSymbolKind::Variable)
        .map(|symbol| symbol.occurrences)
        .unwrap_or_default();
    (highlighted, tree_span, hover_occurrences)
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the writer app; one field per label makes every locale combination compile-checked.
    struct WriterPlayLabels {
        document: &'static str = en: "Document", de: "Dokument";
        empty_query: &'static str = en: "(empty query)", de: "(leere Abfrage)";
        language: &'static str = en: "Language", de: "Sprache";
        jack_description: &'static str = en: "jack — Cypher-inspired trinity query language", de: "jack — von Cypher inspirierte Trinity-Abfragesprache";
        camera: &'static str = en: "Camera", de: "Kamera";
        diagnostics: &'static str = en: "Diagnostics", de: "Diagnosen";
        format: &'static str = en: "Format", de: "Formatieren";
        lint: &'static str = en: "Lint", de: "Prüfen";
        line_numbers: &'static str = en: "Line numbers", de: "Zeilennummern";
        font_size: &'static str = en: "Font size", de: "Schriftgröße";
        line_height: &'static str = en: "Line height", de: "Zeilenhöhe";
        tab_size: &'static str = en: "Tab size", de: "Tabulatorgröße";
        engagement_placeholder: &'static str = en: "Format, lint, line numbers", de: "Format, prüfen, Zeilennummern";
        editor_mode_status: &'static str = en: "Text editor", de: "Texteditor";
        window_main: &'static str = en: "Jack", de: "Jack";
        mode_edit: &'static str = en: "Edit", de: "Bearbeiten";
        panel_tab_content: &'static str = en: "Content", de: "Inhalt";
        panel_tab_outline: &'static str = en: "Outline", de: "Gliederung";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_writer_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn writer_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("formatDocument", "Format Document", "Dokument formatieren"),
        ("lintDocument", "Lint Document", "Dokument prüfen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("textEdit", "Edit Text", "Text bearbeiten"),
        ("setText", "Set Text", "Text festlegen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("commitRename", "Commit Rename", "Umbenennung übernehmen"),
        ("engagementSubmit", "Engagement Submit", "Eingabe bestätigen"),
        ("setDocument", "Set Document", "Dokument festlegen"),
        ("setDocumentJson", "Set Document JSON", "Dokument-JSON festlegen"),
        ("setFixtureJson", "Set Fixture JSON", "Fixture-JSON festlegen"),
        ("requestCompletions", "Request Completions", "Vervollständigungen anfordern"),
        ("textSelect", "Text Select", "Text auswählen"),
        ("setEditorSelection", "Set Editor Selection", "Editor-Auswahl festlegen"),
        ("selectAstNode", "Select Ast Node", "AST-Knoten auswählen"),
        ("setAstSelection", "Set Ast Selection", "AST-Auswahl festlegen"),
        ("setAstHover", "Set Ast Hover", "Überfahren (AST) festlegen"),
        ("textHover", "Text Hover", "Text-Hover"),
        ("toggleLineNumbers", "Toggle Line Numbers", "Zeilennummern umschalten"),
        ("setEditorSetting", "Set Editor Setting", "Editor-Einstellung festlegen"),
        ("engagementInput", "Engagement Input", "Eingabe"),
    ];
    localized_label_map(is_de, ENTRIES)
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_writer_app`.
/// Writer declares no utilities today; kept for parity with the other apps' `app_labels()` wiring.
fn writer_utility_labels(_is_de: bool) -> HashMap<String, String> {
    HashMap::new()
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
    }
}

fn render_document_panel(document: &WriterProjection, config: &WriterConfig, labels: &WriterPlayLabels) -> UiNode {
    if document.language_id != "jack" {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "writer-document".into(),
            label: Some(labels.document.into()),
            default_open: Some(true),
            children: vec![ui_text(document.id.clone()), ui_text(document.language_id.clone())],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let root = parse_jack_ast(&document.text);
    let items = if root.kind == "error" {
        vec![UiTreeItemNode {
            description: Some(root.kind.clone()),
            icon_id: jack_ast_tree_icon(&root.kind).and_then(IconName::from_str),
            ..tree_item(root.id.as_str(), root.label.as_str())
        }]
    } else {
        vec![jack_ast_to_tree_item(&root)]
    };
    let (highlighted_ast_id, _, _) = editor_hover_context(document, config);
    PanelTreeBuilder::new("writer-play-document")
        .section_or_placeholder("writer-play-document.ast", Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()), true, items, labels.empty_query)
        .selected(config.selected_ast_ids.clone())
        .highlighted(highlighted_ast_id.map(|id| vec![id]).unwrap_or_default())
        .selection_change(play_action(WRITER_PLAY_CONTROLLER_ID, "setAstSelection", None))
        .build()
}

fn render_catalogue_panel(labels: &WriterPlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "writer-catalogue".into(),
        label: Some(labels.language.into()),
        default_open: Some(true),
        children: vec![ui_text(labels.jack_description)],
        presence: UiPresence::default(),
        menu: None,
    }])
}

fn render_inspection_panel(document: &WriterProjection, config: &WriterConfig, labels: &WriterPlayLabels) -> UiNode {
    let mut sections = vec![
        UiSectionNode {
            id: "writer-inspector.document".into(),
            label: Some(labels.document.into()),
            default_open: Some(true),
            children: vec![
                ui_text(format!("Schema: {}", document.schema)),
                ui_text(format!("Id: {}", document.id)),
                ui_text(format!("Language: {}", document.language_id)),
                ui_text(format!("Uri: {}", document.uri)),
                ui_text(format!("Lines: {}", document.text.lines().count())),
            ],
            presence: UiPresence::default(),
            menu: None,
        },
        UiSectionNode {
            id: "writer-inspector.camera".into(),
            label: Some(labels.camera.into()),
            default_open: Some(false),
            children: vec![
                ui_text(format!("x: {}", config.camera.x)),
                ui_text(format!("y: {}", config.camera.y)),
                ui_text(format!("zoom: {}", config.camera.zoom)),
            ],
            presence: UiPresence::default(),
            menu: None,
        },
    ];
    if document.language_id == "jack" {
        let graph = example_graph();
        let messages: Vec<String> = lint(&graph, &document.text).into_iter().map(|diag: Diagnostic| diag.message).take(8).collect();
        if !messages.is_empty() {
            sections.push(UiSectionNode {
                id: "writer-inspector.diagnostics".into(),
                label: Some(labels.diagnostics.into()),
                default_open: Some(true),
                children: messages.into_iter().map(ui_text).collect(),
                presence: UiPresence::default(),
                menu: None,
            });
        }
    }
    ui_declarative_sections_to_tree(&sections)
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_main_scene(document: &WriterProjection, config: &WriterConfig) -> UiNode {
    let is_jack = document.language_id == "jack";
    let selection = config.editor_selection.clone().unwrap_or(WriterEditorSelection { start: 0, end: 0 });
    let cursor = selection.end;
    let selection_json = Some(json!({ "start": selection.start, "end": selection.end }).to_string());

    let grammar_tokens = tokenize_language(&document.text, &document.language_id);
    let tokens_json = if is_jack {
        serde_json::to_string(&semantic_tokens(&document.text)).ok()
    } else {
        serde_json::to_string(&grammar_tokens).ok()
    };

    let diagnostics_json = if is_jack {
        let graph = example_graph();
        let diagnostics: Vec<Value> = lint(&graph, &document.text)
            .into_iter()
            .map(|diag: Diagnostic| json!({ "start": diag.start, "end": diag.end, "severity": diag.severity, "message": diag.message }))
            .collect();
        Some(serde_json::to_string(&diagnostics).unwrap_or_else(|_| "[]".into()))
    } else if config.lint_signal > 0 {
        Some(json!([{ "start": 0, "end": document.text.len().max(1), "severity": "info", "message": format!("Lint pass #{}", config.lint_signal) }]).to_string())
    } else {
        None
    };

    let selectable_spans_json = is_jack.then(|| serde_json::to_string(&selectable_spans_for_jack(&document.text, &grammar_tokens)).unwrap_or_else(|_| "[]".into()));
    let placeholders_json = is_jack.then(|| serde_json::to_string(&jack_editor_placeholders(&document.text, cursor)).unwrap_or_else(|_| "[]".into()));
    let newline_gates_json = is_jack.then(|| serde_json::to_string(&jack_newline_gate_offsets(&document.text)).unwrap_or_else(|_| "[]".into()));

    let (_, tree_hover_span, hover_occurrences) = editor_hover_context(document, config);
    let hover_json = Some(match tree_hover_span {
        Some((start, end)) => json!({ "start": start, "end": end }).to_string(),
        None => "null".to_string(),
    });

    let caret_symbol = if is_jack && selection.start == selection.end { jack_symbol_at_offset(&document.text, selection.start) } else { None };
    let (selection_occurrences, rename_json): (Vec<(usize, usize)>, Option<String>) = match &caret_symbol {
        Some(symbol) if symbol.kind == JackSymbolKind::Variable => {
            let occurrences_json: Vec<Value> = symbol.occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
            let rename = json!({ "name": symbol.name, "occurrences": occurrences_json }).to_string();
            (symbol.occurrences.clone(), Some(rename))
        }
        _ => (Vec::new(), None),
    };

    let occurrences_json = is_jack.then(|| {
        let hover: Vec<Value> = hover_occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
        let selection: Vec<Value> = selection_occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
        json!({
            "hover": serde_json::to_string(&hover).unwrap_or_else(|_| "[]".into()),
            "selection": serde_json::to_string(&selection).unwrap_or_else(|_| "[]".into()),
        })
        .to_string()
    });

    let extra_carets_json = (!selection_occurrences.is_empty())
        .then(|| serde_json::to_string(&selection_occurrences.iter().map(|(s, _)| *s).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into()));

    let completions_json = is_jack.then(|| jack_completions_json(&document.text, cursor)).flatten();

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
            overlays_json: config.editor_settings.show_line_numbers.then(|| json!({ "lineNumbers": true }).to_string()),
            placeholders_json,
            extra_carets_json,
            selectable_spans_json,
            settings_json: Some(serde_json::to_string(&config.editor_settings).unwrap_or_else(|_| "{}".into())),
            camera_json: Some(json!({ "x": config.camera.x, "y": config.camera.y, "zoom": config.camera.zoom }).to_string()),
            hover_json,
            newline_gates_json,
            rename_json,
        },
    )
}
//#endregion 🔖️Render

//#region 🔖️Engagement
/// 📤️ What `apply_engagement` computes: an optional document text replacement plus the config
/// operations the interaction always produces (mirrors `Emit`'s document/config split — `handle`
/// below wires this straight into one `Emit`).
struct WriterEngagementOutcome {
    text: Option<String>,
    config_operations: Vec<WriterConfigOperation>,
}

/// 💬️ Natural-language engagement parsing (premigration `applyEngagement`). Accepts both the
/// spaced form (wgpu REPL) and the React shell's PascalCased, separator-stripped drafts (e.g.
/// `"Font16"`, `"LineNumbers"` — see `strip_engagement_prefix`). B1: pure — computes the config
/// operations this interaction produces instead of mutating a `&mut runtime` in place.
fn apply_engagement(config: &WriterConfig, current_text: &str, language_id: &str, value: &str) -> WriterEngagementOutcome {
    let trimmed = value.trim();
    let mut config_operations = vec![
        WriterConfigOperation::SetEngagementInput { value: String::new() },
        WriterConfigOperation::SetRevision { value: config.revision + 1 },
    ];
    if trimmed.is_empty() {
        return WriterEngagementOutcome { text: None, config_operations };
    }
    if engagement_token_matches(trimmed, "format") {
        config_operations.push(WriterConfigOperation::SetFormatSignal { value: config.format_signal + 1 });
        let formatted = format_writer_text(current_text, language_id);
        let text = (formatted != current_text).then_some(formatted);
        return WriterEngagementOutcome { text, config_operations };
    }
    if engagement_token_matches(trimmed, "lint") {
        config_operations.push(WriterConfigOperation::SetLintSignal { value: config.lint_signal + 1 });
        return WriterEngagementOutcome { text: None, config_operations };
    }
    if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
        let mut settings = config.editor_settings.clone();
        settings.show_line_numbers = !settings.show_line_numbers;
        config_operations.push(WriterConfigOperation::SetEditorSettings { settings });
        return WriterEngagementOutcome { text: None, config_operations };
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
        if let Ok(px) = rest.parse::<u32>() {
            let mut settings = config.editor_settings.clone();
            settings.font_px = px;
            config_operations.push(WriterConfigOperation::SetEditorSettings { settings });
        }
        return WriterEngagementOutcome { text: None, config_operations };
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
        if let Ok(size) = rest.parse::<u32>() {
            let mut settings = config.editor_settings.clone();
            settings.tab_size = size.max(1);
            config_operations.push(WriterConfigOperation::SetEditorSettings { settings });
        }
    }
    WriterEngagementOutcome { text: None, config_operations }
}
//#endregion 🔖️Engagement

//#region 🔖️WriterPlayApp
/// 🧪️ B1: unit struct — every former `WriterPlayRuntime` field now lives in
/// `writer_engine::WriterConfig` (see `DocumentApp::Config`), written through
/// `writer_op::WriterConfigOperation`s.
#[derive(Default)]
pub struct WriterPlayApp;

/// 🖱️ On-demand writer text-editor context menu from caret/selection/completions context.
fn writer_context_menu_items(
    text: Option<&semio_framework_plugin::ContextMenuTextContext>,
    is_de: bool,
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::ContextMenuItemSpec;
    let text = text;
    let can_suggest = text.map(|t| t.has_completions).unwrap_or(false);
    let has_selection = text.map(|t| t.has_selection).unwrap_or(false);
    let can_rename = text.map(|t| t.can_rename).unwrap_or(false);
    let item = |id: &str, label: &str, icon: &str, action: &str, disabled: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        disabled: disabled.then_some(true),
        ..Default::default()
    };
    let sep = |id: &str| ContextMenuItemSpec { id: id.into(), separator: Some(true), ..Default::default() };
    let mut items = Vec::new();
    if can_suggest {
        items.push(item("writer-suggest", if is_de { "Vervollständigungen vorschlagen" } else { "Suggest completions" }, "sparkles", "requestCompletions", false));
        items.push(sep("writer-suggest-sep"));
    }
    items.push(item("writer-select-token", if is_de { "Token auswählen" } else { "Select token" }, "text-cursor", "selectToken", false));
    items.push(item("writer-select-line", if is_de { "Zeile auswählen" } else { "Select line" }, "list-ordered", "selectLine", false));
    items.push(item("writer-select-all", if is_de { "Alles auswählen" } else { "Select All" }, "select-all", "selectAll", false));
    if can_rename {
        items.push(item("writer-rename", if is_de { "Umbenennen" } else { "Rename" }, "edit-3", "commitRename", false));
    }
    items.push(sep("writer-clip-sep"));
    items.push(item("writer-cut", if is_de { "Ausschneiden" } else { "Cut" }, "scissors", "cut", !has_selection));
    items.push(item("writer-copy", if is_de { "Kopieren" } else { "Copy" }, "copy", "copy", !has_selection));
    items.push(item("writer-paste", if is_de { "Einfügen" } else { "Paste" }, "clipboard", "paste", false));
    items.push(sep("writer-format-sep"));
    items.push(item("writer-format", if is_de { "Dokument formatieren" } else { "Format document" }, "align-left", "formatDocument", false));
    items.push(item("writer-lint", if is_de { "Dokument prüfen" } else { "Lint document" }, "alert-circle", "lintDocument", false));
    items
}

impl DocumentApp for WriterPlayApp {
    type Projection = WriterProjection;
    type Operation = WriterOperation;
    type Config = WriterConfig;
    type ConfigOperation = WriterConfigOperation;
    type Command = WriterCommand;

    fn app_id(&self) -> &str {
        WRITER_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        WRITER_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> WriterProjection {
        empty_writer_projection()
    }

    fn io(&self) -> Option<AppIo> {
        Some(writer_engine::writer_io())
    }

    fn whole_document_operation(&self, projection: WriterProjection) -> Option<WriterOperation> {
        Some(WriterOperation::SetDocument { document: projection })
    }

    /// 🏷️ Maps each `WriterCommand` variant back to the action id it was declared under in
    /// `create_writer_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &WriterCommand) -> &str {
        match command {
            WriterCommand::TextEdit { .. } => "textEdit",
            WriterCommand::SetText { .. } => "setText",
            WriterCommand::SetDocument { .. } => "setDocument",
            WriterCommand::SetDocumentJson { .. } => "setDocumentJson",
            WriterCommand::SetFixtureJson { .. } => "setFixtureJson",
            WriterCommand::SetActiveExample { .. } => "setActiveExample",
            WriterCommand::FormatDocument => "formatDocument",
            WriterCommand::CommitRename { .. } => "commitRename",
            WriterCommand::SetCamera { .. } => "setCamera",
            WriterCommand::RequestCompletions => "requestCompletions",
            WriterCommand::LintDocument => "lintDocument",
            WriterCommand::TextSelect { .. } => "textSelect",
            WriterCommand::SetEditorSelection { .. } => "setEditorSelection",
            WriterCommand::SelectAstNode { .. } => "selectAstNode",
            WriterCommand::SetAstSelection { .. } => "setAstSelection",
            WriterCommand::SetAstHover { .. } => "setAstHover",
            WriterCommand::TextHover { .. } => "textHover",
            WriterCommand::ToggleLineNumbers => "toggleLineNumbers",
            WriterCommand::SetFontPx { .. } | WriterCommand::SetLineHeight { .. } | WriterCommand::SetTabSize { .. } => "setEditorSetting",
            WriterCommand::EngagementInput { .. } => "engagementInput",
            WriterCommand::EngagementSubmit { .. } => "engagementSubmit",
            WriterCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(
        &self,
        command: &WriterCommand,
        doc: &DocumentView<'_, WriterProjection>,
        cfg: &ConfigView<'_, WriterConfig>,
    ) -> Emit<WriterOperation, WriterConfigOperation> {
        let document = doc.projection;
        let config = cfg.projection;
        match command {
            WriterCommand::TextEdit { text } => {
                // ⌨️ Keystroke-granular edits coalesce under a stable key so a typing burst amends into
                // a few undo steps, not one-per-keystroke. Any interrupting command applies without this
                // key and breaks the coalescing run.
                Emit::amend(vec![WriterOperation::SetText { text: text.clone() }], "writer-text-edit")
            }
            WriterCommand::SetText { text } => {
                // 🪙️ A discrete document replacement (unlike `TextEdit`'s keystroke bursts) — each call
                // is its own undo step, so it must NOT share `TextEdit`'s coalescing key.
                Emit::operations(vec![WriterOperation::SetText { text: text.clone() }])
            }
            WriterCommand::SetDocument { document } => Emit::operations(vec![WriterOperation::SetDocument { document: document.clone() }]),
            WriterCommand::SetDocumentJson { json } | WriterCommand::SetFixtureJson { json } => match serde_json::from_str::<WriterProjection>(json) {
                Ok(document) => Emit::operations(vec![WriterOperation::SetDocument { document }]),
                Err(_) => Emit::default(),
            },
            WriterCommand::SetActiveExample { example_id } => {
                let document = match example_id.as_str() {
                    "jack" => jack_example_document(),
                    "dag.jack" => dag_jack_example_document(),
                    _ => empty_writer_projection(),
                };
                Emit::operations(vec![WriterOperation::SetDocument { document }])
            }
            WriterCommand::FormatDocument => {
                let formatted = format_writer_text(&document.text, &document.language_id);
                let mut emit = Emit::config(vec![WriterConfigOperation::SetFormatSignal { value: config.format_signal + 1 }]);
                if formatted != document.text {
                    emit.document_operations = vec![WriterOperation::SetText { text: formatted }];
                }
                emit
            }
            WriterCommand::CommitRename { text } => {
                let selection = config.editor_selection.clone().unwrap_or(WriterEditorSelection { start: 0, end: 0 });
                if selection.start == selection.end {
                    if let Some(symbol) = jack_symbol_at_offset(&document.text, selection.start) {
                        if symbol.kind == JackSymbolKind::Variable {
                            let renamed = apply_jack_rename(&document.text, &symbol.occurrences, text);
                            return Emit::operations(vec![WriterOperation::SetText { text: renamed }]);
                        }
                    }
                }
                if selection.start <= selection.end && selection.end <= document.text.len() {
                    let mut updated = document.text.clone();
                    updated.replace_range(selection.start..selection.end, text);
                    return Emit::operations(vec![WriterOperation::SetText { text: updated }]);
                }
                Emit::default()
            }
            // 🎥️ View command: the editor viewport never touches the document — config-only.
            WriterCommand::SetCamera { camera } => Emit::config(vec![WriterConfigOperation::SetCamera { camera: camera.clone() }]),
            WriterCommand::RequestCompletions => Emit::config(vec![WriterConfigOperation::SetRevision { value: config.revision + 1 }]),
            WriterCommand::LintDocument => Emit::config(vec![
                WriterConfigOperation::SetLintSignal { value: config.lint_signal + 1 },
                WriterConfigOperation::SetRevision { value: config.revision + 1 },
            ]),
            WriterCommand::TextSelect { start, end } | WriterCommand::SetEditorSelection { start, end } => {
                let mut ops = vec![WriterConfigOperation::SetEditorSelection { selection: Some(WriterEditorSelection { start: *start, end: *end }) }];
                let ids = if document.language_id == "jack" {
                    let root = parse_jack_ast(&document.text);
                    jack_ast_node_for_selection(&root, (*start).min(*end), (*start).max(*end)).map(|node| vec![node.id.clone()]).unwrap_or_default()
                } else {
                    Vec::new()
                };
                ops.push(WriterConfigOperation::SetSelectedAstIds { ids });
                ops.push(WriterConfigOperation::SetRevision { value: config.revision + 1 });
                Emit::config(ops)
            }
            WriterCommand::SelectAstNode { id, start, end } => {
                let ids = if id.is_empty() { Vec::new() } else { vec![id.clone()] };
                Emit::config(vec![
                    WriterConfigOperation::SetSelectedAstIds { ids },
                    WriterConfigOperation::SetEditorSelection { selection: Some(WriterEditorSelection { start: *start, end: *end }) },
                    WriterConfigOperation::SetRevision { value: config.revision + 1 },
                ])
            }
            WriterCommand::SetAstSelection { ids } => {
                let mut ops = vec![WriterConfigOperation::SetSelectedAstIds { ids: ids.clone() }];
                if let Some(id) = ids.first() {
                    if document.language_id == "jack" {
                        let root = parse_jack_ast(&document.text);
                        if let Some(node) = jack_ast_node_by_id(&root, id) {
                            ops.push(WriterConfigOperation::SetEditorSelection { selection: Some(WriterEditorSelection { start: node.start, end: node.end }) });
                        }
                    }
                }
                ops.push(WriterConfigOperation::SetRevision { value: config.revision + 1 });
                Emit::config(ops)
            }
            WriterCommand::SetAstHover { id } => {
                if *id != config.tree_hovered_ast_id {
                    Emit::config(vec![
                        WriterConfigOperation::SetTreeHoveredAstId { id: id.clone() },
                        WriterConfigOperation::SetRevision { value: config.revision + 1 },
                    ])
                } else {
                    Emit::default()
                }
            }
            WriterCommand::TextHover { start, end } => {
                let offset = match (start, end) {
                    (Some(s), Some(e)) => Some(s + e.saturating_sub(*s) / 2),
                    _ => None,
                };
                if offset != config.editor_hover_offset {
                    Emit::config(vec![
                        WriterConfigOperation::SetEditorHoverOffset { offset },
                        WriterConfigOperation::SetRevision { value: config.revision + 1 },
                    ])
                } else {
                    Emit::default()
                }
            }
            WriterCommand::ToggleLineNumbers => {
                let mut settings = config.editor_settings.clone();
                settings.show_line_numbers = !settings.show_line_numbers;
                Emit::config(vec![WriterConfigOperation::SetEditorSettings { settings }, WriterConfigOperation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::SetFontPx { value } => {
                let mut settings = config.editor_settings.clone();
                settings.font_px = *value;
                Emit::config(vec![WriterConfigOperation::SetEditorSettings { settings }, WriterConfigOperation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::SetLineHeight { value } => {
                let mut settings = config.editor_settings.clone();
                settings.line_height = *value;
                Emit::config(vec![WriterConfigOperation::SetEditorSettings { settings }, WriterConfigOperation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::SetTabSize { value } => {
                let mut settings = config.editor_settings.clone();
                settings.tab_size = (*value).max(1);
                Emit::config(vec![WriterConfigOperation::SetEditorSettings { settings }, WriterConfigOperation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::EngagementInput { value } => {
                if *value != config.engagement_input {
                    Emit::config(vec![WriterConfigOperation::SetEngagementInput { value: value.clone() }, WriterConfigOperation::SetRevision { value: config.revision + 1 }])
                } else {
                    Emit::default()
                }
            }
            WriterCommand::EngagementSubmit { value } => {
                let value = value.clone().unwrap_or_else(|| config.engagement_input.clone());
                let outcome = apply_engagement(config, &document.text, &document.language_id, &value);
                Emit {
                    document_operations: outcome.text.map(|text| vec![WriterOperation::SetText { text }]).unwrap_or_default(),
                    config_operations: outcome.config_operations,
                    ..Default::default()
                }
            }
            WriterCommand::SetLocale { value } => Emit::config(vec![WriterConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    /// 🎞️ `"text:out"` exports the writer document's current text as one "chapter" payload (see
    /// `writer_engine::writer_chapter_payload`) — `playbook`'s `"chapters:in"` is the intended
    /// consumer. Falls through to the default whole-document-pack export for `"document:out"`
    /// (duplicated inline, not delegated — Rust traits have no `super` call for an overridden default).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, WriterProjection>) -> Result<Media, MediaError> {
        if port == "text:out" {
            let payload = writer_chapter_payload(doc.projection);
            let json = serde_json::to_string(&payload).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            return Ok(Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json } });
        }
        if port != "document:out" {
            return Err(MediaError::NotImplemented);
        }
        let bytes = doc.projection.encode_pack();
        Ok(Media {
            media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
            payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
        })
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let labels = resolve_labels::<WriterPlayLabels>(config);
        match body_key {
            WRITER_PLAY_BODY_MAIN => render_main_scene(document, config),
            WRITER_PLAY_BODY_DOCUMENT => render_document_panel(document, config, labels),
            WRITER_PLAY_BODY_CATALOGUE => render_catalogue_panel(labels),
            WRITER_PLAY_BODY_INSPECTION => render_inspection_panel(document, config, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.projection;
        let labels = resolve_labels::<WriterPlayLabels>(config);
        let engagement = WindowEngagement {
            session_active: Some(false),
            options: Some(vec![WindowEngagementOption {
                id: "writer-line-numbers".into(),
                label: Some(labels.line_numbers.into()),
                icon_id: Some("list-ordered".into()),
                pressed: Some(config.editor_settings.show_line_numbers),
                disabled: None,
                action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "toggleLineNumbers", None)),
            }]),
            input: Some(WindowEngagementInput {
                id: Some("writer-engagement-input".into()),
                value: Some(config.engagement_input.clone()),
                placeholder: Some(labels.engagement_placeholder.into()),
                disabled: None,
                on_change: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "engagementInput", None)),
                on_submit: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "engagementSubmit", None)),
                on_repeat_last: None,
                on_abort: None,
            }),
            control: None,
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "writer-editor-mode".into(), text: labels.editor_mode_status.into() }]),
            possible_engagements: Some(vec![
                WindowEngagementPossible { id: "writer-format".into(), label: labels.format.into(), detail: None, action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "formatDocument", None)) },
                WindowEngagementPossible { id: "writer-lint".into(), label: labels.lint.into(), detail: None, action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "lintDocument", None)) },
                WindowEngagementPossible { id: "writer-line-numbers".into(), label: labels.line_numbers.into(), detail: None, action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "toggleLineNumbers", None)) },
            ]),
        };
        HashMap::from([(WRITER_PLAY_WINDOW_KIND.to_string(), engagement)])
    }

    fn window_measures(&self, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let settings = &cfg.projection.editor_settings;
        let labels = resolve_labels::<WriterPlayLabels>(cfg.projection);
        let measures = vec![
            WindowMeasure::Slider {
                id: "writer-font-size-measure".into(),
                label: Some(labels.font_size.into()),
                value: settings.font_px as f64,
                min: 10.0,
                max: 24.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "setEditorSetting", Some(json!({ "field": "fontPx" }))),
                },
            WindowMeasure::Slider {
                id: "writer-line-height-measure".into(),
                label: Some(labels.line_height.into()),
                value: settings.line_height as f64,
                min: 16.0,
                max: 40.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "setEditorSetting", Some(json!({ "field": "lineHeight" }))),
                },
            WindowMeasure::Slider {
                id: "writer-tab-size-measure".into(),
                label: Some(labels.tab_size.into()),
                value: settings.tab_size as f64,
                min: 1.0,
                max: 8.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "setEditorSetting", Some(json!({ "field": "tabSize" }))),
                },
            WindowMeasure::Toggle {
                id: "writer-line-numbers-measure".into(),
                icon_id: "list-ordered".into(),
                label: Some(labels.line_numbers.into()),
                pressed: settings.show_line_numbers,
                text: None,
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "toggleLineNumbers", None),
            },
        ];
        HashMap::from([(WRITER_PLAY_WINDOW_KIND.to_string(), measures)])
    }

    fn app_labels(&self, cfg: &ConfigView<'_, WriterConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<WriterPlayLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        AppLabelsOverlay::default()
            .window_kind_label(WRITER_PLAY_WINDOW_KIND, labels.window_main)
            .panel_tab_label(WRITER_PANEL_TAB_DOCUMENT_CONTENT_ID, labels.panel_tab_content)
            .panel_tab_label(WRITER_PANEL_TAB_DOCUMENT_OUTLINE_ID, labels.panel_tab_outline)
            .mode_label("edit", labels.mode_edit)
            .action_labels(writer_action_labels(is_de))
            .utility_labels(writer_utility_labels(is_de))
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, WriterProjection>,
        cfg: &ConfigView<'_, WriterConfig>,
        _registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let is_de = is_de_locale(cfg.projection);
        let text = request.surface.as_ref().and_then(|surface| surface.text.as_ref());
        writer_context_menu_items(text, is_de)
    }
}
//#endregion 🔖️WriterPlayApp

//#region 🔖️Manifest
/// 🙈️ An internal document operation kept out of the command palette — editor events (text edits,
/// camera, rename, engagement submit) and dev-only whole-document setters dispatched from chrome.
fn writer_hidden_operation(id: &str, label: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::Operation) }
}

/// 🙈️ An internal View action kept out of the palette — ephemeral editor/selection/hover/setting events
/// that mutate only runtime scratch and emit no document operations.
fn writer_hidden_view(id: &str, label: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::View) }
}

pub fn create_writer_app() -> App {
    App::from_builder(
        App::builder(WRITER_PLAY_APP_ID, "Writer").document(["semio", "writer"])
            .artifact_kind(ArtifactKindSpec {
                id: "text.document".into(),
                name: "Text Document".into(),
                source_format: "writer.document".into(),
                component_kind: "writer".into(),
                dimension: "text".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
                schema: "writer.document".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("writer")
            .mode("edit", "Edit", "pencil")
            .default_mode_id("edit")
            .window_kind(WRITER_PLAY_WINDOW_KIND, "Jack", WRITER_PLAY_BODY_MAIN, SurfaceKind::TextEditor, "document-jack")
            .default_layout(create_default_layout(
                &[WRITER_PLAY_WINDOW_KIND.into()],
                "row",
                Some(&[100.0]),
                Some(&["Jack".into()]),
            ))
            .panel_tab_tree(PanelTabSpec::group(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                vec![
                    PanelTabSpec::leaf(WRITER_PANEL_TAB_DOCUMENT_CONTENT_ID, "Content", PanelGroup::Workbench, WRITER_PLAY_BODY_DOCUMENT),
                    PanelTabSpec::leaf(WRITER_PANEL_TAB_DOCUMENT_OUTLINE_ID, "Outline", PanelGroup::Workbench, WRITER_PLAY_BODY_DOCUMENT),
                ],
            ))
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
            // 🔧️ Panel-visible P0 effects: format rewrites the buffer (Operation), lint re-runs
            // diagnostics into runtime (View — an effect, not a document operation).
            .operation("formatDocument", "Format Document")
            .view_action("lintDocument", "Lint Document")
            // 🔧️ P1 example switch (whole-document load) with a staged example choice.
            .operation("setActiveExample", "Set Active Example")
            // 🙈️ Internal document operations — text edits (coalesced), aliases, camera, rename, engagement,
            // and dev-only whole-document JSON setters.
            .action_with(writer_hidden_operation("textEdit", "Edit Text"))
            .action_with(writer_hidden_operation("setText", "Set Text"))
            .action_with(writer_hidden_view("setCamera", "Set Camera"))
            .action_with(writer_hidden_operation("commitRename", "Commit Rename"))
            .action_with(writer_hidden_operation("engagementSubmit", "Engagement Submit"))
            .action_with(writer_hidden_operation("setDocument", "Set Document"))
            .action_with(writer_hidden_operation("setDocumentJson", "Set Document JSON"))
            .action_with(writer_hidden_operation("setFixtureJson", "Set Fixture JSON"))
            // 🙈️ Internal View measures — selection, hover, AST navigation, completions, editor settings.
            .action_with(writer_hidden_view("requestCompletions", "Request Completions"))
            .action_with(writer_hidden_view("textSelect", "Text Select"))
            .action_with(writer_hidden_view("setEditorSelection", "Set Editor Selection"))
            .action_with(writer_hidden_view("selectAstNode", "Select Ast Node"))
            .action_with(writer_hidden_view("setAstSelection", "Set Ast Selection"))
            .action_with(writer_hidden_view("setAstHover", "Set Ast Hover"))
            .action_with(writer_hidden_view("textHover", "Text Hover"))
            .action_with(writer_hidden_view("toggleLineNumbers", "Toggle Line Numbers"))
            .action_with(writer_hidden_view("setEditorSetting", "Set Editor Setting"))
            .action_with(writer_hidden_view("engagementInput", "Engagement Input"))
            // 📝️ Staged argument forms: example choice + the dev JSON setters.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new("jack", "Jack"),
                    ActionArgOption::new("dag.jack", "Dag Jack"),
                ]).default_value("jack"),
            ])
            .action_args("setDocumentJson", vec![ActionArgDef::text("json", "Document JSON")])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", "Fixture JSON")])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🎯️ Typed channel surface (mirrors `shooting_ui::create_shooting_app`'s identical wiring) —
            // `writer_engine::writer_io()` is the single source of truth for both the trait's `io()`
            // override and this manifest declaration.
            .config(WriterPlayApp.config_spec())
            .io(writer_engine::writer_io()),
    )
    .example("jack", "Jack", jack_example_json(), "file-text")
    .example("dag.jack", "Dag Jack", dag_jack_example_json(), "file-text")
    .workflow("writer", "Writer", "text.document")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{
        testkit::{self, meta},
        PluginApp, VcsDocumentApp, ViewState,
    };
    use writer_engine::jack_variable_occurrences;

    const CANONICAL_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";

    fn new_app() -> VcsDocumentApp<WriterPlayApp> {
        testkit::new_app::<WriterPlayApp>()
    }

    fn new_app_with_registry() -> VcsDocumentApp<WriterPlayApp> {
        testkit::new_app_with_registry::<WriterPlayApp>(create_writer_app)
    }

    /// ✍️ Loads the canonical jack fixture into the store, returning the app ready to exercise.
    fn app_with_jack() -> VcsDocumentApp<WriterPlayApp> {
        let mut app = new_app();
        app.dispatch_typed(WriterCommand::SetActiveExample { example_id: "jack".into() }, &meta("local")).expect("load jack");
        app
    }

    #[test]
    fn text_edit_burst_coalesces_into_one_undo_step() {
        let mut app = new_app();
        for text in ["h", "he", "hel", "hell", "hello"] {
            app.dispatch_typed(WriterCommand::TextEdit { text: text.into() }, &meta("local")).expect("type");
        }
        assert_eq!(app.projection().expect("projection").text, "hello");
        // The whole typing burst shares one coalesce key, so a single undo restores the pre-burst buffer
        // rather than backing out one keystroke at a time.
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").text, "", "coalesced typing collapses to one undo step");
    }

    #[test]
    fn lint_is_a_view_action_and_example_default_materializes() {
        let mut app = new_app_with_registry();
        // lintDocument is a declared View action: registry kind discipline requires it emit no operations.
        let result = app.dispatch_typed(WriterCommand::LintDocument, &meta("local")).expect("lint");
        assert!(result.operations.is_empty(), "lint re-runs diagnostics into runtime, never the document");
        // setActiveExample fired with the declared default example ("jack").
        app.dispatch_typed(WriterCommand::SetActiveExample { example_id: "jack".into() }, &meta("local")).expect("example");
        assert!(!app.projection().expect("projection").text.is_empty(), "jack default materialized from the registry");
    }

    #[test]
    fn renders_text_editor_scene() {
        let mut app = new_app();
        let node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn renders_document_tree_for_jack() {
        let mut app = new_app();
        let node = app.render(WRITER_PLAY_BODY_DOCUMENT, Some(&jack_example_json()), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Query"));
    }

    #[test]
    fn renders_catalogue_panel() {
        let mut app = new_app();
        let node = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("jack"));
    }

    #[test]
    fn format_document_reformats_jack_query() {
        let mut app = app_with_jack();
        app.dispatch_typed(WriterCommand::SetText { text: "MATCH (a:Piece)   WHERE a.name='core' RETURN a.name".into() }, &meta("local")).expect("set text");
        let result = app.dispatch_typed(WriterCommand::FormatDocument, &meta("local")).expect("format");
        assert_eq!(result.operations.len(), 1);
        assert!(app.projection().expect("projection").text.contains('\n'));
    }

    #[test]
    fn format_document_without_change_emits_no_operation() {
        // A no-operation format (already-formatted or non-jack empty doc) bumps the format signal but must
        // not record a history entry.
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::FormatDocument, &meta("local")).expect("format");
        assert!(result.operations.is_empty());
    }

    #[test]
    fn set_text_action_updates_projection() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetText { text: "MATCH (a) RETURN a".into() }, &meta("local")).expect("set text");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").text, "MATCH (a) RETURN a");
    }

    #[test]
    fn set_text_undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app();
        app.dispatch_typed(WriterCommand::SetText { text: "first".into() }, &meta("local")).expect("first");
        app.dispatch_typed(WriterCommand::SetText { text: "second".into() }, &meta("local")).expect("second");
        assert_eq!(app.projection().expect("projection").text, "second");
        let undo = app.handle_action("undo", None, &meta("local")).expect("undo");
        assert!(undo.operations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.projection().expect("projection").text, "first");
        app.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").text, "second");
    }

    /// 🎥️ `SetCamera` is a config-only command — it must never emit a `WriterOperation` (no VCS edit,
    /// no undo entry) and instead write into `WriterConfig`, reflected in render.
    #[test]
    fn set_camera_command_writes_config_not_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetCamera { camera: WriterCamera { x: 3.0, y: 4.0, zoom: 2.0 } }, &meta("local")).expect("set camera");
        assert!(result.operations.is_empty(), "setCamera must not emit a VCS operation");
        let node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let camera: Value = serde_json::from_str(payload["textEditor"]["cameraJson"].as_str().unwrap()).unwrap();
        assert_eq!(camera["x"], json!(3.0));
        assert_eq!(camera["zoom"], json!(2.0));
    }

    #[test]
    fn view_action_emits_no_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::ToggleLineNumbers, &meta("local")).expect("toggle");
        assert!(result.operations.is_empty());
    }

    #[test]
    fn commit_rename_renames_all_spans_at_the_config_selection() {
        let mut app = app_with_jack();
        let occurrences = jack_variable_occurrences(CANONICAL_QUERY, "a");
        assert_eq!(occurrences.len(), 3);
        let (start, _) = occurrences[0];
        // 🎯️ `CommitRename` reads the rename target off `WriterConfig::editor_selection` — set it via
        // a real selection command first (mirrors what the editor surface does before offering rename).
        app.dispatch_typed(WriterCommand::SetEditorSelection { start, end: start }, &meta("local")).expect("place caret");
        let result = app.dispatch_typed(WriterCommand::CommitRename { text: "piece".into() }, &meta("local")).expect("commit rename");
        assert_eq!(result.operations.len(), 1);
        let text = app.projection().expect("projection").text;
        assert_eq!(text.matches("piece").count(), 3);
        assert_eq!(text.matches("a:Piece").count(), 0);
    }

    #[test]
    fn engagement_submit_parses_font_size() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::EngagementSubmit { value: Some("font 16".into()) }, &meta("local")).expect("submit");
        // Font size is ephemeral config state — no history entry.
        assert!(result.operations.is_empty());
        let measures = app.window_measures();
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
    }

    #[test]
    fn engagement_submit_parses_normalized_shell_drafts() {
        // The React shell PascalCases and strips separators from every draft before submitting it
        // (`normalizeEngagementActionText`), so "font 16" arrives as "Font16", "tab 4" as "Tab4",
        // and "line numbers" as "LineNumbers".
        let mut app = new_app();
        let before_toggle = app
            .window_engagements()
            .get(WRITER_PLAY_WINDOW_KIND)
            .and_then(|engagement| engagement.options.as_ref())
            .and_then(|options| options.first())
            .and_then(|option| option.pressed)
            .expect("line-numbers pressed state");

        app.dispatch_typed(WriterCommand::EngagementSubmit { value: Some("Font16".into()) }, &meta("local")).expect("font");
        app.dispatch_typed(WriterCommand::EngagementSubmit { value: Some("Tab4".into()) }, &meta("local")).expect("tab");
        app.dispatch_typed(WriterCommand::EngagementSubmit { value: Some("LineNumbers".into()) }, &meta("local")).expect("line numbers");

        let measures = app.window_measures();
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-tab-size-measure" && *value == 4.0)));

        let after_toggle = app
            .window_engagements()
            .get(WRITER_PLAY_WINDOW_KIND)
            .and_then(|engagement| engagement.options.as_ref())
            .and_then(|options| options.first())
            .and_then(|option| option.pressed)
            .expect("line-numbers pressed state");
        assert_eq!(after_toggle, !before_toggle);
    }

    #[test]
    fn window_measures_expose_font_line_height_tab_and_toggle() {
        let mut app = new_app();
        let measures = app.window_measures();
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert_eq!(main.len(), 4);
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Toggle { id, .. } if id == "writer-line-numbers-measure")));
    }

    #[test]
    fn window_engagements_expose_format_lint_placeholder() {
        let mut app = new_app();
        let engagements = app.window_engagements();
        let main = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("main engagement");
        let placeholder = main.input.as_ref().and_then(|i| i.placeholder.as_ref()).expect("placeholder");
        assert!(placeholder.contains("Format"));
        assert_eq!(main.possible_engagements.as_ref().map(|v| v.len()), Some(3));
    }

    // 🧰️ `VcsDocumentApp::tools()` (a per-app custom utility bar) no longer exists — utility bars
    // are now derived by the renderer from the utility registry (`writer_utility_labels` above;
    // writer declares no utilities). Format/lint were never single-sourced from that removed
    // utility bar though: they're `WindowEngagementPossible` entries in `window_engagements()`,
    // which is still the one surface for them — assert on that surface instead.
    #[test]
    fn window_engagements_include_format_and_lint_possible_engagements() {
        let mut app = new_app();
        let engagements = app.window_engagements();
        let engagement = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("writer window engagement");
        let ids: Vec<&str> = engagement.possible_engagements.as_ref().expect("possible engagements").iter().map(|possible| possible.id.as_str()).collect();
        assert!(ids.contains(&"writer-format"));
        assert!(ids.contains(&"writer-lint"));
    }

    #[test]
    fn scene_emits_placeholders_selectable_spans_and_newline_gates_for_jack() {
        let mut app = new_app();
        let node = app.render(WRITER_PLAY_BODY_MAIN, Some(&jack_example_json()), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("placeholdersJson"));
        assert!(json.contains("selectableSpansJson"));
        assert!(json.contains("newlineGatesJson"));
    }

    #[test]
    fn set_ast_hover_updates_tree_highlight_and_scene_hover() {
        let mut app = app_with_jack();
        let root = parse_jack_ast(&app.projection().expect("projection").text);
        let result = app.dispatch_typed(WriterCommand::SetAstHover { id: Some(root.id.clone()) }, &meta("local")).expect("hover");
        assert!(result.operations.is_empty());
        let tree_node = app.render(WRITER_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render tree");
        let tree_json = serde_json::to_string(&tree_node).unwrap();
        assert!(tree_json.contains(&root.id));
        let scene_node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render scene");
        let scene_value = serde_json::to_value(&scene_node).unwrap();
        let hover_json = scene_value["textEditor"]["hoverJson"].as_str().expect("hoverJson string");
        let hover_range: Value = serde_json::from_str(hover_json).unwrap();
        assert_eq!(hover_range["start"].as_u64(), Some(root.start as u64));
        assert_eq!(hover_range["end"].as_u64(), Some(root.end as u64));
    }

    #[test]
    fn set_active_example_loads_jack_fixture() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetActiveExample { example_id: "jack".into() }, &meta("local")).expect("load");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.id, "jack");
        assert!(projection.text.contains("MATCH"));
    }

    #[test]
    fn set_active_example_loads_dag_jack_fixture() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::SetActiveExample { example_id: "dag.jack".into() }, &meta("local")).expect("load");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").id, "dag-jack");
    }

    #[test]
    fn set_active_example_falls_back_to_empty_document() {
        let mut app = app_with_jack();
        let result = app.dispatch_typed(WriterCommand::SetActiveExample { example_id: String::new() }, &meta("local")).expect("load");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.id, "empty");
        assert_eq!(projection.text, "");
    }

    #[test]
    fn writer_labels_resolve_native_by_default() {
        let mut app = new_app();
        let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("\"Document\""));
        assert!(inspection_json.contains("\"Camera\""));
        let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("\"Language\""));
        assert!(catalogue_json.contains("Cypher-inspired"));
        let engagements = app.window_engagements();
        let engagements_json = serde_json::to_string(&engagements).unwrap();
        assert!(engagements_json.contains("\"Format\""));
        assert!(engagements_json.contains("\"Lint\""));
        let measures = app.window_measures();
        let measures_json = serde_json::to_string(&measures).unwrap();
        assert!(measures_json.contains("Font size"));
        assert!(measures_json.contains("Line numbers"));
        assert!(!measures_json.contains("Schriftgröße"));
    }

    #[test]
    fn writer_labels_resolve_german_locale() {
        let mut app = new_app();
        app.dispatch_typed(WriterCommand::SetLocale { value: "de".into() }, &meta("local")).expect("set locale");
        let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Dokument"));
        assert!(inspection_json.contains("Kamera"));
        assert!(!inspection_json.contains("\"Camera\""));
        let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Sprache"));
        let measures = app.window_measures();
        let measures_json = serde_json::to_string(&measures).unwrap();
        assert!(measures_json.contains("Schriftgröße"));
        assert!(measures_json.contains("Zeilennummern"));
        let engagements = app.window_engagements();
        let engagements_json = serde_json::to_string(&engagements).unwrap();
        assert!(engagements_json.contains("Texteditor"));
        assert!(engagements_json.contains("Formatieren"));
        assert!(engagements_json.contains("Prüfen"));
    }

    //#region 🔖️PortTests
    #[test]
    fn writer_io_declares_the_extra_text_out_port() {
        let io = writer_engine::writer_io();
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let text_out = ports.iter().find(|port| port.id == "text:out").expect("text:out port declared");
        assert_eq!(text_out.kind_id.as_deref(), Some("text.document"));
        assert_eq!(text_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }

    #[test]
    fn export_media_text_out_projects_the_document_as_a_chapter() {
        let app = WriterPlayApp;
        let document = jack_example_document();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = DocumentView { projection: &document, history: &history };
        let media = app.export_media("text:out", &doc_view).expect("export text:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "text.document");
        let payload: writer_engine::WriterChapterPayload = serde_json::from_str(&json).expect("decode chapter payload");
        assert_eq!(payload.text, document.text);
        assert_eq!(payload.language_id, document.language_id);
    }

    #[test]
    fn export_media_rejects_unknown_ports() {
        let app = WriterPlayApp;
        let document = empty_writer_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = DocumentView { projection: &document, history: &history };
        assert!(matches!(app.export_media("nonsense:out", &doc_view), Err(MediaError::NotImplemented)));
    }
    //#endregion 🔖️PortTests
}
//#endregion 🧪️Tests
