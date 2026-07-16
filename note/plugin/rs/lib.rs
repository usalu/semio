//! 📝 Note plugin — infinite canvas note board bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_note_canvas_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_stack_vertical, ui_text, App,
    NoteCanvasScene, ActionDescriptor, ActionEmit, AppLabelsOverlay, DocumentApp, DocumentView, DwgDrawing, DwgGeometry,
    HostEffect, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiSectionNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER, create_default_layout,
    ActionDefinition, ActionKind, ActionArgDef, ActionArgOption, UtilityDefinition, UtilityCategory, SET_ACTIVE_UTILITY_ACTION_ID,
    WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowMeasure,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use vcs::{Operation, OperationDiff};

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    link: Option<String>,
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
struct NoteImageAsset {
    mime: String,
    data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
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
    grid_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid_subdivisions: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snap_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snap_grid_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pencil_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eraser_radius: Option<f64>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    assets: BTreeMap<String, NoteImageAsset>,
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
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        grid_subdivisions: Some(4.0),
        grid_opacity: Some(0.35),
        snap_enabled: Some(false),
        snap_grid_spacing: Some(8.0),
        pencil_width: Some(3.0),
        eraser_radius: Some(12.0),
        assets: BTreeMap::new(),
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

fn create_block_by_kind(kind: &str, x: f64, y: f64) -> NoteBlockNode {
    let id = create_note_id(kind);
    match kind {
        "image" => NoteBlockNode::Image {
            id,
            name: "Image".into(),
            x,
            y,
            width: 240.0,
            height: 160.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            image_key: "placeholder".into(),
        },
        "table" => NoteBlockNode::Table {
            id,
            name: "Table".into(),
            x,
            y,
            width: 320.0,
            height: 160.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            columns: vec!["A".into(), "B".into(), "C".into()],
            rows: vec![
                vec![
                    NoteTableCell { content: String::new() },
                    NoteTableCell { content: String::new() },
                    NoteTableCell { content: String::new() },
                ],
                vec![
                    NoteTableCell { content: String::new() },
                    NoteTableCell { content: String::new() },
                    NoteTableCell { content: String::new() },
                ],
            ],
        },
        "math" => NoteBlockNode::Math {
            id,
            name: "Math".into(),
            x,
            y,
            width: 200.0,
            height: 80.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            tex: "E = mc^2".into(),
            display_mode: true,
        },
        "ink" => NoteBlockNode::Ink {
            id,
            name: "Ink".into(),
            x,
            y,
            width: 1.0,
            height: 1.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            points: Vec::new(),
            stroke_width: 3.0,
            color: [0.0, 0.0, 0.0, 1.0],
        },
        "group" => NoteBlockNode::Group {
            id,
            name: "Group".into(),
            x,
            y,
            width: 280.0,
            height: 120.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            children: Vec::new(),
        },
        _ => NoteBlockNode::Text {
            id,
            name: "Text".into(),
            x,
            y,
            width: 280.0,
            height: 120.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            paragraphs: vec![NoteTextParagraph {
                runs: vec![NoteTextRun {
                    text: String::new(),
                    bold: None,
                    italic: None,
                    underline: None,
                    link: None,
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

fn reid_block_tree(block: &mut NoteBlockNode, rename_top: bool) {
    let kind = block_kind(block).to_string();
    match block {
        NoteBlockNode::Text { id, name, .. }
        | NoteBlockNode::Image { id, name, .. }
        | NoteBlockNode::Table { id, name, .. }
        | NoteBlockNode::Math { id, name, .. }
        | NoteBlockNode::Ink { id, name, .. }
        | NoteBlockNode::Group { id, name, .. } => {
            *id = create_note_id(&kind);
            if rename_top {
                *name = format!("{name} copy");
            }
        }
    }
    if let NoteBlockNode::Group { children, .. } = block {
        for child in children.iter_mut() {
            reid_block_tree(child, false);
        }
    }
}

fn clone_block(block: &NoteBlockNode) -> NoteBlockNode {
    let mut cloned: NoteBlockNode = serde_json::from_value(serde_json::to_value(block).unwrap()).unwrap();
    reid_block_tree(&mut cloned, true);
    cloned
}

fn offset_block_tree(block: &mut NoteBlockNode, dx: f64, dy: f64) {
    match block {
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
    if let NoteBlockNode::Group { children, .. } = block {
        for child in children.iter_mut() {
            offset_block_tree(child, dx, dy);
        }
    }
}

fn insert_after(blocks: &mut Vec<NoteBlockNode>, target_id: &str, block: NoteBlockNode) -> bool {
    if let Some(index) = blocks.iter().position(|entry| block_id(entry) == target_id) {
        blocks.insert(index + 1, block);
        return true;
    }
    for entry in blocks.iter_mut() {
        if let NoteBlockNode::Group { children, .. } = entry {
            if insert_after(children, target_id, block.clone()) {
                return true;
            }
        }
    }
    false
}

/// 📐 Typed content mutation for a `NoteDocument`. Every content change flows through one of these so
/// the `DocumentVcsStore` records a true inverse (`backwards`). Scalar setters carry the field's own
/// `Option` shape (backwards is a plain prior-value read); block edits use a whole-tree `SetBlocks`
/// snapshot (the recursive reid/clone tree makes per-node ops far messier than a snapshot); asset and
/// full-document loads have dedicated variants.
///
/// See {@link vcs::Operation} and {@link https://../../../vcs/plugin/rs/lib.rs VcsDemoOp}.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
enum NoteOp {
    SetCamera { camera: NoteCamera },
    SetGridVisible { visible: Option<bool> },
    SetGridSpacing { spacing: Option<f64> },
    SetGridSubdivisions { value: Option<f64> },
    SetGridOpacity { opacity: Option<f64> },
    SetSnapEnabled { enabled: Option<bool> },
    SetSnapGridSpacing { spacing: Option<f64> },
    SetPencilWidth { width: Option<f64> },
    SetEraserRadius { radius: Option<f64> },
    SetBlocks { blocks: Vec<NoteBlockNode> },
    PutAsset { key: String, asset: NoteImageAsset },
    SetDocument { document: NoteDocument },
}

/// 🧩 Snapshot diff wrapping the forward `NoteOp` — `apply` replays it, `absorb` keeps the latest
/// (coalescing a whole gesture's `SetBlocks` stream into one edit).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
struct NoteDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    op: Option<NoteOp>,
}

impl OperationDiff<NoteDocument> for NoteDiff {
    fn apply(&self, projection: &NoteDocument) -> NoteDocument {
        match &self.op {
            Some(op) => apply_note_op(projection, op),
            None => projection.clone(),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.op.is_some() {
            self.op = other.op;
        }
    }
}

impl Operation<NoteDocument> for NoteOp {
    type Diff = NoteDiff;

    fn diff(&self, _projection: &NoteDocument) -> NoteDiff {
        NoteDiff { op: Some(self.clone()) }
    }

    fn backwards(&self, projection: &NoteDocument) -> Vec<Self> {
        match self {
            NoteOp::SetCamera { .. } => vec![NoteOp::SetCamera { camera: projection.camera.clone() }],
            NoteOp::SetGridVisible { .. } => vec![NoteOp::SetGridVisible { visible: projection.grid_visible }],
            NoteOp::SetGridSpacing { .. } => vec![NoteOp::SetGridSpacing { spacing: projection.grid_spacing }],
            NoteOp::SetGridSubdivisions { .. } => vec![NoteOp::SetGridSubdivisions { value: projection.grid_subdivisions }],
            NoteOp::SetGridOpacity { .. } => vec![NoteOp::SetGridOpacity { opacity: projection.grid_opacity }],
            NoteOp::SetSnapEnabled { .. } => vec![NoteOp::SetSnapEnabled { enabled: projection.snap_enabled }],
            NoteOp::SetSnapGridSpacing { .. } => vec![NoteOp::SetSnapGridSpacing { spacing: projection.snap_grid_spacing }],
            NoteOp::SetPencilWidth { .. } => vec![NoteOp::SetPencilWidth { width: projection.pencil_width }],
            NoteOp::SetEraserRadius { .. } => vec![NoteOp::SetEraserRadius { radius: projection.eraser_radius }],
            NoteOp::SetBlocks { .. } => vec![NoteOp::SetBlocks { blocks: projection.blocks.clone() }],
            NoteOp::PutAsset { .. } => vec![NoteOp::SetDocument { document: projection.clone() }],
            NoteOp::SetDocument { .. } => vec![NoteOp::SetDocument { document: projection.clone() }],
        }
    }
}

fn apply_note_op(projection: &NoteDocument, op: &NoteOp) -> NoteDocument {
    let mut next = projection.clone();
    match op {
        NoteOp::SetCamera { camera } => next.camera = camera.clone(),
        NoteOp::SetGridVisible { visible } => next.grid_visible = *visible,
        NoteOp::SetGridSpacing { spacing } => next.grid_spacing = *spacing,
        NoteOp::SetGridSubdivisions { value } => next.grid_subdivisions = *value,
        NoteOp::SetGridOpacity { opacity } => next.grid_opacity = *opacity,
        NoteOp::SetSnapEnabled { enabled } => next.snap_enabled = *enabled,
        NoteOp::SetSnapGridSpacing { spacing } => next.snap_grid_spacing = *spacing,
        NoteOp::SetPencilWidth { width } => next.pencil_width = *width,
        NoteOp::SetEraserRadius { radius } => next.eraser_radius = *radius,
        NoteOp::SetBlocks { blocks } => next.blocks = blocks.clone(),
        NoteOp::PutAsset { key, asset } => {
            next.assets.insert(key.clone(), asset.clone());
        }
        NoteOp::SetDocument { document } => next = document.clone(),
    }
    next
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
                    runs: vec![NoteTextRun {
                        text: text.into(),
                        bold: None,
                        italic: None,
                        underline: None,
                        link: None,
                    }],
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
        "tableAddRow" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    let width = columns.len();
                    rows.push((0..width).map(|_| NoteTableCell { content: String::new() }).collect());
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableRemoveRow" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { rows, .. } = &mut updated {
                    if rows.len() > 1 {
                        rows.pop();
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableAddColumn" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    let next_letter = (b'A' + (columns.len() as u8 % 26)) as char;
                    columns.push(next_letter.to_string());
                    for row in rows.iter_mut() {
                        row.push(NoteTableCell { content: String::new() });
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableRemoveColumn" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    if columns.len() > 1 {
                        columns.pop();
                        for row in rows.iter_mut() {
                            row.pop();
                        }
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        _ => {}
    }
    next
}

fn selection_or_view(selected_ids: &[String], view_state: &ViewState) -> Vec<String> {
    if !selected_ids.is_empty() {
        return selected_ids.to_vec();
    }
    selection_from_view(view_state)
}
//#endregion 🔖Document

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the note app; one field per label makes every locale combination compile-checked.
struct NoteLabels {
    catalogue_title: &'static str,
    catalogue_text: &'static str,
    catalogue_image: &'static str,
    catalogue_table: &'static str,
    catalogue_math: &'static str,
    catalogue_ink: &'static str,
    catalogue_group: &'static str,
    inspector_block: &'static str,
    document_empty: &'static str,
    add_text: &'static str,
    add_table: &'static str,
    add_math: &'static str,
    add_image: &'static str,
    add_group: &'static str,
    window_composite: &'static str,
    window_navigator: &'static str,
}

const NOTE_LABELS_NATIVE_EN: NoteLabels = NoteLabels {
    catalogue_title: "Block kinds",
    catalogue_text: "text — rich text block",
    catalogue_image: "image — embedded image",
    catalogue_table: "table — grid block",
    catalogue_math: "math — TeX equation",
    catalogue_ink: "ink — pencil strokes",
    catalogue_group: "group — nested blocks",
    inspector_block: "Block",
    document_empty: "Drop blocks here",
    add_text: "Add Text",
    add_table: "Add Table",
    add_math: "Add Math",
    add_image: "Add Image",
    add_group: "Add Group",
    window_composite: "Canvas",
    window_navigator: "Navigator",
};

const NOTE_LABELS_NATIVE_DE: NoteLabels = NoteLabels {
    catalogue_title: "Blockarten",
    catalogue_text: "Text — reicher Textblock",
    catalogue_image: "Bild — eingebettetes Bild",
    catalogue_table: "Tabelle — Rasterblock",
    catalogue_math: "Mathe — TeX-Formel",
    catalogue_ink: "Tinte — Stiftstriche",
    catalogue_group: "Gruppe — verschachtelte Blöcke",
    inspector_block: "Block",
    document_empty: "Blöcke hier ablegen",
    add_text: "Text hinzufügen",
    add_table: "Tabelle hinzufügen",
    add_math: "Mathe hinzufügen",
    add_image: "Bild hinzufügen",
    add_group: "Gruppe hinzufügen",
    window_composite: "Leinwand",
    window_navigator: "Navigator",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; note has no terminology axis, only language.
fn note_labels(view_state: &ViewState) -> &'static NoteLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &NOTE_LABELS_NATIVE_DE
    } else {
        &NOTE_LABELS_NATIVE_EN
    }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
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
        loading: None,
        selected: None,
        default_open: Some(matches!(block, NoteBlockNode::Group { .. })),
        action: Some(play_action(
            NOTE_PLAY_CONTROLLER_ID,
            "setSelection",
            Some(json!({ "ids": [block_id(block)] })),
        )),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: Some(true),
        drag_data: None,
        items: nested,
        control: None,
        is_hidden: if block_visible(block) { None } else { Some(true) },
    }
}

fn render_document_panel(document: &NoteDocument, selected_ids: &[String], view_state: &ViewState, labels: &NoteLabels) -> UiNode {
    let toolbar = vec![
        ("text", labels.add_text, "type"),
        ("table", labels.add_table, "table"),
        ("math", labels.add_math, "sigma"),
        ("image", labels.add_image, "image"),
        ("group", labels.add_group, "folder-plus"),
    ]
    .into_iter()
    .map(|(kind, label, icon)| UiTreeItemNode {
        id: format!("note-play-blocks.add.{kind}"),
        label: label.into(),
        description: None,
        icon_id: Some(icon.into()),
        loading: None,
        selected: None,
        default_open: None,
        action: Some(play_action(
            NOTE_PLAY_CONTROLLER_ID,
            "addBlock",
            Some(json!({ "kind": kind })),
        )),
        hover_action: None,
        unhover_action: None,
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
            label: labels.document_empty.into(),
            description: None,
            icon_id: Some("sticky-note".into()),
            loading: None,
            selected: None,
            default_open: None,
            action: None,
            hover_action: None,
            unhover_action: None,
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
    let selected_ids: Vec<String> = selection_or_view(selected_ids, view_state)
        .iter()
        .filter_map(|id| find_block(&document.blocks, id).map(block_tree_row_id))
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "note-play-blocks".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            loading: None,
            items: [toolbar, block_items].concat(),
        }],
        selected_ids: Some(selected_ids),
        highlighted_ids: None,
        selection_change: Some(play_action(
            NOTE_PLAY_CONTROLLER_ID,
            "setSelection",
            None,
        )),
        drop_action: None,
        loading: None,
    })
}

fn render_catalogue_panel(labels: &NoteLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "note-catalogue".into(),
        label: Some(labels.catalogue_title.into()),
        default_open: Some(true),
        loading: None,
        children: vec![
            ui_text(labels.catalogue_text),
            ui_text(labels.catalogue_image),
            ui_text(labels.catalogue_table),
            ui_text(labels.catalogue_math),
            ui_text(labels.catalogue_ink),
            ui_text(labels.catalogue_group),
        ],
    }])
}

fn inspector_patch(block_ids: &[String], field: &str) -> ActionDescriptor {
    play_action(
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
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: inspector_patch(block_ids, field),
        })),
    })
}

fn inspector_number_field(block_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(UiInputNode {
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
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: inspector_patch(block_ids, field),
        })),
    })
}

fn render_properties_panel(document: &NoteDocument, selected_ids: &[String], view_state: &ViewState, labels: &NoteLabels) -> UiNode {
    let selected = selection_or_view(selected_ids, view_state);
    let blocks: Vec<&NoteBlockNode> = selected
        .iter()
        .filter_map(|id| find_block(&document.blocks, id))
        .collect();
    if blocks.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", document.schema)),
            ui_text(format!("Blocks: {}", flatten_blocks(&document.blocks).len())),
            ui_text(format!("Utility: {}", view_state.active_utility_id.clone().unwrap_or_else(|| "selectDirect".into()))),
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
        label: labels.inspector_block.into(),
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
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "note-properties.visible.toggle".into(),
                    icon_id: "eye".into(),
                    pressed: visible_mixed.uniform && visible_mixed.pressed,
                    text: None,
                    on_change: inspector_patch(&block_ids, "visible"),
                })),
            }),
            UiNode::Field(UiFieldNode {
                id: "note-properties.locked".into(),
                label: "Locked".into(),
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "note-properties.locked.toggle".into(),
                    icon_id: "lock".into(),
                    pressed: locked_mixed.uniform && locked_mixed.pressed,
                    text: None,
                    on_change: inspector_patch(&block_ids, "locked"),
                })),
            }),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Scenes
fn render_canvas_scene(
    document: &NoteDocument,
    selected_ids: &[String],
    hovered_id: Option<&str>,
    active_utility: &str,
    surface_id: &str,
    view_mode: &str,
) -> UiNode {
    let document_json = serde_json::to_string(document).unwrap_or_else(|_| "{}".into());
    let selection_json = serde_json::to_string(selected_ids).unwrap_or_else(|_| "[]".into());
    build_note_canvas_scene(
        surface_id,
        NOTE_PLAY_CONTROLLER_ID,
        NoteCanvasScene {
            document_json,
            selection_json,
            hovered_id: hovered_id.map(str::to_string),
            active_utility: active_utility.into(),
            view_mode: view_mode.into(),
            interactive: view_mode == "composite",
        },
    )
}
//#endregion 🔖Scenes

//#region 🔖CanvasEvents
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "op")]
enum NoteCanvasEvent {
    #[serde(rename = "addBlock", rename_all = "camelCase")]
    AddBlock {
        block: NoteBlockNode,
        #[serde(default)]
        parent_id: Option<String>,
        #[serde(default)]
        index: Option<usize>,
    },
    #[serde(rename = "updateBlock", rename_all = "camelCase")]
    UpdateBlock { block_id: String, block: NoteBlockNode },
    #[serde(rename = "removeBlock", rename_all = "camelCase")]
    RemoveBlock { block_id: String },
    #[serde(rename = "putAsset", rename_all = "camelCase")]
    PutAsset { key: String, asset: NoteImageAsset },
    #[serde(rename = "setCamera", rename_all = "camelCase")]
    SetCamera { camera: NoteCamera },
}

fn apply_note_canvas_event(document: &mut NoteDocument, event: &NoteCanvasEvent) {
    match event {
        NoteCanvasEvent::AddBlock { block, parent_id, index } => {
            insert_block(&mut document.blocks, parent_id.as_deref(), index.unwrap_or(usize::MAX), block.clone());
        }
        NoteCanvasEvent::UpdateBlock { block_id, block } => {
            update_block_in_tree(&mut document.blocks, block_id, block.clone());
        }
        NoteCanvasEvent::RemoveBlock { block_id } => {
            remove_block_from_tree(&mut document.blocks, block_id);
        }
        NoteCanvasEvent::PutAsset { key, asset } => {
            document.assets.insert(key.clone(), asset.clone());
        }
        NoteCanvasEvent::SetCamera { camera } => {
            document.camera = camera.clone();
        }
    }
}

/// 🔀 Applies a batch of canvas events to a cloned document and returns the minimal `NoteOp`s
/// describing what changed (block-tree snapshot, camera, and per-asset puts) — the empty vec means
/// no content changed (e.g. a gesture that ended where it began).
fn note_ops_from_canvas_events(document: &NoteDocument, events: &[NoteCanvasEvent]) -> Vec<NoteOp> {
    let mut next = document.clone();
    for event in events {
        apply_note_canvas_event(&mut next, event);
    }
    let mut ops = Vec::new();
    if next.blocks != document.blocks {
        ops.push(NoteOp::SetBlocks { blocks: next.blocks.clone() });
    }
    if next.camera != document.camera {
        ops.push(NoteOp::SetCamera { camera: next.camera.clone() });
    }
    for (key, asset) in &next.assets {
        if document.assets.get(key) != Some(asset) {
            ops.push(NoteOp::PutAsset { key: key.clone(), asset: asset.clone() });
        }
    }
    ops
}
//#endregion 🔖CanvasEvents

//#region 🔖Shell
fn note_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    play_action(NOTE_PLAY_CONTROLLER_ID, action, args)
}

fn note_canvas_measures(document: &NoteDocument) -> Vec<WindowMeasure> {
    vec![
        WindowMeasure::Group {
            id: "note-measures.camera".into(),
            label: "Camera".into(),
            default_open: Some(true),
            active_utility_id: None,
            children: vec![WindowMeasure::Slider {
                id: "note-measures.zoom".into(),
                label: Some("Zoom".into()),
                value: document.camera.zoom,
                min: 0.1,
                max: 8.0,
                step: Some(0.05),
                on_change: note_action("setCameraZoom", None),
            }],
        },
        WindowMeasure::Group {
            id: "note-measures.grid".into(),
            label: "Grid".into(),
            default_open: Some(true),
            active_utility_id: None,
            children: vec![
                WindowMeasure::Toggle {
                    id: "note-measures.grid-visible".into(),
                    icon_id: "layout-grid".into(),
                    label: Some("Show grid".into()),
                    pressed: document.grid_visible.unwrap_or(true),
                    text: None,
                    on_change: note_action("setGridVisible", None),
                },
                WindowMeasure::Slider {
                    id: "note-measures.grid-spacing".into(),
                    label: Some("Spacing".into()),
                    value: document.grid_spacing.unwrap_or(32.0),
                    min: 8.0,
                    max: 256.0,
                    step: Some(4.0),
                    on_change: note_action("setGridSpacing", None),
                },
                WindowMeasure::Slider {
                    id: "note-measures.grid-subdivisions".into(),
                    label: Some("Subdivisions".into()),
                    value: document.grid_subdivisions.unwrap_or(4.0),
                    min: 1.0,
                    max: 16.0,
                    step: Some(1.0),
                    on_change: note_action("setGridSubdivisions", None),
                },
                WindowMeasure::Slider {
                    id: "note-measures.grid-opacity".into(),
                    label: Some("Opacity".into()),
                    value: document.grid_opacity.unwrap_or(0.35),
                    min: 0.05,
                    max: 1.0,
                    step: Some(0.05),
                    on_change: note_action("setGridOpacity", None),
                },
            ],
        },
        WindowMeasure::Group {
            id: "note-measures.snap".into(),
            label: "Snap".into(),
            default_open: Some(false),
            active_utility_id: None,
            children: vec![
                WindowMeasure::Toggle {
                    id: "note-measures.snap-enabled".into(),
                    icon_id: "magnet".into(),
                    label: Some("Snap to grid".into()),
                    pressed: document.snap_enabled.unwrap_or(false),
                    text: None,
                    on_change: note_action("setSnapEnabled", None),
                },
                WindowMeasure::Slider {
                    id: "note-measures.snap-spacing".into(),
                    label: Some("Snap spacing".into()),
                    value: document.snap_grid_spacing.unwrap_or(8.0),
                    min: 1.0,
                    max: 128.0,
                    step: Some(1.0),
                    on_change: note_action("setSnapGridSpacing", None),
                },
            ],
        },
        WindowMeasure::Group {
            id: "note-measures.drawing".into(),
            label: "Drawing".into(),
            default_open: Some(false),
            active_utility_id: None,
            children: vec![
                WindowMeasure::Slider {
                    id: "note-measures.pencil-width".into(),
                    label: Some("Pencil width".into()),
                    value: document.pencil_width.unwrap_or(3.0),
                    min: 1.0,
                    max: 24.0,
                    step: Some(1.0),
                    on_change: note_action("setPencilWidth", None),
                },
                WindowMeasure::Slider {
                    id: "note-measures.eraser-radius".into(),
                    label: Some("Eraser radius".into()),
                    value: document.eraser_radius.unwrap_or(12.0),
                    min: 4.0,
                    max: 48.0,
                    step: Some(1.0),
                    on_change: note_action("setEraserRadius", None),
                },
            ],
        },
    ]
}

fn note_navigator_measures(document: &NoteDocument) -> Vec<WindowMeasure> {
    vec![
        WindowMeasure::Slider {
            id: "note-navigator-measures.zoom".into(),
            label: Some("Zoom".into()),
            value: document.camera.zoom,
            min: 0.05,
            max: 2.0,
            step: Some(0.05),
            on_change: note_action("setCameraZoom", None),
        },
        WindowMeasure::Toggle {
            id: "note-navigator-measures.grid-visible".into(),
            icon_id: "layout-grid".into(),
            label: Some("Show grid".into()),
            pressed: document.grid_visible.unwrap_or(true),
            text: None,
            on_change: note_action("setGridVisible", None),
        },
    ]
}

fn note_canvas_engagement(document: &NoteDocument, selected_ids: &[String], engagement_input: &str) -> WindowEngagement {
    let block_count = flatten_blocks(&document.blocks).len();
    let selected_count = selected_ids.len();
    let zoom = document.camera.zoom;
    let snap_status = if document.snap_enabled.unwrap_or(false) {
        format!("snap {}px", document.snap_grid_spacing.unwrap_or(8.0))
    } else {
        "snap off".into()
    };
    let grid_status = if document.grid_visible.unwrap_or(true) {
        format!("grid {}px", document.grid_spacing.unwrap_or(32.0))
    } else {
        "grid off".into()
    };
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("note-engagement".into()),
            value: Some(engagement_input.to_string()),
            placeholder: Some("Block name".into()),
            disabled: Some(selected_ids.len() != 1),
            on_change: Some(note_action("engagementInput", None)),
            on_submit: Some(note_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![
            WindowEngagementStatus { id: "note-status.counts".into(), text: format!("{block_count} blocks · {selected_count} selected · zoom {zoom:.2}") },
            WindowEngagementStatus { id: "note-status.grid".into(), text: format!("{grid_status} · {snap_status}") },
        ]),
        possible_engagements: None,
    }
}

fn note_navigator_engagement(active_utility: &str) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("note-navigator-engagement".into()),
            value: None,
            placeholder: Some("Select all".into()),
            disabled: None,
            on_change: None,
            on_submit: Some(note_action("selectAll", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "note-navigator-status.utility".into(), text: format!("utility: {active_utility}") }]),
        possible_engagements: None,
    }
}

/// 🧰 One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()`/toolbar).
fn note_utility(id: &str, label: &str, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/keybound vocabulary
/// dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn note_internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, kind) }
}
//#endregion 🔖Shell

//#region 🔖NoteApp
/// 🎛️ Ephemeral view state living on the app struct (never in the document): the current multi-selection,
/// the hovered block, and the pending engagement-rename input. Content lives in the store's `NoteDocument`
/// projection; every content mutation returns a typed {@link NoteOp} so the store records a true inverse.
#[derive(Default)]
struct NoteApp {
    selected_ids: Vec<String>,
    hovered_id: Option<String>,
    engagement_input: String,
}

impl NoteApp {
    /// ✂️ Nudge step magnitudes: `1px` fine, `10px` fast.
    const NUDGE_STEP: f64 = 1.0;
    const NUDGE_STEP_FAST: f64 = 10.0;
}

impl DocumentApp for NoteApp {
    type Projection = NoteDocument;
    type Op = NoteOp;

    fn app_id(&self) -> &str {
        NOTE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        NOTE_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> NoteDocument {
        empty_note_document()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, NoteDocument>,
        _view_state: &ViewState,
    ) -> ActionEmit<NoteOp> {
        // "undo"/"redo" never reach here — `VcsDocumentApp` intercepts them into store commands.
        let document = doc.projection;
        match action {
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value::<NoteCamera>(camera.clone()) {
                        return ActionEmit::ops(vec![NoteOp::SetCamera { camera: parsed }]);
                    }
                }
                let zoom = args
                    .and_then(|value| value.get("zoom"))
                    .or_else(|| args.and_then(|value| value.get("value")))
                    .and_then(|value| value.as_f64());
                if let Some(zoom) = zoom {
                    let mut camera = document.camera.clone();
                    camera.zoom = zoom;
                    return ActionEmit::ops(vec![NoteOp::SetCamera { camera }]);
                }
                ActionEmit::default()
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰 Host-owned utility switch: the active utility lives in `view_state.active_utility_id`, never
                // the document. Note keeps no in-progress gesture scratch on the app struct (ink drags
                // coalesce store-side), so there is nothing to clear and no op to emit.
                ActionEmit::default()
            }
            "setGridVisible" | "toggleGrid" => {
                let visible = args
                    .and_then(|value| value.get("visible"))
                    .and_then(|value| value.as_bool())
                    .or_else(|| args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()))
                    .unwrap_or(!document.grid_visible.unwrap_or(true));
                ActionEmit::ops(vec![NoteOp::SetGridVisible { visible: Some(visible) }])
            }
            "setGridSpacing" => match scalar_arg(args, "spacing") {
                Some(spacing) => ActionEmit::ops(vec![NoteOp::SetGridSpacing { spacing: Some(spacing.max(4.0)) }]),
                None => ActionEmit::default(),
            },
            "setGridSubdivisions" => match scalar_arg(args, "subdivisions") {
                Some(subdivisions) => {
                    ActionEmit::ops(vec![NoteOp::SetGridSubdivisions { value: Some(subdivisions.round().clamp(1.0, 16.0)) }])
                }
                None => ActionEmit::default(),
            },
            "setGridOpacity" => match scalar_arg(args, "opacity") {
                Some(opacity) => ActionEmit::ops(vec![NoteOp::SetGridOpacity { opacity: Some(opacity.clamp(0.05, 1.0)) }]),
                None => ActionEmit::default(),
            },
            "setSnapEnabled" | "toggleSnap" => {
                let enabled = args
                    .and_then(|value| value.get("enabled"))
                    .and_then(|value| value.as_bool())
                    .or_else(|| args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()))
                    .unwrap_or(!document.snap_enabled.unwrap_or(false));
                ActionEmit::ops(vec![NoteOp::SetSnapEnabled { enabled: Some(enabled) }])
            }
            "setSnapGridSpacing" => match scalar_arg(args, "spacing") {
                Some(spacing) => ActionEmit::ops(vec![NoteOp::SetSnapGridSpacing { spacing: Some(spacing.max(1.0)) }]),
                None => ActionEmit::default(),
            },
            "setPencilWidth" => match scalar_arg(args, "width") {
                Some(width) => ActionEmit::ops(vec![NoteOp::SetPencilWidth { width: Some(width.clamp(1.0, 24.0)) }]),
                None => ActionEmit::default(),
            },
            "setEraserRadius" => match scalar_arg(args, "radius") {
                Some(radius) => ActionEmit::ops(vec![NoteOp::SetEraserRadius { radius: Some(radius.clamp(4.0, 48.0)) }]),
                None => ActionEmit::default(),
            },
            "addBlock" | "dropBlockKind" => {
                let kind = args
                    .and_then(|value| value.get("kind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("text");
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(80.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(80.0);
                let block = create_block_by_kind(kind, x, y);
                self.selected_ids = vec![block_id(&block).into()];
                let mut blocks = document.blocks.clone();
                blocks.push(block);
                ActionEmit::ops(vec![NoteOp::SetBlocks { blocks }])
            }
            "moveBlock" => {
                let Some(block_id_arg) = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                let Some(block) = find_block(&document.blocks, block_id_arg).cloned() else {
                    return ActionEmit::default();
                };
                let target_row_id = args
                    .and_then(|value| value.get("targetRowId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("note-play-blocks");
                let drop_position = args
                    .and_then(|value| value.get("dropPosition"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("after");
                let target_id = block_id_from_tree_row_id(target_row_id);
                let parent_id = target_id.as_ref().and_then(|id| {
                    find_block(&document.blocks, id).and_then(|entry| {
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
                    find_block(&document.blocks, parent)
                        .and_then(|entry| match entry {
                            NoteBlockNode::Group { children, .. } => Some(children.len()),
                            _ => None,
                        })
                        .unwrap_or(0)
                } else {
                    document.blocks.len()
                };
                let mut blocks = document.blocks.clone();
                remove_block_from_tree(&mut blocks, block_id_arg);
                insert_block(&mut blocks, parent_id.as_deref(), index, block);
                ActionEmit::ops(vec![NoteOp::SetBlocks { blocks }])
            }
            "deleteBlock" | "deleteSelection" => {
                if let Some(block_id) = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()) {
                    let mut blocks = document.blocks.clone();
                    remove_block_from_tree(&mut blocks, block_id);
                    self.selected_ids.retain(|id| id != block_id);
                    return ActionEmit::ops(vec![NoteOp::SetBlocks { blocks }]);
                }
                if !self.selected_ids.is_empty() {
                    let mut blocks = document.blocks.clone();
                    for block_id in self.selected_ids.clone() {
                        remove_block_from_tree(&mut blocks, &block_id);
                    }
                    self.selected_ids.clear();
                    return ActionEmit::ops(vec![NoteOp::SetBlocks { blocks }]);
                }
                ActionEmit::default()
            }
            "duplicateBlock" | "duplicateSelection" => {
                let mut ids: Vec<String> = args
                    .and_then(|value| value.get("blockId"))
                    .and_then(|value| value.as_str())
                    .map(|id| vec![id.to_string()])
                    .unwrap_or_default();
                if ids.is_empty() {
                    ids = self.selected_ids.clone();
                }
                if ids.is_empty() {
                    return ActionEmit::default();
                }
                let mut blocks = document.blocks.clone();
                let mut new_ids = Vec::new();
                for source_id in ids {
                    if let Some(block) = find_block(&blocks, &source_id).cloned() {
                        let mut cloned = clone_block(&block);
                        offset_block_tree(&mut cloned, 24.0, 24.0);
                        new_ids.push(block_id(&cloned).to_string());
                        if !insert_after(&mut blocks, &source_id, cloned.clone()) {
                            blocks.push(cloned);
                        }
                    }
                }
                if new_ids.is_empty() {
                    return ActionEmit::default();
                }
                self.selected_ids = new_ids;
                ActionEmit::ops(vec![NoteOp::SetBlocks { blocks }])
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
                    return ActionEmit::default();
                }
                let mut next = document.clone();
                for block_id in block_ids {
                    next = patch_block_field(&next, &block_id, field, &value);
                }
                ActionEmit::ops(vec![NoteOp::SetBlocks { blocks: next.blocks }])
            }
            "selectAll" => {
                self.selected_ids = flatten_blocks(&document.blocks)
                    .into_iter()
                    .map(|block| block_id(block).into())
                    .collect();
                ActionEmit::default()
            }
            "clearSelection" => {
                self.selected_ids.clear();
                ActionEmit::default()
            }
            "setSelection" => {
                self.selected_ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                ActionEmit::default()
            }
            "setHover" => {
                self.hovered_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                ActionEmit::default()
            }
            "nudgeSelection" | "nudgeSelectionUp" | "nudgeSelectionDown" | "nudgeSelectionLeft" | "nudgeSelectionRight"
            | "nudgeSelectionUpFast" | "nudgeSelectionDownFast" | "nudgeSelectionLeftFast" | "nudgeSelectionRightFast" => {
                let (default_dx, default_dy) = match action {
                    "nudgeSelectionUp" => (0.0, -Self::NUDGE_STEP),
                    "nudgeSelectionDown" => (0.0, Self::NUDGE_STEP),
                    "nudgeSelectionLeft" => (-Self::NUDGE_STEP, 0.0),
                    "nudgeSelectionRight" => (Self::NUDGE_STEP, 0.0),
                    "nudgeSelectionUpFast" => (0.0, -Self::NUDGE_STEP_FAST),
                    "nudgeSelectionDownFast" => (0.0, Self::NUDGE_STEP_FAST),
                    "nudgeSelectionLeftFast" => (-Self::NUDGE_STEP_FAST, 0.0),
                    "nudgeSelectionRightFast" => (Self::NUDGE_STEP_FAST, 0.0),
                    _ => (0.0, 0.0),
                };
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(default_dx);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(default_dy);
                if self.selected_ids.is_empty() {
                    return ActionEmit::default();
                }
                let selected: std::collections::HashSet<String> = self.selected_ids.iter().cloned().collect();
                let nudges: Vec<(String, NoteBlockNode)> = flatten_blocks(&document.blocks)
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
                if nudges.is_empty() {
                    return ActionEmit::default();
                }
                let mut blocks = document.blocks.clone();
                for (id, updated) in nudges {
                    update_block_in_tree(&mut blocks, &id, updated);
                }
                ActionEmit::ops(vec![NoteOp::SetBlocks { blocks }])
            }
            "engagementInput" => {
                self.engagement_input = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_string();
                ActionEmit::default()
            }
            "engagementSubmit" => {
                let emit = if self.selected_ids.len() == 1 {
                    let name = args
                        .and_then(|value| value.get("value"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string)
                        .unwrap_or_else(|| self.engagement_input.clone());
                    let target_id = self.selected_ids[0].clone();
                    let next = patch_block_field(document, &target_id, "name", &Value::String(name));
                    ActionEmit::ops(vec![NoteOp::SetBlocks { blocks: next.blocks }])
                } else {
                    ActionEmit::default()
                };
                self.engagement_input.clear();
                emit
            }
            "navigatorEngagementInput" => ActionEmit::default(),
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("empty");
                let document = if example_id == "semio" {
                    serde_json::from_str::<NoteDocument>(SEMIO_EXAMPLE_JSON).unwrap_or_else(|_| empty_note_document())
                } else {
                    empty_note_document()
                };
                self.selected_ids.clear();
                ActionEmit::ops(vec![NoteOp::SetDocument { document }])
            }
            "setFixtureJson" => {
                let raw = args
                    .and_then(|value| value.get("json"))
                    .or_else(|| args.and_then(|value| value.get("payload")))
                    .cloned();
                let Some(raw) = raw else {
                    return ActionEmit::default();
                };
                let text = raw.as_str().map(str::to_string).unwrap_or_else(|| raw.to_string());
                let Ok(parsed) = serde_json::from_str::<Value>(&text) else {
                    return ActionEmit::default();
                };
                if parsed.get("schema").and_then(|value| value.as_str()) != Some(NOTE_DOCUMENT_SCHEMA) {
                    return ActionEmit::default();
                }
                let Ok(document) = serde_json::from_value::<NoteDocument>(parsed) else {
                    return ActionEmit::default();
                };
                self.selected_ids.clear();
                ActionEmit::ops(vec![NoteOp::SetDocument { document }])
            }
            "saveDownload" => {
                let data = serde_json::to_string_pretty(document).unwrap_or_else(|_| "{}".into());
                ActionEmit::effect(HostEffect::DownloadMediaExport {
                    filename: "semio.note.json".into(),
                    mime_type: "application/json".into(),
                    data,
                    encoding: None,
                })
            }
            "loadRequest" => ActionEmit::effect(HostEffect::RequestFileOpen {
                accept: ".json,.note.json,application/json".into(),
                read_as: None,
                import_action: "setFixtureJson".into(),
            }),
            "applyNoteEvents" => {
                let events: Vec<NoteCanvasEvent> = args
                    .and_then(|value| value.get("eventsJson"))
                    .and_then(|value| value.as_str())
                    .and_then(|json_text| serde_json::from_str(json_text).ok())
                    .unwrap_or_default();
                let phase = args
                    .and_then(|value| value.get("phase"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("atomic");
                let select_ids: Option<Vec<String>> = args
                    .and_then(|value| value.get("selectIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok());
                if let Some(ids) = select_ids {
                    self.selected_ids = ids;
                }
                let ops = note_ops_from_canvas_events(document, &events);
                if ops.is_empty() {
                    return ActionEmit::default();
                }
                // The whole drag (begin → live* → commit) coalesces into ONE undoable edit; a lone
                // `atomic` event batch is its own edit.
                let coalesce_key = match phase {
                    "begin" | "live" | "commit" => Some("note-gesture".into()),
                    _ => None,
                };
                ActionEmit { ops, coalesce_key, ..Default::default() }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, NoteDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = note_labels(view_state);
        let active_utility = view_state.active_utility_id.clone().unwrap_or_else(|| "selectDirect".into());
        match body_key {
            NOTE_PLAY_BODY_COMPOSITE => render_canvas_scene(
                document,
                &self.selected_ids,
                self.hovered_id.as_deref(),
                &active_utility,
                NOTE_PLAY_SURFACE_COMPOSITE,
                "composite",
            ),
            NOTE_PLAY_BODY_NAVIGATOR => render_canvas_scene(
                document,
                &self.selected_ids,
                self.hovered_id.as_deref(),
                &active_utility,
                NOTE_PLAY_SURFACE_NAVIGATOR,
                "navigator",
            ),
            NOTE_PLAY_BODY_DOCUMENT => render_document_panel(document, &self.selected_ids, view_state, labels),
            NOTE_PLAY_BODY_CATALOGUE => render_catalogue_panel(labels),
            NOTE_PLAY_BODY_PROPERTIES => render_properties_panel(document, &self.selected_ids, view_state, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, NoteDocument>, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let active_utility = view_state.active_utility_id.clone().unwrap_or_else(|| "selectDirect".into());
        HashMap::from([
            (NOTE_PLAY_WINDOW_COMPOSITE.to_string(), note_canvas_engagement(doc.projection, &self.selected_ids, &self.engagement_input)),
            (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), note_navigator_engagement(&active_utility)),
        ])
    }

    fn window_measures(&self, doc: &DocumentView<'_, NoteDocument>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([
            (NOTE_PLAY_WINDOW_COMPOSITE.to_string(), note_canvas_measures(doc.projection)),
            (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), note_navigator_measures(doc.projection)),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = note_labels(view_state);
        AppLabelsOverlay {
            app_label: None,
            window_kind_labels: HashMap::from([
                (NOTE_PLAY_WINDOW_COMPOSITE.to_string(), labels.window_composite.to_string()),
                (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), labels.window_navigator.to_string()),
            ]),
            panel_tab_labels: HashMap::new(),
            mode_labels: HashMap::new(),
            action_labels: HashMap::new(),
            utility_labels: HashMap::new(),
        }
    }
}

/// 🔢 Reads a numeric action arg by its named key, falling back to a generic `value` (slider/measure inputs).
fn scalar_arg(args: Option<&Value>, key: &str) -> Option<f64> {
    args.and_then(|value| value.get(key))
        .or_else(|| args.and_then(|value| value.get("value")))
        .and_then(|value| value.as_f64())
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

fn note_block_to_svg(block: &NoteBlockNode, document: &NoteDocument) -> String {
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
        NoteBlockNode::Image { image_key, .. } => {
            if let Some(asset) = document.assets.get(image_key) {
                format!(
                    r#"<g transform="{transform}"><image href="{}" width="{width}" height="{height}"/></g>"#,
                    asset.data
                )
            } else {
                format!(
                    "<g transform=\"{transform}\"><rect width=\"{width}\" height=\"{height}\" fill=\"#ddd\" stroke=\"#888\"/></g>"
                )
            }
        }
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
        .map(|block| note_block_to_svg(block, document))
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
    semio_framework_os::register_2d_export_handlers("2d.note", "note", note_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.note", note_document_json_from_dwg);
}
//#endregion 🔖MediaExport

//#region 🔖MediaImport
fn ink_block_from_points(points: &[[f64; 2]]) -> NoteBlockNode {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for point in points {
        min_x = min_x.min(point[0]);
        min_y = min_y.min(point[1]);
        max_x = max_x.max(point[0]);
        max_y = max_y.max(point[1]);
    }
    let local_points = points.iter().map(|point| [point[0] - min_x, point[1] - min_y]).collect();
    NoteBlockNode::Ink {
        id: create_note_id("dwg-ink"),
        name: "Imported Stroke".into(),
        x: min_x,
        y: min_y,
        width: (max_x - min_x).max(1.0),
        height: (max_y - min_y).max(1.0),
        rotation: 0.0,
        visible: true,
        locked: false,
        points: local_points,
        stroke_width: 1.0,
        color: [0.0, 0.0, 0.0, 1.0],
    }
}

fn text_block_from_dwg(at: &[f64; 3], height: f64, rotation: f64, content: &str) -> NoteBlockNode {
    let font_size = if height > 0.0 { height } else { 12.0 };
    NoteBlockNode::Text {
        id: create_note_id("dwg-text"),
        name: "Imported Text".into(),
        x: at[0],
        y: at[1],
        width: (content.chars().count() as f64 * font_size * 0.6).max(font_size),
        height: font_size * 1.4,
        rotation,
        visible: true,
        locked: false,
        paragraphs: vec![NoteTextParagraph {
            runs: vec![NoteTextRun { text: content.to_string(), bold: None, italic: None, underline: None, link: None }],
        }],
        font_size,
        font_weight: "normal".into(),
        align: "left".into(),
    }
}

fn note_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let mut document = empty_note_document();
    document.id = create_note_id("dwg-import");
    document.title = Some("Imported Drawing".into());
    document.camera = NoteCamera {
        x: (drawing.extmin[0] + drawing.extmax[0]) / 2.0,
        y: (drawing.extmin[1] + drawing.extmax[1]) / 2.0,
        zoom: 1.0,
    };
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::Line { start, end } => {
                document.blocks.push(ink_block_from_points(&[[start[0], start[1]], [end[0], end[1]]]));
            }
            DwgGeometry::LwPolyline { closed, vertices, .. } => {
                if vertices.len() >= 2 {
                    let mut points = vertices.clone();
                    if *closed {
                        points.push(vertices[0]);
                    }
                    document.blocks.push(ink_block_from_points(&points));
                }
            }
            DwgGeometry::Text { at, height, rotation, content } => {
                document.blocks.push(text_block_from_dwg(at, *height, *rotation, content));
            }
            _ => {}
        }
    }
    serde_json::to_value(&document).map_err(|error| error.to_string())
}
//#endregion 🔖MediaImport

//#region 🔖Manifest
fn create_note_app() -> App {
    let document = empty_note_document();
    let mut app = App::from_builder(
        App::builder(NOTE_PLAY_APP_ID, "Note").document(["semio", "note"])
            .icon_id("note")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind_with_engagement(NOTE_PLAY_WINDOW_COMPOSITE, "Canvas", NOTE_PLAY_BODY_COMPOSITE, SurfaceKind::NoteCanvas, note_canvas_engagement(&document, &[], ""))
            .window_kind_with_engagement(NOTE_PLAY_WINDOW_NAVIGATOR, "Navigator", NOTE_PLAY_BODY_NAVIGATOR, SurfaceKind::NoteCanvas, note_navigator_engagement("selectDirect"))
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
            // 📇 Palette-visible selection commands (P0) — ephemeral selection is View, block edits are Operations.
            .view_action("selectAll", "Select All")
            .view_action("clearSelection", "Clear Selection")
            .operation("deleteSelection", "Delete Selection")
            .operation("duplicateSelection", "Duplicate Selection")
            // ➕ Palette-visible block insertion (P1) with a staged argument form.
            .operation("addBlock", "Add Block")
            .operation("setActiveExample", "Set Active Example")
            // 🐚 Import/export footer actions → panel Shell actions emitting host effects (S).
            .shell_action("loadRequest", "Import")
            .shell_action("saveDownload", "Export")
            // 🔧 Internal content operations — inspector/tree/drag/import-bound, not palette commands.
            .action_with(note_internal_action("setCamera", "Set Camera", ActionKind::Operation))
            .action_with(note_internal_action("setCameraZoom", "Set Camera Zoom", ActionKind::Operation))
            .action_with(note_internal_action("setGridVisible", "Set Grid Visible", ActionKind::Operation))
            .action_with(note_internal_action("toggleGrid", "Toggle Grid", ActionKind::Operation))
            .action_with(note_internal_action("setGridSpacing", "Set Grid Spacing", ActionKind::Operation))
            .action_with(note_internal_action("setGridSubdivisions", "Set Grid Subdivisions", ActionKind::Operation))
            .action_with(note_internal_action("setGridOpacity", "Set Grid Opacity", ActionKind::Operation))
            .action_with(note_internal_action("setSnapEnabled", "Set Snap Enabled", ActionKind::Operation))
            .action_with(note_internal_action("toggleSnap", "Toggle Snap", ActionKind::Operation))
            .action_with(note_internal_action("setSnapGridSpacing", "Set Snap Grid Spacing", ActionKind::Operation))
            .action_with(note_internal_action("setPencilWidth", "Set Pencil Width", ActionKind::Operation))
            .action_with(note_internal_action("setEraserRadius", "Set Eraser Radius", ActionKind::Operation))
            .action_with(note_internal_action("dropBlockKind", "Drop Block Kind", ActionKind::Operation))
            .action_with(note_internal_action("moveBlock", "Move Block", ActionKind::Operation))
            .action_with(note_internal_action("deleteBlock", "Delete Block", ActionKind::Operation))
            .action_with(note_internal_action("duplicateBlock", "Duplicate Block", ActionKind::Operation))
            .action_with(note_internal_action("patchBlocks", "Patch Blocks", ActionKind::Operation))
            .action_with(note_internal_action("engagementSubmit", "Engagement Submit", ActionKind::Operation))
            .action_with(note_internal_action("setFixtureJson", "Set Fixture Json", ActionKind::Operation))
            .action_with(note_internal_action("applyNoteEvents", "Apply Note Events", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelection", "Nudge Selection", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionUp", "Nudge Selection Up", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionDown", "Nudge Selection Down", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionLeft", "Nudge Selection Left", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionRight", "Nudge Selection Right", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionUpFast", "Nudge Selection Up Fast", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionDownFast", "Nudge Selection Down Fast", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionLeftFast", "Nudge Selection Left Fast", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionRightFast", "Nudge Selection Right Fast", ActionKind::Operation))
            // 👁️ Ephemeral view state — selection/hover/engagement scratch, never a document op.
            .action_with(note_internal_action("setSelection", "Set Selection", ActionKind::View))
            .action_with(note_internal_action("setHover", "Set Hover", ActionKind::View))
            .action_with(note_internal_action("engagementInput", "Engagement Input", ActionKind::View))
            .action_with(note_internal_action("navigatorEngagementInput", "Navigator Engagement Input", ActionKind::View))
            // 📝 Staged argument forms for the palette-eligible actions.
            .action_args("addBlock", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("text", "Text"),
                    ActionArgOption::new("image", "Image"),
                    ActionArgOption::new("table", "Table"),
                    ActionArgOption::new("math", "Math"),
                    ActionArgOption::new("ink", "Ink"),
                    ActionArgOption::new("group", "Group"),
                ]).required().default_value("text"),
                ActionArgDef::number("x", "X").default_value(0.0),
                ActionArgDef::number("y", "Y").default_value(0.0),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new("empty", "Empty"),
                    ActionArgOption::new("semio", "Semio"),
                ]).required().default_value("empty"),
            ])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", "Document JSON").required()])
            // 🧰 Canvas utilities — one exclusive set per window, active utility host-owned (never a document op).
            .utility(note_utility("selectDirect", "Direct", "cursor", "Select", UtilityCategory::Selection))
            .utility(note_utility("selectMarquee", "Marquee", "selection", "Select", UtilityCategory::Selection))
            .utility(note_utility("text", "Text", "type", "Block", UtilityCategory::Tools))
            .utility(note_utility("image", "Image", "image", "Block", UtilityCategory::Tools))
            .utility(note_utility("table", "Table", "table", "Block", UtilityCategory::Tools))
            .utility(note_utility("math", "Math", "sigma", "Block", UtilityCategory::Tools))
            .utility(note_utility("pencil", "Pencil", "pencil", "Draw", UtilityCategory::Tools))
            .utility(note_utility("eraserStroke", "Stroke Eraser", "eraser", "Draw", UtilityCategory::Tools))
            .utility(note_utility("eraserPoint", "Point Eraser", "eraser", "Draw", UtilityCategory::Tools))
            .utility(note_utility("pan", "Pan", "hand", "View", UtilityCategory::Tools))
            .window_kind_utilities(NOTE_PLAY_WINDOW_COMPOSITE, vec![
                "selectDirect".into(), "selectMarquee".into(),
                "text".into(), "image".into(), "table".into(), "math".into(),
                "pencil".into(), "eraserStroke".into(), "eraserPoint".into(), "pan".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+y", "redo")
            .keybinding("mod+a", "selectAll")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("escape", "clearSelection")
            .keybinding("up", "nudgeSelectionUp")
            .keybinding("down", "nudgeSelectionDown")
            .keybinding("left", "nudgeSelectionLeft")
            .keybinding("right", "nudgeSelectionRight")
            .keybinding("shift+up", "nudgeSelectionUpFast")
            .keybinding("shift+down", "nudgeSelectionDownFast")
            .keybinding("shift+left", "nudgeSelectionLeftFast")
            .keybinding("shift+right", "nudgeSelectionRightFast"),
    );
    for window in app.definition.window_kinds.iter_mut() {
        if window.id == NOTE_PLAY_WINDOW_COMPOSITE {
            window.options.measures = note_canvas_measures(&document);
        } else if window.id == NOTE_PLAY_WINDOW_NAVIGATOR {
            window.options.measures = note_navigator_measures(&document);
        }
    }
    app.example("empty", "Empty", serde_json::to_string(&empty_note_document()).unwrap())
        .example("semio", "Semio", SEMIO_EXAMPLE_JSON)
        .program("note", "Note", "document")
}

semio_framework_plugin::semio_plugin! {
    id: "note", label: "Note", version: "0.1.0",
    setup: register_note_exports,
    apps: [ create_note_app => NoteApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, DwgColor, DwgEntity, DwgLayer, PluginApp, VcsDocumentApp};
    use semio_framework_plugin::app::AppActionRegistry;

    fn meta() -> ActionMeta {
        ActionMeta { actor: "local".into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<NoteApp> {
        VcsDocumentApp::new(NoteApp::default())
    }

    #[test]
    fn renders_composite_canvas() {
        let mut app = new_app();
        let node = app.render(NOTE_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("note-canvas"));
        assert!(json.contains("documentJson"));
    }

    #[test]
    fn renders_navigator_canvas() {
        let mut app = new_app();
        let node = app.render(NOTE_PLAY_BODY_NAVIGATOR, Some(SEMIO_EXAMPLE_JSON), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("note-canvas"));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[test]
    fn parses_semio_example_document() {
        let document: NoteDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).expect("semio note json");
        assert_eq!(document.blocks.len(), 3);
    }

    #[test]
    fn renders_document_tree() {
        let mut app = new_app();
        let node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(SEMIO_EXAMPLE_JSON), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Welcome"));
    }

    #[test]
    fn note_labels_resolve_native_by_default() {
        let mut app = new_app();
        let view_state = ViewState::default();
        let document_node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(SEMIO_EXAMPLE_JSON), &view_state).expect("render");
        let document_json = serde_json::to_string(&document_node).unwrap();
        assert!(document_json.contains("Add Text"));
        assert!(document_json.contains("Add Table"));
        assert!(document_json.contains("Add Math"));
        assert!(document_json.contains("Add Image"));
        assert!(document_json.contains("Add Group"));

        let catalogue_node = app.render(NOTE_PLAY_BODY_CATALOGUE, Some(SEMIO_EXAMPLE_JSON), &view_state).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("Block kinds"));
        assert!(catalogue_json.contains("text — rich text block"));

        let empty_node = app.render(NOTE_PLAY_BODY_DOCUMENT, None, &view_state).expect("render");
        let empty_json = serde_json::to_string(&empty_node).unwrap();
        assert!(empty_json.contains("Drop blocks here"));
    }

    #[test]
    fn note_labels_resolve_german_locale() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let document_node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(SEMIO_EXAMPLE_JSON), &view_state).expect("render");
        let document_json = serde_json::to_string(&document_node).unwrap();
        assert!(document_json.contains("Text hinzufügen"));
        assert!(document_json.contains("Tabelle hinzufügen"));
        assert!(document_json.contains("Mathe hinzufügen"));
        assert!(document_json.contains("Bild hinzufügen"));
        assert!(document_json.contains("Gruppe hinzufügen"));

        let catalogue_node = app.render(NOTE_PLAY_BODY_CATALOGUE, Some(SEMIO_EXAMPLE_JSON), &view_state).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("Blockarten"));
        assert!(catalogue_json.contains("Text — reicher Textblock"));

        let empty_node = app.render(NOTE_PLAY_BODY_DOCUMENT, None, &view_state).expect("render");
        let empty_json = serde_json::to_string(&empty_node).unwrap();
        assert!(empty_json.contains("Blöcke hier ablegen"));
    }

    #[test]
    fn add_block_action_emits_one_op_and_grows_projection() {
        let mut app = new_app();
        let result = app
            .handle_action("addBlock", Some(&json!({ "kind": "text" })), &ViewState::default(), &meta())
            .expect("addBlock");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.blocks.len(), 1);
        assert_eq!(block_kind(&projection.blocks[0]), "text");
    }

    #[test]
    fn add_block_then_undo_round_trip() {
        let mut app = new_app();
        app.handle_action("addBlock", Some(&json!({ "kind": "text" })), &ViewState::default(), &meta()).expect("add");
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);
        let undo = app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo");
        assert!(undo.operations.is_empty(), "undo never emits KernelOperations");
        assert!(app.projection().expect("projection").blocks.is_empty(), "undo restores the empty document");
    }

    #[test]
    fn properties_panel_reads_app_selection() {
        let mut app = new_app();
        app.handle_action("addBlock", Some(&json!({ "kind": "text" })), &ViewState::default(), &meta()).expect("add");
        let id = block_id(&app.projection().expect("projection").blocks[0]).to_string();
        app.handle_action("setSelection", Some(&json!({ "ids": [id] })), &ViewState::default(), &meta()).expect("select");
        let node = app.render(NOTE_PLAY_BODY_PROPERTIES, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("note-properties.block"), "selected block must render an inspector group: {json}");
    }

    #[test]
    fn nudge_direction_actions_move_selection_without_args() {
        for (action, expected_dx, expected_dy) in [
            ("nudgeSelectionUp", 0.0, -1.0),
            ("nudgeSelectionDown", 0.0, 1.0),
            ("nudgeSelectionLeft", -1.0, 0.0),
            ("nudgeSelectionRight", 1.0, 0.0),
        ] {
            let mut app = new_app();
            app.handle_action("addBlock", Some(&json!({ "kind": "text", "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta())
                .expect("add");
            let ops = app.handle_action(action, None, &ViewState::default(), &meta()).expect(action).operations.len();
            assert_eq!(ops, 1, "{action} should emit one op");
            let projection = app.projection().expect("projection");
            let (x, y, ..) = block_bounds(&projection.blocks[0]);
            assert_eq!((x, y), (expected_dx, expected_dy), "{action} moved block to unexpected position");
        }
    }

    #[test]
    fn nudge_fast_actions_use_ten_pixel_step() {
        let mut app = new_app();
        app.handle_action("addBlock", Some(&json!({ "kind": "text", "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta())
            .expect("add");
        app.handle_action("nudgeSelectionRightFast", None, &ViewState::default(), &meta()).expect("nudge");
        let projection = app.projection().expect("projection");
        let (x, y, ..) = block_bounds(&projection.blocks[0]);
        assert_eq!((x, y), (10.0, 0.0));
    }

    #[test]
    fn gesture_begin_live_commit_produces_single_undo_step() {
        let mut app = new_app();
        let block = create_block_by_kind("text", 10.0, 10.0);
        let new_id = block_id(&block).to_string();

        let begin_events = json!([
            { "op": "addBlock", "block": block.clone(), "parentId": null, "index": null }
        ])
        .to_string();
        app.handle_action(
            "applyNoteEvents",
            Some(&json!({ "eventsJson": begin_events, "phase": "begin", "selectIds": [new_id.clone()] })),
            &ViewState::default(),
            &meta(),
        )
        .expect("begin");
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);

        for x in [20.0, 30.0, 40.0] {
            let mut moved = block.clone();
            if let NoteBlockNode::Text { x: block_x, .. } = &mut moved {
                *block_x = x;
            }
            let live_events = json!([
                { "op": "updateBlock", "blockId": new_id, "block": moved }
            ])
            .to_string();
            app.handle_action(
                "applyNoteEvents",
                Some(&json!({ "eventsJson": live_events, "phase": "live" })),
                &ViewState::default(),
                &meta(),
            )
            .expect("live");
        }
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);

        // Commit with no further change emits no op — the gesture is already recorded.
        let commit = app
            .handle_action(
                "applyNoteEvents",
                Some(&json!({ "eventsJson": "[]", "phase": "commit" })),
                &ViewState::default(),
                &meta(),
            )
            .expect("commit");
        assert!(commit.operations.is_empty(), "a no-op commit must not create an edit");
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);

        // The whole begin+live gesture coalesced into ONE undoable edit.
        app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo");
        assert!(
            app.projection().expect("projection").blocks.is_empty(),
            "a single undo should erase the whole gesture"
        );
    }

    #[test]
    fn gesture_with_no_changes_creates_no_edit() {
        let mut app = new_app();
        app.handle_action(
            "applyNoteEvents",
            Some(&json!({ "eventsJson": "[]", "phase": "begin" })),
            &ViewState::default(),
            &meta(),
        )
        .expect("begin");
        app.handle_action(
            "applyNoteEvents",
            Some(&json!({ "eventsJson": "[]", "phase": "commit" })),
            &ViewState::default(),
            &meta(),
        )
        .expect("commit");
        let undo = app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo");
        assert!(undo.events.is_empty(), "no gesture edit should exist to undo");
    }

    #[test]
    fn camera_action_emits_op() {
        let mut app = new_app();
        let zoom = app
            .handle_action("setCameraZoom", Some(&json!({ "value": 2.0 })), &ViewState::default(), &meta())
            .expect("zoom");
        assert_eq!(zoom.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").camera.zoom, 2.0);
    }

    /// 🧰 Switching utilities is the framework View action: host-owned `active_utility_id`, never a document op —
    /// the retired `NoteOp::SetActiveUtility` no longer pollutes undo history or sync.
    fn new_app_with_registry() -> VcsDocumentApp<NoteApp> {
        let definition = create_note_app().definition;
        VcsDocumentApp::with_registry(NoteApp::default(), AppActionRegistry::from_definition(&definition))
    }

    #[test]
    fn set_active_utility_emits_no_ops_and_no_history_entry() {
        let mut app = new_app_with_registry();
        let before = app.projection().expect("projection");
        let view = ViewState { active_utility_id: Some("pencil".into()), ..ViewState::default() };
        let result = app
            .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "pencil" })), &view, &meta())
            .expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document ops");
        assert_eq!(app.projection().expect("projection"), before, "utility switching does not mutate the document");
    }

    #[test]
    fn utility_registry_declares_canvas_utilities_scoped_to_composite_window() {
        let definition = create_note_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(
            utility_ids,
            ["selectDirect", "selectMarquee", "text", "image", "table", "math", "pencil", "eraserStroke", "eraserPoint", "pan"],
        );
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectDirect", "selectMarquee"]);
        let canvas = definition.window_kinds.iter().find(|window| window.id == NOTE_PLAY_WINDOW_COMPOSITE).expect("canvas window");
        assert_eq!(canvas.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite canvas");
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
    }

    #[test]
    fn set_grid_subdivisions_and_opacity_clamp() {
        let mut app = new_app();
        app.handle_action("setGridSubdivisions", Some(&json!({ "value": 40.0 })), &ViewState::default(), &meta())
            .expect("subdivisions");
        assert_eq!(app.projection().expect("projection").grid_subdivisions, Some(16.0));

        app.handle_action("setGridOpacity", Some(&json!({ "value": 5.0 })), &ViewState::default(), &meta())
            .expect("opacity");
        assert_eq!(app.projection().expect("projection").grid_opacity, Some(1.0));
    }

    #[test]
    fn patch_blocks_table_row_and_column_ops_clamp_at_one() {
        let mut app = new_app();
        app.handle_action("addBlock", Some(&json!({ "kind": "table" })), &ViewState::default(), &meta()).expect("add");
        let table_id = block_id(&app.projection().expect("projection").blocks[0]).to_string();

        for (field, expected_rows, expected_columns) in [
            ("tableAddRow", 3, 3),
            ("tableAddColumn", 3, 4),
            ("tableRemoveRow", 2, 4),
            ("tableRemoveRow", 1, 4),
            ("tableRemoveRow", 1, 4),
            ("tableRemoveColumn", 1, 3),
        ] {
            app.handle_action(
                "patchBlocks",
                Some(&json!({ "blockIds": [table_id], "field": field })),
                &ViewState::default(),
                &meta(),
            )
            .expect("patch");
            let projection = app.projection().expect("projection");
            let block = find_block(&projection.blocks, &table_id).unwrap();
            if let NoteBlockNode::Table { rows, columns, .. } = block {
                assert_eq!(rows.len(), expected_rows, "field {field}");
                assert_eq!(columns.len(), expected_columns, "field {field}");
            } else {
                panic!("expected table block");
            }
        }
    }

    #[test]
    fn duplicate_selection_clones_with_offset_and_selects_clones() {
        let mut app = new_app();
        app.handle_action("addBlock", Some(&json!({ "kind": "text", "x": 10.0, "y": 10.0 })), &ViewState::default(), &meta())
            .expect("add");
        let source_id = block_id(&app.projection().expect("projection").blocks[0]).to_string();

        let result = app.handle_action("duplicateSelection", None, &ViewState::default(), &meta()).expect("duplicate");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.blocks.len(), 2);
        let clone = projection.blocks.iter().find(|block| block_id(block) != source_id).expect("clone block");
        let (x, y, ..) = block_bounds(clone);
        assert_eq!((x, y), (34.0, 34.0));
    }

    #[test]
    fn save_download_and_load_request_effects() {
        let mut app = new_app();
        let save = app.handle_action("saveDownload", None, &ViewState::default(), &meta()).expect("save");
        assert!(save.operations.is_empty());
        assert!(
            matches!(save.requested_effects.first(), Some(HostEffect::DownloadMediaExport { filename, .. }) if filename == "semio.note.json"),
            "saveDownload must request a media export: {:?}",
            save.requested_effects
        );

        let load = app.handle_action("loadRequest", None, &ViewState::default(), &meta()).expect("load");
        assert!(
            matches!(load.requested_effects.first(), Some(HostEffect::RequestFileOpen { import_action, .. }) if import_action == "setFixtureJson"),
            "loadRequest must request a file open: {:?}",
            load.requested_effects
        );
    }

    #[test]
    fn set_fixture_json_replaces_document() {
        let mut app = new_app();
        let result = app
            .handle_action("setFixtureJson", Some(&json!({ "payload": SEMIO_EXAMPLE_JSON })), &ViewState::default(), &meta())
            .expect("fixture");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").blocks.len(), 3);
    }

    #[test]
    fn set_active_example_loads_semio_blocks() {
        let mut app = new_app();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "semio" })), &ViewState::default(), &meta())
            .expect("semio");
        assert_eq!(app.projection().expect("projection").blocks.len(), 3);

        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "empty" })), &ViewState::default(), &meta())
            .expect("empty");
        assert!(app.projection().expect("projection").blocks.is_empty());
    }

    #[test]
    fn note_document_round_trips_assets_and_grid_settings() {
        let mut document = empty_note_document();
        document.assets.insert(
            "asset-1".into(),
            NoteImageAsset {
                mime: "image/png".into(),
                data: "data:image/png;base64,abc".into(),
                width: Some(10.0),
                height: Some(20.0),
            },
        );
        document.grid_subdivisions = Some(6.0);
        document.grid_opacity = Some(0.5);
        let json_text = serde_json::to_string(&document).unwrap();
        let parsed: NoteDocument = serde_json::from_str(&json_text).unwrap();
        assert_eq!(parsed.assets.get("asset-1").unwrap().mime, "image/png");
        assert_eq!(parsed.grid_subdivisions, Some(6.0));
        assert_eq!(parsed.grid_opacity, Some(0.5));
    }

    #[test]
    fn clone_block_reids_group_children() {
        let child = create_block_by_kind("text", 0.0, 0.0);
        let child_id = block_id(&child).to_string();
        let group = NoteBlockNode::Group {
            id: "group-1".into(),
            name: "Group".into(),
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            children: vec![child],
        };
        let cloned = clone_block(&group);
        if let NoteBlockNode::Group { children, .. } = &cloned {
            assert_ne!(block_id(&children[0]), child_id);
        } else {
            panic!("expected group block");
        }
    }

    #[test]
    fn imports_dwg_polyline_and_text_into_note_blocks() {
        let drawing = DwgDrawing {
            layers: vec![DwgLayer::default()],
            entities: vec![
                DwgEntity {
                    layer: 0,
                    color: DwgColor::ByLayer,
                    geometry: DwgGeometry::LwPolyline {
                        closed: true,
                        elevation: 0.0,
                        vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]],
                        bulges: vec![0.0, 0.5, 0.0],
                    },
                },
                DwgEntity {
                    layer: 0,
                    color: DwgColor::ByLayer,
                    geometry: DwgGeometry::Text { at: [1.0, 2.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".into() },
                },
            ],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let value = note_document_json_from_dwg(&drawing).unwrap();
        let document: NoteDocument = serde_json::from_value(value).unwrap();
        assert_eq!(document.schema, NOTE_DOCUMENT_SCHEMA);
        assert_eq!(document.blocks.len(), 2);
        assert_eq!(document.camera.x, 5.0);
        assert_eq!(document.camera.y, 5.0);
        let ink_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Ink { .. })).count();
        let text_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Text { .. })).count();
        assert_eq!(ink_count, 1);
        assert_eq!(text_count, 1);
        if let Some(NoteBlockNode::Ink { points, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Ink { .. })) {
            assert_eq!(points.len(), 4);
        } else {
            panic!("expected ink block");
        }
        if let Some(NoteBlockNode::Text { paragraphs, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Text { .. })) {
            assert_eq!(paragraphs[0].runs[0].text, "semio");
        } else {
            panic!("expected text block");
        }
    }

    #[test]
    fn imports_empty_dwg_drawing_as_valid_empty_note_document() {
        let drawing = DwgDrawing::default();
        let value = note_document_json_from_dwg(&drawing).unwrap();
        let document: NoteDocument = serde_json::from_value(value).unwrap();
        assert_eq!(document.schema, NOTE_DOCUMENT_SCHEMA);
        assert!(document.blocks.is_empty());
    }
}
//#endregion 🧪Tests
