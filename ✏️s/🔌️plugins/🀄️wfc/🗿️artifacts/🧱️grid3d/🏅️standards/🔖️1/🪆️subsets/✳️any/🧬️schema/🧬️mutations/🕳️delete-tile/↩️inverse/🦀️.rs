//! ↩️ Inverse for `DeleteTile` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::DeleteTile, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    Ok((|| {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else { return Vec::new() };
    let mut steps = vec![create_tile(tile.clone())];
    steps.extend(base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| create_rule(rule.clone())));
    steps.extend(base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| pin_cell(cell.clone())));
    steps.reverse();
    steps

    })())
}
