//! 🖥️ Note app — DocumentApp impl, render, manifest (constitutional: ui).

use note::{NoteBlockNode, NoteCamera, NoteDocument, NoteImageAsset, NoteTextParagraph, NoteTextRun, NOTE_DOCUMENT_SCHEMA};
use note_engine::{
    block_bounds, block_icon, block_id, block_id_from_tree_row_id, block_kind, block_name, block_visible, clone_block, create_block_by_kind,
    empty_note_document, find_block, flatten_blocks, insert_after, insert_block, offset_block_tree, patch_block_field, remove_block_from_tree,
    semio_example_document, semio_example_json, update_block_in_tree,
};
use note_op::NoteOperation;
use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_ink_canvas_scene, is_de_locale, localized_label_map, resolve_labels, selection_ids,
    tree_item, tree_item_with_action,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_stack_vertical, ui_text, App, MediaClass, MediaForm, MediaType, OsMediaCapability, ArtifactKindSpec,
    InkCanvasScene, ActionDescriptor, ActionEmit, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView,
    HostEffect, PanelTreeBuilder, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiToggleNode, UiTreeItemNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER, create_default_layout,
    ActionDefinition, ActionKind, ActionArgDef, ActionArgOption, UtilityDefinition, UtilityCategory, SET_ACTIVE_UTILITY_ACTION_ID,
    WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowMeasure,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

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
//#endregion 🔖Constants

//#region 🔖CanvasEvents
/// 🖱️ Batched canvas-event wire shape the `ink-canvas-host` surface emits (`addBlock`/`updateBlock`/
/// `removeBlock`/`putAsset`/`setCamera`); diffed into `NoteOperation`s by `note_ops_from_canvas_events`.
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
        NoteCanvasEvent::SetCamera { camera } => {
            document.camera = camera.clone();
        }
    }
}

/// 🔀 Applies a batch of canvas events to a cloned document and returns the minimal `NoteOperation`s
/// describing what changed (block-tree snapshot, camera, and per-asset puts) — the empty vec means
/// no content changed (e.g. a gesture that ended where it began).
fn note_ops_from_canvas_events(document: &NoteDocument, events: &[NoteCanvasEvent]) -> Vec<NoteOperation> {
    let mut next = document.clone();
    for event in events {
        apply_note_canvas_event(&mut next, event);
    }
    let mut operations = Vec::new();
    if next.blocks != document.blocks {
        operations.push(NoteOperation::SetBlocks { blocks: next.blocks.clone() });
    }
    if next.camera != document.camera {
        operations.push(NoteOperation::SetCamera { camera: next.camera.clone() });
    }
    for (key, asset) in &next.assets {
        if document.assets.get(key) != Some(asset) {
            operations.push(NoteOperation::PutAsset { key: key.clone(), asset: asset.clone() });
        }
    }
    operations
}
//#endregion 🔖CanvasEvents

//#region 🔖ActionHelpers
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
        args,
    }
}

/// 🔢 Reads a numeric action arg by its named key, falling back to a generic `value` (slider/measure inputs).
fn scalar_arg(args: Option<&Value>, key: &str) -> Option<f64> {
    args.and_then(|value| value.get(key))
        .or_else(|| args.and_then(|value| value.get("value")))
        .and_then(|value| value.as_f64())
}

fn selection_or_view(selected_ids: &[String], view_state: &ViewState) -> Vec<String> {
    if !selected_ids.is_empty() {
        return selected_ids.to_vec();
    }
    selection_from_view(view_state)
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
//#endregion 🔖ActionHelpers

//#region 🔖Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the note app; one field per label makes every locale combination compile-checked.
    struct NotePlayLabels {
        catalogue_title: &'static str = en: "Block kinds", de: "Blockarten";
        catalogue_text: &'static str = en: "text — rich text block", de: "Text — reicher Textblock";
        catalogue_image: &'static str = en: "image — embedded image", de: "Bild — eingebettetes Bild";
        catalogue_table: &'static str = en: "table — grid block", de: "Tabelle — Rasterblock";
        catalogue_math: &'static str = en: "math — TeX equation", de: "Mathe — TeX-Formel";
        catalogue_ink: &'static str = en: "ink — pencil strokes", de: "Tinte — Stiftstriche";
        catalogue_group: &'static str = en: "group — nested blocks", de: "Gruppe — verschachtelte Blöcke";
        inspector_block: &'static str = en: "Block", de: "Block";
        document_empty: &'static str = en: "Drop blocks here", de: "Blöcke hier ablegen";
        add_text: &'static str = en: "Add Text", de: "Text hinzufügen";
        add_table: &'static str = en: "Add Table", de: "Tabelle hinzufügen";
        add_math: &'static str = en: "Add Math", de: "Mathe hinzufügen";
        add_image: &'static str = en: "Add Image", de: "Bild hinzufügen";
        add_group: &'static str = en: "Add Group", de: "Gruppe hinzufügen";
        window_composite: &'static str = en: "Canvas", de: "Leinwand";
        window_navigator: &'static str = en: "Navigator", de: "Navigator";
        field_name: &'static str = en: "Name", de: "Name";
        field_x: &'static str = en: "X", de: "X";
        field_y: &'static str = en: "Y", de: "Y";
        field_width: &'static str = en: "Width", de: "Breite";
        field_height: &'static str = en: "Height", de: "Höhe";
        field_visible: &'static str = en: "Visible", de: "Sichtbar";
        field_locked: &'static str = en: "Locked", de: "Gesperrt";
        measure_camera: &'static str = en: "Camera", de: "Kamera";
        measure_zoom: &'static str = en: "Zoom", de: "Zoom";
        measure_grid: &'static str = en: "Grid", de: "Raster";
        measure_show_grid: &'static str = en: "Show grid", de: "Raster anzeigen";
        measure_spacing: &'static str = en: "Spacing", de: "Abstand";
        measure_subdivisions: &'static str = en: "Subdivisions", de: "Unterteilungen";
        measure_opacity: &'static str = en: "Opacity", de: "Deckkraft";
        measure_snap: &'static str = en: "Snap", de: "Fangen";
        measure_snap_to_grid: &'static str = en: "Snap to grid", de: "Am Raster einrasten";
        measure_snap_spacing: &'static str = en: "Snap spacing", de: "Rasterabstand";
        measure_drawing: &'static str = en: "Drawing", de: "Zeichnen";
        measure_pencil_width: &'static str = en: "Pencil width", de: "Stiftbreite";
        measure_eraser_radius: &'static str = en: "Eraser radius", de: "Radiergummi-Radius";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action/shell-action/internal-action declared in
/// `create_note_app`'s static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay
/// is how the command palette and Actions rail get a translated label without threading locale through the builder.
fn note_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("selectAll", "Select All", "Alles auswählen"),
        ("clearSelection", "Clear Selection", "Auswahl aufheben"),
        ("deleteSelection", "Delete Selection", "Auswahl löschen"),
        ("duplicateSelection", "Duplicate Selection", "Auswahl duplizieren"),
        ("addBlock", "Add Block", "Block hinzufügen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("loadRequest", "Import", "Importieren"),
        ("saveDownload", "Export", "Exportieren"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setCameraZoom", "Set Camera Zoom", "Kamerazoom festlegen"),
        ("setGridVisible", "Set Grid Visible", "Rastersichtbarkeit festlegen"),
        ("toggleGrid", "Toggle Grid", "Raster umschalten"),
        ("setGridSpacing", "Set Grid Spacing", "Rasterabstand festlegen"),
        ("setGridSubdivisions", "Set Grid Subdivisions", "Rasterunterteilungen festlegen"),
        ("setGridOpacity", "Set Grid Opacity", "Rasterdeckkraft festlegen"),
        ("setSnapEnabled", "Set Snap Enabled", "Einrasten aktivieren"),
        ("toggleSnap", "Toggle Snap", "Einrasten umschalten"),
        ("setSnapGridSpacing", "Set Snap Grid Spacing", "Rasterabstand für Einrasten festlegen"),
        ("setPencilWidth", "Set Pencil Width", "Stiftbreite festlegen"),
        ("setEraserRadius", "Set Eraser Radius", "Radiergummi-Radius festlegen"),
        ("dropBlockKind", "Drop Block Kind", "Blockart ablegen"),
        ("moveBlock", "Move Block", "Block verschieben"),
        ("deleteBlock", "Delete Block", "Block löschen"),
        ("duplicateBlock", "Duplicate Block", "Block duplizieren"),
        ("patchBlocks", "Patch Blocks", "Blöcke aktualisieren"),
        ("engagementSubmit", "Engagement Submit", "Eingabe bestätigen"),
        ("setFixtureJson", "Set Fixture Json", "Fixture-JSON festlegen"),
        ("inkApplyEvents", "Apply Note Events", "Notiz-Ereignisse anwenden"),
        ("nudgeSelection", "Nudge Selection", "Auswahl verschieben"),
        ("nudgeSelectionUp", "Nudge Selection Up", "Auswahl nach oben verschieben"),
        ("nudgeSelectionDown", "Nudge Selection Down", "Auswahl nach unten verschieben"),
        ("nudgeSelectionLeft", "Nudge Selection Left", "Auswahl nach links verschieben"),
        ("nudgeSelectionRight", "Nudge Selection Right", "Auswahl nach rechts verschieben"),
        ("nudgeSelectionUpFast", "Nudge Selection Up Fast", "Auswahl schnell nach oben verschieben"),
        ("nudgeSelectionDownFast", "Nudge Selection Down Fast", "Auswahl schnell nach unten verschieben"),
        ("nudgeSelectionLeftFast", "Nudge Selection Left Fast", "Auswahl schnell nach links verschieben"),
        ("nudgeSelectionRightFast", "Nudge Selection Right Fast", "Auswahl schnell nach rechts verschieben"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("engagementInput", "Engagement Input", "Eingabe"),
        ("navigatorEngagementInput", "Navigator Engagement Input", "Navigator-Eingabe"),
    ];
    localized_label_map(is_de, ENTRIES)
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_note_app`.
fn note_utility_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("selectDirect", "Direct", "Direkt"),
        ("selectMarquee", "Marquee", "Rahmenauswahl"),
        ("text", "Text", "Text"),
        ("image", "Image", "Bild"),
        ("table", "Table", "Tabelle"),
        ("math", "Math", "Mathe"),
        ("pencil", "Pencil", "Stift"),
        ("eraserStroke", "Stroke Eraser", "Strich-Radiergummi"),
        ("eraserPoint", "Point Eraser", "Punkt-Radiergummi"),
        ("pan", "Pan", "Schwenken"),
    ];
    localized_label_map(is_de, ENTRIES)
}
//#endregion 🔖CommandLabels

//#region 🔖Panels
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
        ..tree_item_with_action(
            block_tree_row_id(block),
            block_name(block),
            Some(block_kind(block).into()),
            play_action(NOTE_PLAY_CONTROLLER_ID, "setSelection", Some(json!({ "ids": [block_id(block)] }))),
        )
    }
}

fn block_tree_row_id(block: &NoteBlockNode) -> String {
    format!("note-play-block:{}", block_id(block))
}

fn render_document_panel(document: &NoteDocument, selected_ids: &[String], view_state: &ViewState, labels: &NotePlayLabels) -> UiNode {
    let action_rows: Vec<UiTreeItemNode> = [
        ("text", labels.add_text, "type"),
        ("table", labels.add_table, "table"),
        ("math", labels.add_math, "note-math"),
        ("image", labels.add_image, "image"),
        ("group", labels.add_group, "folder-plus"),
    ]
    .into_iter()
    .map(|(kind, label, icon)| UiTreeItemNode {
        icon_id: Some(icon.into()),
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
    let selected_ids: Vec<String> = selection_or_view(selected_ids, view_state)
        .iter()
        .filter_map(|id| find_block(&document.blocks, id).map(block_tree_row_id))
        .collect();
    PanelTreeBuilder::new("note-play-blocks")
        .section("note-play-blocks", Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()), true, [action_rows, block_items].concat())
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
            presence: UiPresence::default(),
        })),
        presence: UiPresence::default(),
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
            presence: UiPresence::default(),
        })),
        presence: UiPresence::default(),
    })
}

fn render_properties_panel(document: &NoteDocument, selected_ids: &[String], view_state: &ViewState, labels: &NotePlayLabels) -> UiNode {
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
                })),
                presence: UiPresence::default(),
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
                })),
                presence: UiPresence::default(),
            }),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
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
//#endregion 🔖Scenes

//#region 🔖Shell
fn note_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    play_action(NOTE_PLAY_CONTROLLER_ID, action, args)
}

fn note_canvas_measures(document: &NoteDocument, labels: &NotePlayLabels) -> Vec<WindowMeasure> {
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
                value: document.camera.zoom,
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

fn note_navigator_measures(document: &NoteDocument, labels: &NotePlayLabels) -> Vec<WindowMeasure> {
    vec![
        WindowMeasure::Slider {
            id: "note-navigator-measures.zoom".into(),
            label: Some(labels.measure_zoom.into()),
            value: document.camera.zoom,
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

/// 🧰 One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()`/utility bar).
fn note_utility(id: &str, label: &str, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/keybound vocabulary
/// dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn note_internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, kind) }
}
//#endregion 🔖Shell
//#endregion 🔖Render

//#region 🔖NotePlayApp
/// 🎛️ Ephemeral view state living on the app struct (never in the document): the current multi-selection,
/// the hovered block, and the pending engagement-rename input. Content lives in the store's `NoteDocument`
/// projection; every content mutation returns a typed {@link NoteOperation} so the store records a true inverse.
#[derive(Default)]
pub struct NotePlayApp {
    selected_ids: Vec<String>,
    hovered_id: Option<String>,
    engagement_input: String,
}

impl NotePlayApp {
    /// ✂️ Nudge step magnitudes: `1px` fine, `10px` fast.
    const NUDGE_STEP: f64 = 1.0;
    const NUDGE_STEP_FAST: f64 = 10.0;
}

impl DocumentApp for NotePlayApp {
    type Projection = NoteDocument;
    type Operation = NoteOperation;

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
    ) -> ActionEmit<NoteOperation> {
        // "undo"/"redo" never reach here — `VcsDocumentApp` intercepts them into store commands.
        let document = doc.projection;
        match action {
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value::<NoteCamera>(camera.clone()) {
                        return ActionEmit::operations(vec![NoteOperation::SetCamera { camera: parsed }]);
                    }
                }
                let zoom = args
                    .and_then(|value| value.get("zoom"))
                    .or_else(|| args.and_then(|value| value.get("value")))
                    .and_then(|value| value.as_f64());
                if let Some(zoom) = zoom {
                    let mut camera = document.camera.clone();
                    camera.zoom = zoom;
                    return ActionEmit::operations(vec![NoteOperation::SetCamera { camera }]);
                }
                ActionEmit::default()
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰 Host-owned utility switch: the active utility lives in `view_state.active_utility_id`, never
                // the document. Note keeps no in-progress gesture scratch on the app struct (ink drags
                // coalesce store-side), so there is nothing to clear and no operation to emit.
                ActionEmit::default()
            }
            "setGridVisible" | "toggleGrid" => {
                let visible = args
                    .and_then(|value| value.get("visible"))
                    .and_then(|value| value.as_bool())
                    .or_else(|| args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()))
                    .unwrap_or(!document.grid_visible.unwrap_or(true));
                ActionEmit::operations(vec![NoteOperation::SetGridVisible { visible: Some(visible) }])
            }
            "setGridSpacing" => match scalar_arg(args, "spacing") {
                Some(spacing) => ActionEmit::operations(vec![NoteOperation::SetGridSpacing { spacing: Some(spacing.max(4.0)) }]),
                None => ActionEmit::default(),
            },
            "setGridSubdivisions" => match scalar_arg(args, "subdivisions") {
                Some(subdivisions) => {
                    ActionEmit::operations(vec![NoteOperation::SetGridSubdivisions { value: Some(subdivisions.round().clamp(1.0, 16.0)) }])
                }
                None => ActionEmit::default(),
            },
            "setGridOpacity" => match scalar_arg(args, "opacity") {
                Some(opacity) => ActionEmit::operations(vec![NoteOperation::SetGridOpacity { opacity: Some(opacity.clamp(0.05, 1.0)) }]),
                None => ActionEmit::default(),
            },
            "setSnapEnabled" | "toggleSnap" => {
                let enabled = args
                    .and_then(|value| value.get("enabled"))
                    .and_then(|value| value.as_bool())
                    .or_else(|| args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()))
                    .unwrap_or(!document.snap_enabled.unwrap_or(false));
                ActionEmit::operations(vec![NoteOperation::SetSnapEnabled { enabled: Some(enabled) }])
            }
            "setSnapGridSpacing" => match scalar_arg(args, "spacing") {
                Some(spacing) => ActionEmit::operations(vec![NoteOperation::SetSnapGridSpacing { spacing: Some(spacing.max(1.0)) }]),
                None => ActionEmit::default(),
            },
            "setPencilWidth" => match scalar_arg(args, "width") {
                Some(width) => ActionEmit::operations(vec![NoteOperation::SetPencilWidth { width: Some(width.clamp(1.0, 24.0)) }]),
                None => ActionEmit::default(),
            },
            "setEraserRadius" => match scalar_arg(args, "radius") {
                Some(radius) => ActionEmit::operations(vec![NoteOperation::SetEraserRadius { radius: Some(radius.clamp(4.0, 48.0)) }]),
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
                ActionEmit::operations(vec![NoteOperation::SetBlocks { blocks }])
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
                ActionEmit::operations(vec![NoteOperation::SetBlocks { blocks }])
            }
            "deleteBlock" | "deleteSelection" => {
                if let Some(block_id) = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()) {
                    let mut blocks = document.blocks.clone();
                    remove_block_from_tree(&mut blocks, block_id);
                    self.selected_ids.retain(|id| id != block_id);
                    return ActionEmit::operations(vec![NoteOperation::SetBlocks { blocks }]);
                }
                if !self.selected_ids.is_empty() {
                    let mut blocks = document.blocks.clone();
                    for block_id in self.selected_ids.clone() {
                        remove_block_from_tree(&mut blocks, &block_id);
                    }
                    self.selected_ids.clear();
                    return ActionEmit::operations(vec![NoteOperation::SetBlocks { blocks }]);
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
                ActionEmit::operations(vec![NoteOperation::SetBlocks { blocks }])
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
                ActionEmit::operations(vec![NoteOperation::SetBlocks { blocks: next.blocks }])
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
                self.selected_ids = selection_ids(args);
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
                ActionEmit::operations(vec![NoteOperation::SetBlocks { blocks }])
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
                    ActionEmit::operations(vec![NoteOperation::SetBlocks { blocks: next.blocks }])
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
                    .unwrap_or("");
                let document = if example_id == "semio" {
                    semio_example_document()
                } else {
                    empty_note_document()
                };
                self.selected_ids.clear();
                ActionEmit::operations(vec![NoteOperation::SetDocument { document }])
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
                ActionEmit::operations(vec![NoteOperation::SetDocument { document }])
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
                multiple: false,
            }),
            "inkApplyEvents" => {
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
                let operations = note_ops_from_canvas_events(document, &events);
                if operations.is_empty() {
                    return ActionEmit::default();
                }
                // The whole drag (begin → live* → commit) coalesces into ONE undoable edit; a lone
                // `atomic` event batch is its own edit.
                let coalesce_key = match phase {
                    "begin" | "live" | "commit" => Some("note-gesture".into()),
                    _ => None,
                };
                ActionEmit { operations, coalesce_key, ..Default::default() }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, NoteDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = resolve_labels::<NotePlayLabels>(view_state);
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

    fn window_measures(&self, doc: &DocumentView<'_, NoteDocument>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = resolve_labels::<NotePlayLabels>(view_state);
        HashMap::from([
            (NOTE_PLAY_WINDOW_COMPOSITE.to_string(), note_canvas_measures(doc.projection, labels)),
            (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), note_navigator_measures(doc.projection, labels)),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<NotePlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(NOTE_PLAY_WINDOW_COMPOSITE, labels.window_composite)
            .window_kind_label(NOTE_PLAY_WINDOW_NAVIGATOR, labels.window_navigator)
            .action_labels(note_action_labels(is_de))
            .utility_labels(note_utility_labels(is_de))
    }
}
//#endregion 🔖NotePlayApp

//#region 🔖Manifest
pub fn create_note_app() -> App {
    let document = empty_note_document();
    let mut app = App::from_builder(
        App::builder(NOTE_PLAY_APP_ID, "Note").document(["semio", "note"])
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
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind_with_engagement(NOTE_PLAY_WINDOW_COMPOSITE, "Canvas", NOTE_PLAY_BODY_COMPOSITE, SurfaceKind::InkCanvas, note_canvas_engagement(&document, &[], ""), "pen-tool")
            .window_kind_with_engagement(NOTE_PLAY_WINDOW_NAVIGATOR, "Navigator", NOTE_PLAY_BODY_NAVIGATOR, SurfaceKind::InkCanvas, note_navigator_engagement("selectDirect"), "focus")
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
            .action_with(note_internal_action("inkApplyEvents", "Apply Note Events", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelection", "Nudge Selection", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionUp", "Nudge Selection Up", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionDown", "Nudge Selection Down", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionLeft", "Nudge Selection Left", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionRight", "Nudge Selection Right", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionUpFast", "Nudge Selection Up Fast", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionDownFast", "Nudge Selection Down Fast", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionLeftFast", "Nudge Selection Left Fast", ActionKind::Operation))
            .action_with(note_internal_action("nudgeSelectionRightFast", "Nudge Selection Right Fast", ActionKind::Operation))
            // 👁️ Ephemeral view state — selection/hover/engagement scratch, never a document operation.
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
                    ActionArgOption::new("stroke", "Ink"),
                    ActionArgOption::new("group", "Group"),
                ]).required().default_value("text"),
                ActionArgDef::number("x", "X").default_value(0.0),
                ActionArgDef::number("y", "Y").default_value(0.0),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new("semio", "Semio"),
                ]).required().default_value("semio"),
            ])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", "Document JSON").required()])
            // 🧰 Canvas utilities — one exclusive set per window, active utility host-owned (never a document operation).
            .utility(note_utility("selectDirect", "Direct", "cursor", "Select", UtilityCategory::Selection))
            .utility(note_utility("selectMarquee", "Marquee", "selection", "Select", UtilityCategory::Selection))
            .utility(note_utility("text", "Text", "type", "Block", UtilityCategory::Utilities))
            .utility(note_utility("image", "Image", "image", "Block", UtilityCategory::Utilities))
            .utility(note_utility("table", "Table", "table", "Block", UtilityCategory::Utilities))
            .utility(note_utility("math", "Math", "sigma", "Block", UtilityCategory::Utilities))
            .utility(note_utility("pencil", "Pencil", "pencil", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("eraserStroke", "Stroke Eraser", "eraser", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("eraserPoint", "Point Eraser", "eraser", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("pan", "Pan", "hand", "View", UtilityCategory::Utilities))
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
            window.options.measures = note_canvas_measures(&document, &NotePlayLabels::EN);
        } else if window.id == NOTE_PLAY_WINDOW_NAVIGATOR {
            window.options.measures = note_navigator_measures(&document, &NotePlayLabels::EN);
        }
    }
    app.example("semio", "Semio", semio_example_json())
        .workflow("note", "Note", "document")
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;
    use semio_framework_plugin::testkit::{assert_undo_redo_round_trip, meta, new_app, new_app_with_registry};

    #[test]
    fn renders_composite_canvas() {
        let mut app = new_app::<NotePlayApp>();
        let node = app.render(NOTE_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("ink-canvas"));
        assert!(json.contains("documentJson"));
    }

    #[test]
    fn renders_navigator_canvas() {
        let mut app = new_app::<NotePlayApp>();
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
        let mut app = new_app::<NotePlayApp>();
        let node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(&semio_example_json()), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Welcome"));
    }

    #[test]
    fn note_labels_resolve_native_by_default() {
        let mut app = new_app::<NotePlayApp>();
        let view_state = ViewState::default();
        let document_node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(&semio_example_json()), &view_state).expect("render");
        let document_json = serde_json::to_string(&document_node).unwrap();
        assert!(document_json.contains("Add Text"));
        assert!(document_json.contains("Add Table"));
        assert!(document_json.contains("Add Math"));
        assert!(document_json.contains("Add Image"));
        assert!(document_json.contains("Add Group"));

        let catalogue_node = app.render(NOTE_PLAY_BODY_CATALOGUE, Some(&semio_example_json()), &view_state).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("Block kinds"));
        assert!(catalogue_json.contains("text — rich text block"));

        let empty_node = app.render(NOTE_PLAY_BODY_DOCUMENT, None, &view_state).expect("render");
        let empty_json = serde_json::to_string(&empty_node).unwrap();
        assert!(empty_json.contains("Drop blocks here"));
    }

    #[test]
    fn note_labels_resolve_german_locale() {
        let mut app = new_app::<NotePlayApp>();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let document_node = app.render(NOTE_PLAY_BODY_DOCUMENT, Some(&semio_example_json()), &view_state).expect("render");
        let document_json = serde_json::to_string(&document_node).unwrap();
        assert!(document_json.contains("Text hinzufügen"));
        assert!(document_json.contains("Tabelle hinzufügen"));
        assert!(document_json.contains("Mathe hinzufügen"));
        assert!(document_json.contains("Bild hinzufügen"));
        assert!(document_json.contains("Gruppe hinzufügen"));

        let catalogue_node = app.render(NOTE_PLAY_BODY_CATALOGUE, Some(&semio_example_json()), &view_state).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("Blockarten"));
        assert!(catalogue_json.contains("Text — reicher Textblock"));

        let empty_node = app.render(NOTE_PLAY_BODY_DOCUMENT, None, &view_state).expect("render");
        let empty_json = serde_json::to_string(&empty_node).unwrap();
        assert!(empty_json.contains("Drop blocks here"));
    }

    #[test]
    fn add_block_action_emits_one_op_and_grows_projection() {
        let mut app = new_app::<NotePlayApp>();
        let result = app
            .handle_action("addBlock", Some(&json!({ "kind": "text" })), &ViewState::default(), &meta("local"))
            .expect("addBlock");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.blocks.len(), 1);
        assert_eq!(block_kind(&projection.blocks[0]), "text");
    }

    #[test]
    fn add_block_then_undo_round_trip() {
        let mut app = new_app::<NotePlayApp>();
        assert_undo_redo_round_trip(
            &mut app,
            "addBlock",
            Some(&json!({ "kind": "text" })),
            |app| app.projection().expect("projection").blocks.len(),
            0,
            1,
        );
    }

    #[test]
    fn properties_panel_reads_app_selection() {
        let mut app = new_app::<NotePlayApp>();
        app.handle_action("addBlock", Some(&json!({ "kind": "text" })), &ViewState::default(), &meta("local")).expect("add");
        let id = block_id(&app.projection().expect("projection").blocks[0]).to_string();
        app.handle_action("setSelection", Some(&json!({ "ids": [id] })), &ViewState::default(), &meta("local")).expect("select");
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
            let mut app = new_app::<NotePlayApp>();
            app.handle_action("addBlock", Some(&json!({ "kind": "text", "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta("local"))
                .expect("add");
            let operations = app.handle_action(action, None, &ViewState::default(), &meta("local")).expect(action).operations.len();
            assert_eq!(operations, 1, "{action} should emit one operation");
            let projection = app.projection().expect("projection");
            let (x, y, ..) = block_bounds(&projection.blocks[0]);
            assert_eq!((x, y), (expected_dx, expected_dy), "{action} moved block to unexpected position");
        }
    }

    #[test]
    fn nudge_fast_actions_use_ten_pixel_step() {
        let mut app = new_app::<NotePlayApp>();
        app.handle_action("addBlock", Some(&json!({ "kind": "text", "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta("local"))
            .expect("add");
        app.handle_action("nudgeSelectionRightFast", None, &ViewState::default(), &meta("local")).expect("nudge");
        let projection = app.projection().expect("projection");
        let (x, y, ..) = block_bounds(&projection.blocks[0]);
        assert_eq!((x, y), (10.0, 0.0));
    }

    #[test]
    fn gesture_begin_live_commit_produces_single_undo_step() {
        let mut app = new_app::<NotePlayApp>();
        let block = create_block_by_kind("text", 10.0, 10.0);
        let new_id = block_id(&block).to_string();

        let begin_events = json!([
            { "operation": "addBlock", "block": block.clone(), "parentId": null, "index": null }
        ])
        .to_string();
        app.handle_action(
            "inkApplyEvents",
            Some(&json!({ "eventsJson": begin_events, "phase": "begin", "selectIds": [new_id.clone()] })),
            &ViewState::default(),
            &meta("local"),
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
            app.handle_action(
                "inkApplyEvents",
                Some(&json!({ "eventsJson": live_events, "phase": "live" })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("live");
        }
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);

        // Commit with no further change emits no operation — the gesture is already recorded.
        let commit = app
            .handle_action(
                "inkApplyEvents",
                Some(&json!({ "eventsJson": "[]", "phase": "commit" })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("commit");
        assert!(commit.operations.is_empty(), "a no-operation commit must not create an edit");
        assert_eq!(app.projection().expect("projection").blocks.len(), 1);

        // The whole begin+live gesture coalesced into ONE undoable edit.
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert!(
            app.projection().expect("projection").blocks.is_empty(),
            "a single undo should erase the whole gesture"
        );
    }

    #[test]
    fn gesture_with_no_changes_creates_no_edit() {
        let mut app = new_app::<NotePlayApp>();
        app.handle_action(
            "inkApplyEvents",
            Some(&json!({ "eventsJson": "[]", "phase": "begin" })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("begin");
        app.handle_action(
            "inkApplyEvents",
            Some(&json!({ "eventsJson": "[]", "phase": "commit" })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("commit");
        let undo = app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert!(undo.events.is_empty(), "no gesture edit should exist to undo");
    }

    #[test]
    fn camera_action_emits_operation() {
        let mut app = new_app::<NotePlayApp>();
        let zoom = app
            .handle_action("setCameraZoom", Some(&json!({ "value": 2.0 })), &ViewState::default(), &meta("local"))
            .expect("zoom");
        assert_eq!(zoom.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").camera.zoom, 2.0);
    }

    #[test]
    fn set_active_utility_emits_no_ops_and_no_history_entry() {
        let mut app = new_app_with_registry::<NotePlayApp>(create_note_app);
        let before = app.projection().expect("projection");
        let view = ViewState { active_utility_id: Some("pencil".into()), ..ViewState::default() };
        let result = app
            .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "pencil" })), &view, &meta("local"))
            .expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
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
        let mut app = new_app::<NotePlayApp>();
        app.handle_action("setGridSubdivisions", Some(&json!({ "value": 40.0 })), &ViewState::default(), &meta("local"))
            .expect("subdivisions");
        assert_eq!(app.projection().expect("projection").grid_subdivisions, Some(16.0));

        app.handle_action("setGridOpacity", Some(&json!({ "value": 5.0 })), &ViewState::default(), &meta("local"))
            .expect("opacity");
        assert_eq!(app.projection().expect("projection").grid_opacity, Some(1.0));
    }

    #[test]
    fn patch_blocks_table_row_and_column_ops_clamp_at_one() {
        let mut app = new_app::<NotePlayApp>();
        app.handle_action("addBlock", Some(&json!({ "kind": "table" })), &ViewState::default(), &meta("local")).expect("add");
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
                &meta("local"),
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
        let mut app = new_app::<NotePlayApp>();
        app.handle_action("addBlock", Some(&json!({ "kind": "text", "x": 10.0, "y": 10.0 })), &ViewState::default(), &meta("local"))
            .expect("add");
        let source_id = block_id(&app.projection().expect("projection").blocks[0]).to_string();

        let result = app.handle_action("duplicateSelection", None, &ViewState::default(), &meta("local")).expect("duplicate");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.blocks.len(), 2);
        let clone = projection.blocks.iter().find(|block| block_id(block) != source_id).expect("clone block");
        let (x, y, ..) = block_bounds(clone);
        assert_eq!((x, y), (34.0, 34.0));
    }

    #[test]
    fn save_download_and_load_request_effects() {
        let mut app = new_app::<NotePlayApp>();
        let save = app.handle_action("saveDownload", None, &ViewState::default(), &meta("local")).expect("save");
        assert!(save.operations.is_empty());
        assert!(
            matches!(save.requested_effects.first(), Some(HostEffect::DownloadMediaExport { filename, .. }) if filename == "semio.note.json"),
            "saveDownload must request a media export: {:?}",
            save.requested_effects
        );

        let load = app.handle_action("loadRequest", None, &ViewState::default(), &meta("local")).expect("load");
        assert!(
            matches!(load.requested_effects.first(), Some(HostEffect::RequestFileOpen { import_action, .. }) if import_action == "setFixtureJson"),
            "loadRequest must request a file open: {:?}",
            load.requested_effects
        );
    }

    #[test]
    fn set_fixture_json_replaces_document() {
        let mut app = new_app::<NotePlayApp>();
        let result = app
            .handle_action("setFixtureJson", Some(&json!({ "payload": semio_example_json() })), &ViewState::default(), &meta("local"))
            .expect("fixture");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").blocks.len(), 3);
    }

    #[test]
    fn set_active_example_loads_semio_blocks() {
        let mut app = new_app::<NotePlayApp>();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "semio" })), &ViewState::default(), &meta("local"))
            .expect("semio");
        assert_eq!(app.projection().expect("projection").blocks.len(), 3);

        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &ViewState::default(), &meta("local"))
            .expect("empty");
        assert!(app.projection().expect("projection").blocks.is_empty());
    }
}
//#endregion 🧪Tests
