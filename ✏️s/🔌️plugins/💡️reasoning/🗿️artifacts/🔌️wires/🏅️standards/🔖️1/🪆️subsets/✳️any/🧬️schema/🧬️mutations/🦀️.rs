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

use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::schema::{array_mut, entity_id};
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use serde::{Deserialize, Serialize};

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
pub async fn set_node_field(board: &mut DslValue, node_id: &str, key: &str, value: DslValue) {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
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
/// `mutate-wires-1`'s exhaustive case measures itself against. There is deliberately no
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
/// `mutate-wires-1`'s own `Examples` payloads carry — into a real [`WiresMutation`]. The test
/// adapter cannot reach `serde_json` (the generated host links only `semio-repo-test-host` and this
/// crate) and cannot name this crate's private `protocol`/`store` extern-crate aliases either, so
/// the bridge belongs here rather than there.
pub fn decode_wires_mutation_json(text: &str) -> Result<WiresMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
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
mod tests {
    use super::*;
    use crate::artifacts::wires::empty_wires_snapshot;
    use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
    use protocol::{Mutation, SemanticMutation};
    use serde_json::json;
    use store::apply_mutation;

    /// 🏷️ The three declarations of this vocabulary — the enum, [`KINDS`] and the committed catalog
    /// — must agree, in spelling AND in order. The framework never parses Rust, so without this
    /// test `KINDS` could drift from the enum and the catalog could keep measuring `mutate-wires-1`
    /// against a vocabulary the artifact no longer has.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = WiresMutation::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
        assert!(!manifest.contains("\"set-snapshot\"") && !manifest.contains("\"patch-node\""), "the deleted generic leaves must not reappear in the catalog");
    }
    use store::os_store::test_support::assert_op_line_round_trip;

    async fn node(id: &str, text: &str) -> DslValue {
        dsl::to_dsl_value(&json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })).unwrap()
    }

    async fn round_trip(snapshot: &WiresSnapshot, operation: &WiresMutation) -> WiresSnapshot {
        let (forward, _messages) = apply_mutation(snapshot, operation).expect("valid mutation");
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            let (next, _messages) = apply_mutation(&restored, &back).expect("valid inverse mutation");
            restored = next;
        }
        assert_eq!(&restored, snapshot, "inverse() must restore the pre-mutation snapshot");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn create_delete_node_round_trip() {
        let snapshot = empty_wires_snapshot();
        let with_node = round_trip(&snapshot, &create_node(node("node-1", "Alpha")));
        assert_eq!(crate::artifacts::wires::wires_working_board(&with_node).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let removed = round_trip(&with_node, &delete_node("node-1".into()));
        assert!(crate::artifacts::wires::wires_working_board(&removed).get("nodes").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_node_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let moved = round_trip(&snapshot, &move_node("node-1".into(), 40.0, 30.0));
        let found = find_board_node(&moved, "node-1").expect("node-1");
        assert_eq!(found.get("x").and_then(|value| value.as_f64()), Some(40.0));
        assert_eq!(found.get("y").and_then(|value| value.as_f64()), Some(30.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn resize_node_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let resized = round_trip(&snapshot, &resize_node("node-1".into(), Some(48.0), None, None));
        assert_eq!(find_board_node(&resized, "node-1").and_then(|node| node.get("radius").and_then(|value| value.as_f64())), Some(48.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_node_kind_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let changed = round_trip(&snapshot, &change_node_kind("node-1".into(), "topic".into()));
        assert_eq!(find_board_node(&changed, "node-1").and_then(|node| node.get("nodeKind").and_then(|value| value.as_str()).map(str::to_string)), Some("topic".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_node_shape_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let changed = round_trip(&snapshot, &change_node_shape("node-1".into(), "rectangle".into()));
        assert_eq!(find_board_node(&changed, "node-1").and_then(|node| node.get("shape").and_then(|value| value.as_str()).map(str::to_string)), Some("rectangle".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_node_text_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let edited = round_trip(&snapshot, &edit_node_text("node-1".into(), "Renamed".into()));
        assert_eq!(find_board_node(&edited, "node-1").and_then(|node| node.get("text").cloned()), Some(DslValue::String("Renamed".into())));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_node_root_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let set = round_trip(&snapshot, &set_node_root("node-1".into(), true));
        assert_eq!(find_board_node(&set, "node-1").and_then(|node| node.get("root").and_then(|value| value.as_bool())), Some(true));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_disconnect_nodes_round_trip() {
        let mut snapshot = empty_wires_snapshot();
        snapshot = apply_mutation(&snapshot, &create_node(node("node-1", "A"))).expect("valid mutation").0;
        snapshot = apply_mutation(&snapshot, &create_node(node("node-2", "B"))).expect("valid mutation").0;
        let edge = dsl::to_dsl_value(&json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let relationship = dsl::to_dsl_value(&json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 })).unwrap();
        let with_edge = round_trip(&snapshot, &connect_nodes(edge, relationship));
        assert_eq!(crate::artifacts::wires::wires_working_board(&with_edge).get("edges").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        assert_eq!(with_edge.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let removed = round_trip(&with_edge, &disconnect_nodes("edge-1".into()));
        assert!(crate::artifacts::wires::wires_working_board(&removed).get("edges").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
        assert!(removed.wires_fixture.get("relationships").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_create_node() {
        assert_op_line_round_trip(&create_node(node("node-1", "Alpha")));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_move_node() {
        assert_op_line_round_trip(&move_node("node-1".into(), 1.0, 2.0));
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
    /// (reachable here as `protocol::testkit`), exercised against the two most structurally distinct
    /// kinds: an id-keyed create/delete pair (`create-node`) and a single-field addressed setter
    /// (`move-node`).
    #[semio_framework_async_macros::async_test]
    async fn create_node_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_wires_snapshot();
        let mutation = create_node(node("node-1", "Alpha"));
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = create_node(node("node-2", "Beta")).diff(&base).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn move_node_satisfies_the_inverse_law() {
        let base = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let mutation = move_node("node-1".into(), 40.0, 30.0);
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
    }
    //#endregion 🧪️MutationLaws

    //#region 🧪️OutcomeLaws
    /// ⚖️ `📋️contract-freeze.md` §C2 laws, per verb family. `assert_outcome_policy_matrix` cases sit
    /// below, one call per verb family present in `WiresMutation`'s 10 kinds (create; delete;
    /// move+resize; change+set; edit; connect+disconnect).
    #[semio_framework_async_macros::async_test]
    async fn delete_missing_node_is_a_target_missing_error() {
        let base = empty_wires_snapshot();
        protocol::testkit::assert_missing_target_is_error(&base, &delete_node("does-not-exist".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_missing_node_is_a_target_missing_error() {
        let base = empty_wires_snapshot();
        protocol::testkit::assert_missing_target_is_error(&base, &move_node("does-not-exist".into(), 1.0, 2.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn disconnect_missing_edge_is_a_target_missing_error() {
        let base = empty_wires_snapshot();
        protocol::testkit::assert_missing_target_is_error(&base, &disconnect_nodes("does-not-exist".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_node_duplicate_id_never_applies() {
        let base = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let duplicate = create_node(node("node-1", "Alpha Again"));
        protocol::testkit::assert_fatal_never_applies(&duplicate.diff(&base));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_node_outcome_obeys_the_policy_matrix() {
        let base = empty_wires_snapshot();
        protocol::testkit::assert_outcome_policy_matrix(&base, &create_node(node("node-1", "Alpha")));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_node_outcome_obeys_the_policy_matrix() {
        let base = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        protocol::testkit::assert_outcome_policy_matrix(&base, &delete_node("node-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_and_resize_node_outcomes_obey_the_policy_matrix() {
        let base = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        protocol::testkit::assert_outcome_policy_matrix(&base, &move_node("node-1".into(), 40.0, 30.0));
        protocol::testkit::assert_outcome_policy_matrix(&base, &resize_node("node-1".into(), Some(48.0), None, None));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_and_set_node_outcomes_obey_the_policy_matrix() {
        let base = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        protocol::testkit::assert_outcome_policy_matrix(&base, &change_node_kind("node-1".into(), "topic".into()));
        protocol::testkit::assert_outcome_policy_matrix(&base, &change_node_shape("node-1".into(), "rectangle".into()));
        protocol::testkit::assert_outcome_policy_matrix(&base, &set_node_root("node-1".into(), true));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_node_text_outcome_obeys_the_policy_matrix() {
        let base = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        protocol::testkit::assert_outcome_policy_matrix(&base, &edit_node_text("node-1".into(), "Renamed".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_and_disconnect_nodes_outcomes_obey_the_policy_matrix() {
        let mut snapshot = empty_wires_snapshot();
        snapshot = apply_mutation(&snapshot, &create_node(node("node-1", "A"))).expect("valid mutation").0;
        snapshot = apply_mutation(&snapshot, &create_node(node("node-2", "B"))).expect("valid mutation").0;
        let edge = dsl::to_dsl_value(&json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let relationship = dsl::to_dsl_value(&json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 })).unwrap();
        let connect = connect_nodes(edge, relationship);
        let with_edge = round_trip(&snapshot, &connect);
        protocol::testkit::assert_outcome_policy_matrix(&snapshot, &connect);
        protocol::testkit::assert_outcome_policy_matrix(&with_edge, &disconnect_nodes("edge-1".into()));
    }
    //#endregion 🧪️OutcomeLaws

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_wires_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        assert_eq!(WiresMutation::kinds().len(), 10);
        for kind in WiresMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        let mutation = create_node(node("node-1", "Alpha"));
        assert_eq!(mutation.semantics().kind, "create-node");
        assert_eq!(mutation.semantics().record, "CreatedNode");
    }
}
//#endregion 🧪️Tests
