//! 🧬️ WFC 2D artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload, one per `🧬️mutations/<slug>/`
//! triad leaf wired by the crate root's `#[path]` tree. `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Wfc2dSnapshot>` and `impl protocol::SemanticMutation<Wfc2dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
// 🧵 Deliberately NOT `use super::{create_slot, …};` — this file's own `pub use X::x;` builder
// re-exports below, glob-re-exported back into `mutations` by the sibling `pub use component::*;`,
// would collide with a bare-name import of the same sibling submodules (E0252). Fully qualifying
// each variant's payload path breaks that self-referential loop.

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = Wfc2dSnapshot, diff = Wfc2dDiff, schema = "wfc2d")]
pub enum Wfc2dMutation {
    ChangeSeed(super::change_seed::ChangeSeed),
    CreateSlot(super::create_slot::CreateSlot),
    DeleteSlot(super::delete_slot::DeleteSlot),
    MoveSlot(super::move_slot::MoveSlot),
    ResizeSlot(super::resize_slot::ResizeSlot),
    ConnectSlots(super::connect_slots::ConnectSlots),
    DisconnectSlots(super::disconnect_slots::DisconnectSlots),
    PinSlot(super::pin_slot::PinSlot),
    UnpinSlot(super::unpin_slot::UnpinSlot),
    CreateTile(super::create_tile::CreateTile),
    DeleteTile(super::delete_tile::DeleteTile),
    ChangeTileWeight(super::change_tile_weight::ChangeTileWeight),
    ChangeTileMedia(super::change_tile_media::ChangeTileMedia),
    CreateRule(super::create_rule::CreateRule),
    DeleteRule(super::delete_rule::DeleteRule),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Wfc2dMutation`] variant, in declaration order — the exact
/// vocabulary the `wfc2d-1-any` mutation catalog (`../../🔮️oracles/🔣️.json`) declares and the
/// `🧩️mutate-wfc2d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "change-seed",
    "create-slot",
    "delete-slot",
    "move-slot",
    "resize-slot",
    "connect-slots",
    "disconnect-slots",
    "pin-slot",
    "unpin-slot",
    "create-tile",
    "delete-tile",
    "change-tile-weight",
    "change-tile-media",
    "create-rule",
    "delete-rule",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️CanonicalOrder
/// 🔢 The position `id` occupies in an ascending-`id` collection — the ONE insertion index every
/// create/connect diff builder uses. Because the position is a pure function of the id set, a
/// delete's inverse re-creates the row at exactly the index it was removed from, which is what the
/// point-invertibility law asks for (never an append).
pub fn ordered_index<T>(items: &[T], id: &str, key: impl Fn(&T) -> &str) -> usize {
    items.iter().position(|item| key(item) > id).unwrap_or(items.len())
}
//#endregion 🔖️CanonicalOrder

//#region 🔖️Builders
pub use super::change_seed::change_seed;
pub use super::change_tile_media::change_tile_media;
pub use super::change_tile_weight::change_tile_weight;
pub use super::connect_slots::connect_slots;
pub use super::create_rule::create_rule;
pub use super::create_slot::create_slot;
pub use super::create_tile::create_tile;
pub use super::delete_rule::delete_rule;
pub use super::delete_slot::delete_slot;
pub use super::delete_tile::delete_tile;
pub use super::disconnect_slots::disconnect_slots;
pub use super::move_slot::move_slot;
pub use super::pin_slot::pin_slot;
pub use super::resize_slot::resize_slot;
pub use super::unpin_slot::unpin_slot;
//#endregion 🔖️Builders

pub type Wfc2dEnvelope = store::ArtifactEnvelope<Wfc2dSnapshot, Wfc2dMutation>;
pub type Wfc2dStore = store::ArtifactStore<Wfc2dSnapshot, Wfc2dMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant.
pub fn apply_wfc2d_mutation(projection: &mut Wfc2dSnapshot, mutation: &Wfc2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_wfc2d_mutation(projection: &Wfc2dSnapshot, mutation: &Wfc2dMutation) -> Vec<Wfc2dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
