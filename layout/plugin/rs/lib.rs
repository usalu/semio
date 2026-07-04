//! 📐 Layout plugin — blueprint/preview document editor bundled as a hot-swappable WASM component.

use layout_rs::{
    parse_layout_document, resolve_page, Frame, LayoutDocument, LAYOUT_FIXTURE_SCHEMA, Page,
};
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, Canvas2dScene, CommandDescriptor, PluginApp,
    PluginBundle, UiInspectorFieldGroup, UiNode, UiSectionNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const LAYOUT_PLAY_APP_ID: &str = "layout-play";
const LAYOUT_PLAY_SURFACE_BLUEPRINT: &str = "layout.play.blueprint";
const LAYOUT_PLAY_SURFACE_PREVIEW: &str = "layout.play.preview";
const LAYOUT_PLAY_BODY_BLUEPRINT: &str = "layout.play.blueprint";
const LAYOUT_PLAY_BODY_PREVIEW: &str = "layout.play.preview";
const LAYOUT_PLAY_BODY_HIERARCHY: &str = "layout.play.hierarchy";
const LAYOUT_PLAY_BODY_CATALOGUE: &str = "layout.play.catalogue";
const LAYOUT_PLAY_BODY_INSPECTION: &str = "layout.play.inspection";
const LAYOUT_PLAY_BODY_PREFLIGHT: &str = "layout.play.preflight";
const LAYOUT_PLAY_WINDOW_BLUEPRINT: &str = "layout-blueprint";
const LAYOUT_PLAY_WINDOW_PREVIEW: &str = "layout-preview";
const LAYOUT_PLAY_PREFLIGHT_TAB_ID: &str = "layout.panel.preflight";

const LAYOUT_SAMPLE_JSON: &str = include_str!("../../example/sample.layout.json");

const LAYOUT_CATALOGUE_KINDS: &[(&str, &str, &str)] = &[
    ("rect", "Rectangle", "square"),
    ("text", "Text Frame", "type"),
    ("image", "Image Frame", "image"),
];
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LayoutPlayRuntime {
    #[serde(default = "default_active_page_id")]
    active_page_id: String,
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    hovered_id: Option<String>,
}

impl Default for LayoutPlayRuntime {
    fn default() -> Self {
        Self {
            active_page_id: default_active_page_id(),
            selected_ids: Vec::new(),
            hovered_id: None,
        }
    }
}

fn default_active_page_id() -> String {
    "page-1".into()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LayoutPlayEnvelope {
    document: LayoutDocument,
    #[serde(default)]
    undo_stack: Vec<LayoutDocument>,
    #[serde(default)]
    redo_stack: Vec<LayoutDocument>,
    #[serde(default)]
    runtime: LayoutPlayRuntime,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreflightIssue {
    severity: String,
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LayoutCanvasLayer {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_document() -> LayoutDocument {
    parse_layout_document(LAYOUT_SAMPLE_JSON).expect("sample layout fixture")
}

fn default_envelope() -> LayoutPlayEnvelope {
    LayoutPlayEnvelope {
        document: default_document(),
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
        runtime: LayoutPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> LayoutPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &LayoutPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn layout_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: LAYOUT_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}

fn active_page<'a>(doc: &'a LayoutDocument, runtime: &LayoutPlayRuntime) -> Option<&'a Page> {
    doc.pages.iter().find(|page| page.id == runtime.active_page_id).or_else(|| doc.pages.first())
}

fn frame_icon(kind: &str) -> &str {
    match kind {
        "rect" => "square",
        "text" => "type",
        "image" => "image",
        _ => "square",
    }
}

fn page_row_id(page_id: &str) -> String {
    format!("layout-hierarchy.page.{page_id}")
}

fn frame_row_id(frame_id: &str) -> String {
    format!("layout-hierarchy.frame.{frame_id}")
}

fn layer_row_id(page_id: &str, layer_id: &str) -> String {
    format!("layout-hierarchy.layer.{page_id}.{layer_id}")
}

fn push_undo(play: &mut LayoutPlayEnvelope) {
    play.undo_stack.push(play.document.clone());
    if play.undo_stack.len() > 32 {
        play.undo_stack.remove(0);
    }
    play.redo_stack.clear();
}

fn canvas_layers(doc: &LayoutDocument, runtime: &LayoutPlayRuntime, blueprint: bool) -> String {
    let page = match active_page(doc, runtime) {
        Some(page) => page,
        None => return "[]".into(),
    };
    let resolved = resolve_page(doc, page);
    let layers: Vec<LayoutCanvasLayer> = resolved
        .iter()
        .filter(|entry| entry.frame.visible())
        .map(|entry| {
            let bounds = entry.frame.bounds();
            LayoutCanvasLayer {
                id: entry.frame.id().into(),
                kind: entry.frame.kind_str().into(),
                name: entry.frame.id().into(),
                x: if blueprint { bounds.x } else { bounds.x + page.width + 24.0 },
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
            }
        })
        .collect();
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

fn run_layout_preflight(doc: &LayoutDocument) -> Vec<PreflightIssue> {
    let mut issues = Vec::new();
    for page in &doc.pages {
        let resolved = resolve_page(doc, page);
        for entry in resolved {
            let frame = &entry.frame;
            if !frame.visible() {
                continue;
            }
            let bounds = frame.bounds();
            if bounds.x < 0.0
                || bounds.y < 0.0
                || bounds.x + bounds.width > page.width
                || bounds.y + bounds.height > page.height
            {
                issues.push(PreflightIssue {
                    severity: "warning".into(),
                    code: "object.out_of_bounds".into(),
                    message: format!("Object {} extends outside page bounds", frame.id()),
                    object_id: Some(frame.id().into()),
                    page_id: Some(page.id.clone()),
                });
            }
            if frame.kind_str() == "text" {
                if let Frame::Text { bounds, .. } = frame {
                    if bounds.height < 24.0 || bounds.width < 24.0 {
                        issues.push(PreflightIssue {
                            severity: "warning".into(),
                            code: "text.below_minimum_size".into(),
                            message: format!("Text frame {} is below minimum readable size", frame.id()),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        });
                    }
                }
            }
            if let Frame::Image { link_id, .. } = frame {
                let link = doc.links.iter().find(|entry| entry.id == *link_id);
                let missing = link.map(|entry| entry.state.as_deref() == Some("missing")).unwrap_or(true);
                if missing {
                    issues.push(PreflightIssue {
                        severity: "error".into(),
                        code: "asset.missing".into(),
                        message: format!("Linked asset missing for {}", frame.id()),
                        object_id: Some(frame.id().into()),
                        page_id: Some(page.id.clone()),
                    });
                }
            }
        }
        for layer in &page.layers {
            for object_id in &layer.object_ids {
                if !object_id.contains("image") {
                    continue;
                }
                if issues.iter().any(|issue| issue.object_id.as_deref() == Some(object_id.as_str())) {
                    continue;
                }
                let link = doc
                    .links
                    .iter()
                    .find(|entry| entry.state.as_deref() == Some("missing"));
                if let Some(link) = link {
                    issues.push(PreflightIssue {
                        severity: "error".into(),
                        code: "asset.missing".into(),
                        message: format!("Linked asset missing for {object_id}"),
                        object_id: Some(object_id.clone()),
                        page_id: Some(page.id.clone()),
                    });
                    let _ = link;
                }
            }
        }
    }
    issues
}
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn tree_item(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    icon_id: Option<String>,
    command: Option<CommandDescriptor>,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id,
        selected: None,
        default_open: None,
        command,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_hierarchy_tree(play: &LayoutPlayEnvelope) -> UiNode {
    let doc = &play.document;
    let page_items: Vec<UiTreeItemNode> = doc
        .pages
        .iter()
        .map(|page| {
            let frame_items: Vec<UiTreeItemNode> = page
                .frames
                .iter()
                .map(|frame| {
                    tree_item(
                        frame_row_id(frame.id()),
                        frame.id(),
                        Some(format!("{} · {}", page.name, frame.kind_str())),
                        Some(frame_icon(frame.kind_str()).into()),
                        Some(layout_cmd("setSelection", Some(json!({ "ids": [frame.id()] })))),
                    )
                })
                .collect();
            tree_item(
                page_row_id(&page.id),
                page.name.clone(),
                Some(format!("{}×{}", page.width as i64, page.height as i64)),
                Some("file".into()),
                Some(layout_cmd("setActivePage", Some(json!({ "pageId": page.id })))),
            )
            .with_items(if frame_items.is_empty() { None } else { Some(frame_items) })
        })
        .collect();
    let layer_items: Vec<UiTreeItemNode> = doc.pages.iter().flat_map(|page| {
        page.layers.iter().map(|layer| {
            tree_item(
                layer_row_id(&page.id, &layer.id),
                layer.name.clone(),
                Some(page.name.clone()),
                Some("layers".into()),
                Some(layout_cmd("setSelection", Some(json!({ "ids": [layer.id.clone()] })))),
            )
        })
    }).collect();
    let highlighted_ids: Vec<String> = play
        .runtime
        .hovered_id
        .as_ref()
        .map(|id| vec![page_row_id(id), frame_row_id(id)])
        .unwrap_or_default();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "layout-hierarchy.document".into(),
                label: Some("Document".into()),
                default_open: Some(true),
                items: vec![tree_item(
                    "layout-hierarchy.document.root",
                    doc.name.clone(),
                    Some(LAYOUT_FIXTURE_SCHEMA.into()),
                    Some("file-text".into()),
                    None,
                )],
            },
            UiTreeSectionNode {
                id: "layout-hierarchy.pages".into(),
                label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
                default_open: Some(true),
                items: page_items,
            },
            UiTreeSectionNode {
                id: "layout-hierarchy.layers".into(),
                label: Some("Layers".into()),
                default_open: Some(false),
                items: layer_items,
            },
        ],
        selected_ids: Some(
            play.runtime
                .selected_ids
                .iter()
                .flat_map(|id| vec![page_row_id(id), frame_row_id(id), format!("layout-hierarchy.layer.{}.{}", play.runtime.active_page_id, id)])
                .collect(),
        ),
        highlighted_ids: if highlighted_ids.is_empty() { None } else { Some(highlighted_ids) },
        selection_change: Some(layout_cmd("setSelection", None)),
    })
}

trait TreeItemExt {
    fn with_items(self, items: Option<Vec<UiTreeItemNode>>) -> UiTreeItemNode;
}

impl TreeItemExt for UiTreeItemNode {
    fn with_items(mut self, items: Option<Vec<UiTreeItemNode>>) -> UiTreeItemNode {
        self.items = items;
        self
    }
}

fn build_catalogue_tree() -> UiNode {
    let items: Vec<UiTreeItemNode> = LAYOUT_CATALOGUE_KINDS
        .iter()
        .map(|(kind, label, icon)| {
            tree_item(
                format!("layout-catalogue.{kind}"),
                *label,
                Some((*kind).into()),
                Some((*icon).into()),
                Some(layout_cmd("addFrame", Some(json!({ "kind": kind })))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "layout-catalogue.kinds".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(play: &LayoutPlayEnvelope) -> UiNode {
    let doc = &play.document;
    if play.runtime.selected_ids.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", LAYOUT_FIXTURE_SCHEMA)),
            ui_text(format!("Name: {}", doc.name)),
            ui_text(format!("Pages: {}", doc.pages.len())),
            ui_text(format!("Active page: {}", play.runtime.active_page_id)),
        ]);
    }
    let selected_id = &play.runtime.selected_ids[0];
    if let Some(page) = doc.pages.iter().find(|page| page.id == *selected_id) {
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
            id: "layout-play-inspector.page".into(),
            label: "Page".into(),
            default_open: Some(true),
            fields: vec![
                ui_inspector_readonly_field("layout-play-inspector.page-id", "Id", page.id.clone()),
                ui_inspector_readonly_field("layout-play-inspector.page-name", "Name", page.name.clone()),
                ui_inspector_readonly_field(
                    "layout-play-inspector.page-size",
                    "Size",
                    format!("{} × {}", page.width as i64, page.height as i64),
                ),
            ],
        }]);
    }
    for page in &doc.pages {
        if let Some(frame) = page.frames.iter().find(|frame| frame.id() == selected_id) {
            let bounds = frame.bounds();
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
                id: "layout-play-inspector.frame".into(),
                label: "Frame".into(),
                default_open: Some(true),
                fields: vec![
                    ui_inspector_readonly_field("layout-play-inspector.frame-id", "Id", frame.id().to_string()),
                    ui_inspector_readonly_field("layout-play-inspector.frame-kind", "Kind", frame.kind_str().to_string()),
                    ui_inspector_readonly_field("layout-play-inspector.frame-page", "Page", page.name.clone()),
                    ui_inspector_readonly_field(
                        "layout-play-inspector.frame-bounds",
                        "Bounds",
                        format!(
                            "x={} y={} w={} h={}",
                            bounds.x as i64, bounds.y as i64, bounds.width as i64, bounds.height as i64
                        ),
                    ),
                ],
            }]);
        }
    }
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "layout-play-inspector.missing".into(),
        label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
        default_open: Some(true),
        children: vec![ui_text("Selection not found in document.")],
    }])
}

fn build_preflight_tree(play: &LayoutPlayEnvelope) -> UiNode {
    let issues = run_layout_preflight(&play.document);
    let items: Vec<UiTreeItemNode> = if issues.is_empty() {
        vec![tree_item("layout-preflight.empty", "No issues", None, Some("check-circle".into()), None)]
    } else {
        issues
            .iter()
            .map(|issue| {
                tree_item(
                    format!(
                        "layout-preflight.{}.{}",
                        issue.code,
                        issue.object_id.clone().unwrap_or_else(|| issue.message.clone())
                    ),
                    issue.message.clone(),
                    Some(format!("{} · {}", issue.severity, issue.code)),
                    Some(if issue.severity == "error" {
                        "alert-circle"
                    } else {
                        "alert-triangle"
                    }.into()),
                    Some(layout_cmd("focusPreflightIssue", Some(json!({ "issue": issue })))),
                )
            })
            .collect()
    };
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "layout-preflight.issues".into(),
            label: Some("Preflight".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_blueprint(play: &LayoutPlayEnvelope) -> UiNode {
    let camera = &play.document.camera;
    build_canvas_2d_scene(
        LAYOUT_PLAY_SURFACE_BLUEPRINT,
        LAYOUT_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: camera.x,
            camera_y: camera.y,
            zoom: camera.zoom,
            layers_json: canvas_layers(&play.document, &play.runtime, true),
        },
    )
}

fn render_preview(play: &LayoutPlayEnvelope) -> UiNode {
    let camera = &play.document.preview_camera;
    build_canvas_2d_scene(
        LAYOUT_PLAY_SURFACE_PREVIEW,
        LAYOUT_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: camera.x,
            camera_y: camera.y,
            zoom: camera.zoom,
            layers_json: canvas_layers(&play.document, &play.runtime, false),
        },
    )
}
//#endregion 🔖Render

//#region 🔖LayoutPlayApp
struct LayoutPlayApp;

impl PluginApp for LayoutPlayApp {
    fn app_id(&self) -> &str {
        LAYOUT_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("layout envelope json")
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
                    if let Ok(parsed) = serde_json::from_value::<LayoutPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                play.runtime.selected_ids = selection_ids(args);
                return vec![set_document_op(&play)];
            }
            "setActivePage" => {
                if let Some(page_id) = args.and_then(|value| value.get("pageId")).and_then(|value| value.as_str()) {
                    play.runtime.active_page_id = page_id.into();
                    return vec![set_document_op(&play)];
                }
            }
            "setHover" => {
                play.runtime.hovered_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&play)];
            }
            "focusPreflightIssue" => {
                if let Some(issue) = args.and_then(|value| value.get("issue")) {
                    if let Some(object_id) = issue.get("objectId").and_then(|value| value.as_str()) {
                        play.runtime.selected_ids = vec![object_id.into()];
                    }
                    if let Some(page_id) = issue.get("pageId").and_then(|value| value.as_str()) {
                        play.runtime.active_page_id = page_id.into();
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "undo" => {
                if let Some(previous) = play.undo_stack.pop() {
                    play.redo_stack.push(play.document.clone());
                    play.document = previous;
                    return vec![set_document_op(&play)];
                }
            }
            "redo" => {
                if let Some(next) = play.redo_stack.pop() {
                    play.undo_stack.push(play.document.clone());
                    play.document = next;
                    return vec![set_document_op(&play)];
                }
            }
            "addFrame" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("rect");
                let page_id = play.runtime.active_page_id.clone();
                let page_index = play.document.pages.iter().position(|page| page.id == page_id);
                if let Some(page_index) = page_index {
                    let frame_id = format!("frame-{}", play.document.pages[page_index].frames.len() + 1);
                    let layer_id = play.document.pages[page_index]
                        .layer_ids
                        .first()
                        .cloned()
                        .unwrap_or_else(|| "layer-1".into());
                    push_undo(&mut play);
                    let page = &mut play.document.pages[page_index];
                    let frame = match kind {
                        "text" => Frame::Text {
                            id: frame_id.clone(),
                            layer_id,
                            kind: "text".into(),
                            bounds: layout_rs::LayoutBounds {
                                x: 48.0,
                                y: 120.0,
                                width: 200.0,
                                height: 120.0,
                                rotation: 0.0,
                            },
                            locked: None,
                            visible: None,
                            story_id: play.document.stories.first().map(|story| story.id.clone()).unwrap_or_else(|| "story-1".into()),
                            thread_next: None,
                            columns: 1,
                            inset: layout_rs::LayoutRect { x: 4.0, y: 4.0, width: 192.0, height: 112.0 },
                            wrap_mode: "box".into(),
                        },
                        "image" => Frame::Image {
                            id: frame_id.clone(),
                            layer_id,
                            kind: "image".into(),
                            bounds: layout_rs::LayoutBounds {
                                x: 48.0,
                                y: 280.0,
                                width: 160.0,
                                height: 120.0,
                                rotation: 0.0,
                            },
                            locked: None,
                            visible: None,
                            link_id: play.document.links.first().map(|link| link.id.clone()).unwrap_or_else(|| "link-missing".into()),
                        },
                        _ => Frame::Rect {
                            id: frame_id.clone(),
                            layer_id,
                            kind: "rect".into(),
                            bounds: layout_rs::LayoutBounds {
                                x: 48.0,
                                y: 48.0,
                                width: 120.0,
                                height: 64.0,
                                rotation: 0.0,
                            },
                            locked: None,
                            visible: None,
                            fill: Some([0.2, 0.24, 0.3, 1.0]),
                            stroke: None,
                        },
                    };
                    if let Some(layer_id_target) = page.layer_ids.first().cloned() {
                        if let Some(layer) = page.layers.iter_mut().find(|layer| layer.id == layer_id_target) {
                            layer.object_ids.push(frame_id.clone());
                        }
                    }
                    page.frames.push(frame);
                    play.runtime.selected_ids = vec![frame_id];
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
            LAYOUT_PLAY_BODY_BLUEPRINT => render_blueprint(&play),
            LAYOUT_PLAY_BODY_PREVIEW => render_preview(&play),
            LAYOUT_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&play),
            LAYOUT_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            LAYOUT_PLAY_BODY_INSPECTION => build_inspector_tree(&play),
            LAYOUT_PLAY_BODY_PREFLIGHT => build_preflight_tree(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖LayoutPlayApp

//#region 🔖AppFactory
fn create_layout_app() -> App {
    App::from_builder(
        App::builder(LAYOUT_PLAY_APP_ID, "Layout")
            .icon_id("layout")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(LAYOUT_PLAY_WINDOW_BLUEPRINT, "Blueprint", LAYOUT_PLAY_BODY_BLUEPRINT)
            .window_kind(LAYOUT_PLAY_WINDOW_PREVIEW, "Preview", LAYOUT_PLAY_BODY_PREVIEW)
            .default_layout(create_default_layout(
                &[LAYOUT_PLAY_WINDOW_BLUEPRINT.into(), LAYOUT_PLAY_WINDOW_PREVIEW.into()],
                "row",
                Some(&[55.0, 45.0]),
                Some(&["Blueprint".into(), "Preview".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                LAYOUT_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                LAYOUT_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(LAYOUT_PLAY_PREFLIGHT_TAB_ID, "Preflight", "workbench", LAYOUT_PLAY_BODY_PREFLIGHT)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                LAYOUT_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("sample", "Sample", LAYOUT_SAMPLE_JSON)
    .program("layout", "Layout", "layout")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("layout", "Layout", "0.1.0").register_app(create_layout_app(), || Box::new(LayoutPlayApp))
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
    fn renders_blueprint_canvas_scene() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_preview_canvas_scene() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_PREVIEW, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn hierarchy_lists_sample_pages() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("layout-hierarchy.page.page-1"));
        assert!(json.contains("Page 1"));
    }

    #[test]
    fn catalogue_lists_frame_kinds() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("layout-catalogue.rect"));
        assert!(json.contains("Text Frame"));
    }

    #[test]
    fn preflight_finds_missing_asset() {
        let play = default_envelope();
        let issues = run_layout_preflight(&play.document);
        assert!(issues.iter().any(|issue| issue.code == "asset.missing"));
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_PREFLIGHT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("asset.missing") || json.contains("Linked asset missing"));
    }

    #[test]
    fn set_selection_updates_runtime() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "setSelection",
            Some(&json!({ "ids": ["frame-text-1"] })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.selected_ids, vec!["frame-text-1".to_string()]);
    }

    #[test]
    fn sample_fixture_parses() {
        let doc = parse_layout_document(LAYOUT_SAMPLE_JSON).expect("sample fixture");
        assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn add_frame_command_appends_rect() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let before: LayoutPlayEnvelope = serde_json::from_str(&document).expect("parse envelope");
        let before_count = before.document.pages[0].frames.len();
        let ops = app.handle_command("addFrame", Some(&json!({ "kind": "rect" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.document.pages[0].frames.len(), before_count + 1);
    }
}
//#endregion 🧪Tests
