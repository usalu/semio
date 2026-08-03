//! 🖥️ Note app — DocumentApp impl, render, manifest (constitutional: ui).

use note::{NoteBlockNode, NoteCamera, NoteDocument, NoteImageAsset, NOTE_DOCUMENT_SCHEMA};
use note_engine::{
    block_bounds, block_icon, block_id, block_id_from_tree_row_id, block_kind, block_name, block_visible, clone_block, create_block_by_kind,
    empty_note_document, find_block, flatten_blocks, insert_after, insert_block, offset_block_tree, patch_block_field, remove_block_from_tree,
    semio_example_document, semio_example_json, update_block_in_tree, NoteConfig,
};
use note_op::{NoteConfigOperation, NoteOperation};
use note_protocol::NoteCommand;
use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_ink_canvas_scene,
    tree_item, tree_item_with_action,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_stack_vertical, ui_text, App, MediaClass, MediaForm, MediaType, OsMediaCapability, ArtifactKindSpec,
    InkCanvasScene, ActionDescriptor, AppLabels, DocumentApp, DocumentView, ConfigView, Emit, Label, Locale, LocalizedLabel, Terminology,
    HostEffect, PanelTreeBuilder, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiToggleNode, UiTreeItemNode,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER, create_default_layout,
    ActionDefinition, ActionKind, ActionArgDef, ActionArgOption, UtilityDefinition, UtilityCategory, SET_ACTIVE_UTILITY_ACTION_ID,
    WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowMeasure,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

//#region 🔖️Constants
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
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`'s identical helpers.
/// `NoteConfig` carries no terminology axis, so this app is always `Terminology::Native`.
fn is_de_locale(cfg: &NoteConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn note_locale(cfg: &NoteConfig) -> Locale {
    if is_de_locale(cfg) { Locale::De } else { Locale::En }
}

fn resolve_labels<L: AppLabels>(cfg: &NoteConfig) -> &'static L {
    L::labels(note_locale(cfg), Terminology::Native)
}
//#endregion 🔖️Locale

//#region 🔖️CanvasEvents
/// 🖱️ Batched canvas-event wire shape the `ink-canvas-host` surface emits (`addBlock`/`updateBlock`/
/// `removeBlock`/`putAsset`/`setCamera`); content events diff into `NoteOperation`s via
/// `note_ops_from_canvas_events`, `setCamera` diffs into a `NoteConfigOperation::SetCamera` instead
/// (session-only view state, never a document field).
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "operation")]
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
        // 📷️ Camera never touches the document — `inkApplyEvents` pulls it into runtime state before
        // this function ever sees the batch (see the `NoteCanvasEvent::SetCamera` filter there).
        NoteCanvasEvent::SetCamera { .. } => {}
    }
}

/// 🔀️ Applies a batch of canvas events to a cloned document and returns the minimal `NoteOperation`s
/// describing what changed (block-tree snapshot and per-asset puts) — the empty vec means no content
/// changed (e.g. a gesture that ended where it began).
fn note_ops_from_canvas_events(document: &NoteDocument, events: &[NoteCanvasEvent]) -> Vec<NoteOperation> {
    let mut next = document.clone();
    for event in events {
        apply_note_canvas_event(&mut next, event);
    }
    let mut operations = Vec::new();
    if next.blocks != document.blocks {
        operations.push(NoteOperation::SetBlocks { blocks: next.blocks.clone() });
    }
    for (key, asset) in &next.assets {
        if document.assets.get(key) != Some(asset) {
            operations.push(NoteOperation::PutAsset { key: key.clone(), asset: asset.clone() });
        }
    }
    operations
}
//#endregion 🔖️CanvasEvents

//#region 🔖️ActionHelpers
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
    }
}

/// 🩹️ `NoteCommand::PatchBlocks`'s typed field/value pair, reconstructed into the `serde_json::Value`
/// shape `note_engine::patch_block_field` expects — mirrors `shooting_ui`'s `shot_patch_for_field`/
/// `asset_patch_for_field` string-value convention, extended with the numeric/bool fields note's
/// inspector patches that shooting's string-only fields never needed.
fn note_patch_json_value(field: &str, value: &str) -> Value {
    match field {
        "visible" | "locked" => Value::Bool(value.parse::<bool>().unwrap_or(false)),
        "x" | "y" | "width" | "height" | "textSize" | "inkWidth" => value
            .parse::<f64>()
            .ok()
            .and_then(serde_json::Number::from_f64)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        _ => Value::String(value.to_string()),
    }
}
//#endregion 🔖️ActionHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the note app; one field per label makes every locale combination compile-checked.
    struct NotePlayLabels {
        document: native_en "Document", native_de "Dokument", reuse_en "Document", reuse_de "Dokument";
        catalogue_title: native_en "Block kinds", native_de "Blockarten", reuse_en "Block kinds", reuse_de "Blockarten";
        catalogue_text: native_en "text — rich text block", native_de "Text — reicher Textblock", reuse_en "text — rich text block", reuse_de "Text — reicher Textblock";
        catalogue_image: native_en "image — embedded image", native_de "Bild — eingebettetes Bild", reuse_en "image — embedded image", reuse_de "Bild — eingebettetes Bild";
        catalogue_table: native_en "table — grid block", native_de "Tabelle — Rasterblock", reuse_en "table — grid block", reuse_de "Tabelle — Rasterblock";
        catalogue_math: native_en "math — TeX equation", native_de "Mathe — TeX-Formel", reuse_en "math — TeX equation", reuse_de "Mathe — TeX-Formel";
        catalogue_ink: native_en "ink — pencil strokes", native_de "Tinte — Stiftstriche", reuse_en "ink — pencil strokes", reuse_de "Tinte — Stiftstriche";
        catalogue_group: native_en "group — nested blocks", native_de "Gruppe — verschachtelte Blöcke", reuse_en "group — nested blocks", reuse_de "Gruppe — verschachtelte Blöcke";
        inspector_block: native_en "Block", native_de "Block", reuse_en "Block", reuse_de "Block";
        document_empty: native_en "Drop blocks here", native_de "Blöcke hier ablegen", reuse_en "Drop blocks here", reuse_de "Blöcke hier ablegen";
        add_text: native_en "Add Text", native_de "Text hinzufügen", reuse_en "Add Text", reuse_de "Text hinzufügen";
        add_table: native_en "Add Table", native_de "Tabelle hinzufügen", reuse_en "Add Table", reuse_de "Tabelle hinzufügen";
        add_math: native_en "Add Math", native_de "Mathe hinzufügen", reuse_en "Add Math", reuse_de "Mathe hinzufügen";
        add_image: native_en "Add Image", native_de "Bild hinzufügen", reuse_en "Add Image", reuse_de "Bild hinzufügen";
        add_group: native_en "Add Group", native_de "Gruppe hinzufügen", reuse_en "Add Group", reuse_de "Gruppe hinzufügen";
        field_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        field_x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        field_y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        field_width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        field_height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        field_visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        field_locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        measure_camera: native_en "Camera", native_de "Kamera", reuse_en "Camera", reuse_de "Kamera";
        measure_zoom: native_en "Zoom", native_de "Zoom", reuse_en "Zoom", reuse_de "Zoom";
        measure_grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        measure_show_grid: native_en "Show grid", native_de "Raster anzeigen", reuse_en "Show grid", reuse_de "Raster anzeigen";
        measure_spacing: native_en "Spacing", native_de "Abstand", reuse_en "Spacing", reuse_de "Abstand";
        measure_subdivisions: native_en "Subdivisions", native_de "Unterteilungen", reuse_en "Subdivisions", reuse_de "Unterteilungen";
        measure_opacity: native_en "Opacity", native_de "Deckkraft", reuse_en "Opacity", reuse_de "Deckkraft";
        measure_snap: native_en "Snap", native_de "Fangen", reuse_en "Snap", reuse_de "Fangen";
        measure_snap_to_grid: native_en "Snap to grid", native_de "Am Raster einrasten", reuse_en "Snap to grid", reuse_de "Am Raster einrasten";
        measure_snap_spacing: native_en "Snap spacing", native_de "Rasterabstand", reuse_en "Snap spacing", reuse_de "Rasterabstand";
        measure_drawing: native_en "Drawing", native_de "Zeichnen", reuse_en "Drawing", reuse_de "Zeichnen";
        measure_pencil_width: native_en "Pencil width", native_de "Stiftbreite", reuse_en "Pencil width", reuse_de "Stiftbreite";
        measure_eraser_radius: native_en "Eraser radius", native_de "Radiergummi-Radius", reuse_en "Eraser radius", reuse_de "Radiergummi-Radius";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn block_tree_item(block: &NoteBlockNode) -> UiTreeItemNode {
    let nested = match block {
        NoteBlockNode::Group { children, .. } if !children.is_empty() => {
            Some(children.iter().map(block_tree_item).collect())
        }
        _ => None,
    };
    UiTreeItemNode {
        icon_id: Some(block_icon(block_kind(block)).into()),
        default_open: Some(matches!(block, NoteBlockNode::Group { .. })),
        draggable: Some(true),
        items: nested,
        dimmed: if block_visible(block) { None } else { Some(true) },
        menu: None,
        ..tree_item_with_action(
            block_tree_row_id(block),
            Label::data(block_name(block)),
            Some(block_kind(block).into()),
            play_action(NOTE_PLAY_CONTROLLER_ID, "setSelection", Some(json!({ "ids": [block_id(block)] }))),
        )
    }
}

fn block_tree_row_id(block: &NoteBlockNode) -> String {
    format!("note-play-block:{}", block_id(block))
}

fn render_document_panel(document: &NoteDocument, selected_ids: &[String], labels: &NotePlayLabels) -> UiNode {
    let action_rows: Vec<UiTreeItemNode> = [
        ("text", labels.add_text, "type"),
        ("table", labels.add_table, "table-2"),
        ("math", labels.add_math, "note-math"),
        ("image", labels.add_image, "image"),
        ("group", labels.add_group, "folder-plus"),
    ]
    .into_iter()
    .map(|(kind, label, icon)| UiTreeItemNode {
        icon_id: Some(icon.into()),
        menu: None,
        ..tree_item_with_action(
            format!("note-play-blocks.add.{kind}"),
            label,
            None,
            play_action(NOTE_PLAY_CONTROLLER_ID, "addBlock", Some(json!({ "kind": kind }))),
        )
    })
    .collect();
    let block_items: Vec<UiTreeItemNode> = if document.blocks.is_empty() {
        vec![UiTreeItemNode {
            icon_id: Some("sticky-note".into()),
            ..tree_item("note-play-blocks.empty", labels.document_empty)
        }]
    } else {
        document.blocks.iter().map(block_tree_item).collect()
    };
    let selected_ids: Vec<String> = selected_ids
        .iter()
        .filter_map(|id| find_block(&document.blocks, id).map(block_tree_row_id))
        .collect();
    PanelTreeBuilder::new("note-play-blocks")
        .section("note-play-blocks", Some(labels.document.into()), true, [action_rows, block_items].concat())
        .selected(selected_ids)
        .selection_change(play_action(NOTE_PLAY_CONTROLLER_ID, "setSelection", None))
        .build()
}

fn render_catalogue_panel(labels: &NotePlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "note-catalogue".into(),
        label: Some(labels.catalogue_title.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![
            ui_text(labels.catalogue_text),
            ui_text(labels.catalogue_image),
            ui_text(labels.catalogue_table),
            ui_text(labels.catalogue_math),
            ui_text(labels.catalogue_ink),
            ui_text(labels.catalogue_group),
        ],
        menu: None,
    }])
}

fn inspector_patch(block_ids: &[String], field: &str) -> ActionDescriptor {
    play_action(
        NOTE_PLAY_CONTROLLER_ID,
        "patchBlocks",
        Some(json!({ "blockIds": block_ids, "field": field })),
    )
}

fn inspector_text_field(block_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[String], field: &str) -> UiNode {
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
            placeholder: mixed.placeholder.map(Label::data),
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: inspector_patch(block_ids, field),
            presence: UiPresence::default(),
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn inspector_number_field(block_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[f64], field: &str) -> UiNode {
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
                Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER))
            },
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: inspector_patch(block_ids, field),
            presence: UiPresence::default(),
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn render_properties_panel(document: &NoteDocument, selected_ids: &[String], active_utility_id: &str, labels: &NotePlayLabels) -> UiNode {
    let blocks: Vec<&NoteBlockNode> = selected_ids
        .iter()
        .filter_map(|id| find_block(&document.blocks, id))
        .collect();
    if blocks.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(Label::data(format!("Schema: {}", document.schema))),
            ui_text(Label::data(format!("Blocks: {}", flatten_blocks(&document.blocks).len()))),
            ui_text(Label::data(format!("Utility: {active_utility_id}"))),
            ui_text(Label::data(format!(
                "Snap: {}",
                if document.snap_enabled.unwrap_or(false) {
                    format!("{}px", document.snap_grid_spacing.unwrap_or(8.0))
                } else {
                    "off".into()
                }
            ))),
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
        presence: UiPresence::default(),
        fields: vec![
            inspector_text_field(&block_ids, "note-properties.name", labels.field_name, &names, "name"),
            inspector_number_field(&block_ids, "note-properties.x", labels.field_x, &xs, "x"),
            inspector_number_field(&block_ids, "note-properties.y", labels.field_y, &ys, "y"),
            inspector_number_field(&block_ids, "note-properties.width", labels.field_width, &widths, "width"),
            inspector_number_field(&block_ids, "note-properties.height", labels.field_height, &heights, "height"),
            UiNode::Field(UiFieldNode {
                id: "note-properties.visible".into(),
                label: labels.field_visible.into(),
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "note-properties.visible.toggle".into(),
                    icon_id: "eye".into(),
                    text: None,
                    on_change: inspector_patch(&block_ids, "visible"),
                    presence: UiPresence::selected(visible_mixed.uniform && visible_mixed.pressed),
                    menu: None,
                })),
                presence: UiPresence::default(),
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "note-properties.locked".into(),
                label: labels.field_locked.into(),
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "note-properties.locked.toggle".into(),
                    icon_id: "lock".into(),
                    text: None,
                    on_change: inspector_patch(&block_ids, "locked"),
                    presence: UiPresence::selected(locked_mixed.uniform && locked_mixed.pressed),
                    menu: None,
                })),
                presence: UiPresence::default(),
                menu: None,
            }),
        ],
    }])
}
//#endregion 🔖️Panels

//#region 🔖️Render
//#region 🔖️Scenes
fn render_canvas_scene(
    document: &NoteDocument,
    camera: &NoteCamera,
    selected_ids: &[String],
    hovered_id: Option<&str>,
    active_utility: &str,
    surface_id: &str,
    view_mode: &str,
) -> UiNode {
    // 📷️ Camera is session-only runtime state, never part of `NoteDocument` — merged into the wire
    // payload here so the ink-canvas host still gets a `camera` key to render/pan/zoom against.
    let mut document_value = serde_json::to_value(document).unwrap_or_else(|_| json!({}));
    if let Some(map) = document_value.as_object_mut() {
        map.insert("camera".into(), serde_json::to_value(camera).unwrap_or_else(|_| json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 })));
    }
    let document_json = document_value.to_string();
    let selection_json = serde_json::to_string(selected_ids).unwrap_or_else(|_| "[]".into());
    build_ink_canvas_scene(
        surface_id,
        NOTE_PLAY_CONTROLLER_ID,
        InkCanvasScene {
            document_json,
            selection_json,
            hovered_id: hovered_id.map(str::to_string),
            active_utility: active_utility.into(),
            view_mode: view_mode.into(),
            interactive: view_mode == "composite",
        },
    )
}
//#endregion 🔖️Scenes

//#region 🔖️Shell
fn note_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    play_action(NOTE_PLAY_CONTROLLER_ID, action, args)
}

fn note_canvas_measures(document: &NoteDocument, camera: &NoteCamera, labels: &NotePlayLabels) -> Vec<WindowMeasure> {
    vec![
        WindowMeasure::Group {
            id: "note-measures.camera".into(),
            label: labels.measure_camera.into(),
            default_open: Some(true),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: vec![WindowMeasure::Slider {
                id: "note-measures.zoom".into(),
                label: Some(labels.measure_zoom.into()),
                value: camera.zoom,
                min: 0.1,
                max: 8.0,
                step: Some(0.05),
                ready: None,
                loading: None,
                disabled: None,
                reveal: None,
                on_change: note_action("setCameraZoom", None),

                waiting: None,}],
        },
        WindowMeasure::Group {
            id: "note-measures.grid".into(),
            label: labels.measure_grid.into(),
            default_open: Some(true),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: vec![
                WindowMeasure::Toggle {
                    id: "note-measures.grid-visible".into(),
                    icon_id: "layout-grid".into(),
                    label: Some(labels.measure_show_grid.into()),
                    pressed: document.grid_visible.unwrap_or(true),
                    text: None,
                    on_change: note_action("setGridVisible", None),
                },
                WindowMeasure::Slider {
                    id: "note-measures.grid-spacing".into(),
                    label: Some(labels.measure_spacing.into()),
                    value: document.grid_spacing.unwrap_or(32.0),
                    min: 8.0,
                    max: 256.0,
                    step: Some(4.0),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: note_action("setGridSpacing", None),
                    },
                WindowMeasure::Slider {
                    id: "note-measures.grid-subdivisions".into(),
                    label: Some(labels.measure_subdivisions.into()),
                    value: document.grid_subdivisions.unwrap_or(4.0),
                    min: 1.0,
                    max: 16.0,
                    step: Some(1.0),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: note_action("setGridSubdivisions", None),
                    },
                WindowMeasure::Slider {
                    id: "note-measures.grid-opacity".into(),
                    label: Some(labels.measure_opacity.into()),
                    value: document.grid_opacity.unwrap_or(0.35),
                    min: 0.05,
                    max: 1.0,
                    step: Some(0.05),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: note_action("setGridOpacity", None),
                    },
            ],
        },
        WindowMeasure::Group {
            id: "note-measures.snap".into(),
            label: labels.measure_snap.into(),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: vec![
                WindowMeasure::Toggle {
                    id: "note-measures.snap-enabled".into(),
                    icon_id: "magnet".into(),
                    label: Some(labels.measure_snap_to_grid.into()),
                    pressed: document.snap_enabled.unwrap_or(false),
                    text: None,
                    on_change: note_action("setSnapEnabled", None),
                },
                WindowMeasure::Slider {
                    id: "note-measures.snap-spacing".into(),
                    label: Some(labels.measure_snap_spacing.into()),
                    value: document.snap_grid_spacing.unwrap_or(8.0),
                    min: 1.0,
                    max: 128.0,
                    step: Some(1.0),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: note_action("setSnapGridSpacing", None),
                    },
            ],
        },
        note_pencil_utility_options(document, labels),
        note_eraser_utility_options(document, labels, "eraserStroke"),
        note_eraser_utility_options(document, labels, "eraserPoint"),
    ]
}

fn note_pencil_utility_options(document: &NoteDocument, labels: &NotePlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "note-utility-options-pencil".into(),
        label: labels.measure_pencil_width.into(),
        default_open: Some(true),
        active_utility_id: Some("pencil".into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![WindowMeasure::Slider {
            id: "note-measures.pencil-width".into(),
            label: Some(labels.measure_pencil_width.into()),
            value: document.pencil_width.unwrap_or(3.0),
            min: 1.0,
            max: 24.0,
            step: Some(1.0),
            ready: None,
            loading: None,
            disabled: None,
            reveal: None,
            on_change: note_action("setPencilWidth", None),

            waiting: None,}],
    }
}

fn note_eraser_utility_options(document: &NoteDocument, labels: &NotePlayLabels, utility: &str) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("note-utility-options-{utility}"),
        label: labels.measure_eraser_radius.into(),
        default_open: Some(true),
        active_utility_id: Some(utility.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![WindowMeasure::Slider {
            id: format!("note-measures.eraser-radius-{utility}"),
            label: Some(labels.measure_eraser_radius.into()),
            value: document.eraser_radius.unwrap_or(12.0),
            min: 4.0,
            max: 48.0,
            step: Some(1.0),
            ready: None,
            loading: None,
            disabled: None,
            reveal: None,
            on_change: note_action("setEraserRadius", None),

            waiting: None,}],
    }
}

fn note_navigator_measures(document: &NoteDocument, camera: &NoteCamera, labels: &NotePlayLabels) -> Vec<WindowMeasure> {
    vec![
        WindowMeasure::Slider {
            id: "note-navigator-measures.zoom".into(),
            label: Some(labels.measure_zoom.into()),
            value: camera.zoom,
            min: 0.05,
            max: 2.0,
            step: Some(0.05),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: note_action("setCameraZoom", None),
            },
        WindowMeasure::Toggle {
            id: "note-navigator-measures.grid-visible".into(),
            icon_id: "layout-grid".into(),
            label: Some(labels.measure_show_grid.into()),
            pressed: document.grid_visible.unwrap_or(true),
            text: None,
            on_change: note_action("setGridVisible", None),
        },
    ]
}

fn note_canvas_engagement(document: &NoteDocument, camera: &NoteCamera, selected_ids: &[String], engagement_input: &str) -> WindowEngagement {
    let block_count = flatten_blocks(&document.blocks).len();
    let selected_count = selected_ids.len();
    let zoom = camera.zoom;
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

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()`/utility bar).
fn note_utility(id: &str, label: LocalizedLabel, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/keybound vocabulary
/// dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn note_internal_action(id: &str, label: LocalizedLabel, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}
//#endregion 🔖️Shell
//#endregion 🔖️Render

//#region 🔖️NotePlayApp
//#region 🔖️HandleHelpers
/// ✂️ Nudge step magnitudes: `1px` fine, `10px` fast.
const NUDGE_STEP: f64 = 1.0;
const NUDGE_STEP_FAST: f64 = 10.0;

/// 🧬️ Clones each of `ids` (present in `document`), offsets the clone by `(24, 24)`, and selects the
/// clones — the shared body of `NoteCommand::DuplicateBlock`/`DuplicateSelection`.
fn duplicate_blocks(document: &NoteDocument, ids: &[String]) -> Emit<NoteOperation, NoteConfigOperation> {
    let mut blocks = document.blocks.clone();
    let mut new_ids = Vec::new();
    for source_id in ids {
        if let Some(block) = find_block(&blocks, source_id).cloned() {
            let mut cloned = clone_block(&block);
            offset_block_tree(&mut cloned, 24.0, 24.0);
            new_ids.push(block_id(&cloned).to_string());
            if !insert_after(&mut blocks, source_id, cloned.clone()) {
                blocks.push(cloned);
            }
        }
    }
    if new_ids.is_empty() {
        return Emit::default();
    }
    Emit {
        document_operations: vec![NoteOperation::SetBlocks { blocks }],
        config_operations: vec![NoteConfigOperation::SetSelection { block_ids: new_ids }],
        ..Default::default()
    }
}

/// 🧬️ Offsets every unlocked selected block by `(dx, dy)` — the shared body of `NoteCommand::NudgeSelection`
/// and its eight directional/fast variants.
fn nudge(document: &NoteDocument, config: &NoteConfig, dx: f64, dy: f64) -> Emit<NoteOperation, NoteConfigOperation> {
    if config.selected_block_ids.is_empty() {
        return Emit::default();
    }
    let selected: HashSet<String> = config.selected_block_ids.iter().cloned().collect();
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
        return Emit::default();
    }
    let mut blocks = document.blocks.clone();
    for (id, updated) in nudges {
        update_block_in_tree(&mut blocks, &id, updated);
    }
    Emit::operations(vec![NoteOperation::SetBlocks { blocks }])
}
//#endregion 🔖️HandleHelpers

/// 🧪️ B1: unit struct — every former `NotePlayRuntime`/`ViewState`-read field now lives in
/// `note_engine::NoteConfig` (see `DocumentApp::Config`), written through `note_op::NoteConfigOperation`s.
#[derive(Default)]
pub struct NotePlayApp;

impl DocumentApp for NotePlayApp {
    type Projection = NoteDocument;
    type Operation = NoteOperation;
    type Config = NoteConfig;
    type ConfigOperation = NoteConfigOperation;
    type Command = NoteCommand;

    fn app_id(&self) -> &str {
        NOTE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        NOTE_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> NoteDocument {
        empty_note_document()
    }

    /// 🏷️ Maps each `NoteCommand` variant back to the action id it was declared under in
    /// `create_note_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &NoteCommand) -> &str {
        match command {
            NoteCommand::SetGridVisible { .. } => "setGridVisible",
            NoteCommand::SetGridSpacing { .. } => "setGridSpacing",
            NoteCommand::SetGridSubdivisions { .. } => "setGridSubdivisions",
            NoteCommand::SetGridOpacity { .. } => "setGridOpacity",
            NoteCommand::SetSnapEnabled { .. } => "setSnapEnabled",
            NoteCommand::SetSnapGridSpacing { .. } => "setSnapGridSpacing",
            NoteCommand::SetPencilWidth { .. } => "setPencilWidth",
            NoteCommand::SetEraserRadius { .. } => "setEraserRadius",
            NoteCommand::AddBlock { .. } => "addBlock",
            NoteCommand::MoveBlock { .. } => "moveBlock",
            NoteCommand::DeleteBlock { .. } => "deleteBlock",
            NoteCommand::DeleteSelection => "deleteSelection",
            NoteCommand::DuplicateBlock { .. } => "duplicateBlock",
            NoteCommand::DuplicateSelection => "duplicateSelection",
            NoteCommand::PatchBlocks { .. } => "patchBlocks",
            NoteCommand::SetActiveExample { .. } => "setActiveExample",
            NoteCommand::SetFixtureJson { .. } => "setFixtureJson",
            NoteCommand::InkApplyEvents { .. } => "inkApplyEvents",
            NoteCommand::EngagementSubmit { .. } => "engagementSubmit",
            NoteCommand::NudgeSelection { .. } => "nudgeSelection",
            NoteCommand::NudgeSelectionUp => "nudgeSelectionUp",
            NoteCommand::NudgeSelectionDown => "nudgeSelectionDown",
            NoteCommand::NudgeSelectionLeft => "nudgeSelectionLeft",
            NoteCommand::NudgeSelectionRight => "nudgeSelectionRight",
            NoteCommand::NudgeSelectionUpFast => "nudgeSelectionUpFast",
            NoteCommand::NudgeSelectionDownFast => "nudgeSelectionDownFast",
            NoteCommand::NudgeSelectionLeftFast => "nudgeSelectionLeftFast",
            NoteCommand::NudgeSelectionRightFast => "nudgeSelectionRightFast",
            NoteCommand::SetCamera { .. } => "setCamera",
            NoteCommand::SetCameraZoom { .. } => "setCameraZoom",
            NoteCommand::SetActiveUtility { .. } => SET_ACTIVE_UTILITY_ACTION_ID,
            NoteCommand::SetLocale { .. } => "setLocale",
            NoteCommand::SelectAll => "selectAll",
            NoteCommand::ClearSelection => "clearSelection",
            NoteCommand::SetSelection { .. } => "setSelection",
            NoteCommand::SetHover { .. } => "setHover",
            NoteCommand::EngagementInput { .. } => "engagementInput",
            NoteCommand::NavigatorEngagementInput => "navigatorEngagementInput",
            NoteCommand::SaveDownload => "saveDownload",
            NoteCommand::LoadRequest => "loadRequest",
        }
    }

    fn handle(
        &self,
        command: &NoteCommand,
        doc: &DocumentView<'_, NoteDocument>,
        cfg: &ConfigView<'_, NoteConfig>,
    ) -> Emit<NoteOperation, NoteConfigOperation> {
        let document = doc.projection;
        let config = cfg.projection;
        match command {
            // 📷️ Config-only: the free/live canvas camera never touches the document.
            NoteCommand::SetCamera { camera } => Emit::config(vec![NoteConfigOperation::SetCamera { camera: camera.clone() }]),
            NoteCommand::SetCameraZoom { value } => {
                let mut camera = config.camera.clone();
                camera.zoom = *value;
                Emit::config(vec![NoteConfigOperation::SetCamera { camera }])
            }
            // 🧰️ Host-owned utility switch — B1 moved the active utility from the deleted
            // `view_state.active_utility_id` into `cfg.active_utility_id`, so it now needs a real write.
            NoteCommand::SetActiveUtility { utility_id } => Emit::config(vec![NoteConfigOperation::SetActiveUtility { utility_id: utility_id.clone() }]),
            NoteCommand::SetLocale { value } => Emit::config(vec![NoteConfigOperation::SetLocale { value: value.clone() }]),

            NoteCommand::SetGridVisible { value } => {
                let next = value.unwrap_or(!document.grid_visible.unwrap_or(true));
                Emit::operations(vec![NoteOperation::SetGridVisible { visible: Some(next) }])
            }
            NoteCommand::SetGridSpacing { value } => Emit::operations(vec![NoteOperation::SetGridSpacing { spacing: Some(value.max(4.0)) }]),
            NoteCommand::SetGridSubdivisions { value } => Emit::operations(vec![NoteOperation::SetGridSubdivisions { value: Some(value.round().clamp(1.0, 16.0)) }]),
            NoteCommand::SetGridOpacity { value } => Emit::operations(vec![NoteOperation::SetGridOpacity { opacity: Some(value.clamp(0.05, 1.0)) }]),
            NoteCommand::SetSnapEnabled { value } => {
                let next = value.unwrap_or(!document.snap_enabled.unwrap_or(false));
                Emit::operations(vec![NoteOperation::SetSnapEnabled { enabled: Some(next) }])
            }
            NoteCommand::SetSnapGridSpacing { value } => Emit::operations(vec![NoteOperation::SetSnapGridSpacing { spacing: Some(value.max(1.0)) }]),
            NoteCommand::SetPencilWidth { value } => Emit::operations(vec![NoteOperation::SetPencilWidth { width: Some(value.clamp(1.0, 24.0)) }]),
            NoteCommand::SetEraserRadius { value } => Emit::operations(vec![NoteOperation::SetEraserRadius { radius: Some(value.clamp(4.0, 48.0)) }]),

            NoteCommand::AddBlock { kind, x, y } => {
                let block = create_block_by_kind(kind, *x, *y);
                let new_id = block_id(&block).to_string();
                let mut blocks = document.blocks.clone();
                blocks.push(block);
                Emit {
                    document_operations: vec![NoteOperation::SetBlocks { blocks }],
                    config_operations: vec![NoteConfigOperation::SetSelection { block_ids: vec![new_id] }],
                    ..Default::default()
                }
            }
            NoteCommand::MoveBlock { block_id: block_id_arg, target_row_id, drop_position } => {
                let Some(block) = find_block(&document.blocks, block_id_arg).cloned() else {
                    return Emit::default();
                };
                let target_id = block_id_from_tree_row_id(target_row_id);
                let parent_id = target_id.as_ref().and_then(|id| {
                    find_block(&document.blocks, id).and_then(|entry| {
                        if matches!(entry, NoteBlockNode::Group { .. }) { Some(id.clone()) } else { None }
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
                Emit::operations(vec![NoteOperation::SetBlocks { blocks }])
            }
            NoteCommand::DeleteBlock { block_id: target } => {
                let mut blocks = document.blocks.clone();
                remove_block_from_tree(&mut blocks, target);
                let selection: Vec<String> = config.selected_block_ids.iter().filter(|id| *id != target).cloned().collect();
                Emit {
                    document_operations: vec![NoteOperation::SetBlocks { blocks }],
                    config_operations: vec![NoteConfigOperation::SetSelection { block_ids: selection }],
                    ..Default::default()
                }
            }
            NoteCommand::DeleteSelection => {
                if config.selected_block_ids.is_empty() {
                    return Emit::default();
                }
                let mut blocks = document.blocks.clone();
                for id in &config.selected_block_ids {
                    remove_block_from_tree(&mut blocks, id);
                }
                Emit {
                    document_operations: vec![NoteOperation::SetBlocks { blocks }],
                    config_operations: vec![NoteConfigOperation::SetSelection { block_ids: Vec::new() }],
                    ..Default::default()
                }
            }
            NoteCommand::DuplicateBlock { block_id: source } => duplicate_blocks(document, std::slice::from_ref(source)),
            NoteCommand::DuplicateSelection => duplicate_blocks(document, &config.selected_block_ids),
            NoteCommand::PatchBlocks { block_ids, field, value } => {
                if block_ids.is_empty() || field.is_empty() {
                    return Emit::default();
                }
                let json_value = note_patch_json_value(field, value);
                let mut next = document.clone();
                for id in block_ids {
                    next = patch_block_field(&next, id, field, &json_value);
                }
                Emit::operations(vec![NoteOperation::SetBlocks { blocks: next.blocks }])
            }
            NoteCommand::SelectAll => {
                let ids: Vec<String> = flatten_blocks(&document.blocks).into_iter().map(|block| block_id(block).into()).collect();
                Emit::config(vec![NoteConfigOperation::SetSelection { block_ids: ids }])
            }
            NoteCommand::ClearSelection => Emit::config(vec![NoteConfigOperation::SetSelection { block_ids: Vec::new() }]),
            NoteCommand::SetSelection { ids } => Emit::config(vec![NoteConfigOperation::SetSelection { block_ids: ids.clone() }]),
            NoteCommand::SetHover { block_id } => Emit::config(vec![NoteConfigOperation::SetHoveredBlock { block_id: block_id.clone() }]),
            NoteCommand::NudgeSelection { dx, dy } => nudge(document, config, *dx, *dy),
            NoteCommand::NudgeSelectionUp => nudge(document, config, 0.0, -NUDGE_STEP),
            NoteCommand::NudgeSelectionDown => nudge(document, config, 0.0, NUDGE_STEP),
            NoteCommand::NudgeSelectionLeft => nudge(document, config, -NUDGE_STEP, 0.0),
            NoteCommand::NudgeSelectionRight => nudge(document, config, NUDGE_STEP, 0.0),
            NoteCommand::NudgeSelectionUpFast => nudge(document, config, 0.0, -NUDGE_STEP_FAST),
            NoteCommand::NudgeSelectionDownFast => nudge(document, config, 0.0, NUDGE_STEP_FAST),
            NoteCommand::NudgeSelectionLeftFast => nudge(document, config, -NUDGE_STEP_FAST, 0.0),
            NoteCommand::NudgeSelectionRightFast => nudge(document, config, NUDGE_STEP_FAST, 0.0),
            NoteCommand::EngagementInput { value } => Emit::config(vec![NoteConfigOperation::SetEngagementInput { value: value.clone() }]),
            NoteCommand::EngagementSubmit { value } => {
                let mut document_operations = Vec::new();
                if config.selected_block_ids.len() == 1 {
                    let name = value.clone().unwrap_or_else(|| config.engagement_input.clone());
                    let target_id = config.selected_block_ids[0].clone();
                    let next = patch_block_field(document, &target_id, "name", &Value::String(name));
                    document_operations.push(NoteOperation::SetBlocks { blocks: next.blocks });
                }
                Emit {
                    document_operations,
                    config_operations: vec![NoteConfigOperation::SetEngagementInput { value: String::new() }],
                    ..Default::default()
                }
            }
            NoteCommand::NavigatorEngagementInput => Emit::default(),
            NoteCommand::SetActiveExample { example_id } => {
                let next_document = if example_id == "semio" { semio_example_document() } else { empty_note_document() };
                Emit {
                    document_operations: vec![NoteOperation::SetDocument { document: next_document }],
                    config_operations: vec![NoteConfigOperation::SetSelection { block_ids: Vec::new() }],
                    ..Default::default()
                }
            }
            NoteCommand::SetFixtureJson { json } => {
                let next_document = if let Ok(document) = note_dsl::parse_dsl(json) {
                    document
                } else {
                    let Ok(parsed) = serde_json::from_str::<Value>(json) else {
                        return Emit::default();
                    };
                    if parsed.get("schema").and_then(|value| value.as_str()) != Some(NOTE_DOCUMENT_SCHEMA) {
                        return Emit::default();
                    };
                    let Ok(document) = serde_json::from_value::<NoteDocument>(parsed) else {
                        return Emit::default();
                    };
                    document
                };
                Emit {
                    document_operations: vec![NoteOperation::SetDocument { document: next_document }],
                    config_operations: vec![NoteConfigOperation::SetSelection { block_ids: Vec::new() }],
                    ..Default::default()
                }
            }
            NoteCommand::SaveDownload => {
                let data = note_dsl::print_dsl(document);
                Emit::effect(HostEffect::DownloadMediaExport { filename: "🗒️semio.note.dsl".into(), mime_type: "text/plain".into(), data, encoding: None })
            }
            NoteCommand::LoadRequest => Emit::effect(HostEffect::RequestFileOpen {
                accept: ".dsl,.note.dsl,.spk,.ops,application/octet-stream,text/plain".into(),
                read_as: None,
                import_action: "setFixtureJson".into(),
                multiple: false,
            }),
            NoteCommand::InkApplyEvents { events_json, phase, select_ids } => {
                let events: Vec<NoteCanvasEvent> = serde_json::from_str(events_json).unwrap_or_default();
                let mut config_operations = Vec::new();
                if let Some(ids) = select_ids {
                    config_operations.push(NoteConfigOperation::SetSelection { block_ids: ids.clone() });
                }
                // 📷️ Camera rides in the same batch as content edits but never becomes a document
                // operation — diffs into a config operation instead.
                for event in &events {
                    if let NoteCanvasEvent::SetCamera { camera } = event {
                        config_operations.push(NoteConfigOperation::SetCamera { camera: camera.clone() });
                    }
                }
                let operations = note_ops_from_canvas_events(document, &events);
                if operations.is_empty() && config_operations.is_empty() {
                    return Emit::default();
                }
                // The whole drag (begin → live* → commit) coalesces into ONE undoable edit; a lone
                // `atomic` event batch is its own edit. Selection/camera-only batches (no content change)
                // never need coalescing.
                let coalesce_key = if operations.is_empty() {
                    None
                } else {
                    match phase.as_str() {
                        "begin" | "live" | "commit" => Some("note-gesture".into()),
                        _ => None,
                    }
                };
                Emit { document_operations: operations, config_operations, coalesce_key, ..Default::default() }
            }
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, NoteDocument>, cfg: &ConfigView<'_, NoteConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let labels = resolve_labels::<NotePlayLabels>(config);
        match body_key {
            NOTE_PLAY_BODY_COMPOSITE => render_canvas_scene(
                document,
                &config.camera,
                &config.selected_block_ids,
                config.hovered_block_id.as_deref(),
                &config.active_utility_id,
                NOTE_PLAY_SURFACE_COMPOSITE,
                "composite",
            ),
            NOTE_PLAY_BODY_NAVIGATOR => render_canvas_scene(
                document,
                &config.camera,
                &config.selected_block_ids,
                config.hovered_block_id.as_deref(),
                &config.active_utility_id,
                NOTE_PLAY_SURFACE_NAVIGATOR,
                "navigator",
            ),
            NOTE_PLAY_BODY_DOCUMENT => render_document_panel(document, &config.selected_block_ids, labels),
            NOTE_PLAY_BODY_CATALOGUE => render_catalogue_panel(labels),
            NOTE_PLAY_BODY_PROPERTIES => render_properties_panel(document, &config.selected_block_ids, &config.active_utility_id, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, NoteDocument>, cfg: &ConfigView<'_, NoteConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.projection;
        HashMap::from([
            (NOTE_PLAY_WINDOW_COMPOSITE.to_string(), note_canvas_engagement(doc.projection, &config.camera, &config.selected_block_ids, &config.engagement_input)),
            (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), note_navigator_engagement(&config.active_utility_id)),
        ])
    }

    fn window_measures(&self, doc: &DocumentView<'_, NoteDocument>, cfg: &ConfigView<'_, NoteConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let labels = resolve_labels::<NotePlayLabels>(config);
        HashMap::from([
            (NOTE_PLAY_WINDOW_COMPOSITE.to_string(), note_canvas_measures(doc.projection, &config.camera, labels)),
            (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), note_navigator_measures(doc.projection, &config.camera, labels)),
        ])
    }

}
//#endregion 🔖️NotePlayApp

//#region 🔖️Manifest
pub fn create_note_app() -> App {
    let document = empty_note_document();
    let mut app = App::from_builder(
        App::builder(NOTE_PLAY_APP_ID, LocalizedLabel::native("Note", "Notiz")).document(["semio", "note"])
            .artifact_kind(ArtifactKindSpec {
                id: "2d.note".into(),
                name: "2D Note".into(),
                source_format: "note.document".into(),
                component_kind: "note".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Document },
                schema: "note.document".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("note")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(NOTE_PLAY_WINDOW_COMPOSITE, LocalizedLabel::native("Canvas", "Zeichenfläche"), NOTE_PLAY_BODY_COMPOSITE, SurfaceKind::InkCanvas, note_canvas_engagement(&document, &NoteCamera::default(), &[], ""), "pen-tool")
            .window_kind_with_engagement(NOTE_PLAY_WINDOW_NAVIGATOR, LocalizedLabel::native("Navigator", "Navigator"), NOTE_PLAY_BODY_NAVIGATOR, SurfaceKind::InkCanvas, note_navigator_engagement("selectDirect"), "focus")
            .default_layout(create_default_layout(
                &[NOTE_PLAY_WINDOW_COMPOSITE.into(), NOTE_PLAY_WINDOW_NAVIGATOR.into()],
                "row",
                Some(&[72.0, 28.0]),
                Some(&["Canvas".into(), "Navigator".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                NOTE_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                NOTE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                NOTE_PLAY_BODY_PROPERTIES,
            )
            // 📇️ Palette-visible selection commands (P0) — ephemeral selection is View, block edits are Operations.
            .view_action("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"))
            .view_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"))
            .operation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .operation("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"))
            // ➕️ Palette-visible block insertion (P1) with a staged argument form.
            .operation("addBlock", LocalizedLabel::native("Add Block", "Block hinzufügen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🐚️ Import/export footer actions → panel Shell actions emitting host effects (S).
            .shell_action("loadRequest", LocalizedLabel::native("Import", "Importieren"))
            .shell_action("saveDownload", LocalizedLabel::native("Export", "Exportieren"))
            // 🔧️ Internal content operations — inspector/tree/drag/import-bound, not palette commands.
            // B1: the old `"setGridVisible" | "toggleGrid"`/`"setSnapEnabled" | "toggleSnap"`/
            // `"addBlock" | "dropBlockKind"` action-id aliases collapsed onto one `NoteCommand` variant
            // each (see `note_protocol::NoteCommand`'s doc comment) — `toggleGrid`/`toggleSnap`/
            // `dropBlockKind` were never independently wired to any UI element or host caller, so their
            // dead alias declarations are dropped here rather than kept as unreachable synonyms.
            .action_with(note_internal_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Rastersichtbarkeit festlegen"), ActionKind::Operation))
            .action_with(note_internal_action("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"), ActionKind::Operation))
            .action_with(note_internal_action("setGridSubdivisions", LocalizedLabel::native("Set Grid Subdivisions", "Rasterunterteilungen festlegen"), ActionKind::Operation))
            .action_with(note_internal_action("setGridOpacity", LocalizedLabel::native("Set Grid Opacity", "Rasterdeckkraft festlegen"), ActionKind::Operation))
            .action_with(note_internal_action("setSnapEnabled", LocalizedLabel::native("Set Snap Enabled", "Einrasten aktivieren"), ActionKind::Operation))
            .action_with(note_internal_action("setSnapGridSpacing", LocalizedLabel::native("Set Snap Grid Spacing", "Rasterabstand für Einrasten festlegen"), ActionKind::Operation))
            .action_with(note_internal_action("setPencilWidth", LocalizedLabel::native("Set Pencil Width", "Stiftbreite festlegen"), ActionKind::Operation))
            .action_with(note_internal_action("setEraserRadius", LocalizedLabel::native("Set Eraser Radius", "Radiergummi-Radius festlegen"), ActionKind::Operation))
            .action_with(note_internal_action("moveBlock", LocalizedLabel::native("Move Block", "Block verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("deleteBlock", LocalizedLabel::native("Delete Block", "Block löschen"), ActionKind::Operation))
            .action_with(note_internal_action("duplicateBlock", LocalizedLabel::native("Duplicate Block", "Block duplizieren"), ActionKind::Operation))
            .action_with(note_internal_action("patchBlocks", LocalizedLabel::native("Patch Blocks", "Blöcke aktualisieren"), ActionKind::Operation))
            .action_with(note_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Operation))
            .action_with(note_internal_action("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Operation))
            .action_with(note_internal_action("inkApplyEvents", LocalizedLabel::native("Apply Note Events", "Notiz-Ereignisse anwenden"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelection", LocalizedLabel::native("Nudge Selection", "Auswahl verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionUp", LocalizedLabel::native("Nudge Selection Up", "Auswahl nach oben verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionDown", LocalizedLabel::native("Nudge Selection Down", "Auswahl nach unten verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionLeft", LocalizedLabel::native("Nudge Selection Left", "Auswahl nach links verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionRight", LocalizedLabel::native("Nudge Selection Right", "Auswahl nach rechts verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionUpFast", LocalizedLabel::native("Nudge Selection Up Fast", "Auswahl schnell nach oben verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionDownFast", LocalizedLabel::native("Nudge Selection Down Fast", "Auswahl schnell nach unten verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionLeftFast", LocalizedLabel::native("Nudge Selection Left Fast", "Auswahl schnell nach links verschieben"), ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionRightFast", LocalizedLabel::native("Nudge Selection Right Fast", "Auswahl schnell nach rechts verschieben"), ActionKind::Operation))
            // 👁️ Ephemeral view state — selection/hover/engagement/camera scratch, never a document operation.
            .action_with(note_internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(note_internal_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"), ActionKind::View))
            .action_with(note_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(note_internal_action("navigatorEngagementInput", LocalizedLabel::native("Navigator Engagement Input", "Navigator-Eingabe"), ActionKind::View))
            .action_with(note_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(note_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            // 📝️ Staged argument forms for the palette-eligible actions.
            .action_args("addBlock", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Typ"), vec![
                    ActionArgOption::new("text", LocalizedLabel::native("Text", "Text")),
                    ActionArgOption::new("image", LocalizedLabel::native("Image", "Bild")),
                    ActionArgOption::new("table", LocalizedLabel::native("Table", "Tabelle")),
                    ActionArgOption::new("math", LocalizedLabel::native("Math", "Mathe")),
                    ActionArgOption::new("stroke", LocalizedLabel::native("Ink", "Tinte")),
                    ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
                ]).required().default_value("text"),
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).default_value(0.0),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).default_value(0.0),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("semio", LocalizedLabel::native("Semio", "Semio")),
                ]).required().default_value("semio"),
            ])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON")).required()])
            // 🧰️ Canvas utilities — one exclusive set per window, active utility host-owned (never a document operation).
            .utility(note_utility("selectDirect", LocalizedLabel::native("Direct", "Direkt"), "text-cursor", "Select", UtilityCategory::Selection))
            .utility(note_utility("selectMarquee", LocalizedLabel::native("Marquee", "Rahmenauswahl"), "selection", "Select", UtilityCategory::Selection))
            .utility(note_utility("text", LocalizedLabel::native("Text", "Text"), "type", "Block", UtilityCategory::Utilities))
            .utility(note_utility("image", LocalizedLabel::native("Image", "Bild"), "image", "Block", UtilityCategory::Utilities))
            .utility(note_utility("table", LocalizedLabel::native("Table", "Tabelle"), "table-2", "Block", UtilityCategory::Utilities))
            .utility(note_utility("math", LocalizedLabel::native("Math", "Mathe"), "sigma", "Block", UtilityCategory::Utilities))
            .utility(note_utility("pencil", LocalizedLabel::native("Pencil", "Stift"), "pencil", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("eraserStroke", LocalizedLabel::native("Stroke Eraser", "Strich-Radiergummi"), "eraser", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("eraserPoint", LocalizedLabel::native("Point Eraser", "Punkt-Radiergummi"), "eraser", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("pan", LocalizedLabel::native("Pan", "Schwenken"), "hand", "View", UtilityCategory::Utilities))
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
            .keybinding("shift+right", "nudgeSelectionRightFast")
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE) —
            // note has no user-visible sticky config defaults (unlike shooting's default shot/asset
            // format), so `config_spec()` stays the trait default (`ConfigSpec::empty()`); registering it
            // here still declares the config schema for the manifest.
            .config(NotePlayApp::default().config_spec()),
    );
    for window in app.definition.window_kinds.iter_mut() {
        if window.id == NOTE_PLAY_WINDOW_COMPOSITE {
            window.options.measures = note_canvas_measures(&document, &NoteCamera::default(), &NotePlayLabels::NATIVE_EN);
        } else if window.id == NOTE_PLAY_WINDOW_NAVIGATOR {
            window.options.measures = note_navigator_measures(&document, &NoteCamera::default(), &NotePlayLabels::NATIVE_EN);
        }
    }
    app.example("semio", LocalizedLabel::native("Semio", "Semio"), semio_example_json(), "sparkles")
        .workflow("note", "Note", "document")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, ViewState};

    fn new_app() -> VcsDocumentApp<NotePlayApp> {
        testkit::new_app::<NotePlayApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so kind discipline runs.
    fn new_app_with_registry() -> VcsDocumentApp<NotePlayApp> {
        testkit::new_app_with_registry::<NotePlayApp>(create_note_app)
    }

    #[test]
    fn renders_composite_canvas() {
        let mut app = new_app();
        let node = app.render(NOTE_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("ink-canvas"));
        assert!(json.contains("documentJson"));
    }

    #[test]
    fn renders_navigator_canvas() {
        let mut app = new_app();
        let node = app.render(NOTE_PLAY_BODY_NAVIGATOR, Some(&semio_example_json()), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("ink-canvas"));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[test]
    fn parses_semio_example_document() {
        let document = semio_example_document();
        assert_eq!(document.blocks.len(), 3);
    }

    #[test]
    fn renders_document_tree() {
        let mut app = new_app();
        let node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(&semio_example_json()), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Welcome"));
    }

    #[test]
    fn note_labels_resolve_native_by_default() {
        let mut app = new_app();
        let document_node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(&semio_example_json()), &ViewState::default()).expect("render");
        let document_json = serde_json::to_string(&document_node).unwrap();
        assert!(document_json.contains("Add Text"));
        assert!(document_json.contains("Add Table"));
        assert!(document_json.contains("Add Math"));
        assert!(document_json.contains("Add Image"));
        assert!(document_json.contains("Add Group"));

        let catalogue_node = app.render(NOTE_PLAY_BODY_CATALOGUE, Some(&semio_example_json()), &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("Block kinds"));
        assert!(catalogue_json.contains("text — rich text block"));

        let empty_node = app.render(NOTE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let empty_json = serde_json::to_string(&empty_node).unwrap();
        assert!(empty_json.contains("Drop blocks here"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more passing
    /// a `ViewState` into `render`/`window_engagements`/`window_measures` for this purpose.
    #[test]
    fn note_labels_resolve_german_locale() {
        let mut app = new_app();
        app.dispatch_typed(NoteCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let document_node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(&semio_example_json()), &ViewState::default()).expect("render");
        let document_json = serde_json::to_string(&document_node).unwrap();
        assert!(document_json.contains("Text hinzufügen"));
        assert!(document_json.contains("Tabelle hinzufügen"));
        assert!(document_json.contains("Mathe hinzufügen"));
        assert!(document_json.contains("Bild hinzufügen"));
        assert!(document_json.contains("Gruppe hinzufügen"));

        let catalogue_node = app.render(NOTE_PLAY_BODY_CATALOGUE, Some(&semio_example_json()), &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("Blockarten"));
        assert!(catalogue_json.contains("Text — reicher Textblock"));

        // 🐛️ Pre-migration this asserted the ENGLISH placeholder ("Drop blocks here") even under German
        // locale — a stale copy-paste from the default-locale test above that `resolve_labels` never
        // actually exercised correctly before B1 (view_state.locale drove it exactly the same way).
        // Now that `cfg.locale` is set via a real dispatched command (not a host-pushed `ViewState`
        // field), asserting the correct German string catches a real regression instead of a fossilized one.
        let empty_node = app.render(NOTE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let empty_json = serde_json::to_string(&empty_node).unwrap();
        assert!(empty_json.contains("Blöcke hier ablegen"));
    }

    #[test]
    fn add_block_action_emits_one_op_and_grows_projection() {
        let mut app = new_app();
        let result = app.dispatch_typed(NoteCommand::AddBlock { kind: "text".into(), x: 80.0, y: 80.0 }, &testkit::meta("local")).expect("addBlock");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.blocks.len(), 1);
        assert_eq!(block_kind(&projection.blocks[0]), "text");
    }

    #[test]
    fn add_block_then_undo_round_trip() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            NoteCommand::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 },
            |app| app.projection().expect("projection").blocks.len(),
            0,
            1,
        );
    }

    #[test]
    fn properties_panel_reads_app_selection() {
        let mut app = new_app();
        app.dispatch_typed(NoteCommand::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }, &testkit::meta("local")).expect("add");
        let id = block_id(&app.projection().expect("projection").blocks[0]).to_string();
        app.dispatch_typed(NoteCommand::SetSelection { ids: vec![id] }, &testkit::meta("local")).expect("select");
        let node = app.render(NOTE_PLAY_BODY_PROPERTIES, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("note-properties.block"), "selected block must render an inspector group: {json}");
    }

    #[test]
    fn nudge_direction_actions_move_selection_without_args() {
        for (command, expected_dx, expected_dy) in [
            (NoteCommand::NudgeSelectionUp, 0.0, -1.0),
            (NoteCommand::NudgeSelectionDown, 0.0, 1.0),
            (NoteCommand::NudgeSelectionLeft, -1.0, 0.0),
            (NoteCommand::NudgeSelectionRight, 1.0, 0.0),
        ] {
            let mut app = new_app();
            // `addBlock` selects the freshly added block (see `NoteCommand::AddBlock`'s `handle` arm),
            // so the nudge below has something in `cfg.selected_block_ids` to move.
            app.dispatch_typed(NoteCommand::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }, &testkit::meta("local")).expect("add");
            let operations = app.dispatch_typed(command.clone(), &testkit::meta("local")).expect("nudge").operations.len();
            assert_eq!(operations, 1, "{command:?} should emit one operation");
            let projection = app.projection().expect("projection");
            let (x, y, ..) = block_bounds(&projection.blocks[0]);
            assert_eq!((x, y), (expected_dx, expected_dy), "{command:?} moved block to unexpected position");
        }
    }

    #[test]
    fn nudge_fast_actions_use_ten_pixel_step() {
        let mut app = new_app();
        app.dispatch_typed(NoteCommand::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }, &testkit::meta("local")).expect("add");
        app.dispatch_typed(NoteCommand::NudgeSelectionRightFast, &testkit::meta("local")).expect("nudge");
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
            { "operation": "addBlock", "block": block.clone(), "parentId": null, "index": null }
        ])
        .to_string();
        app.dispatch_typed(
            NoteCommand::InkApplyEvents { events_json: begin_events, phase: "begin".into(), select_ids: Some(vec![new_id.clone()]) },
            &testkit::meta("local"),
        )
        .expect("begin");
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);

        for x in [20.0, 30.0, 40.0] {
            let mut moved = block.clone();
            if let NoteBlockNode::Text { x: block_x, .. } = &mut moved {
                *block_x = x;
            }
            let live_events = json!([
                { "operation": "updateBlock", "blockId": new_id, "block": moved }
            ])
            .to_string();
            app.dispatch_typed(NoteCommand::InkApplyEvents { events_json: live_events, phase: "live".into(), select_ids: None }, &testkit::meta("local")).expect("live");
        }
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);

        // Commit with no further change emits no operation — the gesture is already recorded.
        let commit = app
            .dispatch_typed(NoteCommand::InkApplyEvents { events_json: "[]".into(), phase: "commit".into(), select_ids: None }, &testkit::meta("local"))
            .expect("commit");
        assert!(commit.operations.is_empty(), "a no-operation commit must not create an edit");
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);

        // The whole begin+live gesture coalesced into ONE undoable edit.
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert!(
            app.projection().expect("projection").blocks.is_empty(),
            "a single undo should erase the whole gesture"
        );
    }

    #[test]
    fn gesture_with_no_changes_creates_no_edit() {
        let mut app = new_app();
        app.dispatch_typed(NoteCommand::InkApplyEvents { events_json: "[]".into(), phase: "begin".into(), select_ids: None }, &testkit::meta("local")).expect("begin");
        app.dispatch_typed(NoteCommand::InkApplyEvents { events_json: "[]".into(), phase: "commit".into(), select_ids: None }, &testkit::meta("local")).expect("commit");
        let undo = app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert!(undo.events.is_empty(), "no gesture edit should exist to undo");
    }

    /// 🎥️ `setCamera`/`setCameraZoom` are config-only — they must never emit a `NoteOperation` (no VCS
    /// edit, no undo entry on the document store) and instead write into `cfg.camera`, which the
    /// composite scene's `documentJson.camera` then reflects.
    #[test]
    fn set_camera_writes_config_and_emits_no_document_operations() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        let result = app
            .dispatch_typed(NoteCommand::SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 2.0 } }, &testkit::meta("local"))
            .expect("set camera");
        assert!(result.operations.is_empty(), "camera is config-only and emits no document operations");
        assert_eq!(app.projection().expect("projection"), before, "camera never mutates the document");
        let json = serde_json::to_string(&app.render(NOTE_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains(r#"\"zoom\":2.0"#), "composite scene camera reflects config state: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "composite scene camera reflects config state: {json}");
    }

    #[test]
    fn set_camera_zoom_updates_zoom_and_keeps_pan_via_config() {
        let mut app = new_app();
        app.dispatch_typed(NoteCommand::SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 1.0 } }, &testkit::meta("local")).expect("set camera");
        let result = app.dispatch_typed(NoteCommand::SetCameraZoom { value: 3.0 }, &testkit::meta("local")).expect("set camera zoom");
        assert!(result.operations.is_empty(), "camera zoom is config-only and emits no document operations");
        let json = serde_json::to_string(&app.render(NOTE_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains(r#"\"zoom\":3.0"#), "zoom updated: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "pan preserved across zoom-only update: {json}");
    }

    /// 🧰️ B1: the active utility now lives in `cfg.active_utility_id` (the deleted `view_state` no
    /// longer exists) — switching utilities is still document-op-free, but it must actually persist.
    #[test]
    fn set_active_utility_emits_no_document_operations_but_persists_in_config() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        let result = app.dispatch_typed(NoteCommand::SetActiveUtility { utility_id: "pencil".into() }, &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.projection().expect("projection"), before, "utility switching does not mutate the document");
        let node = app.render(NOTE_PLAY_BODY_PROPERTIES, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("Utility: pencil"), "cfg.active_utility_id reflects the switch");
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
        app.dispatch_typed(NoteCommand::SetGridSubdivisions { value: 40.0 }, &testkit::meta("local")).expect("subdivisions");
        assert_eq!(app.projection().expect("projection").grid_subdivisions, Some(16.0));

        app.dispatch_typed(NoteCommand::SetGridOpacity { value: 5.0 }, &testkit::meta("local")).expect("opacity");
        assert_eq!(app.projection().expect("projection").grid_opacity, Some(1.0));
    }

    #[test]
    fn patch_blocks_table_row_and_column_ops_clamp_at_one() {
        let mut app = new_app();
        app.dispatch_typed(NoteCommand::AddBlock { kind: "table".into(), x: 0.0, y: 0.0 }, &testkit::meta("local")).expect("add");
        let table_id = block_id(&app.projection().expect("projection").blocks[0]).to_string();

        for (field, expected_rows, expected_columns) in [
            ("tableAddRow", 3, 3),
            ("tableAddColumn", 3, 4),
            ("tableRemoveRow", 2, 4),
            ("tableRemoveRow", 1, 4),
            ("tableRemoveRow", 1, 4),
            ("tableRemoveColumn", 1, 3),
        ] {
            app.dispatch_typed(NoteCommand::PatchBlocks { block_ids: vec![table_id.clone()], field: field.into(), value: String::new() }, &testkit::meta("local")).expect("patch");
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
        app.dispatch_typed(NoteCommand::AddBlock { kind: "text".into(), x: 10.0, y: 10.0 }, &testkit::meta("local")).expect("add");
        let source_id = block_id(&app.projection().expect("projection").blocks[0]).to_string();

        let result = app.dispatch_typed(NoteCommand::DuplicateSelection, &testkit::meta("local")).expect("duplicate");
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
        let save = app.dispatch_typed(NoteCommand::SaveDownload, &testkit::meta("local")).expect("save");
        assert!(save.operations.is_empty());
        assert!(
            matches!(save.requested_effects.first(), Some(HostEffect::DownloadMediaExport { filename, .. }) if filename == "🗒️semio.note.dsl"),
            "saveDownload must request a media export: {:?}",
            save.requested_effects
        );

        let load = app.dispatch_typed(NoteCommand::LoadRequest, &testkit::meta("local")).expect("load");
        assert!(
            matches!(load.requested_effects.first(), Some(HostEffect::RequestFileOpen { import_action, .. }) if import_action == "setFixtureJson"),
            "loadRequest must request a file open: {:?}",
            load.requested_effects
        );
    }

    #[test]
    fn set_fixture_json_replaces_document() {
        let mut app = new_app();
        let result = app.dispatch_typed(NoteCommand::SetFixtureJson { json: semio_example_json() }, &testkit::meta("local")).expect("fixture");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").blocks.len(), 3);
    }

    #[test]
    fn set_active_example_loads_semio_blocks() {
        let mut app = new_app();
        app.dispatch_typed(NoteCommand::SetActiveExample { example_id: "semio".into() }, &testkit::meta("local")).expect("semio");
        assert_eq!(app.projection().expect("projection").blocks.len(), 3);

        app.dispatch_typed(NoteCommand::SetActiveExample { example_id: String::new() }, &testkit::meta("local")).expect("empty");
        assert!(app.projection().expect("projection").blocks.is_empty());
    }

    #[test]
    fn world_pick_style_registry_enforcement_allows_the_active_utility_switch() {
        // 🧬️ Mirrors `shooting_ui`'s registry-backed coverage: dispatching through
        // `new_app_with_registry` exercises `AppActionRegistry` kind discipline for a View command.
        let mut app = new_app_with_registry();
        let result = app.dispatch_typed(NoteCommand::SetActiveUtility { utility_id: "pencil".into() }, &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "SetActiveUtility (View) emits no operations even under registry enforcement");
    }
}
//#endregion 🧪️Tests
