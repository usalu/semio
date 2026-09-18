//! ↩️ Inverse for `DeleteTile` — recreates the tile first, then every rule and pin the delete
//! cascaded away; each create lands at its own canonical sorted position, so the restored document
//! is byte-identical to the base.

use crate::mutations::{create_rule, create_tile, pin_cell, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::DeleteTile, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_tile(tile.clone())];
    restore.extend(base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| create_rule(rule.clone())));
    restore.extend(base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| pin_cell(cell.x, cell.y, cell.tile_id.clone())));
    restore
}
