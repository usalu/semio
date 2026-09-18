//! 🧬️ `wfc3d` artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload, one per `🧬️mutations/<slug>/`
//! leaf wired by the crate root. `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Wfc3dSnapshot>` and `impl protocol::SemanticMutation<Wfc3dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.
//!
//! 🧵 Deliberately NOT `use super::{create_slot, …};` — this file's own `pub use X::x;` builder
//! re-exports below, glob-re-exported back into `mutations` by the crate root's `pub use
//! component::*;`, would collide with a bare-name import of the same sibling submodules (E0252).
//! Fully qualifying each variant's payload path instead breaks that self-referential loop.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = Wfc3dSnapshot, diff = Wfc3dDiff, schema = "wfc3d")]
pub enum Wfc3dMutation {
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
    ChangeSeed(super::change_seed::ChangeSeed),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Wfc3dMutation`] variant, in declaration order — the exact
/// vocabulary the `wfc3d-1-any` mutation catalog (`../../🔮️oracles/🔣️.json`) declares. The framework
/// never parses Rust, so the aggregate unit test is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
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
    "change-seed",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

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

pub type Wfc3dEnvelope = store::ArtifactEnvelope<Wfc3dSnapshot, Wfc3dMutation>;
pub type Wfc3dStore = store::ArtifactStore<Wfc3dSnapshot, Wfc3dMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant.
pub fn apply_wfc3d_mutation(projection: &mut Wfc3dSnapshot, mutation: &Wfc3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_wfc3d_mutation(projection: &Wfc3dSnapshot, mutation: &Wfc3dMutation) -> Vec<Wfc3dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
