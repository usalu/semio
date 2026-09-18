//! 🧬️ `s.wfc.grid2d` — semantic document mutation dispatch. Every variant wraps one handcrafted
//! `protocol::MutationKind` payload from its own `🧬️mutations/<slug>/` triad leaf; `dsl::Mutations`
//! generates the `protocol::Mutation`/`protocol::SemanticMutation` impls, so no apply/diff/inverse
//! dispatch is hand-written here. The canonical-position helpers below are the law every collection
//! mutation obeys: tiles and rules are kept sorted by id, pinned/masked cells row-major, so a
//! delete's inverse restores POSITION as well as VALUE.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcTile2d};
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = Grid2dSnapshot, diff = Grid2dDiff, schema = "wfcgrid2d")]
pub enum Grid2dMutation {
    ChangeSeed(super::change_seed::ChangeSeed),
    ResizeGrid(super::resize_grid::ResizeGrid),
    ChangeCellSize(super::change_cell_size::ChangeCellSize),
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
pub use super::change_cell_size::change_cell_size;
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

pub type Grid2dEnvelope = store::ArtifactEnvelope<Grid2dSnapshot, Grid2dMutation>;
pub type Grid2dStore = store::ArtifactStore<Grid2dSnapshot, Grid2dMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant.
pub fn apply_grid2d_mutation(projection: &mut Grid2dSnapshot, mutation: &Grid2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_grid2d_mutation(projection: &Grid2dSnapshot, mutation: &Grid2dMutation) -> Vec<Grid2dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
