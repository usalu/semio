//! 📝 Note plugin — infinite canvas note board bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_canvas_2d_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, Canvas2dScene,
    CommandDescriptor, PluginApp, PluginBundle, UiInspectorFieldGroup, UiNode, UiSectionNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, create_default_layout,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const NOTE_PLAY_APP_ID: &str = "note-play";
const NOTE_PLAY_CONTROLLER_ID: &str = "note-play";
const NOTE_PLAY_SURFACE_COMPOSITE: &str = "note.play.composite";
const NOTE_PLAY_SURFACE_NAVIGATOR: &str = "note.play.navigator";
const NOTE_PLAY_BODY_COMPOSITE: &str = "note.play.composite";
const NOTE_PLAY_BODY_NAVIGATOR: &str = "note.play.navigator";
const NOTE_PLAY_BODY_HIERARCHY: &str = "note.play.hierarchy";
const NOTE_PLAY_BODY_CATALOGUE: &str = "note.play.catalogue";
const NOTE_PLAY_BODY_PROPERTIES: &str = "note.play.properties";
const NOTE_PLAY_WINDOW_COMPOSITE: &str = "note-composite";
const NOTE_PLAY_WINDOW_NAVIGATOR: &str = "note-navigator";
const NOTE_DOCUMENT_SCHEMA: &str = "note.document";

const SEMIO_EXAMPLE_JSON: &str = include_str!("../../example/semio.note.json");

static NOTE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoteCamera {
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
#[serde(tag = "kind", rename_all = "camelCase")]
enum NoteBlockNode {
    #[serde(rename = "text", rename_all = "camelCase")]
    Text {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        paragraphs: Vec<NoteTextParagraph>,
        font_size: f64,
        font_weight: String,
        align: String,
    },
    #[serde(rename = "image", rename_all = "camelCase")]
    Image {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        image_key: String,
    },
    #[serde(rename = "table", rename_all = "camelCase")]
    Table {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        columns: Vec<String>,
        rows: Vec<Vec<NoteTableCell>>,
    },
    #[serde(rename = "math", rename_all = "camelCase")]
    Math {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        tex: String,
        display_mode: bool,
    },
    #[serde(rename = "ink", rename_all = "camelCase")]
    Ink {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        points: Vec<[f64; 2]>,
        stroke_width: f64,
        color: [f64; 4],
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        children: Vec<NoteBlockNode>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoteTextRun {
    text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoteTextParagraph {
    runs: Vec<NoteTextRun>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct NoteTableCell {
    content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoteDocument {
    schema: String,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(default = "default_camera")]
    camera: NoteCamera,
    #[serde(default)]
    blocks: Vec<NoteBlockNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pencil_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eraser_radius: Option<f64>,
}

fn default_camera() -> NoteCamera {
    NoteCamera {
        x: 0.0,
        y: 0.0,
        zoom: 1.0,
    }
}

fn create_note_id(prefix: &str) -> String {
    let next = NOTE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

fn empty_note_document() -> NoteDocument {
    NoteDocument {
        schema: NOTE_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        title: None,
        camera: default_camera(),
        blocks: Vec::new(),
        active_tool: Some("selectDirect".into()),
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        pencil_width: Some(3.0),
        eraser_radius: Some(12.0),
    }
}

fn block_id(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { id, .. }
        | NoteBlockNode::Image { id, .. }
        | NoteBlockNode::Table { id, .. }
        | NoteBlockNode::Math { id, .. }
        | NoteBlockNode::Ink { id, .. }
        | NoteBlockNode::Group { id, .. } => id,
    }
}

fn block_name(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { name, .. }
        | NoteBlockNode::Image { name, .. }
        | NoteBlockNode::Table { name, .. }
        | NoteBlockNode::Math { name, .. }
        | NoteBlockNode::Ink { name, .. }
        | NoteBlockNode::Group { name, .. } => name,
    }
}

fn block_kind(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { .. } => "text",
        NoteBlockNode::Image { .. } => "image",
        NoteBlockNode::Table { .. } => "table",
        NoteBlockNode::Math { .. } => "math",
        NoteBlockNode::Ink { .. } => "ink",
        NoteBlockNode::Group { .. } => "group",
    }
}

fn block_visible(block: &NoteBlockNode) -> bool {
    match block {
        NoteBlockNode::Text { visible, .. }
        | NoteBlockNode::Image { visible, .. }
        | NoteBlockNode::Table { visible, .. }
        | NoteBlockNode::Math { visible, .. }
        | NoteBlockNode::Ink { visible, .. }
        | NoteBlockNode::Group { visible, .. } => *visible,
    }
}

fn block_tree_row_id(block: &NoteBlockNode) -> String {
    format!("note-play-block:{}", block_id(block))
}

fn find_block<'a>(blocks: &'a [NoteBlockNode], target_id: &str) -> Option<&'a NoteBlockNode> {
    for block in blocks {
        if block_id(block) == target_id {
            return Some(block);
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if let Some(found) = find_block(children, target_id) {
                return Some(found);
            }
        }
    }
    None
}

fn flatten_blocks(blocks: &[NoteBlockNode]) -> Vec<&NoteBlockNode> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [NoteBlockNode], out: &mut Vec<&'a NoteBlockNode>) {
        for block in blocks {
            out.push(block);
            if let NoteBlockNode::Group { children, .. } = block {
                visit(children, out);
            }
        }
    }
    visit(blocks, &mut out);
    out
}

fn create_block_by_kind(kind: &str) -> NoteBlockNode {
    let id = create_note_id(kind);
    let name = match kind {
        "text" => "Text",
        "image" => "Image",
        "table" => "Table",
        "math" => "Math",
        "ink" => "Ink",
        _ => "Group",
    }
    .into();
    let shared = (id, name, 0.0, 0.0, 280.0, 120.0, 0.0, true, false);
    match kind {
        "image" => NoteBlockNode::Image {
            id: shared.0,
            name: shared.1,
            x: shared.2,
            y: shared.3,
            width: shared.4,
            height: shared.5,
            rotation: shared.6,
            visible: shared.7,
            locked: shared.8,
            image_key: "placeholder".into(),
        },
        "table" => NoteBlockNode::Table {
            id: shared.0,
            name: shared.1,
            x: shared.2,
            y: shared.3,
            width: shared.4,
            height: shared.5,
            rotation: shared.6,
            visible: shared.7,
            locked: shared.8,
            columns: vec!["A".into(), "B".into()],
            rows: vec![vec![
                NoteTableCell {
                    content: String::new(),
                },
                NoteTableCell {
                    content: String::new(),
                },
            ]],
        },
        "math" => NoteBlockNode::Math {
            id: shared.0,
            name: shared.1,
            x: shared.2,
            y: shared.3,
            width: shared.4,
            height: shared.5,
            rotation: shared.6,
            visible: shared.7,
            locked: shared.8,
            tex: "E = mc^2".into(),
            display_mode: true,
        },
        "ink" => NoteBlockNode::Ink {
            id: shared.0,
            name: shared.1,
            x: shared.2,
            y: shared.3,
            width: shared.4,
            height: shared.5,
            rotation: shared.6,
            visible: shared.7,
            locked: shared.8,
            points: Vec::new(),
            stroke_width: 3.0,
            color: [0.0, 0.0, 0.0, 1.0],
        },
        "group" => NoteBlockNode::Group {
            id: shared.0,
            name: shared.1,
            x: shared.2,
            y: shared.3,
            width: shared.4,
            height: shared.5,
            rotation: shared.6,
            visible: shared.7,
            locked: shared.8,
            children: Vec::new(),
        },
        _ => NoteBlockNode::Text {
            id: shared.0,
            name: shared.1,
            x: shared.2,
            y: shared.3,
            width: shared.4,
            height: shared.5,
            rotation: shared.6,
            visible: shared.7,
            locked: shared.8,
            paragraphs: vec![NoteTextParagraph {
                runs: vec![NoteTextRun {
                    text: String::new(),
                }],
            }],
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        },
    }
}

fn remove_block_from_tree(blocks: &mut Vec<NoteBlockNode>, target_id: &str) -> bool {
    if let Some(index) = blocks.iter().position(|block| block_id(block) == target_id) {
        blocks.remove(index);
        return true;
    }
    for block in blocks.iter_mut() {
        if let NoteBlockNode::Group { children, .. } = block {
            if remove_block_from_tree(children, target_id) {
                return true;
            }
        }
    }
    false
}

fn clone_block(block: &NoteBlockNode) -> NoteBlockNode {
    let mut cloned: NoteBlockNode = serde_json::from_value(serde_json::to_value(block).unwrap()).unwrap();
    match &mut cloned {
        NoteBlockNode::Text { id, name, .. }
        | NoteBlockNode::Image { id, name, .. }
        | NoteBlockNode::Table { id, name, .. }
        | NoteBlockNode::Math { id, name, .. }
        | NoteBlockNode::Ink { id, name, .. }
        | NoteBlockNode::Group { id, name, .. } => {
            *id = create_note_id(block_kind(block));
            *name = format!("{name} copy");
        }
    }
    cloned
}
//#endregion 🔖Document

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
            value.as_array().map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect()
            })
        })
        .unwrap_or_default()
}

fn block_icon(kind: &str) -> &str {
    match kind {
        "text" => "type",
        "image" => "image",
        "table" => "table",
        "math" => "sigma",
        "ink" => "pencil",
        _ => "folder",
    }
}

fn block_tree_item(block: &NoteBlockNode) -> UiTreeItemNode {
    let nested = match block {
        NoteBlockNode::Group { children, .. } if !children.is_empty() => {
            Some(children.iter().map(block_tree_item).collect())
        }
        _ => None,
    };
    UiTreeItemNode {
        id: block_tree_row_id(block),
        label: block_name(block).into(),
        description: Some(block_kind(block).into()),
        icon_id: Some(block_icon(block_kind(block)).into()),
        selected: None,
        default_open: Some(matches!(block, NoteBlockNode::Group { .. })),
        command: Some(play_cmd(
            NOTE_PLAY_CONTROLLER_ID,
            "setSelection",
            Some(json!({ "ids": [block_id(block)] })),
        )),
        draggable: Some(true),
        drag_data: None,
        items: nested,
        control: None,
        is_hidden: if block_visible(block) { None } else { Some(true) },
    }
}

fn render_hierarchy_panel(document: &NoteDocument, view_state: &ViewState) -> UiNode {
    let toolbar = vec![
        ("text", "Add Text", "type"),
        ("table", "Add Table", "table"),
        ("math", "Add Math", "sigma"),
        ("image", "Add Image", "image"),
        ("group", "Add Group", "folder-plus"),
    ]
    .into_iter()
    .map(|(kind, label, icon)| UiTreeItemNode {
        id: format!("note-play-blocks.add.{kind}"),
        label: label.into(),
        description: None,
        icon_id: Some(icon.into()),
        selected: None,
        default_open: None,
        command: Some(play_cmd(
            NOTE_PLAY_CONTROLLER_ID,
            "addBlock",
            Some(json!({ "kind": kind })),
        )),
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    })
    .collect::<Vec<_>>();
    let block_items: Vec<UiTreeItemNode> = if document.blocks.is_empty() {
        vec![UiTreeItemNode {
            id: "note-play-blocks.empty".into(),
            label: "Drop blocks here".into(),
            description: None,
            icon_id: Some("sticky-note".into()),
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
        document.blocks.iter().map(block_tree_item).collect()
    };
    let selected_ids: Vec<String> = selection_from_view(view_state)
        .iter()
        .filter_map(|id| find_block(&document.blocks, id).map(block_tree_row_id))
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "note-play-blocks".into(),
            label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
            default_open: Some(true),
            items: [toolbar, block_items].concat(),
        }],
        selected_ids: Some(selected_ids),
        highlighted_ids: None,
        selection_change: Some(play_cmd(
            NOTE_PLAY_CONTROLLER_ID,
            "setSelection",
            None,
        )),
    })
}

fn render_catalogue_panel() -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "note-catalogue".into(),
        label: Some("Block kinds".into()),
        default_open: Some(true),
        children: vec![
            ui_text("text — rich text block"),
            ui_text("image — embedded image"),
            ui_text("table — grid block"),
            ui_text("math — TeX equation"),
            ui_text("ink — pencil strokes"),
            ui_text("group — nested blocks"),
        ],
    }])
}

fn render_properties_panel(document: &NoteDocument, view_state: &ViewState) -> UiNode {
    let selected = selection_from_view(view_state);
    let blocks: Vec<&NoteBlockNode> = selected
        .iter()
        .filter_map(|id| find_block(&document.blocks, id))
        .collect();
    if blocks.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", document.schema)),
            ui_text(format!("Blocks: {}", flatten_blocks(&document.blocks).len())),
            ui_text(format!("Tool: {}", document.active_tool.clone().unwrap_or_else(|| "selectDirect".into()))),
        ]);
    }
    let names: Vec<String> = blocks.iter().map(|block| block_name(*block).into()).collect();
    let xs: Vec<f64> = blocks
        .iter()
        .map(|block| match block {
            NoteBlockNode::Text { x, .. }
            | NoteBlockNode::Image { x, .. }
            | NoteBlockNode::Table { x, .. }
            | NoteBlockNode::Math { x, .. }
            | NoteBlockNode::Ink { x, .. }
            | NoteBlockNode::Group { x, .. } => *x,
        })
        .collect();
    let mixed_name = ui_inspector_mixed_text(&names);
    let mixed_x = ui_inspector_mixed_number(&xs);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "note-properties.block".into(),
        label: "Block".into(),
        default_open: Some(true),
        fields: vec![
            ui_inspector_readonly_field(
                "note-properties.name",
                "Name",
                mixed_name.placeholder.unwrap_or(mixed_name.value),
            ),
            ui_inspector_readonly_field(
                "note-properties.x",
                "X",
                if mixed_x.uniform {
                    mixed_x.value.to_string()
                } else {
                    "Mixed".into()
                },
            ),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Scenes
fn render_canvas_scene(document: &NoteDocument, surface_id: &str) -> UiNode {
    build_canvas_2d_scene(
        surface_id,
        NOTE_PLAY_CONTROLLER_ID,
        Canvas2dScene {
            camera_x: document.camera.x,
            camera_y: document.camera.y,
            zoom: document.camera.zoom,
            layers_json: serde_json::to_string(&document.blocks).unwrap_or_else(|_| "[]".into()),
        },
    )
}
//#endregion 🔖Scenes

//#region 🔖NoteApp
struct NoteApp;

impl PluginApp for NoteApp {
    fn app_id(&self) -> &str {
        NOTE_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&empty_note_document()).expect("note document json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut document: NoteDocument =
            serde_json::from_str(document_json).unwrap_or_else(|_| empty_note_document());
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        document = parsed;
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
            }
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        document.camera = parsed;
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
                if let Some(zoom) = args.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()) {
                    document.camera.zoom = zoom;
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    document.active_tool = Some(tool.into());
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setGridVisible" | "toggleGrid" => {
                let visible = args
                    .and_then(|value| value.get("visible"))
                    .and_then(|value| value.as_bool())
                    .unwrap_or(!(document.grid_visible.unwrap_or(true)));
                document.grid_visible = Some(visible);
                return vec![json!({ "op": "setDocument", "document": document }).to_string()];
            }
            "setGridSpacing" => {
                if let Some(spacing) = args.and_then(|value| value.get("spacing")).and_then(|value| value.as_f64()) {
                    document.grid_spacing = Some(spacing);
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setPencilWidth" => {
                if let Some(width) = args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()) {
                    document.pencil_width = Some(width);
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setEraserRadius" => {
                if let Some(radius) = args.and_then(|value| value.get("radius")).and_then(|value| value.as_f64()) {
                    document.eraser_radius = Some(radius);
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "addBlock" => {
                let kind = args
                    .and_then(|value| value.get("kind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("text");
                document.blocks.push(create_block_by_kind(kind));
                return vec![json!({ "op": "setDocument", "document": document }).to_string()];
            }
            "deleteBlock" | "deleteSelection" => {
                if let Some(block_id) = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()) {
                    remove_block_from_tree(&mut document.blocks, block_id);
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "duplicateBlock" | "duplicateSelection" => {
                if let Some(block_id) = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()) {
                    if let Some(block) = find_block(&document.blocks, block_id).cloned() {
                        document.blocks.push(clone_block(&block));
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
            }
            "selectAll" => {
                let ids: Vec<String> = flatten_blocks(&document.blocks)
                    .into_iter()
                    .map(|block| block_id(block).into())
                    .collect();
                return vec![json!({ "op": "setSelection", "ids": ids }).to_string()];
            }
            "clearSelection" | "setSelection" | "setHover" | "undo" | "redo" => {}
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let document: NoteDocument =
            serde_json::from_str(document_json).unwrap_or_else(|_| empty_note_document());
        match body_key {
            NOTE_PLAY_BODY_COMPOSITE => render_canvas_scene(&document, NOTE_PLAY_SURFACE_COMPOSITE),
            NOTE_PLAY_BODY_NAVIGATOR => render_canvas_scene(&document, NOTE_PLAY_SURFACE_NAVIGATOR),
            NOTE_PLAY_BODY_HIERARCHY => render_hierarchy_panel(&document, view_state),
            NOTE_PLAY_BODY_CATALOGUE => render_catalogue_panel(),
            NOTE_PLAY_BODY_PROPERTIES => render_properties_panel(&document, view_state),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖NoteApp

//#region 🔖Manifest
fn create_note_app() -> App {
    App::from_builder(
        App::builder(NOTE_PLAY_APP_ID, "Note")
            .icon_id("note")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(NOTE_PLAY_WINDOW_COMPOSITE, "Canvas", NOTE_PLAY_BODY_COMPOSITE)
            .window_kind(NOTE_PLAY_WINDOW_NAVIGATOR, "Navigator", NOTE_PLAY_BODY_NAVIGATOR)
            .default_layout(create_default_layout(
                &[NOTE_PLAY_WINDOW_COMPOSITE.into(), NOTE_PLAY_WINDOW_NAVIGATOR.into()],
                "row",
                Some(&[72.0, 28.0]),
                Some(&["Canvas".into(), "Navigator".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                NOTE_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                NOTE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                NOTE_PLAY_BODY_PROPERTIES,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("escape", "clearSelection")
            .keybinding("up", "nudgeSelection")
            .keybinding("down", "nudgeSelection")
            .keybinding("left", "nudgeSelection")
            .keybinding("right", "nudgeSelection"),
    )
    .example("empty", "Empty", serde_json::to_string(&empty_note_document()).unwrap())
    .example("semio", "Semio", SEMIO_EXAMPLE_JSON)
    .program("note", "Note", "document")
}

fn note_bundle() -> PluginBundle {
    PluginBundle::new("note", "Note", "0.1.0").register_app(create_note_app(), || Box::new(NoteApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(note_bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_composite_canvas() {
        let app = NoteApp;
        let document = serde_json::to_string(&empty_note_document()).unwrap();
        let node = app.render(NOTE_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_navigator_canvas() {
        let app = NoteApp;
        let document = SEMIO_EXAMPLE_JSON.to_string();
        let node = app.render(NOTE_PLAY_BODY_NAVIGATOR, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn parses_semio_example_document() {
        let document: NoteDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).expect("semio note json");
        assert_eq!(document.blocks.len(), 3);
    }

    #[test]
    fn renders_hierarchy_tree() {
        let app = NoteApp;
        let document = SEMIO_EXAMPLE_JSON.to_string();
        let node = app.render(NOTE_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Welcome"));
    }

    #[test]
    fn add_block_command() {
        let mut app = NoteApp;
        let document = serde_json::to_string(&empty_note_document()).unwrap();
        let ops = app.handle_command(
            "addBlock",
            Some(&json!({ "kind": "text" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        assert!(ops[0].contains("\"kind\":\"text\""));
    }
}
//#endregion 🧪Tests
