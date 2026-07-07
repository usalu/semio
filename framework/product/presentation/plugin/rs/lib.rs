//! 📽️ Presentation plugin — tile play app bundled as a hot-swappable WASM component.

use presentation_deck::{
    apply_presentation_edit, build_tile_morph_prompt, default_presentation_deck, parse_grid_engagement,
    populate_tile_drafts_from_grid, PresentationDeck, PresentationEdit, FigureTileDraft, FigureTileFrame,
    FigureTileGridSeedSpec, FigureTileSource, PRESENTATION_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{PanelGroup, 
    build_canvas_2d_scene, create_default_layout, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, App,
    Canvas2dScene, CommandDescriptor, PluginApp, PluginBundle, UiControlNode, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiSectionNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const PRESENTATION_PLAY_APP_ID: &str = "presentation-tile-play";
const PRESENTATION_PLAY_CONTROLLER_ID: &str = "presentation-tile-play";
const PRESENTATION_PLAY_SURFACE_ID: &str = "presentation.tile.play";
const PRESENTATION_PLAY_BODY_MAIN: &str = "presentation.tile.play.main";
const PRESENTATION_PLAY_BODY_DOCUMENT: &str = "presentation.tile.play.document";
const PRESENTATION_PLAY_BODY_CATALOGUE: &str = "presentation.tile.play.catalogue";
const PRESENTATION_PLAY_BODY_DETAILS: &str = "presentation.tile.play.details";
const PRESENTATION_PLAY_WINDOW_MAIN: &str = "tile-editor";

static TILE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PresentationPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    engagement_input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    clipboard_prompt: Option<String>,
    #[serde(default)]
    clipboard_epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PresentationPlayEnvelope {
    deck: PresentationDeck,
    #[serde(default)]
    runtime: PresentationPlayRuntime,
}

fn default_envelope() -> PresentationPlayEnvelope {
    PresentationPlayEnvelope {
        deck: default_presentation_deck(),
        runtime: PresentationPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> PresentationPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &PresentationPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn presentation_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PRESENTATION_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn new_tile_id(prefix: &str) -> String {
    let serial = TILE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{serial}")
}

fn apply_edit(envelope: &mut PresentationPlayEnvelope, edit: PresentationEdit) {
    envelope.deck = apply_presentation_edit(envelope.deck.clone(), &edit);
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}
//#endregion 🔖Envelope

//#region 🔖CanvasLayers
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TileCanvasLayer {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    /// 🖼️ Image src for `kind: "image"` layers, rendered by both the React and wgpu canvas-2d hosts.
    #[serde(skip_serializing_if = "Option::is_none")]
    data_url: Option<String>,
}

fn frame_to_canvas(frame: &FigureTileFrame, scale: f64) -> (f64, f64, f64, f64) {
    (
        frame.x * scale,
        frame.y * scale,
        frame.width * scale,
        frame.height * scale,
    )
}

/// 🖼️ Renders the actual source figure (image) as the backdrop layer, with crop tiles drawn on top of it.
fn deck_to_canvas_layers(deck: &PresentationDeck, selected: &[String]) -> String {
    const SCALE: f64 = 1000.0;
    let mut layers = Vec::new();
    let (sx, sy, sw, sh) = frame_to_canvas(&deck.source.frame, SCALE);
    let has_image_src = !deck.source.src.trim().is_empty() && deck.source.kind != "pdf";
    layers.push(TileCanvasLayer {
        id: "source-frame".into(),
        kind: if has_image_src { "image".into() } else { "source".into() },
        name: deck.source.src.clone(),
        x: sx,
        y: sy,
        width: sw,
        height: sh,
        data_url: has_image_src.then(|| deck.source.src.clone()),
    });
    for tile in &deck.tiles {
        let (x, y, width, height) = frame_to_canvas(&tile.crop, SCALE);
        let selected_flag = selected.contains(&tile.id);
        layers.push(TileCanvasLayer {
            id: tile.id.clone(),
            kind: if selected_flag { "tile-selected" } else { "tile" }.into(),
            name: tile.name.clone(),
            x,
            y,
            width,
            height,
            data_url: None,
        });
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖CanvasLayers

//#region 🔖Panels
fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
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
    }
}

fn build_document_tree(envelope: &PresentationPlayEnvelope) -> UiNode {
    let items: Vec<UiTreeItemNode> = envelope
        .deck
        .tiles
        .iter()
        .map(|tile| UiTreeItemNode {
            id: tile.id.clone(),
            label: tile.name.clone(),
            description: Some(format!(
                "x={:.3} y={:.3} w={:.3} h={:.3}",
                tile.crop.x, tile.crop.y, tile.crop.width, tile.crop.height
            )),
            icon_id: None,
            selected: Some(envelope.runtime.selected_ids.contains(&tile.id)),
            default_open: None,
            command: Some(presentation_cmd("setSelectedIds", Some(json!({ "ids": [tile.id] })))),
        hover_command: None,
        unhover_command: None,
        actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "presentation-tile-play.tiles".into(),
            label: Some("Tiles".into()),
            default_open: Some(true),
            items: if items.is_empty() {
                vec![tree_item("empty", "(no tiles — seed a grid)")]
            } else {
                items
            },
        }],
        selected_ids: Some(envelope.runtime.selected_ids.clone()),
        highlighted_ids: None,
        selection_change: Some(presentation_cmd("setSelectedIds", Some(json!({ "ids": [] })))),
    })
}

fn inspector_crop_field(tile_ids: &[String], field: &str, label: &str, values: &[f64]) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: format!("presentation.play.tile.crop.{field}"),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("presentation.play.tile.crop.{field}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform {
                format!("{:.6}", values.first().copied().unwrap_or(0.0))
            } else {
                String::new()
            },
            placeholder: if mixed.uniform {
                None
            } else {
                Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
            },
            commit: Some("blur".into()),
            on_change: presentation_cmd(
                "patchTileCrops",
                Some(json!({ "ids": tile_ids, "field": field })),
            ),
        }),
    })
}

fn build_details_tree(envelope: &PresentationPlayEnvelope) -> UiNode {
    if envelope.runtime.selected_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "presentation.play.details.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Select a tile in the canvas or workbench document.")],
        }]);
    }
    let tiles: Vec<&FigureTileDraft> = envelope
        .runtime
        .selected_ids
        .iter()
        .filter_map(|id| envelope.deck.tiles.iter().find(|tile| &tile.id == id))
        .collect();
    if tiles.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "presentation.play.details.not-found".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Selected tile not found.")],
        }]);
    }
    let tile_ids: Vec<String> = tiles.iter().map(|tile| tile.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&tiles.iter().map(|tile| tile.name.clone()).collect::<Vec<_>>());
    let mut identity_fields: Vec<UiNode> = vec![UiNode::Field(UiFieldNode {
        id: "presentation.play.tile.name".into(),
        label: "Name".into(),
        child: UiControlNode::Input(UiInputNode {
            id: "presentation.play.tile.name.input".into(),
            input_kind: "text".into(),
            value: name_mixed.value,
            placeholder: name_mixed.placeholder,
            commit: Some("blur".into()),
            on_change: presentation_cmd("renameTiles", Some(json!({ "ids": tile_ids }))),
        }),
    })];
    identity_fields.push(ui_inspector_readonly_field(
        "presentation.play.tile.id",
        "Id",
        if tile_ids.len() == 1 {
            tile_ids.first().cloned().unwrap_or_default()
        } else {
            format!("{} selected", tile_ids.len())
        },
    ));
    if tile_ids.len() == 1 {
        identity_fields.push(UiNode::Button(semio_framework_plugin::UiButtonNode {
            id: Some(format!("presentation.play.tile.{}.delete", tile_ids[0])),
            icon_id: "trash-2".into(),
            label: "Delete tile".into(),
            command: presentation_cmd("deleteTile", Some(json!({ "id": tile_ids[0] }))),
            style: None,
        }));
    }
    identity_fields.push(UiNode::Button(semio_framework_plugin::UiButtonNode {
        id: Some("presentation.play.details.delete-selection".into()),
        icon_id: "trash-2".into(),
        label: "Delete selection".into(),
        command: presentation_cmd("deleteSelection", None),
        style: None,
    }));
    let groups = vec![
        UiInspectorFieldGroup {
            id: "presentation.play.details.crop".into(),
            label: "Crop".into(),
            default_open: None,
            fields: vec![
                inspector_crop_field(&tile_ids, "x", "X", &tiles.iter().map(|tile| tile.crop.x).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "y", "Y", &tiles.iter().map(|tile| tile.crop.y).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "width", "Width", &tiles.iter().map(|tile| tile.crop.width).collect::<Vec<_>>()),
                inspector_crop_field(
                    &tile_ids,
                    "height",
                    "Height",
                    &tiles.iter().map(|tile| tile.crop.height).collect::<Vec<_>>(),
                ),
            ],
        },
        UiInspectorFieldGroup {
            id: "presentation.play.details.identity".into(),
            label: "Identity".into(),
            default_open: None,
            fields: identity_fields,
        },
    ];
    ui_inspector_groups_to_tree(&groups)
}

fn catalogue_button(id: &str, label: &str, command: &str, args: Option<Value>) -> UiNode {
    UiNode::Button(semio_framework_plugin::UiButtonNode {
        id: Some(id.into()),
        icon_id: "plus".into(),
        label: label.into(),
        command: presentation_cmd(command, args),
        style: None,
    })
}

fn build_catalogue_tree(envelope: &PresentationPlayEnvelope) -> UiNode {
    ui_declarative_sections_to_tree(&[
        UiSectionNode {
            id: "presentation.play.catalogue.templates".into(),
            label: Some("Tile templates".into()),
            default_open: Some(true),
            children: vec![
                ui_text("Seed morph tiles from figure templates."),
                catalogue_button(
                    "presentation.play.catalogue.seed-2x2",
                    "Split 2×2 grid",
                    "seedGrid",
                    Some(json!({ "rows": 2, "columns": 2 })),
                ),
                catalogue_button(
                    "presentation.play.catalogue.seed-3x5",
                    "Split 3×5 catalogue grid",
                    "seedGrid",
                    Some(json!({ "rows": 3, "columns": 5 })),
                ),
                catalogue_button("presentation.play.catalogue.add-tile", "Add single tile", "addTile", None),
                catalogue_button("presentation.play.catalogue.clear", "Clear tiles", "clearTiles", None),
            ],
        },
        UiSectionNode {
            id: "presentation.play.catalogue.figure".into(),
            label: Some("Figure templates".into()),
            default_open: Some(true),
            children: vec![
                catalogue_button(
                    "presentation.play.catalogue.figure.catalogue",
                    "Use catalogue figure",
                    "setSource",
                    Some(json!(default_presentation_deck().source)),
                ),
                UiNode::Field(UiFieldNode {
                    id: "presentation.play.catalogue.figure.src".into(),
                    label: "Active source".into(),
                    child: UiControlNode::Input(UiInputNode {
                        id: "presentation.play.catalogue.figure.src.readonly".into(),
                        input_kind: "text".into(),
                        value: envelope.deck.source.src.clone(),
                        placeholder: None,
                        commit: None,
                        on_change: presentation_cmd("noop", None),
                    }),
                }),
                ui_text(format!("Media kind: {}", envelope.deck.source.kind)),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_canvas(envelope: &PresentationPlayEnvelope) -> UiNode {
    build_canvas_2d_scene(
        PRESENTATION_PLAY_SURFACE_ID,
        PRESENTATION_PLAY_CONTROLLER_ID,
        Canvas2dScene {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            layers_json: deck_to_canvas_layers(&envelope.deck, &envelope.runtime.selected_ids),
        },
    )
}
//#endregion 🔖Render

//#region 🔖PresentationPlayApp
struct PresentationPlayApp;

impl PluginApp for PresentationPlayApp {
    fn app_id(&self) -> &str {
        PRESENTATION_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("presentation envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelectedIds" => {
                let ids = selection_ids(args);
                let valid: std::collections::HashSet<&str> = envelope.deck.tiles.iter().map(|tile| tile.id.as_str()).collect();
                envelope.runtime.selected_ids = ids.into_iter().filter(|id| valid.contains(id.as_str())).collect();
                return vec![set_document_op(&envelope)];
            }
            "seedGrid" => {
                let rows = args.and_then(|v| v.get("rows")).and_then(|v| v.as_u64()).unwrap_or(3) as u32;
                let columns = args.and_then(|v| v.get("columns")).and_then(|v| v.as_u64()).unwrap_or(5) as u32;
                let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
                    source: &envelope.deck.source,
                    rows,
                    columns,
                    gap: 0.0,
                    key_prefix: "tile",
                });
                apply_edit(&mut envelope, PresentationEdit::SetTiles { tiles: tiles.clone() });
                envelope.runtime.selected_ids = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
                return vec![set_document_op(&envelope)];
            }
            "addTile" => {
                let id = new_tile_id("tile");
                let crop = args
                    .and_then(|v| v.get("crop"))
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or(FigureTileFrame {
                        x: 0.1,
                        y: 0.1,
                        width: 0.2,
                        height: 0.2,
                    });
                apply_edit(
                    &mut envelope,
                    PresentationEdit::AddTile {
                        tile: FigureTileDraft {
                            id: id.clone(),
                            name: id.clone(),
                            crop,
                        },
                        index: None,
                    },
                );
                envelope.runtime.selected_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "deleteTile" => {
                let target = args
                    .and_then(|v| v.get("id"))
                    .and_then(|v| v.as_str())
                    .map(|id| vec![id.to_string()])
                    .unwrap_or_else(|| envelope.runtime.selected_ids.clone());
                if !target.is_empty() {
                    apply_edit(&mut envelope, PresentationEdit::RemoveTiles { tile_ids: target.clone() });
                    envelope.runtime.selected_ids.retain(|id| !target.contains(id));
                }
                return vec![set_document_op(&envelope)];
            }
            "deleteSelection" => {
                if !envelope.runtime.selected_ids.is_empty() {
                    let remove = envelope.runtime.selected_ids.clone();
                    apply_edit(&mut envelope, PresentationEdit::RemoveTiles { tile_ids: remove });
                    envelope.runtime.selected_ids.clear();
                }
                return vec![set_document_op(&envelope)];
            }
            "renameTile" | "renameTiles" => {
                let ids: Vec<String> = args
                    .and_then(|v| v.get("ids"))
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();
                let name = args
                    .and_then(|v| v.get("value").or_else(|| v.get("name")))
                    .and_then(|v| v.as_str())
                    .map(str::trim)
                    .filter(|value| !value.is_empty());
                if let Some(name) = name {
                    let valid: Vec<String> = ids
                        .into_iter()
                        .filter(|id| envelope.deck.tiles.iter().any(|tile| &tile.id == id))
                        .collect();
                    if !valid.is_empty() {
                        apply_edit(
                            &mut envelope,
                            PresentationEdit::RenameTiles {
                                tile_ids: valid,
                                name: name.into(),
                            },
                        );
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "patchTileCrops" | "patchTileCrop" => {
                let ids: Vec<String> = args
                    .and_then(|v| v.get("ids"))
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|v| v.get("field")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64());
                if let Some(value) = value {
                    let valid: Vec<String> = ids
                        .into_iter()
                        .filter(|id| envelope.deck.tiles.iter().any(|tile| &tile.id == id))
                        .collect();
                    if !valid.is_empty() && !field.is_empty() {
                        apply_edit(
                            &mut envelope,
                            PresentationEdit::PatchTileCrops {
                                tile_ids: valid,
                                field: field.into(),
                                value,
                            },
                        );
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "setSource" => {
                if let Some(source_value) = args {
                    let mut partial = envelope.deck.source.clone();
                    if let Some(src) = source_value.get("src").and_then(|v| v.as_str()) {
                        partial.src = src.into();
                    }
                    if let Some(kind) = source_value.get("kind").and_then(|v| v.as_str()) {
                        partial.kind = kind.into();
                    }
                    if let Some(frame_value) = source_value.get("frame") {
                        if let Ok(frame) = serde_json::from_value::<FigureTileFrame>(frame_value.clone()) {
                            partial.frame = frame;
                        }
                    }
                    if source_value.get("src").is_none() && source_value.get("kind").is_none() && source_value.get("frame").is_none() {
                        if let Ok(source) = serde_json::from_value::<FigureTileSource>(source_value.clone()) {
                            partial = source;
                        }
                    }
                    let replaced = partial.src != envelope.deck.source.src;
                    apply_edit(
                        &mut envelope,
                        PresentationEdit::ReplaceSource {
                            source: partial,
                            reset_tiles: replaced,
                        },
                    );
                    if replaced {
                        envelope.runtime.selected_ids.clear();
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "setFrame" => {
                if let Some(frame_value) = args.and_then(|v| v.get("frame")) {
                    if let Ok(frame) = serde_json::from_value::<FigureTileFrame>(frame_value.clone()) {
                        let mut source = envelope.deck.source.clone();
                        source.frame = frame;
                        apply_edit(
                            &mut envelope,
                            PresentationEdit::ReplaceSource {
                                source,
                                reset_tiles: false,
                            },
                        );
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveExample" => {
                let example_id = args.and_then(|v| v.get("exampleId")).and_then(|v| v.as_str()).unwrap_or("demo");
                if example_id == "demo" || example_id.is_empty() {
                    envelope.deck = default_presentation_deck();
                }
                envelope.runtime.selected_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "clearTiles" => {
                apply_edit(&mut envelope, PresentationEdit::ClearTiles);
                envelope.runtime.selected_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "copyPrompt" => {
                envelope.runtime.clipboard_prompt =
                    Some(build_tile_morph_prompt(&envelope.deck.source, &envelope.deck.tiles));
                envelope.runtime.clipboard_epoch += 1;
                return vec![set_document_op(&envelope)];
            }
            "engagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    envelope.runtime.engagement_input = value.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "engagementSubmit" => {
                let value = args
                    .and_then(|v| v.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(envelope.runtime.engagement_input.as_str());
                let trimmed = value.trim();
                if let Some((rows, columns)) = parse_grid_engagement(trimmed) {
                    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
                        source: &envelope.deck.source,
                        rows,
                        columns,
                        gap: 0.0,
                        key_prefix: "tile",
                    });
                    apply_edit(&mut envelope, PresentationEdit::SetTiles { tiles: tiles.clone() });
                    envelope.runtime.selected_ids = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
                    envelope.runtime.engagement_input.clear();
                } else {
                    let token = trimmed.to_lowercase();
                    match token.as_str() {
                        "add" => {
                            let id = new_tile_id("tile");
                            apply_edit(
                                &mut envelope,
                                PresentationEdit::AddTile {
                                    tile: FigureTileDraft {
                                        id: id.clone(),
                                        name: id.clone(),
                                        crop: FigureTileFrame {
                                            x: 0.1,
                                            y: 0.1,
                                            width: 0.2,
                                            height: 0.2,
                                        },
                                    },
                                    index: None,
                                },
                            );
                            envelope.runtime.selected_ids = vec![id];
                            envelope.runtime.engagement_input.clear();
                        }
                        "clear" => {
                            apply_edit(&mut envelope, PresentationEdit::ClearTiles);
                            envelope.runtime.selected_ids.clear();
                            envelope.runtime.engagement_input.clear();
                        }
                        "copy" | "copy prompt" => {
                            envelope.runtime.clipboard_prompt =
                                Some(build_tile_morph_prompt(&envelope.deck.source, &envelope.deck.tiles));
                            envelope.runtime.clipboard_epoch += 1;
                            envelope.runtime.engagement_input.clear();
                        }
                        _ => {}
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "canvasPointerDown" => {
                if let Some(layer_id) = args.and_then(|v| v.get("layerId")).and_then(|v| v.as_str()) {
                    if envelope.deck.tiles.iter().any(|tile| tile.id == layer_id) {
                        envelope.runtime.selected_ids = vec![layer_id.into()];
                        return vec![set_document_op(&envelope)];
                    }
                }
                envelope.runtime.selected_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            PRESENTATION_PLAY_BODY_MAIN => render_main_canvas(&envelope),
            PRESENTATION_PLAY_BODY_DOCUMENT => build_document_tree(&envelope),
            PRESENTATION_PLAY_BODY_CATALOGUE => build_catalogue_tree(&envelope),
            PRESENTATION_PLAY_BODY_DETAILS => build_details_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖PresentationPlayApp

//#region 🔖Manifest
fn create_presentation_app() -> App {
    App::from_builder(
        App::builder(PRESENTATION_PLAY_APP_ID, "Presentation").document(["semio", "presentation"])
            .icon_id("presentation")
            .mode("main", "Edit")
            .default_mode_id("main")
            .window_kind(PRESENTATION_PLAY_WINDOW_MAIN, "Tile editor", PRESENTATION_PLAY_BODY_MAIN)
            .default_layout(create_default_layout(
                &[PRESENTATION_PLAY_WINDOW_MAIN.into()],
                "stack",
                None,
                Some(&["Tile editor".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                PRESENTATION_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                PRESENTATION_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                PRESENTATION_PLAY_BODY_DETAILS,
            ),
    )
    .example("demo", "Demo", serde_json::to_string(&default_envelope()).unwrap())
    .program("presentation", "Presentation", "deck")
}

fn presentation_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value.get("deck").unwrap_or(value), "Presentation", 1280, 720)
}

fn register_presentation_exports() {
    semio_framework_os::register_2d_svg_png_export_handlers("presentation.deck", "presentation", presentation_document_json_to_svg);
}

fn bundle() -> PluginBundle {
    register_presentation_exports();
    PluginBundle::new("presentation", "Presentation", "0.1.0").register_app(create_presentation_app(), || {
        Box::new(PresentationPlayApp)
    })
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::plugin_exports!();
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_canvas_scene() {
        let app = PresentationPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PRESENTATION_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn seed_grid_command_adds_tiles() {
        let mut app = PresentationPlayApp;
        let mut document = app.initial_document_json();
        for op in app.handle_command("seedGrid", Some(&json!({ "rows": 2, "columns": 2 })), &document, &ViewState::default()) {
            if let Ok(value) = serde_json::from_str::<Value>(&op) {
                if value.get("op").and_then(|v| v.as_str()) == Some("setDocument") {
                    document = serde_json::to_string(&value.get("document").unwrap()).unwrap();
                }
            }
        }
        let envelope = parse_envelope(&document);
        assert_eq!(envelope.deck.tiles.len(), 4);
    }

    #[test]
    fn deck_schema_is_presentation() {
        let envelope = default_envelope();
        assert_eq!(envelope.deck.schema, PRESENTATION_DOCUMENT_SCHEMA);
    }

    #[test]
    fn source_frame_renders_as_actual_image_layer_behind_tiles() {
        let mut app = PresentationPlayApp;
        let mut document = app.initial_document_json();
        for op in app.handle_command("seedGrid", Some(&json!({ "rows": 1, "columns": 2 })), &document, &ViewState::default()) {
            if let Ok(value) = serde_json::from_str::<Value>(&op) {
                if value.get("op").and_then(|v| v.as_str()) == Some("setDocument") {
                    document = serde_json::to_string(&value.get("document").unwrap()).unwrap();
                }
            }
        }
        let envelope = parse_envelope(&document);
        let layers_json = deck_to_canvas_layers(&envelope.deck, &envelope.runtime.selected_ids);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        assert!(!envelope.deck.source.src.trim().is_empty());
        let source_layer = layers.first().expect("source layer is first (renders behind tiles)");
        assert_eq!(source_layer.get("id").and_then(|v| v.as_str()), Some("source-frame"));
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
        assert_eq!(source_layer.get("dataUrl").and_then(|v| v.as_str()), Some(envelope.deck.source.src.as_str()));
        for tile_layer in &layers[1..] {
            assert_ne!(tile_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
            assert!(tile_layer.get("dataUrl").is_none() || tile_layer.get("dataUrl") == Some(&Value::Null));
        }
    }

    #[test]
    fn document_lists_seeded_tiles() {
        let mut app = PresentationPlayApp;
        let mut document = app.initial_document_json();
        for op in app.handle_command("seedGrid", Some(&json!({ "rows": 1, "columns": 2 })), &document, &ViewState::default()) {
            if let Ok(value) = serde_json::from_str::<Value>(&op) {
                if let Some(doc) = value.get("document") {
                    document = doc.to_string();
                }
            }
        }
        let node = app.render(PRESENTATION_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("tile-r0-c0"));
    }
}
//#endregion 🧪Tests
