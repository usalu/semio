//! ⚡️ Wires artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves below); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<WiresSnapshot>`
//! and `impl protocol::SemanticMutation<WiresSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here (the old `impl Mutation for WiresMutation` + free
//! `apply_wires_mutation`/`inverse_wires_mutation` functions are gone).
//!
//! The ten leaves below are `#[path]`-mounted as siblings of this dispatch file directly in the
//! plugin's `🦀️.rs` (this facet's fan-out ticket, SEMANTIC-MUTATIONS-OVERHAUL wave-C, owns
//! `🦀️.rs` for this plugin); the six old generic leaves (`➕add-node`, `➖remove-node`,
//! `✂️remove-edge`, `➕add-relationship`, `🖼️set-snapshot`, `🩹patch-node`) and their `🦀️.rs`
//! mounts were deleted as part of that same trueing pass.

use crate::diff::WiresDiff;
use crate::schema::{array_mut, entity_id};
use crate::WiresSnapshot;
use dsl::DslValue;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️NodeFieldHelpers
/// 🧬️ Sets one field on the addressed board node inside `board` in place — the shared sparse-diff
/// primitive every single-field node mutation (`move-node`/`resize-node`/`change-node-kind`/
/// `change-node-shape`/`edit-node-text`/`set-node-root`) builds its `🔺️diff` from. No-op when
/// `node_id` isn't found (the diff simply carries no change for a missing target).
pub fn set_node_field(board: &mut DslValue, node_id: &str, key: &str, value: DslValue) {
    if let Some(DslValue::Object(entries)) = array_mut(board, "nodes").iter_mut().find(|node| entity_id(node, "id") == Some(node_id)) {
        match entries.iter_mut().find(|(entry_key, _)| entry_key.as_str() == key) {
            Some((_, slot)) => *slot = value,
            None => entries.push((key.to_string(), value)),
        }
    }
}
//#endregion 🔖️NodeFieldHelpers

//#region 🔖️Mutations
/// 🩹 Every leaf module is addressed `super::<slug>::...` here rather than via a bare `use super::X;`
/// single-ident import — a baseline bug this pass fixed (`E0252`, "the name `create_node` is defined
/// multiple times"): a bare `use super::create_node;` collides with `🔖️Builders`' own
/// `pub use create_node::create_node` (the builder FN of the same name) in the value
/// namespace. Fully-qualifying every reference removes the need for the colliding import outright.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WiresSnapshot, diff = WiresDiff, schema = "s.reasoning.wires")]
pub enum WiresMutation {
    CreateNode(CreateNode),
    DeleteNode(DeleteNode),
    MoveNode(MoveNode),
    ResizeNode(ResizeNode),
    ChangeNodeKind(ChangeNodeKind),
    ChangeNodeShape(ChangeNodeShape),
    EditNodeText(EditNodeText),
    SetNodeRoot(SetNodeRoot),
    ConnectNodes(ConnectNodes),
    DisconnectNodes(DisconnectNodes),
}
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use super::change_node_kind::{change_node_kind, ChangeNodeKind};
pub use super::change_node_shape::{change_node_shape, ChangeNodeShape};
pub use super::connect_nodes::{connect_nodes, ConnectNodes};
pub use super::create_node::{create_node, CreateNode};
pub use super::delete_node::{delete_node, DeleteNode};
pub use super::disconnect_nodes::{disconnect_nodes, DisconnectNodes};
pub use super::edit_node_text::{edit_node_text, EditNodeText};
pub use super::move_node::{move_node, MoveNode};
pub use super::resize_node::{resize_node, ResizeNode};
pub use super::set_node_root::{set_node_root, SetNodeRoot};
//#endregion 🔖️Builders

/// 🏷️ Kebab-case spelling of every [`WiresMutation`] variant, in declaration order — the vocabulary
/// the `wires-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// `📡️mutate-wires-1`'s exhaustive case measures itself against. There is deliberately no
/// `no-mutation` and no `set-snapshot`: the six generic leaves this facet used to carry
/// (`➕add-node`, `➖remove-node`, `✂️remove-edge`, `➕add-relationship`, `🖼️set-snapshot`,
/// `🩹patch-node`) were deleted in the same trueing pass that produced these ten, and whole-document
/// replace reaches the store through `ArtifactStore::reset` instead.
/// [`kinds_match_the_enum_and_the_catalog`] keeps this list honest against the enum, since the
/// framework never parses Rust.
pub const KINDS: &[&str] = &["create-node", "delete-node", "move-node", "resize-node", "change-node-kind", "change-node-shape", "edit-node-text", "set-node-root", "connect-nodes", "disconnect-nodes"];

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's internally-tagged (`{"mutation": "moveNode", …}`, camelCase payload
/// fields) JSON projection — exactly the shape the committed
/// `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` specification vectors and
/// `📡️mutate-wires-1`'s own `Examples` payloads carry — into a real [`WiresMutation`]. The test
/// adapter cannot name this crate's private `dsl`/`protocol`/`store` extern-crate aliases (the
/// generated host links only `semio-repo-test-host` and this crate), so the bridge belongs here
/// rather than there.
pub fn decode_wires_mutation_json(text: &str) -> Result<WiresMutation, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies `mutation` in place and returns every diagnostic it raised as `(code, severity)`
/// pairs. Six of this vocabulary's ten committed specification vectors are NO-OP vectors — an
/// `applied` outcome carrying a `Warning`-level `mutation.no-op` — so the severity is load-bearing
/// here and not a side channel: a refusal and a degenerate application are different answers.
pub fn apply_wires_mutation_reporting(snapshot: &mut WiresSnapshot, mutation: &WiresMutation) -> Vec<(String, String)> {
    let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ The mutation's OWN computed undo steps, which is what an `inverse-<kind>` scenario has to
/// apply for the metamorphic law to mean anything.
pub fn inverse_wires_mutation_steps(mutation: &WiresMutation, base: &WiresSnapshot) -> Vec<WiresMutation> {
    <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
