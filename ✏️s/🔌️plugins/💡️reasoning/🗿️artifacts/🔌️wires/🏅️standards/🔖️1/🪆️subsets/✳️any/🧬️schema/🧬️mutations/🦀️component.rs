//! ⚡️ Wires artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves below); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<WiresSnapshot>`
//! and `impl protocol::SemanticMutation<WiresSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here (the old `impl Mutation for WiresMutation` + free
//! `apply_wires_mutation`/`inverse_wires_mutation` functions are gone).
//!
//! The ten leaves below are `#[path]`-mounted as siblings of this dispatch file directly in the
//! plugin's `📦️glue.rs` (this facet's fan-out ticket, SEMANTIC-MUTATIONS-OVERHAUL wave-C, owns
//! `📦️glue.rs` for this plugin); the six old generic leaves (`➕add-node`, `➖remove-node`,
//! `✂️remove-edge`, `➕add-relationship`, `🖼️set-snapshot`, `🩹patch-node`) and their `📦️glue.rs`
//! mounts were deleted as part of that same trueing pass.

use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::schema::{array_mut, entity_id};
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
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
/// `pub use create_node::mutation::create_node` (the builder FN of the same name) in the value
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
pub use super::change_node_kind::mutation::{change_node_kind, ChangeNodeKind};
pub use super::change_node_shape::mutation::{change_node_shape, ChangeNodeShape};
pub use super::connect_nodes::mutation::{connect_nodes, ConnectNodes};
pub use super::create_node::mutation::{create_node, CreateNode};
pub use super::delete_node::mutation::{delete_node, DeleteNode};
pub use super::disconnect_nodes::mutation::{disconnect_nodes, DisconnectNodes};
pub use super::edit_node_text::mutation::{edit_node_text, EditNodeText};
pub use super::move_node::mutation::{move_node, MoveNode};
pub use super::resize_node::mutation::{resize_node, ResizeNode};
pub use super::set_node_root::mutation::{set_node_root, SetNodeRoot};
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::empty_wires_snapshot;
    use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
    use protocol::{Mutation, SemanticMutation};
    use serde_json::json;
    use store::apply_mutation;
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
        snapshot = apply_mutation(&snapshot, &create_node(node("node-1", "A")))
            .expect("valid mutation")
            .0;
        snapshot = apply_mutation(&snapshot, &create_node(node("node-2", "B")))
            .expect("valid mutation")
            .0;
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
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
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
    /// ⚖️ `📋️contract-freeze.md` §C2 laws, per verb family (`assert_outcome_policy_matrix` is not yet
    /// landed in `📡️spr/🧪️testkit` — TODO(1-D testkit laws pending) once it lands).
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
    //#endregion 🧪️OutcomeLaws

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_wires_mutation_descriptors();
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
