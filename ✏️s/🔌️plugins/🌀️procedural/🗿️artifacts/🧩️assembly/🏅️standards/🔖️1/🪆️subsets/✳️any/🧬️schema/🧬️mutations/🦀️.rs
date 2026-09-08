//! 🧬️ Assembly artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload, one per
//! `🧬️mutations/<slug>/` triad leaf wired by `🦀️.rs`. `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<AssemblySnapshot>` and `impl protocol::SemanticMutation<AssemblySnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.

use crate::diff::AssemblyDiff;
use crate::schema::snapshot::AssemblySnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
// 🧵 Deliberately NOT `use super::{create_slot, ...};` — this file's own `pub use X::mutation::x;`
// builder re-exports below, glob-re-exported back into `mutations` by the sibling `pub use
// component::*;` in `🦀️.rs`, would collide with a bare-name import of the same sibling
// submodules (E0252, hit and fixed once already this wave) — fully qualifying each variant's
// payload path below instead breaks that self-referential loop.

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = AssemblySnapshot, diff = AssemblyDiff, schema = "assembly")]
pub enum AssemblyMutation {
    CreateSlot(super::create_slot::CreateSlot),
    DeleteSlot(super::delete_slot::DeleteSlot),
    CreateRule(super::create_rule::CreateRule),
    DeleteRule(super::delete_rule::DeleteRule),
    ChangeWeight(super::change_weight::ChangeWeight),
    RemoveWeight(super::remove_weight::RemoveWeight),
    ConnectSlots(super::connect_slots::ConnectSlots),
    DisconnectSlots(super::disconnect_slots::DisconnectSlots),
    ChangeSeed(super::change_seed::ChangeSeed),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`AssemblyMutation`] variant, in declaration order — the exact
/// vocabulary the `assembly-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧩️mutate-assembly-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &["create-slot", "delete-slot", "create-rule", "delete-rule", "change-weight", "remove-weight", "connect-slots", "disconnect-slots", "change-seed"];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use super::change_seed::change_seed;
pub use super::change_weight::change_weight;
pub use super::connect_slots::connect_slots;
pub use super::create_rule::create_rule;
pub use super::create_slot::create_slot;
pub use super::delete_rule::delete_rule;
pub use super::delete_slot::delete_slot;
pub use super::disconnect_slots::disconnect_slots;
pub use super::remove_weight::remove_weight;
//#endregion 🔖️Builders

pub type AssemblyEnvelope = store::ArtifactEnvelope<AssemblySnapshot, AssemblyMutation>;
pub type AssemblyStore = store::ArtifactStore<AssemblySnapshot, AssemblyMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant.
pub fn apply_assembly_mutation(projection: &mut AssemblySnapshot, mutation: &AssemblyMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_assembly_mutation(projection: &AssemblySnapshot, mutation: &AssemblyMutation) -> Vec<AssemblyMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
