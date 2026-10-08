//! 🔺️ Sparse diff builder for `CreateTile` — a real id-keyed upsert at the canonical sorted
//! position (never an append, so `delete-tile`'s inverse round-trips position as well as value).

use crate::diff::{Grid2dDiff, Grid2dRow, Grid2dTilesDelta};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::CreateTile, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if payload.tile.id.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A tile id may not be empty.".to_string(), ["tile".to_string()]);
    }
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A tile with id \"{}\" already exists.", payload.tile.id), [payload.tile.id.clone()]);
    }
    if !(payload.tile.weight.is_finite() && payload.tile.weight > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" must carry a finite positive weight, got {}.", payload.tile.id, payload.tile.weight), [payload.tile.id.clone()]);
    }
    protocol::MutationOutcome::new(Grid2dDiff { tiles: Grid2dTilesDelta::insertion(Grid2dRow::insert_at(&base.tiles, &payload.tile), payload.tile.clone()), ..Default::default() })
}
