//! 🧬️ `s.wfc.grid3d` — semantic document mutation dispatch. Every variant is a single-field tuple
//! wrapping a handcrafted `protocol::MutationKind` payload, one per `🧬️mutations/<slug>/` triad leaf
//! mounted by the crate root. `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Grid3dSnapshot>`/`SemanticMutation` from those payloads.
//!
//! 📐️ Every collection insert lands at the CANONICAL SORTED position ([`ordered_index`]), never at
//! the end: a delete and its inverse create then round-trip at the same index, which is what the
//! `inverse_restores_before` fixture assertion measures.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::Grid3dSnapshot;
use protocol::{ColdMutation, Mutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Grid3dSnapshot, diff = Grid3dDiff, schema = "wfcgrid3d", reference = Grid3dSnapshot, reference_diff = Grid3dDiff)]
pub enum Grid3dMutation {
    ChangeSeed(crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed),
    ResizeGrid(crate::standards::v1::subsets::any::schema::mutations::resize_grid::ResizeGrid),
    ChangeCellSizes(crate::standards::v1::subsets::any::schema::mutations::change_cell_sizes::ChangeCellSizes),
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
/// 🏷️ The kebab-case spelling of every [`Grid3dMutation`] variant, in declaration order — the exact
/// vocabulary `🔮️oracles/🔣️.json`'s mutation catalog and the `🧩️mutate-wfc-grid3d-1` exhaustive
/// case measure themselves against. The framework never parses Rust, so the unit test beside this
/// file is what keeps the list honest against both.
pub const KINDS: &[&str] = &[
    "change-seed",
    "resize-grid",
    "change-cell-sizes",
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

//#region 🔖️Builders
pub use crate::standards::v1::subsets::any::schema::mutations::change_cell_sizes::change_cell_sizes;
pub use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
pub use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
pub use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
pub use crate::standards::v1::subsets::any::schema::mutations::mask_cell::mask_cell;
pub use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
//#endregion 🔖️Builders

//#region 📐️CanonicalOrder
/// 📐️ Where `key` belongs in an already-sorted collection: the index of the existing member with
/// that key, or the insertion point that keeps the collection sorted. Returning the EXISTING index
/// for a key already present is what lets one diff lane serve both upsert-in-place and insert.
pub fn ordered_index<T>(items: &[T], key: &str, item_key: impl Fn(&T) -> String) -> usize {
    items.iter().position(|item| item_key(item).as_str() >= key).unwrap_or(items.len())
}
//#endregion 📐️CanonicalOrder

pub type Grid3dEnvelope = store::os_store::store_schema::ArtifactEnvelope<Grid3dSnapshot, Grid3dMutation>;
pub type Grid3dStore = store::ArtifactStore<Grid3dSnapshot, Grid3dMutation>;

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_grid3d_mutation(projection: &Grid3dSnapshot, mutation: &Grid3dMutation) -> Result<Vec<Grid3dMutation>, semio_framework_value::ValueError> {
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
/// 🏗️ Builder of the `change_periodicity` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_periodicity(periodic_x: bool, periodic_y: bool, periodic_z: bool) -> Grid3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_periodicity::change_periodicity::<Grid3dMutation>(periodic_x, periodic_y, periodic_z)
}

/// 🏗️ Builder of the `change_seed` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_seed(seed: u64) -> Grid3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed::<Grid3dMutation>(seed)
}

/// 🏗️ Builder of the `change_tile_weight` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_tile_weight(tile_id: String, weight: f64) -> Grid3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight::<Grid3dMutation>(tile_id, weight)
}

/// 🏗️ Builder of the `delete_rule` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_rule(id: String) -> Grid3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule::<Grid3dMutation>(id)
}

/// 🏗️ Builder of the `delete_tile` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_tile(id: String) -> Grid3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile::<Grid3dMutation>(id)
}

/// 🏗️ Builder of the `resize_grid` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn resize_grid(width: u32, height: u32, depth: u32) -> Grid3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::resize_grid::resize_grid::<Grid3dMutation>(width, height, depth)
}

/// 🏗️ Builder of the `unmask_cell` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn unmask_cell(x: u32, y: u32, z: u32) -> Grid3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::unmask_cell::unmask_cell::<Grid3dMutation>(x, y, z)
}

/// 🏗️ Builder of the `unpin_cell` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn unpin_cell(x: u32, y: u32, z: u32) -> Grid3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::unpin_cell::unpin_cell::<Grid3dMutation>(x, y, z)
}
//#endregion 🔖️ConcreteBuilders
