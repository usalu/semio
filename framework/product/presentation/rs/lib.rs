//! 🎞️ Presentation deck document + typed VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, CollectionDiff, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore,
    ItemPatch, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PRESENTATION_DOCUMENT_SCHEMA: &str = "presentation.deck";

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileSource {
    pub src: String,
    pub kind: String,
    pub frame: FigureTileFrame,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_aspect: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_page: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraft {
    pub id: String,
    pub name: String,
    pub crop: FigureTileFrame,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationDeck {
    pub schema: String,
    pub source: FigureTileSource,
    pub tiles: Vec<FigureTileDraft>,
}

pub type PresentationEnvelope = DocumentVcsEnvelope<PresentationDeck, PresentationOp>;
pub type PresentationStore = DocumentVcsStore<PresentationDeck, PresentationOp>;

pub fn empty_presentation_projection() -> PresentationDeck {
    PresentationDeck {
        schema: PRESENTATION_DOCUMENT_SCHEMA.into(),
        source: default_figure_tile_source(),
        tiles: Vec::new(),
    }
}

pub fn default_figure_tile_source() -> FigureTileSource {
    FigureTileSource {
        src: "/bauteilbörse.png".into(),
        kind: "figure".into(),
        frame: FigureTileFrame {
            x: 0.127,
            y: 0.1,
            width: 0.746,
            height: 0.75,
        },
        source_aspect: Some(1222.0 / 896.0),
        pdf_page: None,
    }
}

pub fn default_presentation_deck() -> PresentationDeck {
    PresentationDeck {
        schema: PRESENTATION_DOCUMENT_SCHEMA.into(),
        source: default_figure_tile_source(),
        tiles: Vec::new(),
    }
}
//#endregion 🔖Domain

//#region 🔖TilePlay
pub const NORMALIZED_RECT_MIN_FRACTION: f64 = 0.02;

pub struct SplitFigureGridSpec<'a> {
    pub rows: u32,
    pub columns: u32,
    pub frame: &'a FigureTileFrame,
    pub gap: f64,
    pub key_prefix: &'a str,
}

pub struct SplitGridCell {
    pub key: String,
    pub crop: FigureTileFrame,
}

pub struct FigureTileGridSeedSpec<'a> {
    pub source: &'a FigureTileSource,
    pub rows: u32,
    pub columns: u32,
    pub gap: f64,
    pub key_prefix: &'a str,
}

pub fn clamp_normalized_fraction(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

pub fn clamp_tile_crop(crop: FigureTileFrame) -> FigureTileFrame {
    let width = crop.width.max(NORMALIZED_RECT_MIN_FRACTION);
    let height = crop.height.max(NORMALIZED_RECT_MIN_FRACTION);
    let x = clamp_normalized_fraction(crop.x.min(1.0 - width));
    let y = clamp_normalized_fraction(crop.y.min(1.0 - height));
    FigureTileFrame { x, y, width, height }
}

pub fn parse_grid_engagement(text: &str) -> Option<(u32, u32)> {
    let trimmed = text.trim();
    let lower = trimmed.to_lowercase();
    let normalized = lower.replace('×', "x");
    let parts: Vec<&str> = normalized.split('x').map(str::trim).collect();
    if parts.len() != 2 {
        return None;
    }
    let rows: u32 = parts[0].parse().ok()?;
    let columns: u32 = parts[1].parse().ok()?;
    if rows < 1 || columns < 1 {
        return None;
    }
    Some((rows, columns))
}

pub fn split_figure_grid(spec: SplitFigureGridSpec<'_>) -> Vec<SplitGridCell> {
    let rows = spec.rows.max(1);
    let columns = spec.columns.max(1);
    let gap = spec.gap;
    let frame = spec.frame;
    let cell_width = (frame.width - gap * (columns as f64 - 1.0)) / columns as f64;
    let cell_height = (frame.height - gap * (rows as f64 - 1.0)) / rows as f64;
    let crop_width = frame.width / columns as f64;
    let crop_height = frame.height / rows as f64;
    let mut cells = Vec::new();
    for row in 0..rows {
        for column in 0..columns {
            cells.push(SplitGridCell {
                key: format!("{}-r{row}-c{column}", spec.key_prefix),
                crop: FigureTileFrame {
                    x: frame.x + column as f64 * crop_width,
                    y: frame.y + row as f64 * crop_height,
                    width: crop_width,
                    height: crop_height,
                },
            });
        }
    }
    let _ = (cell_width, cell_height);
    cells
}

pub fn populate_tile_drafts_from_grid(spec: FigureTileGridSeedSpec<'_>) -> Vec<FigureTileDraft> {
    split_figure_grid(SplitFigureGridSpec {
        rows: spec.rows,
        columns: spec.columns,
        frame: &spec.source.frame,
        gap: spec.gap,
        key_prefix: spec.key_prefix,
    })
    .into_iter()
    .map(|cell| FigureTileDraft {
        id: cell.key.clone(),
        name: cell.key,
        crop: cell.crop,
    })
    .collect()
}

pub fn build_tile_morph_prompt(source: &FigureTileSource, drafts: &[FigureTileDraft]) -> String {
    fn format_frame(frame: &FigureTileFrame) -> String {
        format!(
            "{{ x: {:.6}, y: {:.6}, width: {:.6}, height: {:.6} }}",
            frame.x, frame.y, frame.width, frame.height
        )
    }
    let kind = if source.kind.is_empty() { "figure" } else { source.kind.as_str() };
    let mut lines = vec![
        "Wire a one-to-many morph for presentation deck tiles using the parameters below.".into(),
        String::new(),
        "## Source media".into(),
        format!("- kind: {kind}"),
        format!("- src: {}", serde_json::to_string(&source.src).unwrap_or_else(|_| "\"\"".into())),
    ];
    if let Some(aspect) = source.source_aspect {
        lines.push(format!("- sourceAspect: {aspect}"));
    }
    if kind == "pdf" {
        if let Some(page) = source.pdf_page {
            lines.push(format!("- pdfPage: {page}"));
        }
    }
    lines.push(format!("- frame: {}", format_frame(&source.frame)));
    lines.push(String::new());
    lines.push("## Tiles (normalized source crops; overlap allowed)".into());
    for draft in drafts {
        lines.push(format!(
            "- {} ({}): crop {}",
            draft.name,
            draft.id,
            format_frame(&draft.crop)
        ));
    }
    let embodiment_hint = match kind {
        "video" => "Use video embodiments for tile participants and the source clip.",
        "pdf" => "Use pdf embodiments for tile participants and the source document page.",
        _ => "Register one participant per tile with a tile figure embodiment using each crop above.",
    };
    lines.push(String::new());
    lines.push("## Task".into());
    lines.push(format!("1. {embodiment_hint}"));
    lines.push("2. On the source slide, place the full media with morphTo slots pointing at each tile participant.".into());
    lines.push("3. Use reveal.js auto-animate; morph from the actual disposition including ephemeral modifications.".into());
    lines.join("\n")
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum PresentationEdit {
    SetSource { source: FigureTileSource },
    ReplaceSource { source: FigureTileSource, reset_tiles: bool },
    SetSourceFrame { frame: FigureTileFrame },
    AddTile { tile: FigureTileDraft, index: Option<usize> },
    RemoveTile { tile_id: String },
    RemoveTiles { tile_ids: Vec<String> },
    RenameTile { tile_id: String, name: String },
    RenameTiles { tile_ids: Vec<String>, name: String },
    SetTiles { tiles: Vec<FigureTileDraft> },
    ClearTiles,
    PatchTileCrop { tile_id: String, crop: FigureTileFrame },
    PatchTileCrops { tile_ids: Vec<String>, field: String, value: f64 },
    SetDocument { document: PresentationDeck },
}

pub fn apply_presentation_edit(deck: PresentationDeck, edit: &PresentationEdit) -> PresentationDeck {
    match edit {
        PresentationEdit::SetSource { source } => PresentationDeck { source: source.clone(), ..deck },
        PresentationEdit::ReplaceSource { source, reset_tiles } => PresentationDeck {
            source: source.clone(),
            tiles: if *reset_tiles { Vec::new() } else { deck.tiles },
            ..deck
        },
        PresentationEdit::SetSourceFrame { frame } => PresentationDeck {
            source: FigureTileSource {
                frame: frame.clone(),
                ..deck.source
            },
            ..deck
        },
        PresentationEdit::AddTile { tile, index } => {
            let mut tiles = deck.tiles;
            let at = index.unwrap_or(tiles.len()).min(tiles.len());
            tiles.insert(at, tile.clone());
            PresentationDeck { tiles, ..deck }
        }
        PresentationEdit::RemoveTile { tile_id } => PresentationDeck {
            tiles: deck.tiles.into_iter().filter(|tile| tile.id != *tile_id).collect(),
            ..deck
        },
        PresentationEdit::RemoveTiles { tile_ids } => {
            let remove: std::collections::HashSet<&str> = tile_ids.iter().map(String::as_str).collect();
            PresentationDeck {
                tiles: deck.tiles.into_iter().filter(|tile| !remove.contains(tile.id.as_str())).collect(),
                ..deck
            }
        }
        PresentationEdit::RenameTile { tile_id, name } => apply_presentation_edit(
            deck,
            &PresentationEdit::RenameTiles {
                tile_ids: vec![tile_id.clone()],
                name: name.clone(),
            },
        ),
        PresentationEdit::RenameTiles { tile_ids, name } => {
            let targets: std::collections::HashSet<&str> = tile_ids.iter().map(String::as_str).collect();
            PresentationDeck {
                tiles: deck
                    .tiles
                    .into_iter()
                    .map(|tile| {
                        if targets.contains(tile.id.as_str()) {
                            FigureTileDraft { name: name.clone(), ..tile }
                        } else {
                            tile
                        }
                    })
                    .collect(),
                ..deck
            }
        }
        PresentationEdit::SetTiles { tiles } => PresentationDeck {
            tiles: tiles.clone(),
            ..deck
        },
        PresentationEdit::ClearTiles => PresentationDeck {
            tiles: Vec::new(),
            ..deck
        },
        PresentationEdit::PatchTileCrop { tile_id, crop } => PresentationDeck {
            tiles: deck
                .tiles
                .into_iter()
                .map(|tile| {
                    if tile.id == *tile_id {
                        FigureTileDraft {
                            crop: clamp_tile_crop(crop.clone()),
                            ..tile
                        }
                    } else {
                        tile
                    }
                })
                .collect(),
            ..deck
        },
        PresentationEdit::PatchTileCrops { tile_ids, field, value } => {
            let targets: std::collections::HashSet<&str> = tile_ids.iter().map(String::as_str).collect();
            PresentationDeck {
                tiles: deck
                    .tiles
                    .into_iter()
                    .map(|tile| {
                        if !targets.contains(tile.id.as_str()) {
                            return tile;
                        }
                        let mut crop = tile.crop.clone();
                        match field.as_str() {
                            "x" => crop.x = *value,
                            "y" => crop.y = *value,
                            "width" => crop.width = *value,
                            "height" => crop.height = *value,
                            _ => {}
                        }
                        FigureTileDraft {
                            crop: clamp_tile_crop(crop),
                            ..tile
                        }
                    })
                    .collect(),
                ..deck
            }
        }
        PresentationEdit::SetDocument { document } => document.clone(),
    }
}
//#endregion 🔖TilePlay

//#region 🔖Ops
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum PresentationOp {
    SetSource {
        source: FigureTileSource,
    },
    AddTile {
        tile: FigureTileDraft,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveTile {
        tile_id: String,
    },
    RenameTile {
        tile_id: String,
        name: String,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationDiff {
    pub source: Option<FigureTileSource>,
    pub tiles: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>,
}

impl OperationDiff<PresentationDeck> for PresentationDiff {
    fn apply(&self, projection: &PresentationDeck) -> PresentationDeck {
        let mut next = projection.clone();
        if let Some(source) = &self.source {
            next.source = source.clone();
        }
        if let Some(tiles) = &self.tiles {
            for id in &tiles.removed {
                next.tiles.retain(|tile| tile.id != *id);
            }
            for patch in &tiles.modified {
                for tile in &mut next.tiles {
                    if tile.id == patch.id {
                        if let Some(name) = &patch.patch.name {
                            tile.name = name.clone();
                        }
                    }
                }
            }
            for added in &tiles.added {
                next.tiles.push(added.clone());
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.source.is_some() {
            self.source = other.source;
        }
        match (&mut self.tiles, other.tiles) {
            (Some(a), Some(b)) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            (None, Some(b)) => self.tiles = Some(b),
            _ => {}
        }
    }
}

impl Operation<PresentationDeck> for PresentationOp {
    type Diff = PresentationDiff;

    fn diff(&self, _projection: &PresentationDeck) -> PresentationDiff {
        match self {
            PresentationOp::SetSource { source } => PresentationDiff {
                source: Some(source.clone()),
                ..Default::default()
            },
            PresentationOp::AddTile { tile, .. } => PresentationDiff {
                tiles: Some(CollectionDiff {
                    added: vec![tile.clone()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            PresentationOp::RemoveTile { tile_id } => PresentationDiff {
                tiles: Some(CollectionDiff {
                    removed: vec![tile_id.clone()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            PresentationOp::RenameTile { tile_id, name } => PresentationDiff {
                tiles: Some(CollectionDiff {
                    modified: vec![ItemPatch {
                        id: tile_id.clone(),
                        patch: FigureTileDraftPatch {
                            name: Some(name.clone()),
                        },
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
        }
    }

    fn backwards(&self, projection: &PresentationDeck) -> Vec<Self> {
        match self {
            PresentationOp::SetSource { .. } => vec![PresentationOp::SetSource {
                source: projection.source.clone(),
            }],
            PresentationOp::AddTile { tile, .. } => vec![PresentationOp::RemoveTile {
                tile_id: tile.id.clone(),
            }],
            PresentationOp::RemoveTile { tile_id } => projection
                .tiles
                .iter()
                .find(|t| t.id == *tile_id)
                .map(|tile| vec![PresentationOp::AddTile {
                    tile: tile.clone(),
                    index: None,
                }])
                .unwrap_or_default(),
            PresentationOp::RenameTile { tile_id, .. } => projection
                .tiles
                .iter()
                .find(|t| t.id == *tile_id)
                .map(|tile| {
                    vec![PresentationOp::RenameTile {
                        tile_id: tile_id.clone(),
                        name: tile.name.clone(),
                    }]
                })
                .unwrap_or_default(),
        }
    }
}
//#endregion 🔖Ops

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct PresentationDocumentVcs {
        store: RefCell<PresentationStore>,
    }

    #[wasm_bindgen]
    impl PresentationDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<PresentationDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: PresentationEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    PresentationStore::new(envelope)
                }
                None => PresentationStore::new(create_document_vcs_envelope(
                    PRESENTATION_DOCUMENT_SCHEMA,
                    "presentation",
                    empty_presentation_projection(),
                    None,
                )),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_seed_produces_tiles() {
        let source = default_figure_tile_source();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
            source: &source,
            rows: 3,
            columns: 5,
            gap: 0.0,
            key_prefix: "tile",
        });
        assert_eq!(tiles.len(), 15);
        assert_eq!(tiles[0].id, "tile-r0-c0");
    }

    #[test]
    fn parse_grid_engagement_accepts_cross() {
        assert_eq!(parse_grid_engagement("3×5"), Some((3, 5)));
        assert_eq!(parse_grid_engagement("2x2"), Some((2, 2)));
    }

    #[test]
    fn morph_prompt_lists_tiles() {
        let source = default_figure_tile_source();
        let tiles = vec![FigureTileDraft {
            id: "t1".into(),
            name: "t1".into(),
            crop: FigureTileFrame {
                x: 0.1,
                y: 0.1,
                width: 0.2,
                height: 0.2,
            },
        }];
        let prompt = build_tile_morph_prompt(&source, &tiles);
        assert!(prompt.contains("t1"));
        assert!(prompt.contains("Source media"));
    }

    #[test]
    fn apply_edit_seeds_and_clears() {
        let deck = default_presentation_deck();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
            source: &deck.source,
            rows: 2,
            columns: 2,
            gap: 0.0,
            key_prefix: "tile",
        });
        let seeded = apply_presentation_edit(deck, &PresentationEdit::SetTiles { tiles: tiles.clone() });
        assert_eq!(seeded.tiles.len(), 4);
        let cleared = apply_presentation_edit(seeded, &PresentationEdit::ClearTiles);
        assert!(cleared.tiles.is_empty());
    }

    #[test]
    fn presentation_deck_materializes() {
        let mut store = PresentationStore::new(create_document_vcs_envelope(
            PRESENTATION_DOCUMENT_SCHEMA,
            "presentation",
            empty_presentation_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![PresentationOp::AddTile {
                    tile: FigureTileDraft {
                        id: "t1".into(),
                        name: "A".into(),
                        crop: FigureTileFrame {
                            x: 0.0,
                            y: 0.0,
                            width: 1.0,
                            height: 1.0,
                        },
                    },
                    index: None,
                }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").tiles.len(), 1);
    }
}
//#endregion 🧪Tests
