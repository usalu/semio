//! 📽️ Presentation plugin — tile play app bundled as a hot-swappable WASM component.

use presentation_deck::{
    build_tile_morph_prompt, clamp_tile_crop, default_presentation_deck, parse_grid_engagement,
    populate_tile_drafts_from_grid, FigureTileDraft, FigureTileDraftPatch, FigureTileFrame,
    FigureTileGridSeedSpec, FigureTileSource, PresentationDeck, PresentationOp, PRESENTATION_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{SurfaceKind, PanelGroup, ActionArgDef, ActionArgOption, HostEffect,
    build_canvas_2d_scene, create_default_layout, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionEmit, App,
    Canvas2dScene, ActionDescriptor, DocumentApp, DocumentView, UiFieldNode, UiInputNode, UiInspectorFieldGroup,
    UiNode, UiSectionNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use vcs::CollectionOp;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};

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

//#region 🔖Runtime
/// 🎛️ Ephemeral view state (selection, engagement draft) — lives in the app struct,
/// not the document, so it never pollutes undo history.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PresentationPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    engagement_input: String,
}

/// 📋 Host effect delivering the generated tile-morph prompt to the user as a downloadable markdown
/// file — the genuine shell side-effect that replaces the retired ephemeral clipboard scratch (the
/// landed `HostEffect` contract carries no clipboard variant, so the prompt is exported as media).
fn tile_morph_prompt_effect(deck: &PresentationDeck) -> HostEffect {
    HostEffect::DownloadMediaExport {
        filename: "tile-morph-prompt.md".into(),
        mime_type: "text/markdown".into(),
        data: build_tile_morph_prompt(&deck.source, &deck.tiles),
        encoding: None,
    }
}

fn presentation_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: PRESENTATION_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args,
    }
}

fn new_tile_id(prefix: &str) -> String {
    let serial = TILE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{serial}")
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}

/// 🧹 Retains only the ids that reference an existing tile in `deck`.
fn valid_tile_ids(deck: &PresentationDeck, ids: Vec<String>) -> Vec<String> {
    let valid: HashSet<&str> = deck.tiles.iter().map(|tile| tile.id.as_str()).collect();
    ids.into_iter().filter(|id| valid.contains(id.as_str())).collect()
}
//#endregion 🔖Runtime

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

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the presentation tile-play app; one field per label makes every locale compile-checked.
struct PresentationLabels {
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

const PRESENTATION_LABELS_NATIVE_EN: PresentationLabels = PresentationLabels {
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

const PRESENTATION_LABELS_NATIVE_DE: PresentationLabels = PresentationLabels {
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
fn presentation_labels(view_state: &ViewState) -> &'static PresentationLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &PRESENTATION_LABELS_NATIVE_DE
    } else {
        &PRESENTATION_LABELS_NATIVE_EN
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
        loading: None,
    }
}

fn build_document_tree(deck: &PresentationDeck, selected: &[String], labels: &PresentationLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = deck
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
            selected: Some(selected.contains(&tile.id)),
            default_open: None,
            action: Some(presentation_action("setSelectedIds", Some(json!({ "ids": [tile.id] })))),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
            loading: None,
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "presentation-tile-play.tiles".into(),
            loading: None,
            label: Some(labels.tiles_section.into()),
            default_open: Some(true),
            items: if items.is_empty() {
                vec![tree_item("empty", labels.no_tiles)]
            } else {
                items
            },
        }],
        selected_ids: Some(selected.to_vec()),
        highlighted_ids: None,
        selection_change: Some(presentation_action("setSelectedIds", Some(json!({ "ids": [] })))),
        drop_action: None,
        loading: None,
    })
}

fn inspector_crop_field(tile_ids: &[String], field: &str, label: &str, values: &[f64]) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: format!("presentation.play.tile.crop.{field}"),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
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
            on_change: presentation_action(
                "patchTileCrops",
                Some(json!({ "ids": tile_ids, "field": field })),
            ),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn build_details_tree(deck: &PresentationDeck, selected: &[String], labels: &PresentationLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "presentation.play.details.empty".into(),
            loading: None,
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.details_select_tile)],
        }]);
    }
    let tiles: Vec<&FigureTileDraft> = selected
        .iter()
        .filter_map(|id| deck.tiles.iter().find(|tile| &tile.id == id))
        .collect();
    if tiles.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "presentation.play.details.not-found".into(),
            loading: None,
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.details_tile_not_found)],
        }]);
    }
    let tile_ids: Vec<String> = tiles.iter().map(|tile| tile.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&tiles.iter().map(|tile| tile.name.clone()).collect::<Vec<_>>());
    let mut identity_fields: Vec<UiNode> = vec![UiNode::Field(UiFieldNode {
        id: "presentation.play.tile.name".into(),
        label: labels.field_name.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: "presentation.play.tile.name.input".into(),
            input_kind: "text".into(),
            value: name_mixed.value,
            placeholder: name_mixed.placeholder,
            commit: Some("blur".into()),
            on_change: presentation_action("renameTiles", Some(json!({ "ids": tile_ids }))),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })];
    identity_fields.push(ui_inspector_readonly_field(
        "presentation.play.tile.id",
        labels.field_id,
        if tile_ids.len() == 1 {
            tile_ids.first().cloned().unwrap_or_default()
        } else {
            format!("{} {}", tile_ids.len(), labels.selected_suffix)
        },
    ));
    if tile_ids.len() == 1 {
        identity_fields.push(UiNode::Button(semio_framework_plugin::UiButtonNode {
            id: Some(format!("presentation.play.tile.{}.delete", tile_ids[0])),
            icon_id: "trash-2".into(),
            label: labels.delete_tile.into(),
            action: presentation_action("deleteTile", Some(json!({ "id": tile_ids[0] }))),
            style: None,
            disabled: None,
            loading: None,
        }));
    }
    identity_fields.push(UiNode::Button(semio_framework_plugin::UiButtonNode {
        id: Some("presentation.play.details.delete-selection".into()),
        icon_id: "trash-2".into(),
        label: labels.delete_selection.into(),
        action: presentation_action("deleteSelection", None),
        style: None,
        disabled: None,
        loading: None,
    }));
    let groups = vec![
        UiInspectorFieldGroup {
            id: "presentation.play.details.crop".into(),
            label: labels.group_crop.into(),
            default_open: None,
            fields: vec![
                inspector_crop_field(&tile_ids, "x", labels.field_x, &tiles.iter().map(|tile| tile.crop.x).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "y", labels.field_y, &tiles.iter().map(|tile| tile.crop.y).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "width", labels.field_width, &tiles.iter().map(|tile| tile.crop.width).collect::<Vec<_>>()),
                inspector_crop_field(
                    &tile_ids,
                    "height",
                    labels.field_height,
                    &tiles.iter().map(|tile| tile.crop.height).collect::<Vec<_>>(),
                ),
            ],
        },
        UiInspectorFieldGroup {
            id: "presentation.play.details.identity".into(),
            label: labels.group_identity.into(),
            default_open: None,
            fields: identity_fields,
        },
    ];
    ui_inspector_groups_to_tree(&groups)
}

fn catalogue_button(id: &str, label: &str, action: &str, args: Option<Value>) -> UiNode {
    UiNode::Button(semio_framework_plugin::UiButtonNode {
        id: Some(id.into()),
        icon_id: "plus".into(),
        label: label.into(),
        action: presentation_action(action, args),
        style: None,
        disabled: None,
        loading: None,
    })
}

fn build_catalogue_tree(deck: &PresentationDeck, labels: &PresentationLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[
        UiSectionNode {
            id: "presentation.play.catalogue.templates".into(),
            loading: None,
            label: Some(labels.catalogue_tile_templates.into()),
            default_open: Some(true),
            children: vec![
                ui_text(labels.catalogue_seed_desc),
                catalogue_button(
                    "presentation.play.catalogue.seed-2x2",
                    labels.catalogue_seed_2x2,
                    "seedGrid",
                    Some(json!({ "rows": 2, "columns": 2 })),
                ),
                catalogue_button(
                    "presentation.play.catalogue.seed-3x5",
                    labels.catalogue_seed_3x5,
                    "seedGrid",
                    Some(json!({ "rows": 3, "columns": 5 })),
                ),
                catalogue_button("presentation.play.catalogue.add-tile", labels.catalogue_add_tile, "addTile", None),
                catalogue_button("presentation.play.catalogue.clear", labels.catalogue_clear_tiles, "clearTiles", None),
            ],
        },
        UiSectionNode {
            id: "presentation.play.catalogue.figure".into(),
            loading: None,
            label: Some(labels.catalogue_figure_templates.into()),
            default_open: Some(true),
            children: vec![
                catalogue_button(
                    "presentation.play.catalogue.figure.catalogue",
                    labels.catalogue_use_figure,
                    "setSource",
                    Some(json!(default_presentation_deck().source)),
                ),
                UiNode::Field(UiFieldNode {
                    id: "presentation.play.catalogue.figure.src".into(),
                    label: labels.catalogue_active_source.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
                        id: "presentation.play.catalogue.figure.src.readonly".into(),
                        input_kind: "text".into(),
                        value: deck.source.src.clone(),
                        placeholder: None,
                        commit: None,
                        on_change: presentation_action("noop", None),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                ui_text(format!("{}: {}", labels.catalogue_media_kind, deck.source.kind)),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_canvas(deck: &PresentationDeck, selected: &[String]) -> UiNode {
    build_canvas_2d_scene(
        PRESENTATION_PLAY_SURFACE_ID,
        PRESENTATION_PLAY_CONTROLLER_ID,
        Canvas2dScene {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            layers_json: deck_to_canvas_layers(deck, selected),
        },
    )
}
//#endregion 🔖Render

//#region 🔖PresentationPlayApp
#[derive(Default)]
struct PresentationPlayApp {
    runtime: PresentationPlayRuntime,
}

impl DocumentApp for PresentationPlayApp {
    type Projection = PresentationDeck;
    type Op = PresentationOp;

    fn app_id(&self) -> &str {
        PRESENTATION_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PRESENTATION_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> PresentationDeck {
        default_presentation_deck()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, PresentationDeck>,
        _view_state: &ViewState,
    ) -> ActionEmit<PresentationOp> {
        let deck = doc.projection;
        match action {
            "setSelectedIds" => {
                self.runtime.selected_ids = valid_tile_ids(deck, selection_ids(args));
                ActionEmit::default()
            }
            "seedGrid" => {
                let rows = args.and_then(|v| v.get("rows")).and_then(|v| v.as_u64()).unwrap_or(3) as u32;
                let columns = args.and_then(|v| v.get("columns")).and_then(|v| v.as_u64()).unwrap_or(5) as u32;
                let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
                    source: &deck.source,
                    rows,
                    columns,
                    gap: 0.0,
                    key_prefix: "tile",
                });
                self.runtime.selected_ids = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
                ActionEmit::ops(vec![PresentationOp::SetTiles { tiles }])
            }
            "addTile" => {
                let id = new_tile_id("tile");
                let crop = args
                    .and_then(|v| v.get("crop"))
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or(FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 });
                let tile = FigureTileDraft { id: id.clone(), name: id.clone(), crop };
                self.runtime.selected_ids = vec![id];
                ActionEmit::ops(vec![PresentationOp::Tiles(CollectionOp::Add {
                    index: deck.tiles.len(),
                    item: tile,
                })])
            }
            "deleteTile" => {
                let target = args
                    .and_then(|v| v.get("id"))
                    .and_then(|v| v.as_str())
                    .map(|id| vec![id.to_string()])
                    .unwrap_or_else(|| self.runtime.selected_ids.clone());
                let target = valid_tile_ids(deck, target);
                if target.is_empty() {
                    return ActionEmit::default();
                }
                self.runtime.selected_ids.retain(|id| !target.contains(id));
                ActionEmit::ops(target.into_iter().map(|id| PresentationOp::Tiles(CollectionOp::Remove { id })).collect())
            }
            "deleteSelection" => {
                let target = valid_tile_ids(deck, self.runtime.selected_ids.clone());
                if target.is_empty() {
                    return ActionEmit::default();
                }
                self.runtime.selected_ids.clear();
                ActionEmit::ops(target.into_iter().map(|id| PresentationOp::Tiles(CollectionOp::Remove { id })).collect())
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
                match name {
                    Some(name) => {
                        let valid = valid_tile_ids(deck, ids);
                        if valid.is_empty() {
                            return ActionEmit::default();
                        }
                        ActionEmit::ops(
                            valid
                                .into_iter()
                                .map(|id| {
                                    PresentationOp::Tiles(CollectionOp::Patch {
                                        id,
                                        patch: FigureTileDraftPatch { name: Some(name.into()), crop: None },
                                    })
                                })
                                .collect(),
                        )
                    }
                    None => ActionEmit::default(),
                }
            }
            "patchTileCrops" | "patchTileCrop" => {
                let ids: Vec<String> = args
                    .and_then(|v| v.get("ids"))
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|v| v.get("field")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64());
                match value {
                    Some(value) if !field.is_empty() => {
                        let targets: HashSet<&str> = ids.iter().map(String::as_str).collect();
                        let ops: Vec<PresentationOp> = deck
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
                                PresentationOp::Tiles(CollectionOp::Patch {
                                    id: tile.id.clone(),
                                    patch: FigureTileDraftPatch { name: None, crop: Some(clamp_tile_crop(crop)) },
                                })
                            })
                            .collect();
                        if ops.is_empty() {
                            ActionEmit::default()
                        } else {
                            ActionEmit::ops(ops)
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
                    let mut ops = vec![PresentationOp::SetSource { source: partial }];
                    if replaced {
                        ops.push(PresentationOp::SetTiles { tiles: Vec::new() });
                        self.runtime.selected_ids.clear();
                    }
                    return ActionEmit::ops(ops);
                }
                ActionEmit::default()
            }
            "setFrame" => {
                if let Some(frame_value) = args.and_then(|v| v.get("frame")) {
                    if let Ok(frame) = serde_json::from_value::<FigureTileFrame>(frame_value.clone()) {
                        let mut source = deck.source.clone();
                        source.frame = frame;
                        return ActionEmit::ops(vec![PresentationOp::SetSource { source }]);
                    }
                }
                ActionEmit::default()
            }
            "setActiveExample" => {
                let example_id = args.and_then(|v| v.get("exampleId")).and_then(|v| v.as_str()).unwrap_or("demo");
                if example_id == "demo" || example_id.is_empty() {
                    self.runtime.selected_ids.clear();
                    return ActionEmit::ops(vec![PresentationOp::SetDeck { deck: default_presentation_deck() }]);
                }
                ActionEmit::default()
            }
            "clearTiles" => {
                self.runtime.selected_ids.clear();
                ActionEmit::ops(vec![PresentationOp::SetTiles { tiles: Vec::new() }])
            }
            "copyPrompt" => ActionEmit::effect(tile_morph_prompt_effect(deck)),
            "engagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    self.runtime.engagement_input = value.into();
                }
                ActionEmit::default()
            }
            "engagementSubmit" => {
                let value = args
                    .and_then(|v| v.get("value"))
                    .and_then(|v| v.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| self.runtime.engagement_input.clone());
                let trimmed = value.trim();
                if let Some((rows, columns)) = parse_grid_engagement(trimmed) {
                    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
                        source: &deck.source,
                        rows,
                        columns,
                        gap: 0.0,
                        key_prefix: "tile",
                    });
                    self.runtime.selected_ids = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
                    self.runtime.engagement_input.clear();
                    return ActionEmit::ops(vec![PresentationOp::SetTiles { tiles }]);
                }
                match trimmed.to_lowercase().as_str() {
                    "add" => {
                        let id = new_tile_id("tile");
                        let tile = FigureTileDraft {
                            id: id.clone(),
                            name: id.clone(),
                            crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 },
                        };
                        self.runtime.selected_ids = vec![id];
                        self.runtime.engagement_input.clear();
                        ActionEmit::ops(vec![PresentationOp::Tiles(CollectionOp::Add {
                            index: deck.tiles.len(),
                            item: tile,
                        })])
                    }
                    "clear" => {
                        self.runtime.selected_ids.clear();
                        self.runtime.engagement_input.clear();
                        ActionEmit::ops(vec![PresentationOp::SetTiles { tiles: Vec::new() }])
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
    fn handle_command(
        &mut self,
        command: &str,
        _args: Option<&Value>,
        doc: &DocumentView<'_, PresentationDeck>,
        _view_state: &ViewState,
    ) -> ActionEmit<PresentationOp> {
        let deck = doc.projection;
        match command {
            "presentation.resetGrid" => {
                let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
                    source: &deck.source,
                    rows: 3,
                    columns: 5,
                    gap: 0.0,
                    key_prefix: "tile",
                });
                self.runtime.selected_ids = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
                ActionEmit::ops(vec![PresentationOp::SetTiles { tiles }])
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, PresentationDeck>, view_state: &ViewState) -> UiNode {
        let deck = doc.projection;
        let selected = &self.runtime.selected_ids;
        let labels = presentation_labels(view_state);
        match body_key {
            PRESENTATION_PLAY_BODY_MAIN => render_main_canvas(deck, selected),
            PRESENTATION_PLAY_BODY_DOCUMENT => build_document_tree(deck, selected, labels),
            PRESENTATION_PLAY_BODY_CATALOGUE => build_catalogue_tree(deck, labels),
            PRESENTATION_PLAY_BODY_DETAILS => build_details_tree(deck, selected, labels),
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
            .window_kind(PRESENTATION_PLAY_WINDOW_MAIN, "Tile editor", PRESENTATION_PLAY_BODY_MAIN, SurfaceKind::Canvas2d)
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
            // 👁️ Ephemeral view state — selection, engagement draft.
            .view_action("setSelectedIds", "Set Selected Ids")
            .view_action("engagementInput", "Engagement Input")
            .view_action("canvasPointerDown", "Canvas Pointer Down")
            .view_action("noop", "No Op")
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
            .app_command("presentation.resetGrid", "Reset to Default Grid", "document"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_presentation_deck()).unwrap())
    .program("presentation", "Presentation", "deck")
}

fn presentation_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Presentation", 1280, 720)
}

/// 📥 Builds a degenerate-but-valid one-slide deck from a rasterized DWG drawing, for the DWG import path.
fn presentation_document_json_from_dwg(drawing: &semio_framework_core::DwgDrawing) -> Result<Value, String> {
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height)?;
    let frame = FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    let deck = PresentationDeck {
        schema: PRESENTATION_DOCUMENT_SCHEMA.into(),
        source: FigureTileSource {
            src: format!("data:image/png;base64,{png_base64}"),
            kind: "image".into(),
            frame: frame.clone(),
            source_aspect: Some(width as f64 / height.max(1) as f64),
            pdf_page: None,
        },
        tiles: vec![FigureTileDraft {
            id: "imported-drawing".into(),
            name: "Imported Drawing".into(),
            crop: frame,
        }],
    };
    serde_json::to_value(&deck).map_err(|error| error.to_string())
}

fn register_presentation_exports() {
    semio_framework_os::register_2d_export_handlers("presentation.deck", "presentation", presentation_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("presentation.deck", presentation_document_json_from_dwg);
}

semio_framework_plugin::semio_plugin! {
    id: "presentation",
    label: "Presentation",
    version: "0.1.0",
    setup: register_presentation_exports,
    apps: [ create_presentation_app => PresentationPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionKind, ActionMeta, PluginApp, VcsDocumentApp};
    use semio_framework_plugin::app::AppActionRegistry;

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<PresentationPlayApp> {
        VcsDocumentApp::new(PresentationPlayApp::default())
    }

    /// 🧬 A wrapper carrying the real registry so kind discipline (Shell/View-emits-ops rejection) and
    /// declared-arg materialization run exactly as in production.
    fn new_app_with_registry() -> VcsDocumentApp<PresentationPlayApp> {
        let definition = create_presentation_app().definition;
        VcsDocumentApp::with_registry(PresentationPlayApp::default(), AppActionRegistry::from_definition(&definition))
    }

    #[test]
    fn copy_prompt_is_shell_effect_not_view_mutation() {
        let mut app = new_app_with_registry();
        app.handle_action("seedGrid", Some(&json!({ "rows": 1, "columns": 2 })), &ViewState::default(), &meta("local"))
            .expect("seed grid");
        let result = app.handle_action("copyPrompt", None, &ViewState::default(), &meta("local")).expect("copy prompt");
        assert!(result.operations.is_empty(), "copyPrompt is a host effect, not a document op");
        assert!(
            matches!(result.requested_effects.as_slice(), [HostEffect::DownloadMediaExport { mime_type, .. }] if mime_type == "text/markdown"),
            "copyPrompt emits exactly one media-export host effect carrying the morph prompt",
        );
        let definition = create_presentation_app().definition;
        assert!(
            definition.actions.iter().any(|action| action.id == "copyPrompt" && matches!(action.kind, ActionKind::Shell)),
            "copyPrompt is declared Shell-kind (host side-effect), never View",
        );
    }

    #[test]
    fn seed_grid_materializes_declared_arg_defaults_via_registry() {
        let mut app = new_app_with_registry();
        app.handle_action("seedGrid", None, &ViewState::default(), &meta("local")).expect("seed grid with defaults");
        assert_eq!(app.projection().expect("projection").tiles.len(), 4, "declared rows=2/columns=2 defaults seed a 2x2 grid");
    }

    fn seed_2x2(app: &mut VcsDocumentApp<PresentationPlayApp>) {
        app.handle_action("seedGrid", Some(&json!({ "rows": 2, "columns": 2 })), &ViewState::default(), &meta("local"))
            .expect("seed grid");
    }

    #[test]
    fn renders_canvas_scene() {
        let mut app = new_app();
        let node = app.render(PRESENTATION_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
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
    fn deck_schema_is_presentation() {
        assert_eq!(default_presentation_deck().schema, PRESENTATION_DOCUMENT_SCHEMA);
    }

    #[test]
    fn source_frame_renders_as_actual_image_layer_behind_tiles() {
        let mut app = new_app();
        app.handle_action("seedGrid", Some(&json!({ "rows": 1, "columns": 2 })), &ViewState::default(), &meta("local"))
            .expect("seed grid");
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
        app.handle_action("seedGrid", Some(&json!({ "rows": 1, "columns": 2 })), &ViewState::default(), &meta("local"))
            .expect("seed grid");
        let node = app.render(PRESENTATION_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("tile-r0-c0"));
    }

    #[test]
    fn add_delete_and_rename_tile_round_trip_through_ops() {
        let mut app = new_app();
        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("add tile");
        let tile_id = app.projection().expect("projection").tiles[0].id.clone();
        app.handle_action("renameTiles", Some(&json!({ "ids": [tile_id], "value": "Hero" })), &ViewState::default(), &meta("local"))
            .expect("rename");
        assert_eq!(app.projection().expect("projection").tiles[0].name, "Hero");
        app.handle_action("deleteTile", Some(&json!({ "id": tile_id })), &ViewState::default(), &meta("local"))
            .expect("delete");
        assert!(app.projection().expect("projection").tiles.is_empty());
    }

    #[test]
    fn patch_tile_crop_clamps_and_is_reversible() {
        let mut app = new_app();
        app.handle_action("addTile", None, &ViewState::default(), &meta("local")).expect("add tile");
        let tile_id = app.projection().expect("projection").tiles[0].id.clone();
        app.handle_action(
            "patchTileCrops",
            Some(&json!({ "ids": [tile_id], "field": "width", "value": 0.5 })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("patch crop");
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
    fn presentation_labels_resolve_native_by_default() {
        let app = new_app();
        // render needs &mut for cache; use a fresh app to render catalogue.
        let mut app = app;
        let node = app.render(PRESENTATION_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Tile templates"));
        assert!(json.contains("Split 2×2 grid"));
        assert!(json.contains("Active source"));
        assert!(!json.contains("Kachelvorlagen"));
    }

    #[test]
    fn presentation_labels_translate_panels_in_german() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let catalogue_node = app.render(PRESENTATION_PLAY_BODY_CATALOGUE, None, &view_state).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("Kachelvorlagen"));
        assert!(catalogue_json.contains("2×2-Raster teilen"));
        assert!(catalogue_json.contains("Aktive Quelle"));
        assert!(!catalogue_json.contains("Tile templates"));

        let document_node = app.render(PRESENTATION_PLAY_BODY_DOCUMENT, None, &view_state).expect("render");
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
                geometry: semio_framework_core::DwgGeometry::LwPolyline {
                    closed: true,
                    elevation: 0.0,
                    vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]],
                    bulges: vec![0.0, 0.0, 0.0, 0.0],
                },
            }],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let document = presentation_document_json_from_dwg(&drawing).expect("from_dwg");
        let deck: PresentationDeck = serde_json::from_value(document).expect("deck");
        assert_eq!(deck.schema, PRESENTATION_DOCUMENT_SCHEMA);
        assert_eq!(deck.tiles.len(), 1);
        assert_eq!(deck.tiles[0].name, "Imported Drawing");
        assert!(deck.source.src.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn from_dwg_never_errors_on_empty_drawing() {
        let drawing = semio_framework_core::DwgDrawing::default();
        let document = presentation_document_json_from_dwg(&drawing).expect("from_dwg on empty drawing");
        let deck: PresentationDeck = serde_json::from_value(document).expect("deck");
        assert_eq!(deck.tiles.len(), 1);
    }

    /// 🧪 Two independent instances start empty, apply DISJOINT edits (A adds a tile, B sets the
    /// source), and exchanging ops over a `MemoryBackbone` converges both sides to contain BOTH edits —
    /// impossible with whole-document snapshots, which would clobber one another.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        use vcs::MemoryBackbone;
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://presentation-convergence", "mem://presentation-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a
            .handle_action("addTile", Some(&json!({ "crop": { "x": 0.0, "y": 0.0, "width": 0.3, "height": 0.3 } })), &ViewState::default(), &meta("actor-a"))
            .expect("a adds tile");
        instance_b
            .handle_action("setSource", Some(&json!({ "kind": "video" })), &ViewState::default(), &meta("actor-b"))
            .expect("b sets source kind");

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
