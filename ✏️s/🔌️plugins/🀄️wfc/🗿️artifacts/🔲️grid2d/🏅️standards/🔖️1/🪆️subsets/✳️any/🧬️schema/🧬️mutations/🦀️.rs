//! 🧬️ `s.wfc.grid2d` — semantic document mutation dispatch. Every variant wraps one handcrafted
//! `protocol::MutationKind` payload from its own `🧬️mutations/<slug>/` triad leaf; `dsl::Mutations`
//! generates the `protocol::Mutation`/`protocol::SemanticMutation` impls, so no apply/diff/inverse
//! dispatch is hand-written here. The canonical-position helpers below are the law every collection
//! mutation obeys: tiles and rules are kept sorted by id, pinned/masked cells row-major, so a
//! delete's inverse restores POSITION as well as VALUE.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcTile2d};
use protocol::{ColdMutation, Mutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Grid2dSnapshot, diff = Grid2dDiff, schema = "wfcgrid2d", reference = Grid2dSnapshot, reference_diff = Grid2dDiff)]
pub enum Grid2dMutation {
    ChangeSeed(crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed),
    ResizeGrid(crate::standards::v1::subsets::any::schema::mutations::resize_grid::ResizeGrid),
    ChangeCellSize(crate::standards::v1::subsets::any::schema::mutations::change_cell_size::ChangeCellSize),
    ChangePeriodicity(crate::standards::v1::subsets::any::schema::mutations::change_periodicity::ChangePeriodicity),
    CreateTile(crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile),
    DeleteTile(crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile),
    ChangeTileWeight(crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::ChangeTileWeight),
    ChangeTileMedia(crate::standards::v1::subsets::any::schema::mutations::change_tile_media::ChangeTileMedia),
    CreateRule(crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule),
    DeleteRule(crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule),
    PinCell(crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell),
    UnpinCell(crate::standards::v1::subsets::any::schema::mutations::unpin_cell::UnpinCell),
    MaskCell(crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell),
    UnmaskCell(crate::standards::v1::subsets::any::schema::mutations::unmask_cell::UnmaskCell),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Grid2dMutation`] variant, in declaration order — the exact
/// vocabulary `🔮️oracles/🔣️.json`'s mutation catalog and the `🥒️.feature`/`🐍️.py` oracle replay
/// measure themselves against.
pub const KINDS: &[&str] = &[
    "change-seed",
    "resize-grid",
    "change-cell-size",
    "change-periodicity",
    "create-tile",
    "delete-tile",
    "change-tile-weight",
    "change-tile-media",
    "create-rule",
    "delete-rule",
    "pin-cell",
    "unpin-cell",
    "mask-cell",
    "unmask-cell",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️CanonicalPosition
/// 📐️ Where an id-keyed row belongs in a list kept sorted by id — the insertion point, never the
/// end of the list (`📓️explore-artifact-taxonomy-template.md` §3.3: an append-shaped insert makes
/// its own `inverse_restores_before` test fail the moment a row is removed from the middle).
pub fn ordered_index<T>(items: &[T], id: &str, key: impl Fn(&T) -> &str) -> usize {
    items.iter().position(|item| key(item) > id).unwrap_or(items.len())
}

pub fn ordered_tile_index(tiles: &[WfcTile2d], id: &str) -> usize {
    ordered_index(tiles, id, |tile| tile.id.as_str())
}

pub fn ordered_rule_index(rules: &[WfcAdjacencyRule2d], id: &str) -> usize {
    ordered_index(rules, id, |rule| rule.id.as_str())
}

/// 📐️ Where a grid cell belongs in a row-major cell list.
pub fn ordered_cell_index<T>(cells: &[T], x: u32, y: u32, key: impl Fn(&T) -> (u32, u32)) -> usize {
    cells.iter().position(|cell| key(cell) > (y, x)).unwrap_or(cells.len())
}
//#endregion 🔖️CanonicalPosition

//#region 🔖️Builders
pub use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
//#endregion 🔖️Builders

pub type Grid2dEnvelope = store::os_store::store_schema::ArtifactEnvelope<Grid2dSnapshot, Grid2dMutation>;
pub type Grid2dStore = store::ArtifactStore<Grid2dSnapshot, Grid2dMutation>;

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_grid2d_mutation(projection: &Grid2dSnapshot, mutation: &Grid2dMutation) -> Result<Vec<Grid2dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ConcreteBuilders
/// 🏗️ Builder of the `change_cell_size` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_cell_size(cell_width: f64, cell_height: f64) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_cell_size::change_cell_size::<Grid2dMutation>(cell_width, cell_height)
}

/// 🏗️ Builder of the `change_periodicity` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_periodicity(periodic_x: bool, periodic_y: bool) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_periodicity::change_periodicity::<Grid2dMutation>(periodic_x, periodic_y)
}

/// 🏗️ Builder of the `change_seed` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_seed(seed: u64) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed::<Grid2dMutation>(seed)
}

/// 🏗️ Builder of the `change_tile_weight` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_tile_weight(id: String, weight: f64) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight::<Grid2dMutation>(id, weight)
}

/// 🏗️ Builder of the `create_rule` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn create_rule(rule: WfcAdjacencyRule2d) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule::<Grid2dMutation>(rule)
}

/// 🏗️ Builder of the `create_tile` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn create_tile(tile: WfcTile2d) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile::<Grid2dMutation>(tile)
}

/// 🏗️ Builder of the `delete_rule` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_rule(id: String) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule::<Grid2dMutation>(id)
}

/// 🏗️ Builder of the `delete_tile` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_tile(id: String) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile::<Grid2dMutation>(id)
}

/// 🏗️ Builder of the `mask_cell` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn mask_cell(x: u32, y: u32) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::mask_cell::mask_cell::<Grid2dMutation>(x, y)
}

/// 🏗️ Builder of the `pin_cell` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn pin_cell(x: u32, y: u32, tile_id: String) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell::<Grid2dMutation>(x, y, tile_id)
}

/// 🏗️ Builder of the `resize_grid` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn resize_grid(width: u32, height: u32) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::resize_grid::resize_grid::<Grid2dMutation>(width, height)
}

/// 🏗️ Builder of the `unmask_cell` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn unmask_cell(x: u32, y: u32) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::unmask_cell::unmask_cell::<Grid2dMutation>(x, y)
}

/// 🏗️ Builder of the `unpin_cell` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn unpin_cell(x: u32, y: u32) -> Grid2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::unpin_cell::unpin_cell::<Grid2dMutation>(x, y)
}
//#endregion 🔖️ConcreteBuilders
