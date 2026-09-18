//! ↩️ Inverse for `DeleteTile` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::DeleteTile, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else { return Vec::new() };
    let mut steps = vec![crate::mutations::create_tile(tile.clone())];
    steps.extend(base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| crate::mutations::create_rule(rule.clone())));
    steps.extend(base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| crate::mutations::pin_cell(cell.clone())));
    steps
}
