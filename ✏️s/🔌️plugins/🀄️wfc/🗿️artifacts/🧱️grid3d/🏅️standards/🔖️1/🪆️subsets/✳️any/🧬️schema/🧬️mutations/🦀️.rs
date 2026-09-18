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
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = Grid3dSnapshot, diff = Grid3dDiff, schema = "wfcgrid3d")]
pub enum Grid3dMutation {
    ChangeSeed(super::change_seed::ChangeSeed),
    ResizeGrid(super::resize_grid::ResizeGrid),
    ChangeCellSizes(super::change_cell_sizes::ChangeCellSizes),
    ChangePeriodicity(super::change_periodicity::ChangePeriodicity),
    CreateTile(super::create_tile::CreateTile),
    DeleteTile(super::delete_tile::DeleteTile),
    ChangeTileWeight(super::change_tile_weight::ChangeTileWeight),
    ChangeTileMedia(super::change_tile_media::ChangeTileMedia),
    CreateRule(super::create_rule::CreateRule),
    DeleteRule(super::delete_rule::DeleteRule),
    PinCell(super::pin_cell::PinCell),
    UnpinCell(super::unpin_cell::UnpinCell),
    MaskCell(super::mask_cell::MaskCell),
    UnmaskCell(super::unmask_cell::UnmaskCell),
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
pub use super::change_cell_sizes::change_cell_sizes;
pub use super::change_periodicity::change_periodicity;
pub use super::change_seed::change_seed;
pub use super::change_tile_media::change_tile_media;
pub use super::change_tile_weight::change_tile_weight;
pub use super::create_rule::create_rule;
pub use super::create_tile::create_tile;
pub use super::delete_rule::delete_rule;
pub use super::delete_tile::delete_tile;
pub use super::mask_cell::mask_cell;
pub use super::pin_cell::pin_cell;
pub use super::resize_grid::resize_grid;
pub use super::unmask_cell::unmask_cell;
pub use super::unpin_cell::unpin_cell;
//#endregion 🔖️Builders

//#region 📐️CanonicalOrder
/// 📐️ Where `key` belongs in an already-sorted collection: the index of the existing member with
/// that key, or the insertion point that keeps the collection sorted. Returning the EXISTING index
/// for a key already present is what lets one diff lane serve both upsert-in-place and insert.
pub fn ordered_index<T>(items: &[T], key: &str, item_key: impl Fn(&T) -> String) -> usize {
    items.iter().position(|item| item_key(item).as_str() >= key).unwrap_or(items.len())
}
//#endregion 📐️CanonicalOrder

pub type Grid3dEnvelope = store::ArtifactEnvelope<Grid3dSnapshot, Grid3dMutation>;
pub type Grid3dStore = store::ArtifactStore<Grid3dSnapshot, Grid3dMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant.
pub fn apply_grid3d_mutation(projection: &mut Grid3dSnapshot, mutation: &Grid3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_grid3d_mutation(projection: &Grid3dSnapshot, mutation: &Grid3dMutation) -> Vec<Grid3dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
