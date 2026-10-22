//! 🧬️ WFC 2D artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload, one per `🧬️mutations/<slug>/`
//! triad leaf wired by the crate root's `#[path]` tree. `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Wfc2dSnapshot>` and `impl protocol::SemanticMutation<Wfc2dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.
//!
//! 🧵 Deliberately NOT `use super::{create_slot, …};` — this file's own `pub use X::x;` builder
//! re-exports below, glob-re-exported back into `mutations` by the sibling `pub use component::*;`,
//! would collide with a bare-name import of the same sibling submodules (E0252). Fully qualifying
//! each variant's payload path breaks that self-referential loop.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{ColdMutation, Mutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Wfc2dSnapshot, diff = Wfc2dDiff, schema = "wfc2d", reference = Wfc2dSnapshot, reference_diff = Wfc2dDiff)]
pub enum Wfc2dMutation {
    ChangeSeed(crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed),
    CreateSlot(crate::standards::v1::subsets::any::schema::mutations::create_slot::CreateSlot),
    DeleteSlot(crate::standards::v1::subsets::any::schema::mutations::delete_slot::DeleteSlot),
    MoveSlot(crate::standards::v1::subsets::any::schema::mutations::move_slot::MoveSlot),
    ResizeSlot(crate::standards::v1::subsets::any::schema::mutations::resize_slot::ResizeSlot),
    ConnectSlots(crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots),
    DisconnectSlots(crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::DisconnectSlots),
    PinSlot(crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot),
    UnpinSlot(crate::standards::v1::subsets::any::schema::mutations::unpin_slot::UnpinSlot),
    CreateTile(crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile),
    DeleteTile(crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile),
    ChangeTileWeight(crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::ChangeTileWeight),
    ChangeTileMedia(crate::standards::v1::subsets::any::schema::mutations::change_tile_media::ChangeTileMedia),
    CreateRule(crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule),
    DeleteRule(crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule),
    DragSlots(crate::standards::v1::subsets::any::schema::mutations::drag_slots::DragSlots),
    SetSlotPositions(crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::SetSlotPositions),
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
    "drag-slots",
    "set-slot-positions",
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
pub use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
pub use crate::standards::v1::subsets::any::schema::mutations::connect_slots::connect_slots;
pub use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
pub use crate::standards::v1::subsets::any::schema::mutations::create_slot::create_slot;
pub use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
pub use crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::Wfc2dSlotPosition;
//#endregion 🔖️Builders

pub type Wfc2dEnvelope = store::os_store::store_schema::ArtifactEnvelope<Wfc2dSnapshot, Wfc2dMutation>;
pub type Wfc2dStore = store::ArtifactStore<Wfc2dSnapshot, Wfc2dMutation>;

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_wfc2d_mutation(projection: &Wfc2dSnapshot, mutation: &Wfc2dMutation) -> Result<Vec<Wfc2dMutation>, semio_framework_value::ValueError> {
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
/// 🏗️ Builder of the `change_seed` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_seed(seed: u64) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed::<Wfc2dMutation>(seed)
}

/// 🏗️ Builder of the `change_tile_weight` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_tile_weight(tile_id: String, weight: f64) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight::<Wfc2dMutation>(tile_id, weight)
}

/// 🏗️ Builder of the `delete_rule` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_rule(id: String) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule::<Wfc2dMutation>(id)
}

/// 🏗️ Builder of the `delete_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_slot(id: String) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_slot::delete_slot::<Wfc2dMutation>(id)
}

/// 🏗️ Builder of the `delete_tile` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_tile(id: String) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile::<Wfc2dMutation>(id)
}

/// 🏗️ Builder of the `disconnect_slots` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn disconnect_slots(id: String) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::disconnect_slots::<Wfc2dMutation>(id)
}

/// 🏗️ Builder of the `drag_slots` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn drag_slots(targets: Vec<String>, dx: f64, dy: f64) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::drag_slots::drag_slots::<Wfc2dMutation>(targets, dx, dy)
}

/// 🏗️ Builder of the `move_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn move_slot(id: String, x: f64, y: f64) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::move_slot::move_slot::<Wfc2dMutation>(id, x, y)
}

/// 🏗️ Builder of the `pin_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn pin_slot(id: String, tile_id: String) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::pin_slot::pin_slot::<Wfc2dMutation>(id, tile_id)
}

/// 🏗️ Builder of the `resize_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn resize_slot(id: String, width: f64, height: f64) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::resize_slot::resize_slot::<Wfc2dMutation>(id, width, height)
}

/// 🏗️ Builder of the `set_slot_positions` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn set_slot_positions(positions: Vec<Wfc2dSlotPosition>) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::set_slot_positions::<Wfc2dMutation>(positions)
}

/// 🏗️ Builder of the `unpin_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn unpin_slot(id: String) -> Wfc2dMutation {
    crate::standards::v1::subsets::any::schema::mutations::unpin_slot::unpin_slot::<Wfc2dMutation>(id)
}
//#endregion 🔖️ConcreteBuilders
