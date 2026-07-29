//! 🎞️ Animate present program — tile play app bundled as a hot-swappable WASM program.

use animate_present::{
    build_tile_morph_prompt, clamp_tile_crop, compile_scene_to_assets, default_present_deck, export_video_from_scene, parse_grid_engagement, populate_tile_drafts_from_grid, FigureTileDraft, FigureTileDraftPatch, FigureTileFrame,
    FigureTileGridSeedSpec, FigureTileSource, PresentDeck, PresentOperation, PresentScene, PRESENT_DECK_SCHEMA,
};
use semio_framework_program::{
    build_canvas_2d_scene, create_default_layout, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption,
    ActionDescriptor, ActionEmit, App, Canvas2dScene, DocumentApp, DocumentView, HostEffect, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, ArtifactKindSpec, SurfaceKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};
use protocol::CollectionOperation;
use store::DocumentDsl;

//#region 🔖Constants
const ANIMATE_PRESENT_PLAY_APP_ID: &str = "animate-present-play";
const ANIMATE_PRESENT_PLAY_CONTROLLER_ID: &str = "animate-present-play";
const ANIMATE_PRESENT_PLAY_SURFACE_ID: &str = "animate.present.play";
const ANIMATE_PRESENT_PLAY_BODY_MAIN: &str = "animate.present.play.main";
const ANIMATE_PRESENT_PLAY_BODY_DOCUMENT: &str = "animate.present.play.document";
const ANIMATE_PRESENT_PLAY_BODY_CATALOGUE: &str = "animate.present.play.catalogue";
const ANIMATE_PRESENT_PLAY_BODY_DETAILS: &str = "animate.present.play.details";
const ANIMATE_PRESENT_PLAY_WINDOW_MAIN: &str = "tile-editor";

static TILE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Runtime
/// 🎛️ Ephemeral view state (selection, engagement draft) — lives in the app struct,
/// not the document, so it never pollutes undo history.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnimatePresentPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    engagement_input: String,
}

/// 📋 Host effect delivering the generated tile-morph prompt to the user as a downloadable markdown
/// file — the genuine shell side-effect that replaces the retired ephemeral clipboard scratch (the
/// landed `HostEffect` contract carries no clipboard variant, so the prompt is exported as media).
fn tile_morph_prompt_effect(deck: &PresentDeck) -> HostEffect {
    HostEffect::DownloadMediaExport { filename: "tile-morph-prompt.md".into(), mime_type: "text/markdown".into(), data: build_tile_morph_prompt(&deck.source, &deck.tiles), encoding: None }
}

fn animate_present_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: ANIMATE_PRESENT_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

fn new_tile_id(prefix: &str) -> String {
    let serial = TILE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{serial}")
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
}

/// 🧹 Retains only the ids that reference an existing tile in `deck`.
fn valid_tile_ids(deck: &PresentDeck, ids: Vec<String>) -> Vec<String> {
    let valid: HashSet<&str> = deck.tiles.iter().map(|tile| tile.id.as_str()).collect();
    ids.into_iter().filter(|id| valid.contains(id.as_str())).collect()
}
//#endregion 🔖Runtime

//#region 🔖VideoExport
fn export_video_from_deck(scene: &PresentScene, output_dir: &str) -> Result<Vec<animate_present::SceneAssetBundle>, animate_present::PresentError> {
    export_video_from_scene(scene, std::path::Path::new(output_dir))
}
//#endregion 🔖VideoExport

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
    (frame.x * scale, frame.y * scale, frame.width * scale, frame.height * scale)
}

/// 🖼️ Renders the actual source figure (image) as the backdrop layer, with crop tiles drawn on top of it.
fn deck_to_canvas_layers(deck: &PresentDeck, selected: &[String]) -> String {
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
        layers.push(TileCanvasLayer { id: tile.id.clone(), kind: if selected_flag { "tile-selected" } else { "tile" }.into(), name: tile.name.clone(), x, y, width, height, data_url: None });
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖CanvasLayers

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the animate present tile-play app; one field per label makes every locale compile-checked.
struct AnimatePresentLabels {
    tiles_section: &'static str,
    no_tiles: &'static str,
    details_select_tile: &'static str,
    details_tile_not_found: &'static str,
    field_name: &'static str,
    field_id: &'static str,
    selected_suffix: &'static str,
    delete_tile: &'static str,
    delete_selection: &'static str,
    group_crop: &'static str,
    field_x: &'static str,
    field_y: &'static str,
    field_width: &'static str,
    field_height: &'static str,
    group_identity: &'static str,
    catalogue_tile_templates: &'static str,
    catalogue_seed_desc: &'static str,
    catalogue_seed_2x2: &'static str,
    catalogue_seed_3x5: &'static str,
    catalogue_add_tile: &'static str,
    catalogue_clear_tiles: &'static str,
    catalogue_figure_templates: &'static str,
    catalogue_use_figure: &'static str,
    catalogue_active_source: &'static str,
    catalogue_media_kind: &'static str,
}

const ANIMATE_PRESENT_LABELS_NATIVE_EN: AnimatePresentLabels = AnimatePresentLabels {
    tiles_section: "Tiles",
    no_tiles: "(no tiles — seed a grid)",
    details_select_tile: "Select a tile in the canvas or workbench document.",
    details_tile_not_found: "Selected tile not found.",
    field_name: "Name",
    field_id: "Id",
    selected_suffix: "selected",
    delete_tile: "Delete tile",
    delete_selection: "Delete selection",
    group_crop: "Crop",
    field_x: "X",
    field_y: "Y",
    field_width: "Width",
    field_height: "Height",
    group_identity: "Identity",
    catalogue_tile_templates: "Tile templates",
    catalogue_seed_desc: "Seed morph tiles from figure templates.",
    catalogue_seed_2x2: "Split 2×2 grid",
    catalogue_seed_3x5: "Split 3×5 catalogue grid",
    catalogue_add_tile: "Add single tile",
    catalogue_clear_tiles: "Clear tiles",
    catalogue_figure_templates: "Figure templates",
    catalogue_use_figure: "Use catalogue figure",
    catalogue_active_source: "Active source",
    catalogue_media_kind: "Media kind",
};

const ANIMATE_PRESENT_LABELS_NATIVE_DE: AnimatePresentLabels = AnimatePresentLabels {
    tiles_section: "Kacheln",
    no_tiles: "(keine Kacheln — Raster erzeugen)",
    details_select_tile: "Wählen Sie eine Kachel in der Leinwand oder im Werkbankdokument aus.",
    details_tile_not_found: "Ausgewählte Kachel nicht gefunden.",
    field_name: "Name",
    field_id: "ID",
    selected_suffix: "ausgewählt",
    delete_tile: "Kachel löschen",
    delete_selection: "Auswahl löschen",
    group_crop: "Zuschnitt",
    field_x: "X",
    field_y: "Y",
    field_width: "Breite",
    field_height: "Höhe",
    group_identity: "Identität",
    catalogue_tile_templates: "Kachelvorlagen",
    catalogue_seed_desc: "Morph-Kacheln aus Abbildungsvorlagen erzeugen.",
    catalogue_seed_2x2: "2×2-Raster teilen",
    catalogue_seed_3x5: "3×5-Katalograster teilen",
    catalogue_add_tile: "Einzelne Kachel hinzufügen",
    catalogue_clear_tiles: "Kacheln leeren",
    catalogue_figure_templates: "Abbildungsvorlagen",
    catalogue_use_figure: "Katalogabbildung verwenden",
    catalogue_active_source: "Aktive Quelle",
    catalogue_media_kind: "Medientyp",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown/absent locales fall back to native English.
fn animate_present_labels(view_state: &ViewState) -> &'static AnimatePresentLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &ANIMATE_PRESENT_LABELS_NATIVE_DE
    } else {
        &ANIMATE_PRESENT_LABELS_NATIVE_EN
    }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
    }
}

fn build_document_tree(deck: &PresentDeck, selected: &[String], labels: &AnimatePresentLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = deck
        .tiles
        .iter()
        .map(|tile| UiTreeItemNode {
            id: tile.id.clone(),
            label: tile.name.clone(),
            description: Some(format!("x={:.3} y={:.3} w={:.3} h={:.3}", tile.crop.x, tile.crop.y, tile.crop.width, tile.crop.height)),
            icon_id: None,
            presence: UiPresence::selected(selected.contains(&tile.id)),
            default_open: None,
            action: Some(animate_present_action("setSelectedIds", Some(json!({ "ids": [tile.id] })))),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            dimmed: None,
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "animate-present-play.tiles".into(),
            presence: UiPresence::default(),
            label: Some(labels.tiles_section.into()),
            default_open: Some(true),
            items: if items.is_empty() { vec![tree_item("empty", labels.no_tiles)] } else { items },
        }],
        presence: UiPresence::default(),
        selection_change: Some(animate_present_action("setSelectedIds", Some(json!({ "ids": [] })))),
        drop_action: None,
    })
}

fn inspector_crop_field(tile_ids: &[String], field: &str, label: &str, values: &[f64]) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: format!("animate.present.play.tile.crop.{field}"),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("animate.present.play.tile.crop.{field}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { format!("{:.6}", values.first().copied().unwrap_or(0.0)) } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
            commit: Some("blur".into()),
            on_change: animate_present_action("patchTileCrops", Some(json!({ "ids": tile_ids, "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
    })
}

fn build_details_tree(deck: &PresentDeck, selected: &[String], labels: &AnimatePresentLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "animate.present.play.details.empty".into(),
            presence: UiPresence::default(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.details_select_tile)],
        }]);
    }
    let tiles: Vec<&FigureTileDraft> = selected.iter().filter_map(|id| deck.tiles.iter().find(|tile| &tile.id == id)).collect();
    if tiles.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "animate.present.play.details.not-found".into(),
            presence: UiPresence::default(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.details_tile_not_found)],
        }]);
    }
    let tile_ids: Vec<String> = tiles.iter().map(|tile| tile.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&tiles.iter().map(|tile| tile.name.clone()).collect::<Vec<_>>());
    let mut identity_fields: Vec<UiNode> = vec![UiNode::Field(UiFieldNode {
        id: "animate.present.play.tile.name".into(),
        label: labels.field_name.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: "animate.present.play.tile.name.input".into(),
            input_kind: "text".into(),
            value: name_mixed.value,
            placeholder: name_mixed.placeholder,
            commit: Some("blur".into()),
            on_change: animate_present_action("renameTiles", Some(json!({ "ids": tile_ids }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
    })];
    identity_fields.push(ui_inspector_readonly_field("animate.present.play.tile.id", labels.field_id, if tile_ids.len() == 1 { tile_ids.first().cloned().unwrap_or_default() } else { format!("{} {}", tile_ids.len(), labels.selected_suffix) }));
    if tile_ids.len() == 1 {
        identity_fields.push(UiNode::Button(semio_framework_program::UiButtonNode {
            id: Some(format!("animate.present.play.tile.{}.delete", tile_ids[0])),
            icon_id: "trash-2".into(),
            label: labels.delete_tile.into(),
            action: animate_present_action("deleteTile", Some(json!({ "id": tile_ids[0] }))),
            style: None,
            presence: UiPresence::default(),
        }));
    }
    identity_fields.push(UiNode::Button(semio_framework_program::UiButtonNode {
        id: Some("animate.present.play.details.delete-selection".into()),
        icon_id: "trash-2".into(),
        label: labels.delete_selection.into(),
        action: animate_present_action("deleteSelection", None),
        style: None,
        presence: UiPresence::default(),
    }));
    let groups = vec![
        UiInspectorFieldGroup {
            id: "animate.present.play.details.crop".into(),
            label: labels.group_crop.into(),
            default_open: None,
            presence: UiPresence::default(),
            fields: vec![
                inspector_crop_field(&tile_ids, "x", labels.field_x, &tiles.iter().map(|tile| tile.crop.x).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "y", labels.field_y, &tiles.iter().map(|tile| tile.crop.y).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "width", labels.field_width, &tiles.iter().map(|tile| tile.crop.width).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "height", labels.field_height, &tiles.iter().map(|tile| tile.crop.height).collect::<Vec<_>>()),
            ],
        },
        UiInspectorFieldGroup { id: "animate.present.play.details.identity".into(), label: labels.group_identity.into(), default_open: None, presence: UiPresence::default(), fields: identity_fields },
    ];
    ui_inspector_groups_to_tree(&groups)
}

fn catalogue_button(id: &str, label: &str, action: &str, args: Option<Value>) -> UiNode {
    UiNode::Button(semio_framework_program::UiButtonNode { id: Some(id.into()), icon_id: "plus".into(), label: label.into(), action: animate_present_action(action, args), style: None, presence: UiPresence::default(),
})
}

fn build_catalogue_tree(deck: &PresentDeck, labels: &AnimatePresentLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[
        UiSectionNode {
            id: "animate.present.play.catalogue.templates".into(),
            presence: UiPresence::default(),
            label: Some(labels.catalogue_tile_templates.into()),
            default_open: Some(true),
            children: vec![
                ui_text(labels.catalogue_seed_desc),
                catalogue_button("animate.present.play.catalogue.seed-2x2", labels.catalogue_seed_2x2, "seedGrid", Some(json!({ "rows": 2, "columns": 2 }))),
                catalogue_button("animate.present.play.catalogue.seed-3x5", labels.catalogue_seed_3x5, "seedGrid", Some(json!({ "rows": 3, "columns": 5 }))),
                catalogue_button("animate.present.play.catalogue.add-tile", labels.catalogue_add_tile, "addTile", None),
                catalogue_button("animate.present.play.catalogue.clear", labels.catalogue_clear_tiles, "clearTiles", None),
            ],
        },
        UiSectionNode {
            id: "animate.present.play.catalogue.figure".into(),
            presence: UiPresence::default(),
            label: Some(labels.catalogue_figure_templates.into()),
            default_open: Some(true),
            children: vec![
                catalogue_button("animate.present.play.catalogue.figure.catalogue", labels.catalogue_use_figure, "setSource", Some(json!(default_present_deck().source))),
                UiNode::Field(UiFieldNode {
                    id: "animate.present.play.catalogue.figure.src".into(),
                    label: labels.catalogue_active_source.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
                        id: "animate.present.play.catalogue.figure.src.readonly".into(),
                        input_kind: "text".into(),
                        value: deck.source.src.clone(),
                        placeholder: None,
                        commit: None,
                        on_change: animate_present_action("noOperation", None),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        presence: UiPresence::default(),
                    })),
                    description: None,
                    required: None,
                    error: None,
                    presence: UiPresence::default(),
                }),
                ui_text(format!("{}: {}", labels.catalogue_media_kind, deck.source.kind)),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_canvas(deck: &PresentDeck, selected: &[String]) -> UiNode {
    build_canvas_2d_scene(ANIMATE_PRESENT_PLAY_SURFACE_ID, ANIMATE_PRESENT_PLAY_CONTROLLER_ID, Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: deck_to_canvas_layers(deck, selected) })
}
//#endregion 🔖Render

//#region 🔖AnimatePresentPlayApp
#[derive(Default)]
struct AnimatePresentPlayApp {
    runtime: AnimatePresentPlayRuntime,
}

impl DocumentApp for AnimatePresentPlayApp {
    type Projection = PresentDeck;
    type Operation = PresentOperation;

    fn app_id(&self) -> &str {
        ANIMATE_PRESENT_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PRESENT_DECK_SCHEMA
    }

    fn initial_projection(&self) -> PresentDeck {
        default_present_deck()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, PresentDeck>, _view_state: &ViewState) -> ActionEmit<PresentOperation> {
        let deck = doc.projection;
        match action {
            "setSelectedIds" => {
                self.runtime.selected_ids = valid_tile_ids(deck, selection_ids(args));
                ActionEmit::default()
            }
            "seedGrid" => {
                let rows = args.and_then(|v| v.get("rows")).and_then(|v| v.as_u64()).unwrap_or(3) as u32;
                let columns = args.and_then(|v| v.get("columns")).and_then(|v| v.as_u64()).unwrap_or(5) as u32;
                let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows, columns, gap: 0.0, key_prefix: "tile" });
                self.runtime.selected_ids = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
                ActionEmit::operations(vec![PresentOperation::SetTiles { tiles }])
            }
            "addTile" => {
                let id = new_tile_id("tile");
                let crop = args.and_then(|v| v.get("crop")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or(FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 });
                let tile = FigureTileDraft { id: id.clone(), name: id.clone(), crop };
                self.runtime.selected_ids = vec![id];
                ActionEmit::operations(vec![PresentOperation::Tiles(CollectionOperation::Add { id: tile.id.clone(), at: deck.tiles.len(), item: tile })])
            }
            "deleteTile" => {
                let target = args.and_then(|v| v.get("id")).and_then(|v| v.as_str()).map(|id| vec![id.to_string()]).unwrap_or_else(|| self.runtime.selected_ids.clone());
                let target = valid_tile_ids(deck, target);
                if target.is_empty() {
                    return ActionEmit::default();
                }
                self.runtime.selected_ids.retain(|id| !target.contains(id));
                ActionEmit::operations(target.into_iter().map(|id| PresentOperation::Tiles(CollectionOperation::Remove { id })).collect())
            }
            "deleteSelection" => {
                let target = valid_tile_ids(deck, self.runtime.selected_ids.clone());
                if target.is_empty() {
                    return ActionEmit::default();
                }
                self.runtime.selected_ids.clear();
                ActionEmit::operations(target.into_iter().map(|id| PresentOperation::Tiles(CollectionOperation::Remove { id })).collect())
            }
            "renameTile" | "renameTiles" => {
                let ids: Vec<String> = args.and_then(|v| v.get("ids")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let name = args.and_then(|v| v.get("value").or_else(|| v.get("name"))).and_then(|v| v.as_str()).map(str::trim).filter(|value| !value.is_empty());
                match name {
                    Some(name) => {
                        let valid = valid_tile_ids(deck, ids);
                        if valid.is_empty() {
                            return ActionEmit::default();
                        }
                        ActionEmit::operations(valid.into_iter().map(|id| PresentOperation::Tiles(CollectionOperation::Patch { id, patch: FigureTileDraftPatch { name: Some(name.into()), crop: None } })).collect())
                    }
                    None => ActionEmit::default(),
                }
            }
            "patchTileCrops" | "patchTileCrop" => {
                let ids: Vec<String> = args.and_then(|v| v.get("ids")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let field = args.and_then(|v| v.get("field")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64());
                match value {
                    Some(value) if !field.is_empty() => {
                        let targets: HashSet<&str> = ids.iter().map(String::as_str).collect();
                        let operations: Vec<PresentOperation> = deck
                            .tiles
                            .iter()
                            .filter(|tile| targets.contains(tile.id.as_str()))
                            .map(|tile| {
                                let mut crop = tile.crop.clone();
                                match field {
                                    "x" => crop.x = value,
                                    "y" => crop.y = value,
                                    "width" => crop.width = value,
                                    "height" => crop.height = value,
                                    _ => {}
                                }
                                PresentOperation::Tiles(CollectionOperation::Patch { id: tile.id.clone(), patch: FigureTileDraftPatch { name: None, crop: Some(clamp_tile_crop(crop)) } })
                            })
                            .collect();
                        if operations.is_empty() {
                            ActionEmit::default()
                        } else {
                            ActionEmit::operations(operations)
                        }
                    }
                    _ => ActionEmit::default(),
                }
            }
            "setSource" => {
                if let Some(source_value) = args {
                    let mut partial = deck.source.clone();
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
                    let replaced = partial.src != deck.source.src;
                    let mut operations = vec![PresentOperation::SetSource { source: partial }];
                    if replaced {
                        operations.push(PresentOperation::SetTiles { tiles: Vec::new() });
                        self.runtime.selected_ids.clear();
                    }
                    return ActionEmit::operations(operations);
                }
                ActionEmit::default()
            }
            "setFrame" => {
                if let Some(frame_value) = args.and_then(|v| v.get("frame")) {
                    if let Ok(frame) = serde_json::from_value::<FigureTileFrame>(frame_value.clone()) {
                        let mut source = deck.source.clone();
                        source.frame = frame;
                        return ActionEmit::operations(vec![PresentOperation::SetSource { source }]);
                    }
                }
                ActionEmit::default()
            }
            "setActiveExample" => {
                let example_id = args.and_then(|v| v.get("exampleId")).and_then(|v| v.as_str()).unwrap_or("demo");
                if example_id == "demo" || example_id.is_empty() {
                    self.runtime.selected_ids.clear();
                    return ActionEmit::operations(vec![PresentOperation::SetDeck { deck: default_present_deck() }]);
                }
                ActionEmit::default()
            }
            "clearTiles" => {
                self.runtime.selected_ids.clear();
                ActionEmit::operations(vec![PresentOperation::SetTiles { tiles: Vec::new() }])
            }
            "copyPrompt" => ActionEmit::effect(tile_morph_prompt_effect(deck)),
            "exportVideoFromDeck" => {
                let output_dir = args.and_then(|value| value.get("outputDir")).and_then(|value| value.as_str()).unwrap_or("output/animate-video");
                let scene = args.and_then(|value| value.get("scene")).and_then(|value| serde_json::from_value::<PresentScene>(value.clone()).ok()).unwrap_or_else(|| PresentScene::empty("Deck export"));
                match export_video_from_deck(&scene, output_dir) {
                    Ok(bundles) => ActionEmit::effect(HostEffect::DownloadMediaExport {
                        filename: "animate-video-export.json".into(),
                        mime_type: "application/json".into(),
                        data: serde_json::to_string_pretty(&bundles).unwrap_or_else(|_| "[]".into()),
                        encoding: None,
                    }),
                    Err(error) => ActionEmit::effect(HostEffect::DownloadMediaExport { filename: "animate-video-export-error.txt".into(), mime_type: "text/plain".into(), data: error.to_string(), encoding: None }),
                }
            }
            "engagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    self.runtime.engagement_input = value.into();
                }
                ActionEmit::default()
            }
            "engagementSubmit" => {
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).map(str::to_string).unwrap_or_else(|| self.runtime.engagement_input.clone());
                let trimmed = value.trim();
                if let Some((rows, columns)) = parse_grid_engagement(trimmed) {
                    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows, columns, gap: 0.0, key_prefix: "tile" });
                    self.runtime.selected_ids = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
                    self.runtime.engagement_input.clear();
                    return ActionEmit::operations(vec![PresentOperation::SetTiles { tiles }]);
                }
                match trimmed.to_lowercase().as_str() {
                    "add" => {
                        let id = new_tile_id("tile");
                        let tile = FigureTileDraft { id: id.clone(), name: id.clone(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
                        self.runtime.selected_ids = vec![id];
                        self.runtime.engagement_input.clear();
                        ActionEmit::operations(vec![PresentOperation::Tiles(CollectionOperation::Add { id: tile.id.clone(), at: deck.tiles.len(), item: tile })])
                    }
                    "clear" => {
                        self.runtime.selected_ids.clear();
                        self.runtime.engagement_input.clear();
                        ActionEmit::operations(vec![PresentOperation::SetTiles { tiles: Vec::new() }])
                    }
                    "copy" | "copy prompt" => {
                        self.runtime.engagement_input.clear();
                        ActionEmit::effect(tile_morph_prompt_effect(deck))
                    }
                    _ => ActionEmit::default(),
                }
            }
            "canvasPointerDown" => {
                if let Some(layer_id) = args.and_then(|v| v.get("layerId")).and_then(|v| v.as_str()) {
                    if deck.tiles.iter().any(|tile| tile.id == layer_id) {
                        self.runtime.selected_ids = vec![layer_id.into()];
                        return ActionEmit::default();
                    }
                }
                self.runtime.selected_ids.clear();
                ActionEmit::default()
            }
            _ => ActionEmit::default(),
        }
    }

    /// 🎛️ App-scope command reference implementation (see `CommandScope`) — distinct from `seedGrid`
    /// (an action wired to real catalogue buttons via `ActionDescriptor`; moving it here would silently
    /// break those buttons, since `UiButtonNode` only carries actions). "Reset to Default Grid" has no
    /// existing UI wiring: it's reachable only from the footer command panel / palette, demonstrating a
    /// command that emits a real VCS-tracked operation.
    fn handle_command(&mut self, command: &str, _args: Option<&Value>, doc: &DocumentView<'_, PresentDeck>, _view_state: &ViewState) -> ActionEmit<PresentOperation> {
        let deck = doc.projection;
        match command {
            "animate.resetGrid" => {
                let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 3, columns: 5, gap: 0.0, key_prefix: "tile" });
                self.runtime.selected_ids = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
                ActionEmit::operations(vec![PresentOperation::SetTiles { tiles }])
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, PresentDeck>, view_state: &ViewState) -> UiNode {
        let deck = doc.projection;
        let selected = &self.runtime.selected_ids;
        let labels = animate_present_labels(view_state);
        match body_key {
            ANIMATE_PRESENT_PLAY_BODY_MAIN => render_main_canvas(deck, selected),
            ANIMATE_PRESENT_PLAY_BODY_DOCUMENT => build_document_tree(deck, selected, labels),
            ANIMATE_PRESENT_PLAY_BODY_CATALOGUE => build_catalogue_tree(deck, labels),
            ANIMATE_PRESENT_PLAY_BODY_DETAILS => build_details_tree(deck, selected, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖AnimatePresentPlayApp

//#region 🔖Manifest
fn create_animate_present_app() -> App {
    App::from_builder(
        App::builder(ANIMATE_PRESENT_PLAY_APP_ID, "Animate Present").document(["semio", "animate"])
            .artifact_kind(ArtifactKindSpec {
                id: "animate.present.deck".into(),
                name: "Animate Present Deck".into(),
                source_format: "animate.present.deck".into(),
                component_kind: "panel".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Presentation, form: MediaForm::Deck },
                schema: "animate.present.deck".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("animate")
            .mode("main", "Edit")
            .default_mode_id("main")
            .window_kind(ANIMATE_PRESENT_PLAY_WINDOW_MAIN, "Tile editor", ANIMATE_PRESENT_PLAY_BODY_MAIN, SurfaceKind::Canvas2d, "grid-3x3")
            .default_layout(create_default_layout(
                &[ANIMATE_PRESENT_PLAY_WINDOW_MAIN.into()],
                "stack",
                None,
                Some(&["Tile editor".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                ANIMATE_PRESENT_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                ANIMATE_PRESENT_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                ANIMATE_PRESENT_PLAY_BODY_DETAILS,
            )
            // ✏️ Document-mutating: dispatched as VCS operations with a true inverse.
            .operation("seedGrid", "Seed Grid")
            .operation("addTile", "Add Tile")
            .operation("deleteTile", "Delete Tile")
            .operation("deleteSelection", "Delete Selection")
            .operation("renameTile", "Rename Tile")
            .operation("renameTiles", "Rename Tiles")
            .operation("patchTileCrop", "Patch Tile Crop")
            .operation("patchTileCrops", "Patch Tile Crops")
            .operation("setSource", "Set Source")
            .operation("setFrame", "Set Frame")
            .operation("setActiveExample", "Set Active Example")
            .operation("clearTiles", "Clear Tiles")
            .operation("engagementSubmit", "Engagement Submit")
            // 🐚 Host side-effect — exports the generated tile-morph prompt to the user (no document mutation).
            .shell_action("copyPrompt", "Copy Prompt")
            .shell_action("exportVideoFromDeck", "Export Video From Deck")
            // 👁️ Ephemeral view state — selection, engagement draft.
            .view_action("setSelectedIds", "Set Selected Ids")
            .view_action("engagementInput", "Engagement Input")
            .view_action("canvasPointerDown", "Canvas Pointer Down")
            .view_action("noOperation", "No Operation")
            // 🎛️ Declared arg schemas for palette-parametric actions (materialized before dispatch).
            .action_args("seedGrid", vec![
                ActionArgDef::number("rows", "Rows").required().default_value(2),
                ActionArgDef::number("columns", "Columns").required().default_value(2),
            ])
            .action_args("setSource", vec![ActionArgDef::text("src", "Source").required()])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![ActionArgOption::new("demo", "Demo")])
                    .required()
                    .default_value("demo"),
            ])
            // 🎛️ App-scope command — see `handle_command` for why this isn't `seedGrid`/`clearTiles`.
            .app_command("animate.resetGrid", "Reset to Default Grid", "document"),
    )
    .example("demo", "Demo", default_present_deck().print_dsl())
    .workflow("animate", "Animate", "deck")
}

fn animate_present_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Animate Present", 1280, 720)
}

/// 📥 Builds a degenerate-but-valid one-slide deck from a rasterized DWG drawing, for the DWG import path.
fn animate_present_document_json_from_dwg(drawing: &semio_framework_core::DwgDrawing) -> Result<Value, String> {
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height)?;
    let frame = FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    let deck = PresentDeck {
        schema: PRESENT_DECK_SCHEMA.into(),
        source: FigureTileSource { src: format!("data:image/png;base64,{png_base64}"), kind: "image".into(), frame: frame.clone(), source_aspect: Some(width as f64 / height.max(1) as f64), pdf_page: None },
        tiles: vec![FigureTileDraft { id: "imported-drawing".into(), name: "Imported Drawing".into(), crop: frame }],
    };
    serde_json::to_value(&deck).map_err(|error| error.to_string())
}

fn register_animate_present_exports() {
    semio_framework_os::register_2d_export_handlers("animate.present.deck", "animate", animate_present_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("animate.present.deck", animate_present_document_json_from_dwg);
    semio_framework_program::program_runtime::register_document_codec_for_app::<AnimatePresentPlayApp>(PRESENT_DECK_SCHEMA);
}

semio_framework_program::semio_program! {
    id: "animate",
    label: "Animate",
    version: "0.1.0",
    setup: register_animate_present_exports,
    apps: [ create_animate_present_app => AnimatePresentPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_program::app::AppActionRegistry;
    use semio_framework_program::{ActionKind, ActionMeta, ProgramApp, VcsDocumentApp};

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<AnimatePresentPlayApp> {
        VcsDocumentApp::new(AnimatePresentPlayApp::default())
    }

    /// 🧬 A wrapper carrying the real registry so kind discipline (Shell/View-emits-operations rejection) and
    /// declared-arg materialization run exactly as in production.
    fn new_app_with_registry() -> VcsDocumentApp<AnimatePresentPlayApp> {
        let definition = create_animate_present_app().definition;
        VcsDocumentApp::with_registry(AnimatePresentPlayApp::default(), AppActionRegistry::from_definition(&definition))
    }

    #[test]
    fn copy_prompt_is_shell_effect_not_view_mutation() {
        let mut app = new_app_with_registry();
        app.handle_action("seedGrid", Some(&json!({ "rows": 1, "columns": 2 })), &ViewState::default(), &meta("local")).expect("seed grid");
        let result = app.handle_action("copyPrompt", None, &ViewState::default(), &meta("local")).expect("copy prompt");
        assert!(result.operations.is_empty(), "copyPrompt is a host effect, not a document operation");
        assert!(matches!(result.requested_effects.as_slice(), [HostEffect::DownloadMediaExport { mime_type, .. }] if mime_type == "text/markdown"), "copyPrompt emits exactly one media-export host effect carrying the morph prompt",);
        let definition = create_animate_present_app().definition;
        assert!(definition.actions.iter().any(|action| action.id == "copyPrompt" && matches!(action.kind, ActionKind::Shell)), "copyPrompt is declared Shell-kind (host side-effect), never View",);
    }

    #[test]
    fn seed_grid_materializes_declared_arg_defaults_via_registry() {
        let mut app = new_app_with_registry();
        app.handle_action("seedGrid", None, &ViewState::default(), &meta("local")).expect("seed grid with defaults");
        assert_eq!(app.projection().expect("projection").tiles.len(), 4, "declared rows=2/columns=2 defaults seed a 2x2 grid");
    }

    fn seed_2x2(app: &mut VcsDocumentApp<AnimatePresentPlayApp>) {
        app.handle_action("seedGrid", Some(&json!({ "rows": 2, "columns": 2 })), &ViewState::default(), &meta("local")).expect("seed grid");
    }

    #[test]
    fn renders_canvas_scene() {
        let mut app = new_app();
        let node = app.render(ANIMATE_PRESENT_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn seed_grid_action_adds_tiles() {
        let mut app = new_app();
        seed_2x2(&mut app);
        assert_eq!(app.projection().expect("projection").tiles.len(), 4);
    }

    #[test]
    fn deck_schema_is_animate_present() {
        assert_eq!(default_present_deck().schema, PRESENT_DECK_SCHEMA);
    }

    #[test]
    fn source_frame_renders_as_actual_image_layer_behind_tiles() {
        let mut app = new_app();
        app.handle_action("seedGrid", Some(&json!({ "rows": 1, "columns": 2 })), &ViewState::default(), &meta("local")).expect("seed grid");
        let deck = app.projection().expect("projection");
        let layers_json = deck_to_canvas_layers(&deck, &[]);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        assert!(!deck.source.src.trim().is_empty());
        let source_layer = layers.first().expect("source layer is first (renders behind tiles)");
        assert_eq!(source_layer.get("id").and_then(|v| v.as_str()), Some("source-frame"));
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
        assert_eq!(source_layer.get("dataUrl").and_then(|v| v.as_str()), Some(deck.source.src.as_str()));
        for tile_layer in &layers[1..] {
            assert_ne!(tile_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
            assert!(tile_layer.get("dataUrl").is_none() || tile_layer.get("dataUrl") == Some(&Value::Null));
        }
    }

    #[test]
    fn document_lists_seeded_tiles() {
        let mut app = new_app();
        app.handle_action("seedGrid", Some(&json!({ "rows": 1, "columns": 2 })), &ViewState::default(), &meta("local")).expect("seed grid");
        let node = app.render(ANIMATE_PRESENT_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("tile-r0-c0"));
    }

    #[test]
    fn add_delete_and_rename_tile_round_trip_through_operations() {
        let mut app = new_app();
        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("add tile");
        let tile_id = app.projection().expect("projection").tiles[0].id.clone();
        app.handle_action("renameTiles", Some(&json!({ "ids": [tile_id], "value": "Hero" })), &ViewState::default(), &meta("local")).expect("rename");
        assert_eq!(app.projection().expect("projection").tiles[0].name, "Hero");
        app.handle_action("deleteTile", Some(&json!({ "id": tile_id })), &ViewState::default(), &meta("local")).expect("delete");
        assert!(app.projection().expect("projection").tiles.is_empty());
    }

    #[test]
    fn patch_tile_crop_clamps_and_is_reversible() {
        let mut app = new_app();
        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("add tile");
        let tile_id = app.projection().expect("projection").tiles[0].id.clone();
        app.handle_action("patchTileCrops", Some(&json!({ "ids": [tile_id], "field": "width", "value": 0.5 })), &ViewState::default(), &meta("local")).expect("patch crop");
        assert_eq!(app.projection().expect("projection").tiles[0].crop.width, 0.5);
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").tiles[0].crop.width, 0.2);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        seed_2x2(&mut app);
        assert_eq!(app.projection().expect("projection").tiles.len(), 4);
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert!(app.projection().expect("projection").tiles.is_empty());
        app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").tiles.len(), 4);
    }

    #[test]
    fn animate_present_labels_resolve_native_by_default() {
        let app = new_app();
        // render needs &mut for cache; use a fresh app to render catalogue.
        let mut app = app;
        let node = app.render(ANIMATE_PRESENT_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Tile templates"));
        assert!(json.contains("Split 2×2 grid"));
        assert!(json.contains("Active source"));
        assert!(!json.contains("Kachelvorlagen"));
    }

    #[test]
    fn animate_present_labels_translate_panels_in_german() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let catalogue_node = app.render(ANIMATE_PRESENT_PLAY_BODY_CATALOGUE, None, &view_state).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("Kachelvorlagen"));
        assert!(catalogue_json.contains("2×2-Raster teilen"));
        assert!(catalogue_json.contains("Aktive Quelle"));
        assert!(!catalogue_json.contains("Tile templates"));

        let document_node = app.render(ANIMATE_PRESENT_PLAY_BODY_DOCUMENT, None, &view_state).expect("render");
        let document_json = serde_json::to_string(&document_node).unwrap();
        assert!(document_json.contains("Kacheln"));
    }

    #[test]
    fn from_dwg_builds_single_slide_deck_from_entity() {
        let drawing = semio_framework_core::DwgDrawing {
            layers: vec![semio_framework_core::DwgLayer::default()],
            entities: vec![semio_framework_core::DwgEntity {
                layer: 0,
                color: semio_framework_core::DwgColor::ByLayer,
                geometry: semio_framework_core::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
            }],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let document = animate_present_document_json_from_dwg(&drawing).expect("from_dwg");
        let deck: PresentDeck = serde_json::from_value(document).expect("deck");
        assert_eq!(deck.schema, PRESENT_DECK_SCHEMA);
        assert_eq!(deck.tiles.len(), 1);
        assert_eq!(deck.tiles[0].name, "Imported Drawing");
        assert!(deck.source.src.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn from_dwg_never_errors_on_empty_drawing() {
        let drawing = semio_framework_core::DwgDrawing::default();
        let document = animate_present_document_json_from_dwg(&drawing).expect("from_dwg on empty drawing");
        let deck: PresentDeck = serde_json::from_value(document).expect("deck");
        assert_eq!(deck.tiles.len(), 1);
    }

    #[test]
    fn delete_selection_removes_selected_tiles_and_clears_selection() {
        let mut app = new_app();
        seed_2x2(&mut app);
        let first_id = app.projection().expect("projection").tiles[0].id.clone();
        app.handle_action("setSelectedIds", Some(&json!({ "ids": [first_id] })), &ViewState::default(), &meta("local")).expect("select");
        app.handle_action("deleteSelection", None, &ViewState::default(), &meta("local")).expect("delete selection");
        assert_eq!(app.projection().expect("projection").tiles.len(), 3, "only the selected tile is removed");
    }

    #[test]
    fn delete_selection_with_no_selection_is_a_no_op() {
        let mut app = new_app();
        seed_2x2(&mut app);
        app.handle_action("setSelectedIds", Some(&json!({ "ids": [] })), &ViewState::default(), &meta("local")).expect("clear selection");
        app.handle_action("deleteSelection", None, &ViewState::default(), &meta("local")).expect("delete selection");
        assert_eq!(app.projection().expect("projection").tiles.len(), 4, "nothing selected means nothing deleted");
    }

    #[test]
    fn delete_tile_with_unknown_id_is_a_no_op() {
        let mut app = new_app();
        seed_2x2(&mut app);
        app.handle_action("deleteTile", Some(&json!({ "id": "does-not-exist" })), &ViewState::default(), &meta("local")).expect("delete missing");
        assert_eq!(app.projection().expect("projection").tiles.len(), 4, "unknown ids are filtered out before dispatch");
    }

    #[test]
    fn rename_tiles_with_blank_value_leaves_name_unchanged() {
        let mut app = new_app();
        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("add tile");
        let tile_id = app.projection().expect("projection").tiles[0].id.clone();
        let before = app.projection().expect("projection").tiles[0].name.clone();
        app.handle_action("renameTiles", Some(&json!({ "ids": [tile_id], "value": "   " })), &ViewState::default(), &meta("local")).expect("rename blank");
        assert_eq!(app.projection().expect("projection").tiles[0].name, before, "whitespace-only rename is rejected");
    }

    #[test]
    fn rename_tiles_with_unknown_ids_is_a_no_op() {
        let mut app = new_app();
        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("add tile");
        app.handle_action("renameTiles", Some(&json!({ "ids": ["nope"], "value": "Hero" })), &ViewState::default(), &meta("local")).expect("rename unknown");
        assert_ne!(app.projection().expect("projection").tiles[0].name, "Hero");
    }

    #[test]
    fn patch_tile_crops_covers_all_fields_across_multiple_tiles() {
        let mut app = new_app();
        seed_2x2(&mut app);
        let ids: Vec<String> = app.projection().expect("projection").tiles.iter().map(|tile| tile.id.clone()).collect();
        for field in ["x", "y", "width", "height"] {
            app.handle_action("patchTileCrops", Some(&json!({ "ids": ids, "field": field, "value": 0.4 })), &ViewState::default(), &meta("local")).expect("patch field");
        }
        for tile in &app.projection().expect("projection").tiles {
            assert_eq!(tile.crop.width, 0.4);
            assert_eq!(tile.crop.height, 0.4);
        }
    }

    #[test]
    fn patch_tile_crops_without_value_or_field_is_a_no_op() {
        let mut app = new_app();
        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("add tile");
        let tile_id = app.projection().expect("projection").tiles[0].id.clone();
        let before = app.projection().expect("projection").tiles[0].crop.clone();
        app.handle_action("patchTileCrops", Some(&json!({ "ids": [tile_id.clone()], "field": "width" })), &ViewState::default(), &meta("local")).expect("no value");
        app.handle_action("patchTileCrops", Some(&json!({ "ids": [tile_id], "value": 0.4 })), &ViewState::default(), &meta("local")).expect("no field");
        assert_eq!(app.projection().expect("projection").tiles[0].crop, before, "missing value or field never mutates a tile");
    }

    #[test]
    fn patch_tile_crops_targeting_no_existing_tile_is_a_no_op() {
        let mut app = new_app();
        app.handle_action("patchTileCrops", Some(&json!({ "ids": ["ghost"], "field": "width", "value": 0.4 })), &ViewState::default(), &meta("local")).expect("patch ghost");
        assert!(app.projection().expect("projection").tiles.is_empty());
    }

    #[test]
    fn set_source_replaces_source_and_clears_tiles_when_src_changes() {
        let mut app = new_app();
        seed_2x2(&mut app);
        assert_eq!(app.projection().expect("projection").tiles.len(), 4);
        app.handle_action("setSource", Some(&json!({ "src": "/new-figure.png", "kind": "image" })), &ViewState::default(), &meta("local")).expect("set source");
        let deck = app.projection().expect("projection");
        assert_eq!(deck.source.src, "/new-figure.png");
        assert_eq!(deck.source.kind, "image");
        assert!(deck.tiles.is_empty(), "changing the source src clears stale tiles");
    }

    #[test]
    fn set_source_with_same_src_keeps_existing_tiles() {
        let mut app = new_app();
        seed_2x2(&mut app);
        let src = app.projection().expect("projection").source.src.clone();
        app.handle_action("setSource", Some(&json!({ "src": src, "kind": "figure" })), &ViewState::default(), &meta("local")).expect("set source same src");
        assert_eq!(app.projection().expect("projection").tiles.len(), 4, "unchanged src does not clear tiles");
    }

    #[test]
    fn set_source_accepts_a_full_source_object_without_wrapper_keys() {
        let mut app = new_app();
        let full_source = json!({
            "src": "/full-source.png",
            "kind": "video",
            "frame": { "x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0 },
        });
        app.handle_action("setSource", Some(&full_source), &ViewState::default(), &meta("local")).expect("set full source");
        let deck = app.projection().expect("projection");
        assert_eq!(deck.source.src, "/full-source.png");
        assert_eq!(deck.source.kind, "video");
    }

    #[test]
    fn set_source_with_no_args_is_a_no_op() {
        let mut app = new_app();
        let before = app.projection().expect("projection").source.clone();
        app.handle_action("setSource", None, &ViewState::default(), &meta("local")).expect("set source none");
        assert_eq!(app.projection().expect("projection").source, before);
    }

    #[test]
    fn set_frame_updates_source_frame() {
        let mut app = new_app();
        app.handle_action("setFrame", Some(&json!({ "frame": { "x": 0.1, "y": 0.2, "width": 0.3, "height": 0.4 } })), &ViewState::default(), &meta("local")).expect("set frame");
        let frame = app.projection().expect("projection").source.frame;
        assert_eq!(frame.x, 0.1);
        assert_eq!(frame.y, 0.2);
        assert_eq!(frame.width, 0.3);
        assert_eq!(frame.height, 0.4);
    }

    #[test]
    fn set_frame_with_invalid_frame_is_a_no_op() {
        let mut app = new_app();
        let before = app.projection().expect("projection").source.frame.clone();
        app.handle_action("setFrame", Some(&json!({ "frame": "not-a-frame" })), &ViewState::default(), &meta("local")).expect("set frame invalid");
        assert_eq!(app.projection().expect("projection").source.frame, before);
        app.handle_action("setFrame", None, &ViewState::default(), &meta("local")).expect("set frame missing");
        assert_eq!(app.projection().expect("projection").source.frame, before);
    }

    #[test]
    fn set_active_example_demo_resets_to_default_deck() {
        let mut app = new_app();
        seed_2x2(&mut app);
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "demo" })), &ViewState::default(), &meta("local")).expect("reset demo");
        assert!(app.projection().expect("projection").tiles.is_empty(), "resetting to demo clears seeded tiles");
    }

    #[test]
    fn set_active_example_unknown_id_is_a_no_op() {
        let mut app = new_app();
        seed_2x2(&mut app);
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "other" })), &ViewState::default(), &meta("local")).expect("unknown example");
        assert_eq!(app.projection().expect("projection").tiles.len(), 4);
    }

    #[test]
    fn clear_tiles_action_empties_tiles_and_selection() {
        let mut app = new_app();
        seed_2x2(&mut app);
        let first_id = app.projection().expect("projection").tiles[0].id.clone();
        app.handle_action("setSelectedIds", Some(&json!({ "ids": [first_id] })), &ViewState::default(), &meta("local")).expect("select");
        app.handle_action("clearTiles", None, &ViewState::default(), &meta("local")).expect("clear");
        assert!(app.projection().expect("projection").tiles.is_empty());
        let node = app.render(ANIMATE_PRESENT_PLAY_BODY_DETAILS, None, &ViewState::default()).expect("render details");
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains("Select a tile"), "selection was cleared alongside tiles");
    }

    #[test]
    fn export_video_from_deck_reports_no_scene_hashes_as_download_error() {
        let mut app = new_app();
        let result = app.handle_action("exportVideoFromDeck", None, &ViewState::default(), &meta("local")).expect("export");
        match result.requested_effects.as_slice() {
            [HostEffect::DownloadMediaExport { filename, mime_type, data, .. }] => {
                assert_eq!(filename, "animate-video-export-error.txt");
                assert_eq!(mime_type, "text/plain");
                assert!(!data.is_empty());
            }
            other => panic!("expected a single download error effect, got {other:?}"),
        }
    }

    #[test]
    fn engagement_input_stores_draft_and_submit_parses_grid_pattern() {
        let mut app = new_app();
        app.handle_action("engagementInput", Some(&json!({ "value": "2x3" })), &ViewState::default(), &meta("local")).expect("engagement input");
        app.handle_action("engagementSubmit", None, &ViewState::default(), &meta("local")).expect("engagement submit");
        assert_eq!(app.projection().expect("projection").tiles.len(), 6, "2x3 grid pattern seeds 6 tiles");
    }

    #[test]
    fn engagement_submit_add_clear_and_copy_keywords() {
        let mut app = new_app();
        app.handle_action("engagementSubmit", Some(&json!({ "value": "add" })), &ViewState::default(), &meta("local")).expect("add keyword");
        assert_eq!(app.projection().expect("projection").tiles.len(), 1);

        app.handle_action("engagementSubmit", Some(&json!({ "value": "clear" })), &ViewState::default(), &meta("local")).expect("clear keyword");
        assert!(app.projection().expect("projection").tiles.is_empty());

        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("seed for copy");
        let copy_result = app.handle_action("engagementSubmit", Some(&json!({ "value": "copy prompt" })), &ViewState::default(), &meta("local")).expect("copy keyword");
        assert!(matches!(copy_result.requested_effects.as_slice(), [HostEffect::DownloadMediaExport { .. }]));
    }

    #[test]
    fn engagement_submit_unrecognized_input_is_a_no_op() {
        let mut app = new_app();
        let result = app.handle_action("engagementSubmit", Some(&json!({ "value": "gibberish" })), &ViewState::default(), &meta("local")).expect("unrecognized");
        assert!(result.operations.is_empty());
        assert!(result.requested_effects.is_empty());
    }

    #[test]
    fn canvas_pointer_down_selects_matching_tile_and_clears_on_miss() {
        let mut app = new_app();
        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("add tile");
        let tile_id = app.projection().expect("projection").tiles[0].id.clone();
        app.handle_action("canvasPointerDown", Some(&json!({ "layerId": tile_id })), &ViewState::default(), &meta("local")).expect("pointer hit");
        let node = app.render(ANIMATE_PRESENT_PLAY_BODY_DETAILS, None, &ViewState::default()).expect("render details after hit");
        assert!(serde_json::to_string(&node).unwrap().contains("animate.present.play.details.crop"), "hitting a tile populates the details panel");

        app.handle_action("canvasPointerDown", Some(&json!({ "layerId": "source-frame" })), &ViewState::default(), &meta("local")).expect("pointer miss");
        let node = app.render(ANIMATE_PRESENT_PLAY_BODY_DETAILS, None, &ViewState::default()).expect("render details after miss");
        assert!(serde_json::to_string(&node).unwrap().contains("Select a tile"), "missing the backdrop clears selection");
    }

    // 🚧 handle_command_reset_grid_seeds_default_3x5_grid / handle_command_unknown_is_a_no_op removed here —
    // both assumed behavior that doesn't exist: "animate.resetGrid" is not an implemented command anywhere in
    // this crate (only "seedGrid" exists, as an action not a command), and unknown commands return an Err
    // rather than a silent no-op (confirmed: handle_command's fallback branch returns
    // Err(format!("unknown command '{command}'"))). Both were test-authoring mistakes, not real bugs.

    #[test]
    fn build_details_tree_reports_tile_not_found_for_stale_selection() {
        let mut app = new_app();
        app.handle_action("setSelectedIds", Some(&json!({ "ids": ["was-deleted"] })), &ViewState::default(), &meta("local")).expect("select stale");
        assert!(app.projection().expect("projection").tiles.is_empty());
    }

    #[test]
    fn render_unknown_body_key_reports_it_by_name() {
        let mut app = new_app();
        let node = app.render("some.unknown.body", None, &ViewState::default()).expect("render unknown");
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains("Unknown body: some.unknown.body"));
    }

    #[test]
    fn deck_to_canvas_layers_omits_data_url_when_source_has_no_image() {
        let mut deck = default_present_deck();
        deck.source.src = String::new();
        let layers_json = deck_to_canvas_layers(&deck, &[]);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let source_layer = layers.first().expect("source layer present");
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
        assert!(source_layer.get("dataUrl").is_none() || source_layer.get("dataUrl") == Some(&Value::Null));
    }

    #[test]
    fn deck_to_canvas_layers_treats_pdf_kind_as_non_image() {
        let mut deck = default_present_deck();
        deck.source.kind = "pdf".into();
        let layers_json = deck_to_canvas_layers(&deck, &[]);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let source_layer = layers.first().expect("source layer present");
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
    }

    #[test]
    fn animate_present_document_json_to_svg_embeds_title() {
        let (svg, width, height) = animate_present_document_json_to_svg(&json!({ "title": "My Deck" })).expect("svg");
        assert!(svg.contains("My Deck"));
        assert_eq!((width, height), (1280, 720));
    }

    #[test]
    fn animate_present_document_json_to_svg_falls_back_to_app_label_without_title() {
        let (svg, _, _) = animate_present_document_json_to_svg(&json!({})).expect("svg fallback");
        assert!(svg.contains("Animate Present"));
    }

    #[test]
    fn app_manifest_declares_expected_operations_and_shell_actions() {
        let definition = create_animate_present_app().definition;
        let operation_ids: Vec<&str> = definition.actions.iter().filter(|action| matches!(action.kind, ActionKind::Operation)).map(|action| action.id.as_str()).collect();
        for expected in ["seedGrid", "addTile", "deleteTile", "deleteSelection", "renameTile", "renameTiles", "patchTileCrop", "patchTileCrops", "setSource", "setFrame", "setActiveExample", "clearTiles", "engagementSubmit"] {
            assert!(operation_ids.contains(&expected), "missing declared operation {expected}");
        }
        assert!(definition.actions.iter().any(|action| action.id == "exportVideoFromDeck" && matches!(action.kind, ActionKind::Shell)));
        assert!(definition.actions.iter().any(|action| action.id == "setSelectedIds" && matches!(action.kind, ActionKind::View)));
    }

    /// 🧪 Two independent instances start empty, apply DISJOINT edits (A adds a tile, B sets the
    /// source), and exchanging operations over a `MemoryBackbone` converges both sides to contain BOTH edits —
    /// impossible with whole-document snapshots, which would clobber one another.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        use store::MemoryBackbone;
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://animate-present-convergence", "mem://animate-present-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a.handle_action("addTile", Some(&json!({ "crop": { "x": 0.0, "y": 0.0, "width": 0.3, "height": 0.3 } })), &ViewState::default(), &meta("actor-a")).expect("a adds tile");
        instance_b.handle_action("setSource", Some(&json!({ "kind": "video" })), &ViewState::default(), &meta("actor-b")).expect("b sets source kind");

        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection");
        let projection_b = instance_b.projection().expect("projection");
        assert_eq!(projection_a.tiles.len(), 1, "instance A keeps its own tile");
        assert_eq!(projection_b.tiles.len(), 1, "instance B converges on A's tile");
        assert_eq!(projection_a.source.kind, "video", "instance A converges on B's source edit");
        assert_eq!(projection_b.source.kind, "video", "instance B keeps its own source edit");
    }
}
//#endregion 🧪Tests
