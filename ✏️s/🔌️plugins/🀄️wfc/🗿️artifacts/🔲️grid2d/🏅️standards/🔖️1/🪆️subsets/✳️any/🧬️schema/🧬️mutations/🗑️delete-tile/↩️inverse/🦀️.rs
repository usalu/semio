//! ↩️ Inverse for `DeleteTile` — recreates the tile first, then every rule and pin the delete
//! cascaded away; each create lands at its own canonical sorted position, so the restored document
//! is byte-identical to the base.

use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::DeleteTile, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    Ok((|| {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_tile(tile.clone())];
    restore.extend(base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| create_rule(rule.clone())));
    restore.extend(base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| pin_cell(cell.x, cell.y, cell.tile_id.clone())));
    restore.reverse();
    restore

    })())
}
