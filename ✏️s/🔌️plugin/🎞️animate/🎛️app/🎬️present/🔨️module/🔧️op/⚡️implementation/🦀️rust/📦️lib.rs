//! ⚡️ Animate present app — operation enum + laws (constitutional: op).

use present::{FigureTileDraft, FigureTileDraftPatch, FigureTileSource, PresentDeck};
use protocol::{collection_diff_from_operation, invert_collection_operation, CollectionDiff, CollectionOperation, Operation, OperationDiff, Patchable};
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
//#region 🔖️CollectionSupport
// 🪪️ `Identified<String> for FigureTileDraft` / `Patchable<FigureTileDraftPatch> for FigureTileDraft`
// live in `present` (the `rs` crate), not here — Rust's orphan rules require a foreign-trait impl
// (`protocol::Identified`/`protocol::Patchable`) to share a crate with the type it's implemented for,
// and `FigureTileDraft` is defined in `present`.

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

fn absorb_tile_diff(target: &mut Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>, incoming: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>) {
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
//#endregion 🔖️CollectionSupport

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum PresentOperation {
    Tiles(CollectionOperation<String, FigureTileDraft, FigureTileDraftPatch>),
    SetSource { source: FigureTileSource },
    SetTiles { tiles: Vec<FigureTileDraft> },
    SetDeck { deck: PresentDeck }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentDiff {
    pub deck: Option<PresentDeck>,
    pub source: Option<FigureTileSource>,
    pub tiles: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>,
    pub set_tiles: Option<Vec<FigureTileDraft>>,
}

impl OperationDiff<PresentDeck> for PresentDiff {
    fn apply(&self, projection: &PresentDeck) -> PresentDeck {
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

impl Operation<PresentDeck> for PresentOperation {
    type Diff = PresentDiff;

    fn diff(&self, projection: &PresentDeck) -> PresentDiff {
        match self {
            PresentOperation::Tiles(operation) => PresentDiff { tiles: Some(collection_diff_from_operation(&projection.tiles, operation)), ..Default::default() },
            PresentOperation::SetSource { source } => PresentDiff { source: Some(source.clone()), ..Default::default() },
            PresentOperation::SetTiles { tiles } => PresentDiff { set_tiles: Some(tiles.clone()), ..Default::default() },
            PresentOperation::SetDeck { deck } => PresentDiff { deck: Some(deck.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &PresentDeck) -> Vec<Self> {
        match self {
            PresentOperation::Tiles(operation) => vec![PresentOperation::Tiles(invert_collection_operation(&projection.tiles, operation))],
            PresentOperation::SetSource { .. } => vec![PresentOperation::SetSource { source: projection.source.clone() }],
            PresentOperation::SetTiles { .. } => vec![PresentOperation::SetTiles { tiles: projection.tiles.clone() }],
            PresentOperation::SetDeck { .. } => vec![PresentOperation::SetDeck { deck: projection.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🔖️OpText
/// ⚡️ DSL-facing mirror of `PresentOperation`, declared purely so `#[derive(dsl::DslOps)]` has
/// something to attach to: `PresentOperation::Tiles` wraps `vcs::CollectionOperation<..>`, a foreign
/// generic type the derive can't classify (and can't gain a `DslField` impl here either — both the
/// trait and the type live outside this crate, so Rust's orphan rules forbid it). Every `Tiles(...)`
/// case is flattened into its own tagged variant instead; `SetSource`/`SetTiles`/`SetDeck` carry
/// straight through unchanged. `From`/`Into` below keep this an implementation detail — nothing
/// outside `impl protocol::OpText for PresentOperation` ever names it.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum PresentOperationDsl {
    TilesAdd {
        index: usize,
        #[dsl(block)]
        item: FigureTileDraft,
    },
    TilesRemove { id: String },
    TilesMove { id: String, to_index: usize },
    TilesPatch {
        id: String,
        #[dsl(block)]
        patch: FigureTileDraftPatch,
    },
    SetSource {
        #[dsl(block)]
        source: FigureTileSource,
    },
    SetTiles {
        #[dsl(table)]
        tiles: Vec<FigureTileDraft>,
    },
    SetDeck {
        #[dsl(block)]
        deck: PresentDeck,
    }
}

impl From<&PresentOperation> for PresentOperationDsl {
    fn from(operation: &PresentOperation) -> Self {
        match operation {
            PresentOperation::Tiles(CollectionOperation::Add { id: _id, item, at }) => PresentOperationDsl::TilesAdd { index: *at, item: item.clone() },
            PresentOperation::Tiles(CollectionOperation::Remove { id }) => PresentOperationDsl::TilesRemove { id: id.clone() },
            PresentOperation::Tiles(CollectionOperation::Move { id, to }) => PresentOperationDsl::TilesMove { id: id.clone(), to_index: *to },
            PresentOperation::Tiles(CollectionOperation::Patch { id, patch }) => PresentOperationDsl::TilesPatch { id: id.clone(), patch: patch.clone() },
            PresentOperation::SetSource { source } => PresentOperationDsl::SetSource { source: source.clone() },
            PresentOperation::SetTiles { tiles } => PresentOperationDsl::SetTiles { tiles: tiles.clone() },
            PresentOperation::SetDeck { deck } => PresentOperationDsl::SetDeck { deck: deck.clone() },
        }
    }
}

impl From<PresentOperationDsl> for PresentOperation {
    fn from(operation: PresentOperationDsl) -> Self {
        match operation {
            PresentOperationDsl::TilesAdd { index, item } => PresentOperation::Tiles(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
            PresentOperationDsl::TilesRemove { id } => PresentOperation::Tiles(CollectionOperation::Remove { id }),
            PresentOperationDsl::TilesMove { id, to_index } => PresentOperation::Tiles(CollectionOperation::Move { id, to: to_index }),
            PresentOperationDsl::TilesPatch { id, patch } => PresentOperation::Tiles(CollectionOperation::Patch { id, patch }),
            PresentOperationDsl::SetSource { source } => PresentOperation::SetSource { source },
            PresentOperationDsl::SetTiles { tiles } => PresentOperation::SetTiles { tiles },
            PresentOperationDsl::SetDeck { deck } => PresentOperation::SetDeck { deck },
        }
    }
}

/// ⚡️ One-line op-text for every `PresentOperation` variant, routed through {@link PresentOperationDsl}
/// (see its doc comment for why a direct `#[derive(dsl::DslOps)]` on `PresentOperation` itself isn't
/// possible).
impl protocol::OpText for PresentOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        PresentOperationDsl::parse_op(line).map(PresentOperationDsl::into)
    }

    fn print_op(&self) -> String {
        PresentOperationDsl::from(self).print_op()
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `PresentOperationDsl` already derives `OpBinary`
/// via `#[derive(dsl::DslOps)]`, so this is a pure `From`/`Into` forward.
impl protocol::OpBinary for PresentOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        PresentOperationDsl::from(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        PresentOperationDsl::decode_op(bytes).map(PresentOperationDsl::into)
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use present::{default_figure_tile_source, default_present_deck, FigureTileFrame};
    use present_engine::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
    use store::test_support;

    fn round_trip(deck: &PresentDeck, operation: &PresentOperation) -> PresentDeck {
        let forward = vcs::apply_operation(deck, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(deck) {
            restored = vcs::apply_operation(&restored, &back);
        }
        assert_eq!(&restored, deck, "backwards() must exactly restore the pre-operation deck");
        forward
    }

    #[test]
    fn set_tiles_and_clear_round_trip() {
        let deck = default_present_deck();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let seeded = round_trip(&deck, &PresentOperation::SetTiles { tiles: tiles.clone() });
        assert_eq!(seeded.tiles.len(), 4);
        let cleared = round_trip(&seeded, &PresentOperation::SetTiles { tiles: Vec::new() });
        assert!(cleared.tiles.is_empty());
    }

    #[test]
    fn tile_add_patch_remove_round_trip() {
        let deck = default_present_deck();
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        let added = round_trip(&deck, &PresentOperation::Tiles(CollectionOperation::Add { id: tile.id.clone(), item: tile, at: 0 }));
        assert_eq!(added.tiles.len(), 1);
        let renamed = round_trip(&added, &PresentOperation::Tiles(CollectionOperation::Patch { id: "t1".into(), patch: FigureTileDraftPatch { name: Some("Renamed".into()), crop: None } }));
        assert_eq!(renamed.tiles[0].name, "Renamed");
        let recropped = round_trip(&renamed, &PresentOperation::Tiles(CollectionOperation::Patch { id: "t1".into(), patch: FigureTileDraftPatch { name: None, crop: Some(FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 }) } }));
        assert_eq!(recropped.tiles[0].crop.width, 0.4);
        let removed = round_trip(&recropped, &PresentOperation::Tiles(CollectionOperation::Remove { id: "t1".into() }));
        assert!(removed.tiles.is_empty());
    }

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_tiles_add() {
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Add { id: tile.id.clone(), item: tile, at: 0 }));
    }

    #[test]
    fn op_text_round_trip_tiles_remove() {
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Remove { id: "t1".into() }));
    }

    #[test]
    fn op_text_round_trip_tiles_move() {
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Move { id: "t1".into(), to: 2 }));
    }

    #[test]
    fn op_text_round_trip_tiles_patch_full() {
        let patch = FigureTileDraftPatch { name: Some("Renamed".into()), crop: Some(FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 }) };
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Patch { id: "t1".into(), patch }));
    }

    #[test]
    fn op_text_round_trip_tiles_patch_empty() {
        let patch = FigureTileDraftPatch { name: None, crop: None };
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Patch { id: "t1".into(), patch }));
    }

    #[test]
    fn op_text_round_trip_set_source() {
        test_support::assert_op_line_round_trip(&PresentOperation::SetSource { source: default_figure_tile_source() });
    }

    #[test]
    fn op_text_round_trip_set_tiles() {
        let source = default_figure_tile_source();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        test_support::assert_op_line_round_trip(&PresentOperation::SetTiles { tiles });
    }

    #[test]
    fn op_text_round_trip_set_deck() {
        test_support::assert_op_line_round_trip(&PresentOperation::SetDeck { deck: default_present_deck() });
    }
    //#endregion 🔖️OpTextTests
}
//#endregion 🧪️Tests
