//! 🔺️ Animate present artifact — the operation diff (constitutional: diff).

use crate::artifacts::present::{FigureTileDraft, FigureTileDraftPatch, FigureTileSource, PresentDeck};
use protocol::{CollectionDiff, OperationDiff, Patchable};

//#region 🔖️CollectionSupport
// 🪪️ `Identified<String> for FigureTileDraft` / `Patchable<FigureTileDraftPatch> for FigureTileDraft`
// live in the artifact's own component file, not here — Rust's orphan rules require a foreign-trait
// impl (`protocol::Identified`/`protocol::Patchable`) to share a crate with the type it's implemented
// for, and this is satisfied by any node inside this crate, so the impls stay next to the type.

pub(crate) fn apply_tile_diff(tiles: &mut Vec<FigureTileDraft>, diff: &CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>) {
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

pub(crate) fn absorb_tile_diff(target: &mut Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>, incoming: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>) {
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

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
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
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::default_present_deck;
    use crate::artifacts::present::op::PresentOperation;
    use protocol::Operation;

    /// ⚖️ LAW: `op.diff(base)` applied to `base` equals applying the operation, and the diff carries only
    /// the touched slot — the `OperationDiff` contract undo/redo rides on.
    #[test]
    fn set_source_diff_applies_onto_the_base_projection() {
        let base = default_present_deck();
        let mut next_source = base.source.clone();
        next_source.kind = "video".into();
        let operation = PresentOperation::SetSource { source: next_source.clone() };
        let diff: PresentDiff = operation.diff(&base);
        assert_eq!(diff.source, Some(next_source));
        assert!(diff.deck.is_none() && diff.tiles.is_none() && diff.set_tiles.is_none());
        assert_eq!(diff.apply(&base).source.kind, "video");
    }
}
//#endregion 🧪️Tests
