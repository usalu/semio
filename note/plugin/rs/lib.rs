//! 📝 Note plugin — infinite canvas note board bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind, PanelGroup, 
    build_canvas_2d_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App,
    Canvas2dScene, CommandDescriptor, PluginApp, PluginBundle, UiControlNode, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiSectionNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER, create_default_layout,
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
const NOTE_PLAY_BODY_DOCUMENT: &str = "note.play.document";
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
    snap_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snap_grid_spacing: Option<f64>,
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
        snap_enabled: Some(false),
        snap_grid_spacing: Some(8.0),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NotePlayEnvelope {
    #[serde(flatten)]
    document: NoteDocument,
    #[serde(default)]
    undo_stack: Vec<NoteDocument>,
    #[serde(default)]
    redo_stack: Vec<NoteDocument>,
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    hovered_id: Option<String>,
}

fn parse_envelope(document_json: &str) -> NotePlayEnvelope {
    if let Ok(envelope) = serde_json::from_str::<NotePlayEnvelope>(document_json) {
        return envelope;
    }
    let document: NoteDocument = serde_json::from_str(document_json).unwrap_or_else(|_| empty_note_document());
    NotePlayEnvelope {
        document,
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
        selected_ids: Vec::new(),
        hovered_id: None,
    }
}

fn set_document_op(envelope: &NotePlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn push_undo(play: &mut NotePlayEnvelope) {
    play.undo_stack.push(play.document.clone());
    if play.undo_stack.len() > 32 {
        play.undo_stack.remove(0);
    }
    play.redo_stack.clear();
}

fn block_tree_row_id_from_str(block_id: &str) -> String {
    format!("note-play-block:{block_id}")
}

fn block_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id.strip_prefix("note-play-block:").map(str::to_string)
}

fn insert_block(blocks: &mut Vec<NoteBlockNode>, parent_id: Option<&str>, index: usize, block: NoteBlockNode) {
    if let Some(parent_id) = parent_id {
        for node in blocks.iter_mut() {
            if let NoteBlockNode::Group { id, children, .. } = node {
                if id == parent_id {
                    let index = index.min(children.len());
                    children.insert(index, block);
                    return;
                }
                insert_block(children, Some(parent_id), index, block.clone());
            }
        }
        return;
    }
    let index = index.min(blocks.len());
    blocks.insert(index, block);
}

fn update_block_in_tree(blocks: &mut [NoteBlockNode], target_id: &str, next_block: NoteBlockNode) -> bool {
    for block in blocks.iter_mut() {
        if block_id(block) == target_id {
            *block = next_block;
            return true;
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if update_block_in_tree(children, target_id, next_block.clone()) {
                return true;
            }
        }
    }
    false
}

fn mutate_block_in_tree(
    blocks: &mut [NoteBlockNode],
    target_id: &str,
    mutator: &mut impl FnMut(&NoteBlockNode) -> NoteBlockNode,
) -> bool {
    for block in blocks.iter_mut() {
        if block_id(block) == target_id {
            *block = mutator(block);
            return true;
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if mutate_block_in_tree(children, target_id, mutator) {
                return true;
            }
        }
    }
    false
}

fn block_bounds(block: &NoteBlockNode) -> (f64, f64, f64, f64) {
    match block {
        NoteBlockNode::Text { x, y, width, height, .. }
        | NoteBlockNode::Image { x, y, width, height, .. }
        | NoteBlockNode::Table { x, y, width, height, .. }
        | NoteBlockNode::Math { x, y, width, height, .. }
        | NoteBlockNode::Ink { x, y, width, height, .. }
        | NoteBlockNode::Group { x, y, width, height, .. } => (*x, *y, *width, *height),
    }
}

fn patch_block_field(document: &NoteDocument, block_id: &str, field: &str, value: &Value) -> NoteDocument {
    let Some(block) = find_block(&document.blocks, block_id).cloned() else {
        return document.clone();
    };
    let mut next = document.clone();
    match field {
        "name" => {
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { name, .. }
                    | NoteBlockNode::Image { name, .. }
                    | NoteBlockNode::Table { name, .. }
                    | NoteBlockNode::Math { name, .. }
                    | NoteBlockNode::Ink { name, .. }
                    | NoteBlockNode::Group { name, .. } => *name = value.as_str().unwrap_or("").into(),
                }
                cloned
            });
        }
        "visible" => {
            let pressed = value.as_bool().unwrap_or(true);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { visible, .. }
                    | NoteBlockNode::Image { visible, .. }
                    | NoteBlockNode::Table { visible, .. }
                    | NoteBlockNode::Math { visible, .. }
                    | NoteBlockNode::Ink { visible, .. }
                    | NoteBlockNode::Group { visible, .. } => *visible = pressed,
                }
                cloned
            });
        }
        "locked" => {
            let pressed = value.as_bool().unwrap_or(false);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { locked, .. }
                    | NoteBlockNode::Image { locked, .. }
                    | NoteBlockNode::Table { locked, .. }
                    | NoteBlockNode::Math { locked, .. }
                    | NoteBlockNode::Ink { locked, .. }
                    | NoteBlockNode::Group { locked, .. } => *locked = pressed,
                }
                cloned
            });
        }
        "x" | "y" | "width" | "height" => {
            let number = value.as_f64().unwrap_or(0.0);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Image { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Table { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Math { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Ink { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Group { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                }
                cloned
            });
        }
        "textContent" => {
            if let NoteBlockNode::Text { .. } = block {
                let text = value.as_str().unwrap_or("");
                let paragraphs = vec![NoteTextParagraph {
                    runs: vec![NoteTextRun { text: text.into() }],
                }];
                let mut updated = block;
                if let NoteBlockNode::Text { paragraphs: p, .. } = &mut updated {
                    *p = paragraphs;
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "textSize" => {
            if let NoteBlockNode::Text { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Text { font_size, .. } = &mut updated {
                    *font_size = value.as_f64().unwrap_or(18.0);
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "mathTex" => {
            if let NoteBlockNode::Math { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Math { tex, .. } = &mut updated {
                    *tex = value.as_str().unwrap_or("").into();
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "inkWidth" => {
            if let NoteBlockNode::Ink { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Ink { stroke_width, .. } = &mut updated {
                    *stroke_width = value.as_f64().unwrap_or(3.0);
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        _ => {}
    }
    next
}

fn selection_from_envelope(play: &NotePlayEnvelope, view_state: &ViewState) -> Vec<String> {
    if !play.selected_ids.is_empty() {
        return play.selected_ids.clone();
    }
    selection_from_view(view_state)
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
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: Some(true),
        drag_data: None,
        items: nested,
        control: None,
        is_hidden: if block_visible(block) { None } else { Some(true) },
    }
}

fn render_document_panel(document: &NoteDocument, play: &NotePlayEnvelope, view_state: &ViewState) -> UiNode {
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
        hover_command: None,
        unhover_command: None,
        actions: None,
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
        document.blocks.iter().map(block_tree_item).collect()
    };
    let selected_ids: Vec<String> = selection_from_envelope(play, view_state)
        .iter()
        .filter_map(|id| find_block(&document.blocks, id).map(block_tree_row_id))
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "note-play-blocks".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
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
        drop_command: None,
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

fn inspector_patch(block_ids: &[String], field: &str) -> CommandDescriptor {
    play_cmd(
        NOTE_PLAY_CONTROLLER_ID,
        "patchBlocks",
        Some(json!({ "blockIds": block_ids, "field": field })),
    )
}

fn inspector_text_field(block_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: None,
            on_change: inspector_patch(block_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
        }),
        description: None,
        required: None,
        error: None,
    })
}

fn inspector_number_field(block_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform {
                mixed.value.to_string()
            } else {
                String::new()
            },
            placeholder: if mixed.uniform {
                None
            } else {
                Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
            },
            commit: None,
            on_change: inspector_patch(block_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
        }),
        description: None,
        required: None,
        error: None,
    })
}

fn render_properties_panel(document: &NoteDocument, play: &NotePlayEnvelope, view_state: &ViewState) -> UiNode {
    let selected = selection_from_envelope(play, view_state);
    let blocks: Vec<&NoteBlockNode> = selected
        .iter()
        .filter_map(|id| find_block(&document.blocks, id))
        .collect();
    if blocks.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", document.schema)),
            ui_text(format!("Blocks: {}", flatten_blocks(&document.blocks).len())),
            ui_text(format!("Tool: {}", document.active_tool.clone().unwrap_or_else(|| "selectDirect".into()))),
            ui_text(format!(
                "Snap: {}",
                if document.snap_enabled.unwrap_or(false) {
                    format!("{}px", document.snap_grid_spacing.unwrap_or(8.0))
                } else {
                    "off".into()
                }
            )),
        ]);
    }
    let block_ids: Vec<String> = blocks.iter().map(|block| block_id(*block).into()).collect();
    let names: Vec<String> = blocks.iter().map(|block| block_name(*block).into()).collect();
    let xs: Vec<f64> = blocks.iter().map(|block| block_bounds(block).0).collect();
    let ys: Vec<f64> = blocks.iter().map(|block| block_bounds(block).1).collect();
    let widths: Vec<f64> = blocks.iter().map(|block| block_bounds(block).2).collect();
    let heights: Vec<f64> = blocks.iter().map(|block| block_bounds(block).3).collect();
    let visibles: Vec<bool> = blocks.iter().map(|block| block_visible(block)).collect();
    let locked: Vec<bool> = blocks
        .iter()
        .map(|block| match block {
            NoteBlockNode::Text { locked, .. }
            | NoteBlockNode::Image { locked, .. }
            | NoteBlockNode::Table { locked, .. }
            | NoteBlockNode::Math { locked, .. }
            | NoteBlockNode::Ink { locked, .. }
            | NoteBlockNode::Group { locked, .. } => *locked,
        })
        .collect();
    let visible_mixed = ui_inspector_mixed_toggle(&visibles);
    let locked_mixed = ui_inspector_mixed_toggle(&locked);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "note-properties.block".into(),
        label: "Block".into(),
        default_open: Some(true),
        fields: vec![
            inspector_text_field(&block_ids, "note-properties.name", "Name", &names, "name"),
            inspector_number_field(&block_ids, "note-properties.x", "X", &xs, "x"),
            inspector_number_field(&block_ids, "note-properties.y", "Y", &ys, "y"),
            inspector_number_field(&block_ids, "note-properties.width", "Width", &widths, "width"),
            inspector_number_field(&block_ids, "note-properties.height", "Height", &heights, "height"),
            UiNode::Field(UiFieldNode {
                id: "note-properties.visible".into(),
                label: "Visible".into(),
                child: UiControlNode::Toggle(UiToggleNode {
                    id: "note-properties.visible.toggle".into(),
                    icon_id: "eye".into(),
                    pressed: visible_mixed.uniform && visible_mixed.pressed,
                    text: None,
                    on_change: inspector_patch(&block_ids, "visible"),
                }),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "note-properties.locked".into(),
                label: "Locked".into(),
                child: UiControlNode::Toggle(UiToggleNode {
                    id: "note-properties.locked.toggle".into(),
                    icon_id: "lock".into(),
                    pressed: locked_mixed.uniform && locked_mixed.pressed,
                    text: None,
                    on_change: inspector_patch(&block_ids, "locked"),
                }),
                description: None,
                required: None,
                error: None,
            }),
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
        serde_json::to_string(&NotePlayEnvelope {
            document: empty_note_document(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            selected_ids: Vec::new(),
            hovered_id: None,
        })
        .expect("note document json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<NotePlayEnvelope>(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                    if let Ok(parsed) = serde_json::from_value::<NoteDocument>(next.clone()) {
                        play.document = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        push_undo(&mut play);
                        play.document.camera = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
                if let Some(zoom) = args.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()) {
                    push_undo(&mut play);
                    play.document.camera.zoom = zoom;
                    return vec![set_document_op(&play)];
                }
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    push_undo(&mut play);
                    play.document.active_tool = Some(tool.into());
                    return vec![set_document_op(&play)];
                }
            }
            "setGridVisible" | "toggleGrid" => {
                let visible = args
                    .and_then(|value| value.get("visible"))
                    .and_then(|value| value.as_bool())
                    .unwrap_or(!(play.document.grid_visible.unwrap_or(true)));
                push_undo(&mut play);
                play.document.grid_visible = Some(visible);
                return vec![set_document_op(&play)];
            }
            "setGridSpacing" => {
                if let Some(spacing) = args.and_then(|value| value.get("spacing")).and_then(|value| value.as_f64()) {
                    push_undo(&mut play);
                    play.document.grid_spacing = Some(spacing);
                    return vec![set_document_op(&play)];
                }
            }
            "setSnapEnabled" | "toggleSnap" => {
                let enabled = args
                    .and_then(|value| value.get("enabled"))
                    .and_then(|value| value.as_bool())
                    .unwrap_or(!(play.document.snap_enabled.unwrap_or(false)));
                push_undo(&mut play);
                play.document.snap_enabled = Some(enabled);
                return vec![set_document_op(&play)];
            }
            "setSnapGridSpacing" => {
                if let Some(spacing) = args.and_then(|value| value.get("spacing")).and_then(|value| value.as_f64()) {
                    push_undo(&mut play);
                    play.document.snap_grid_spacing = Some(spacing.max(1.0));
                    return vec![set_document_op(&play)];
                }
            }
            "setPencilWidth" => {
                if let Some(width) = args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()) {
                    push_undo(&mut play);
                    play.document.pencil_width = Some(width);
                    return vec![set_document_op(&play)];
                }
            }
            "setEraserRadius" => {
                if let Some(radius) = args.and_then(|value| value.get("radius")).and_then(|value| value.as_f64()) {
                    push_undo(&mut play);
                    play.document.eraser_radius = Some(radius);
                    return vec![set_document_op(&play)];
                }
            }
            "addBlock" => {
                let kind = args
                    .and_then(|value| value.get("kind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("text");
                push_undo(&mut play);
                play.document.blocks.push(create_block_by_kind(kind));
                return vec![set_document_op(&play)];
            }
            "dropBlockKind" => {
                let kind = args
                    .and_then(|value| value.get("kind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("text");
                push_undo(&mut play);
                let block = create_block_by_kind(kind);
                play.selected_ids = vec![block_id(&block).into()];
                play.document.blocks.push(block);
                return vec![set_document_op(&play)];
            }
            "moveBlock" => {
                let block_id_arg = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str());
                let target_row_id = args
                    .and_then(|value| value.get("targetRowId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("note-play-blocks");
                let drop_position = args
                    .and_then(|value| value.get("dropPosition"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("after");
                let Some(block_id_arg) = block_id_arg else {
                    return Vec::new();
                };
                let Some(block) = find_block(&play.document.blocks, block_id_arg).cloned() else {
                    return Vec::new();
                };
                let target_id = block_id_from_tree_row_id(target_row_id);
                let parent_id = target_id.as_ref().and_then(|id| {
                    find_block(&play.document.blocks, id).and_then(|entry| {
                        if matches!(entry, NoteBlockNode::Group { .. }) {
                            Some(id.clone())
                        } else {
                            None
                        }
                    })
                });
                let index = if drop_position == "before" {
                    0
                } else if let Some(ref parent) = parent_id {
                    find_block(&play.document.blocks, parent)
                        .and_then(|entry| match entry {
                            NoteBlockNode::Group { children, .. } => Some(children.len()),
                            _ => None,
                        })
                        .unwrap_or(0)
                } else {
                    play.document.blocks.len()
                };
                push_undo(&mut play);
                remove_block_from_tree(&mut play.document.blocks, block_id_arg);
                insert_block(&mut play.document.blocks, parent_id.as_deref(), index, block);
                return vec![set_document_op(&play)];
            }
            "deleteBlock" | "deleteSelection" => {
                if let Some(block_id) = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()) {
                    push_undo(&mut play);
                    remove_block_from_tree(&mut play.document.blocks, block_id);
                    play.selected_ids.retain(|id| id != block_id);
                    return vec![set_document_op(&play)];
                }
                if !play.selected_ids.is_empty() {
                    push_undo(&mut play);
                    for block_id in play.selected_ids.clone() {
                        remove_block_from_tree(&mut play.document.blocks, &block_id);
                    }
                    play.selected_ids.clear();
                    return vec![set_document_op(&play)];
                }
            }
            "duplicateBlock" | "duplicateSelection" => {
                if let Some(block_id) = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()) {
                    if let Some(block) = find_block(&play.document.blocks, block_id).cloned() {
                        push_undo(&mut play);
                        play.document.blocks.push(clone_block(&block));
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "patchBlocks" => {
                let block_ids: Vec<String> = args
                    .and_then(|value| value.get("blockIds"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if block_ids.is_empty() || field.is_empty() {
                    return Vec::new();
                }
                push_undo(&mut play);
                for block_id in block_ids {
                    play.document = patch_block_field(&play.document, &block_id, field, &value);
                }
                return vec![set_document_op(&play)];
            }
            "selectAll" => {
                play.selected_ids = flatten_blocks(&play.document.blocks)
                    .into_iter()
                    .map(|block| block_id(block).into())
                    .collect();
                return vec![set_document_op(&play)];
            }
            "clearSelection" => {
                play.selected_ids.clear();
                return vec![set_document_op(&play)];
            }
            "setSelection" => {
                play.selected_ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                return vec![set_document_op(&play)];
            }
            "setHover" => {
                play.hovered_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&play)];
            }
            "nudgeSelection" | "nudgeSelectionUp" | "nudgeSelectionDown" | "nudgeSelectionLeft" | "nudgeSelectionRight" => {
                const NUDGE_STEP: f64 = 1.0;
                let (default_dx, default_dy) = match command {
                    "nudgeSelectionUp" => (0.0, -NUDGE_STEP),
                    "nudgeSelectionDown" => (0.0, NUDGE_STEP),
                    "nudgeSelectionLeft" => (-NUDGE_STEP, 0.0),
                    "nudgeSelectionRight" => (NUDGE_STEP, 0.0),
                    _ => (0.0, 0.0),
                };
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(default_dx);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(default_dy);
                if play.selected_ids.is_empty() {
                    return Vec::new();
                }
                push_undo(&mut play);
                let selected: std::collections::HashSet<String> = play.selected_ids.iter().cloned().collect();
                let nudges: Vec<(String, NoteBlockNode)> = flatten_blocks(&play.document.blocks)
                    .into_iter()
                    .filter(|block| selected.contains(block_id(block)))
                    .filter_map(|block| {
                        let locked = matches!(
                            block,
                            NoteBlockNode::Group { locked: true, .. }
                                | NoteBlockNode::Text { locked: true, .. }
                                | NoteBlockNode::Image { locked: true, .. }
                                | NoteBlockNode::Table { locked: true, .. }
                                | NoteBlockNode::Math { locked: true, .. }
                                | NoteBlockNode::Ink { locked: true, .. }
                        );
                        if locked {
                            return None;
                        }
                        let id = block_id(block).to_string();
                        let mut updated = block.clone();
                        match &mut updated {
                            NoteBlockNode::Text { x, y, .. }
                            | NoteBlockNode::Image { x, y, .. }
                            | NoteBlockNode::Table { x, y, .. }
                            | NoteBlockNode::Math { x, y, .. }
                            | NoteBlockNode::Ink { x, y, .. }
                            | NoteBlockNode::Group { x, y, .. } => {
                                *x += dx;
                                *y += dy;
                            }
                        }
                        Some((id, updated))
                    })
                    .collect();
                for (id, updated) in nudges {
                    update_block_in_tree(&mut play.document.blocks, &id, updated);
                }
                return vec![set_document_op(&play)];
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
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            NOTE_PLAY_BODY_COMPOSITE => render_canvas_scene(&play.document, NOTE_PLAY_SURFACE_COMPOSITE),
            NOTE_PLAY_BODY_NAVIGATOR => render_canvas_scene(&play.document, NOTE_PLAY_SURFACE_NAVIGATOR),
            NOTE_PLAY_BODY_DOCUMENT => render_document_panel(&play.document, &play, view_state),
            NOTE_PLAY_BODY_CATALOGUE => render_catalogue_panel(),
            NOTE_PLAY_BODY_PROPERTIES => render_properties_panel(&play.document, &play, view_state),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖NoteApp

//#region 🔖MediaExport
fn escape_svg_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn note_document_bounds(document: &NoteDocument) -> (u32, u32) {
    let mut max_x = 1024.0_f64;
    let mut max_y = 1024.0_f64;
    for block in flatten_blocks(&document.blocks) {
        if !block_visible(block) {
            continue;
        }
        let (x, y, width, height) = match block {
            NoteBlockNode::Text { x, y, width, height, .. }
            | NoteBlockNode::Image { x, y, width, height, .. }
            | NoteBlockNode::Table { x, y, width, height, .. }
            | NoteBlockNode::Math { x, y, width, height, .. }
            | NoteBlockNode::Ink { x, y, width, height, .. }
            | NoteBlockNode::Group { x, y, width, height, .. } => (*x, *y, *width, *height),
        };
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

fn note_block_to_svg(block: &NoteBlockNode) -> String {
    let (x, y, rotation, width, height) = match block {
        NoteBlockNode::Text { x, y, rotation, width, height, .. }
        | NoteBlockNode::Image { x, y, rotation, width, height, .. }
        | NoteBlockNode::Table { x, y, rotation, width, height, .. }
        | NoteBlockNode::Math { x, y, rotation, width, height, .. }
        | NoteBlockNode::Ink { x, y, rotation, width, height, .. }
        | NoteBlockNode::Group { x, y, rotation, width, height, .. } => (*x, *y, *rotation, *width, *height),
    };
    let transform = format!("translate({x} {y}) rotate({rotation})");
    match block {
        NoteBlockNode::Text {
            paragraphs,
            font_size,
            font_weight,
            ..
        } => {
            let text = paragraphs
                .iter()
                .map(|paragraph| paragraph.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join(""))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                r#"<g transform="{transform}"><text x="0" y="{font_size}" font-size="{font_size}" font-weight="{font_weight}" fill="black">{}</text></g>"#,
                escape_svg_text(&text)
            )
        }
        NoteBlockNode::Image { .. } => format!(
            "<g transform=\"{transform}\"><rect width=\"{width}\" height=\"{height}\" fill=\"#ddd\" stroke=\"#888\"/></g>"
        ),
        NoteBlockNode::Ink {
            points,
            stroke_width,
            color,
            ..
        } => {
            if points.len() < 2 {
                return String::new();
            }
            let mut d = format!("M {} {}", points[0][0], points[0][1]);
            for point in points.iter().skip(1) {
                d.push_str(&format!(" L {} {}", point[0], point[1]));
            }
            let stroke = format!(
                "rgba({},{},{},{})",
                (color[0] * 255.0).round() as u8,
                (color[1] * 255.0).round() as u8,
                (color[2] * 255.0).round() as u8,
                color[3]
            );
            format!(
                r#"<g transform="{transform}"><path d="{d}" fill="none" stroke="{stroke}" stroke-width="{stroke_width}" stroke-linecap="round" stroke-linejoin="round"/></g>"#
            )
        }
        _ => format!(
            "<g transform=\"{transform}\"><rect width=\"{width}\" height=\"{height}\" fill=\"none\" stroke=\"#888\"/></g>"
        ),
    }
}

fn note_document_to_svg(document: &NoteDocument) -> (String, u32, u32) {
    let (width, height) = note_document_bounds(document);
    let body = flatten_blocks(&document.blocks)
        .into_iter()
        .filter(|block| block_visible(block))
        .map(note_block_to_svg)
        .collect::<Vec<_>>()
        .join("");
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{body}</svg>"#
    );
    (svg, width, height)
}

fn note_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let document: NoteDocument = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    Ok(note_document_to_svg(&document))
}

fn register_note_exports() {
    semio_framework_os::register_2d_svg_png_export_handlers("2d.note", "note", note_document_json_to_svg);
}
//#endregion 🔖MediaExport

//#region 🔖Manifest
fn create_note_app() -> App {
    App::from_builder(
        App::builder(NOTE_PLAY_APP_ID, "Note").document(["semio", "note"])
            .icon_id("note")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(NOTE_PLAY_WINDOW_COMPOSITE, "Canvas", NOTE_PLAY_BODY_COMPOSITE, SurfaceKind::Canvas2d)
            .window_kind(NOTE_PLAY_WINDOW_NAVIGATOR, "Navigator", NOTE_PLAY_BODY_NAVIGATOR, SurfaceKind::Canvas2d)
            .default_layout(create_default_layout(
                &[NOTE_PLAY_WINDOW_COMPOSITE.into(), NOTE_PLAY_WINDOW_NAVIGATOR.into()],
                "row",
                Some(&[72.0, 28.0]),
                Some(&["Canvas".into(), "Navigator".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                NOTE_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                NOTE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                NOTE_PLAY_BODY_PROPERTIES,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("escape", "clearSelection")
            .keybinding("up", "nudgeSelectionUp")
            .keybinding("down", "nudgeSelectionDown")
            .keybinding("left", "nudgeSelectionLeft")
            .keybinding("right", "nudgeSelectionRight"),
    )
    .example("empty", "Empty", serde_json::to_string(&empty_note_document()).unwrap())
    .example("semio", "Semio", SEMIO_EXAMPLE_JSON)
    .program("note", "Note", "document")
}

fn note_bundle() -> PluginBundle {
    register_note_exports();
    PluginBundle::new("note", "Note", "0.1.0").register_app(create_note_app(), || Box::new(NoteApp))
}

semio_framework_plugin::plugin_exports!(note_bundle);
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
    fn renders_document_tree() {
        let app = NoteApp;
        let document = SEMIO_EXAMPLE_JSON.to_string();
        let node = app.render(NOTE_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Welcome"));
    }

    #[test]
    fn add_block_command() {
        let mut app = NoteApp;
        let document = serde_json::to_string(&empty_note_document()).unwrap();
        let ops = app.handle_command_patch_ops(
            "addBlock",
            Some(&json!({ "kind": "text" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        assert!(ops[0].contains("\"kind\":\"text\""));
    }

    #[test]
    fn nudge_direction_commands_move_selection_without_args() {
        let mut app = NoteApp;
        let mut play = parse_envelope(&serde_json::to_string(&empty_note_document()).unwrap());
        let block = create_block_by_kind("text");
        let block_id = block_id(&block).to_string();
        play.document.blocks.push(block);
        play.selected_ids = vec![block_id.clone()];
        let document = serde_json::to_string(&play).unwrap();

        for (command, expected_dx, expected_dy) in [
            ("nudgeSelectionUp", 0.0, -1.0),
            ("nudgeSelectionDown", 0.0, 1.0),
            ("nudgeSelectionLeft", -1.0, 0.0),
            ("nudgeSelectionRight", 1.0, 0.0),
        ] {
            let ops = app.handle_command_patch_ops(command, None, &document, &ViewState::default());
            assert_eq!(ops.len(), 1, "{command} should emit a setDocument op");
            let updated: NotePlayEnvelope =
                serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
            let moved = find_block(&updated.document.blocks, &block_id).unwrap();
            let (x, y, ..) = block_bounds(moved);
            assert_eq!((x, y), (expected_dx, expected_dy), "{command} moved block to unexpected position");
        }
    }
}
//#endregion 🧪Tests
