//! 🎞️ Presentation deck document + typed VCS on `vcs`.

use vcs::{
    collection_diff_from_op, create_document_vcs_envelope, invert_collection_op, CollectionDiff, CollectionOp,
    DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Identified, Operation, OperationDiff, Patchable,
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

//#endregion 🔖TilePlay

//#region 🔖Ops
//#region 🔖CollectionSupport
impl Identified<String> for FigureTileDraft {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹 Sparse patch of a `FigureTileDraft` — the mutable per-tile fields (name and crop).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
    pub crop: Option<FigureTileFrame>,
}

impl Patchable<FigureTileDraftPatch> for FigureTileDraft {
    fn apply_patch(&mut self, patch: &FigureTileDraftPatch) -> FigureTileDraftPatch {
        let inverse = FigureTileDraftPatch {
            name: patch.name.as_ref().map(|_| self.name.clone()),
            crop: patch.crop.as_ref().map(|_| self.crop.clone()),
        };
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(crop) = &patch.crop {
            self.crop = crop.clone();
        }
        inverse
    }
}

/// ▶️ Applies a `CollectionDiff` (removed → modified → added) to an owned `Vec` — `vcs::CollectionDiff`
/// has no generic apply of its own since `modified` patches require the item's `Patchable` impl.
fn apply_tile_diff(tiles: &mut Vec<FigureTileDraft>, diff: &CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>) {
    for id in &diff.removed {
        tiles.retain(|tile| tile.id != *id);
    }
    for patch in &diff.modified {
        if let Some(tile) = tiles.iter_mut().find(|tile| tile.id == patch.id) {
            tile.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        tiles.push(added.clone());
    }
}

/// ➕ Merges an incoming tile `CollectionDiff` into an existing one (coalescing two edits' diffs).
fn absorb_tile_diff(
    target: &mut Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>,
    incoming: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>,
) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}
//#endregion 🔖CollectionSupport

/// 📦 Typed presentation-deck operation. Tile add/remove/patch/move flow through a generic
/// {@link CollectionOp} for granular convergence; `SetSource`/`SetTiles` are scalar/bulk writes and
/// `SetDeck` replaces the whole projection (import/reset). Ephemeral view state (selection, engagement
/// draft, clipboard) lives in the plugin's runtime, never in the document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum PresentationOp {
    Tiles(CollectionOp<String, FigureTileDraft, FigureTileDraftPatch>),
    SetSource { source: FigureTileSource },
    SetTiles { tiles: Vec<FigureTileDraft> },
    SetDeck { deck: PresentationDeck },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationDiff {
    pub deck: Option<PresentationDeck>,
    pub source: Option<FigureTileSource>,
    pub tiles: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>,
    pub set_tiles: Option<Vec<FigureTileDraft>>,
}

impl OperationDiff<PresentationDeck> for PresentationDiff {
    fn apply(&self, projection: &PresentationDeck) -> PresentationDeck {
        if let Some(deck) = &self.deck {
            return deck.clone();
        }
        let mut next = projection.clone();
        if let Some(source) = &self.source {
            next.source = source.clone();
        }
        if let Some(tiles) = &self.set_tiles {
            next.tiles = tiles.clone();
        }
        if let Some(diff) = &self.tiles {
            apply_tile_diff(&mut next.tiles, diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.deck.is_some() {
            self.deck = other.deck;
            return;
        }
        if other.source.is_some() {
            self.source = other.source;
        }
        if other.set_tiles.is_some() {
            self.set_tiles = other.set_tiles;
        }
        absorb_tile_diff(&mut self.tiles, other.tiles);
    }
}

impl Operation<PresentationDeck> for PresentationOp {
    type Diff = PresentationDiff;

    fn diff(&self, projection: &PresentationDeck) -> PresentationDiff {
        match self {
            PresentationOp::Tiles(op) => PresentationDiff {
                tiles: Some(collection_diff_from_op(&projection.tiles, op)),
                ..Default::default()
            },
            PresentationOp::SetSource { source } => PresentationDiff {
                source: Some(source.clone()),
                ..Default::default()
            },
            PresentationOp::SetTiles { tiles } => PresentationDiff {
                set_tiles: Some(tiles.clone()),
                ..Default::default()
            },
            PresentationOp::SetDeck { deck } => PresentationDiff {
                deck: Some(deck.clone()),
                ..Default::default()
            },
        }
    }

    fn backwards(&self, projection: &PresentationDeck) -> Vec<Self> {
        match self {
            PresentationOp::Tiles(op) => vec![PresentationOp::Tiles(invert_collection_op(&projection.tiles, op))],
            PresentationOp::SetSource { .. } => vec![PresentationOp::SetSource {
                source: projection.source.clone(),
            }],
            PresentationOp::SetTiles { .. } => vec![PresentationOp::SetTiles {
                tiles: projection.tiles.clone(),
            }],
            PresentationOp::SetDeck { .. } => vec![PresentationOp::SetDeck {
                deck: projection.clone(),
            }],
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

    fn round_trip(deck: &PresentationDeck, op: &PresentationOp) -> PresentationDeck {
        let forward = vcs::apply_operation(deck, op);
        let mut restored = forward.clone();
        for back in op.backwards(deck) {
            restored = vcs::apply_operation(&restored, &back);
        }
        assert_eq!(&restored, deck, "backwards() must exactly restore the pre-op deck");
        forward
    }

    #[test]
    fn set_tiles_and_clear_round_trip() {
        let deck = default_presentation_deck();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
            source: &deck.source,
            rows: 2,
            columns: 2,
            gap: 0.0,
            key_prefix: "tile",
        });
        let seeded = round_trip(&deck, &PresentationOp::SetTiles { tiles: tiles.clone() });
        assert_eq!(seeded.tiles.len(), 4);
        let cleared = round_trip(&seeded, &PresentationOp::SetTiles { tiles: Vec::new() });
        assert!(cleared.tiles.is_empty());
    }

    #[test]
    fn tile_add_patch_remove_round_trip() {
        let deck = default_presentation_deck();
        let tile = FigureTileDraft {
            id: "t1".into(),
            name: "A".into(),
            crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 },
        };
        let added = round_trip(&deck, &PresentationOp::Tiles(CollectionOp::Add { index: 0, item: tile }));
        assert_eq!(added.tiles.len(), 1);
        let renamed = round_trip(
            &added,
            &PresentationOp::Tiles(CollectionOp::Patch {
                id: "t1".into(),
                patch: FigureTileDraftPatch { name: Some("Renamed".into()), crop: None },
            }),
        );
        assert_eq!(renamed.tiles[0].name, "Renamed");
        let recropped = round_trip(
            &renamed,
            &PresentationOp::Tiles(CollectionOp::Patch {
                id: "t1".into(),
                patch: FigureTileDraftPatch { name: None, crop: Some(FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 }) },
            }),
        );
        assert_eq!(recropped.tiles[0].crop.width, 0.4);
        let removed = round_trip(&recropped, &PresentationOp::Tiles(CollectionOp::Remove { id: "t1".into() }));
        assert!(removed.tiles.is_empty());
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
                operations: vec![PresentationOp::Tiles(CollectionOp::Add {
                    index: 0,
                    item: FigureTileDraft {
                        id: "t1".into(),
                        name: "A".into(),
                        crop: FigureTileFrame {
                            x: 0.0,
                            y: 0.0,
                            width: 1.0,
                            height: 1.0,
                        },
                    },
                })],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").tiles.len(), 1);
    }
}
//#endregion 🧪Tests
