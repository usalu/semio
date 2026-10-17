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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Wfc3dSnapshot, diff = Wfc3dDiff, schema = "wfc3d")]
pub enum Wfc3dMutation {
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
    ChangeSeed(crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed),
    DragSlots(crate::standards::v1::subsets::any::schema::mutations::drag_slots::DragSlots),
    SetSlotPositions(crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::SetSlotPositions),
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
    "drag-slots",
    "set-slot-positions",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
pub use crate::standards::v1::subsets::any::schema::mutations::connect_slots::connect_slots;
pub use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
pub use crate::standards::v1::subsets::any::schema::mutations::create_slot::create_slot;
pub use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
pub use crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::Wfc3dSlotPosition;
//#endregion 🔖️Builders

pub type Wfc3dEnvelope = store::os_store::store_schema::ArtifactEnvelope<Wfc3dSnapshot, Wfc3dMutation>;
pub type Wfc3dStore = store::ArtifactStore<Wfc3dSnapshot, Wfc3dMutation>;

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_wfc3d_mutation(projection: &Wfc3dSnapshot, mutation: &Wfc3dMutation) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
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
pub fn change_seed(seed: u64) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed::<Wfc3dMutation>(seed)
}

/// 🏗️ Builder of the `change_tile_weight` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_tile_weight(id: String, weight: f64) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight::<Wfc3dMutation>(id, weight)
}

/// 🏗️ Builder of the `delete_rule` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_rule(id: String) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule::<Wfc3dMutation>(id)
}

/// 🏗️ Builder of the `delete_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_slot(id: String) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_slot::delete_slot::<Wfc3dMutation>(id)
}

/// 🏗️ Builder of the `delete_tile` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_tile(id: String) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile::<Wfc3dMutation>(id)
}

/// 🏗️ Builder of the `disconnect_slots` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn disconnect_slots(id: String) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::disconnect_slots::<Wfc3dMutation>(id)
}

/// 🏗️ Builder of the `drag_slots` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn drag_slots(targets: Vec<String>, dx: f64, dy: f64, dz: f64) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::drag_slots::drag_slots::<Wfc3dMutation>(targets, dx, dy, dz)
}

/// 🏗️ Builder of the `move_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn move_slot(id: String, x: f64, y: f64, z: f64) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::move_slot::move_slot::<Wfc3dMutation>(id, x, y, z)
}

/// 🏗️ Builder of the `pin_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn pin_slot(id: String, tile_id: String) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::pin_slot::pin_slot::<Wfc3dMutation>(id, tile_id)
}

/// 🏗️ Builder of the `resize_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn resize_slot(id: String, width: f64, height: f64, depth: f64) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::resize_slot::resize_slot::<Wfc3dMutation>(id, width, height, depth)
}

/// 🏗️ Builder of the `set_slot_positions` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn set_slot_positions(positions: Vec<Wfc3dSlotPosition>) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::set_slot_positions::<Wfc3dMutation>(positions)
}

/// 🏗️ Builder of the `unpin_slot` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn unpin_slot(id: String) -> Wfc3dMutation {
    crate::standards::v1::subsets::any::schema::mutations::unpin_slot::unpin_slot::<Wfc3dMutation>(id)
}
//#endregion 🔖️ConcreteBuilders
