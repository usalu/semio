//! 🔺️ Sparse diff builder for `CreateTile` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dRow, Grid3dTilesDelta};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::CreateTile, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if payload.tile.id.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A tile id cannot be empty.".to_string(), [String::new()]);
    }
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A tile with id \"{}\" already exists.", payload.tile.id), [payload.tile.id.clone()]);
    }
    if !payload.tile.weight.is_finite() || payload.tile.weight <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" needs a finite positive weight.", payload.tile.id), [payload.tile.id.clone()]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { tiles: Grid3dTilesDelta::insertion(Grid3dRow::insert_at(&base.tiles, &payload.tile), payload.tile.clone()), ..Default::default() })
}
