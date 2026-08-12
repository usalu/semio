//! ⚡️ Wires artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves below); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<WiresSnapshot>`
//! and `impl protocol::SemanticMutation<WiresSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here (the old `impl Mutation for WiresMutation` + free
//! `apply_wires_mutation`/`inverse_wires_mutation` functions are gone).
//!
//! The ten leaves below are self-wired directly (`#[path]`, `🔖️LeafWiring` region) rather than in
//! `📦️glue.rs` — this facet's fan-out (ticket 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL) is scoped to
//! files inside this artifact directory only; `📦️glue.rs` is plugin-shared and out of that scope
//! (same precedent as the already-migrated `🎪️demonstrator/🎪️playground` facet). The six old
//! generic leaves (`➕add-node`, `➖remove-node`, `✂️remove-edge`, `➕add-relationship`,
//! `🖼️set-snapshot`, `🩹patch-node`) stay physically present as orphan stubs (see their own doc
//! comments) because `📦️glue.rs` still `#[path]`-wires them as `pub mod` submodules — cleanup
//! tracked as a `sharedFileRequests` entry in this ticket's wave2 report.

use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::engine::{array_mut, entity_id};
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
pub fn set_node_field(board: &mut DslValue, node_id: &str, key: &str, value: DslValue) {
    if let Some(DslValue::Object(entries)) = array_mut(board, "nodes").iter_mut().find(|node| entity_id(node, "id") == Some(node_id)) {
        match entries.iter_mut().find(|(entry_key, _)| entry_key.as_str() == key) {
            Some((_, slot)) => *slot = value,
            None => entries.push((key.to_string(), value)),
        }
    }
}
//#endregion 🔖️NodeFieldHelpers

//#region 🔖️LeafWiring
#[path = "."]
pub mod create_node {
    #[path = "🌱create-node/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🌱create-node/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🌱create-node/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_node {
    #[path = "🗑️delete-node/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🗑️delete-node/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🗑️delete-node/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod move_node {
    #[path = "🧭move-node/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🧭move-node/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🧭move-node/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod resize_node {
    #[path = "📐resize-node/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "📐resize-node/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "📐resize-node/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_node_kind {
    #[path = "🏷️change-node-kind/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🏷️change-node-kind/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🏷️change-node-kind/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_node_shape {
    #[path = "🔷change-node-shape/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔷change-node-shape/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔷change-node-shape/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod edit_node_text {
    #[path = "✏️edit-node-text/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "✏️edit-node-text/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "✏️edit-node-text/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod set_node_root {
    #[path = "🚩set-node-root/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🚩set-node-root/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🚩set-node-root/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod connect_nodes {
    #[path = "🔗connect-nodes/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔗connect-nodes/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔗connect-nodes/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod disconnect_nodes {
    #[path = "✂️disconnect-nodes/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "✂️disconnect-nodes/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "✂️disconnect-nodes/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
//#endregion 🔖️LeafWiring

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WiresSnapshot, diff = WiresDiff, schema = "s.reasoning.wires")]
pub enum WiresMutation {
    CreateNode(create_node::mutation::CreateNode),
    DeleteNode(delete_node::mutation::DeleteNode),
    MoveNode(move_node::mutation::MoveNode),
    ResizeNode(resize_node::mutation::ResizeNode),
    ChangeNodeKind(change_node_kind::mutation::ChangeNodeKind),
    ChangeNodeShape(change_node_shape::mutation::ChangeNodeShape),
    EditNodeText(edit_node_text::mutation::EditNodeText),
    SetNodeRoot(set_node_root::mutation::SetNodeRoot),
    ConnectNodes(connect_nodes::mutation::ConnectNodes),
    DisconnectNodes(disconnect_nodes::mutation::DisconnectNodes),
}
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use change_node_kind::mutation::{change_node_kind, ChangeNodeKind};
pub use change_node_shape::mutation::{change_node_shape, ChangeNodeShape};
pub use connect_nodes::mutation::{connect_nodes, ConnectNodes};
pub use create_node::mutation::{create_node, CreateNode};
pub use delete_node::mutation::{delete_node, DeleteNode};
pub use disconnect_nodes::mutation::{disconnect_nodes, DisconnectNodes};
pub use edit_node_text::mutation::{edit_node_text, EditNodeText};
pub use move_node::mutation::{move_node, MoveNode};
pub use resize_node::mutation::{resize_node, ResizeNode};
pub use set_node_root::mutation::{set_node_root, SetNodeRoot};
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::empty_wires_snapshot;
    use crate::artifacts::wires::engine::find_board_node;
    use protocol::{Mutation, SemanticMutation};
    use serde_json::json;
    use store::apply_mutation;
    use store::os_store::test_support::assert_op_line_round_trip;

    fn node(id: &str, text: &str) -> DslValue {
        dsl::to_dsl_value(&json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })).unwrap()
    }

    fn round_trip(snapshot: &WiresSnapshot, operation: &WiresMutation) -> WiresSnapshot {
        let forward = apply_mutation(snapshot, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "inverse() must restore the pre-mutation snapshot");
        forward
    }

    #[test]
    fn create_delete_node_round_trip() {
        let snapshot = empty_wires_snapshot();
        let with_node = round_trip(&snapshot, &create_node(node("node-1", "Alpha")));
        assert_eq!(with_node.board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let removed = round_trip(&with_node, &delete_node("node-1".into()));
        assert!(removed.board_fixture.get("nodes").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    #[test]
    fn move_node_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let moved = round_trip(&snapshot, &move_node("node-1".into(), 40.0, 30.0));
        let found = find_board_node(&moved, "node-1").expect("node-1");
        assert_eq!(found.get("x").and_then(|value| value.as_f64()), Some(40.0));
        assert_eq!(found.get("y").and_then(|value| value.as_f64()), Some(30.0));
    }

    #[test]
    fn resize_node_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let resized = round_trip(&snapshot, &resize_node("node-1".into(), Some(48.0), None, None));
        assert_eq!(find_board_node(&resized, "node-1").and_then(|node| node.get("radius")).and_then(|value| value.as_f64()), Some(48.0));
    }

    #[test]
    fn change_node_kind_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let changed = round_trip(&snapshot, &change_node_kind("node-1".into(), "topic".into()));
        assert_eq!(find_board_node(&changed, "node-1").and_then(|node| node.get("nodeKind")).and_then(|value| value.as_str()), Some("topic"));
    }

    #[test]
    fn change_node_shape_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let changed = round_trip(&snapshot, &change_node_shape("node-1".into(), "rectangle".into()));
        assert_eq!(find_board_node(&changed, "node-1").and_then(|node| node.get("shape")).and_then(|value| value.as_str()), Some("rectangle"));
    }

    #[test]
    fn edit_node_text_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let edited = round_trip(&snapshot, &edit_node_text("node-1".into(), "Renamed".into()));
        assert_eq!(find_board_node(&edited, "node-1").and_then(|node| node.get("text")), Some(&DslValue::String("Renamed".into())));
    }

    #[test]
    fn set_node_root_round_trip() {
        let snapshot = round_trip(&empty_wires_snapshot(), &create_node(node("node-1", "Alpha")));
        let set = round_trip(&snapshot, &set_node_root("node-1".into(), true));
        assert_eq!(find_board_node(&set, "node-1").and_then(|node| node.get("root")).and_then(|value| value.as_bool()), Some(true));
    }

    #[test]
    fn connect_disconnect_nodes_round_trip() {
        let mut snapshot = empty_wires_snapshot();
        snapshot = apply_mutation(&snapshot, &create_node(node("node-1", "A")));
        snapshot = apply_mutation(&snapshot, &create_node(node("node-2", "B")));
        let edge = dsl::to_dsl_value(&json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let relationship = dsl::to_dsl_value(&json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 })).unwrap();
        let with_edge = round_trip(&snapshot, &connect_nodes(edge, relationship));
        assert_eq!(with_edge.board_fixture.get("edges").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        assert_eq!(with_edge.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let removed = round_trip(&with_edge, &disconnect_nodes("edge-1".into()));
        assert!(removed.board_fixture.get("edges").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
        assert!(removed.wires_fixture.get("relationships").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    #[test]
    fn op_text_round_trip_create_node() {
        assert_op_line_round_trip(&create_node(node("node-1", "Alpha")));
    }

    #[test]
    fn op_text_round_trip_move_node() {
        assert_op_line_round_trip(&move_node("node-1".into(), 1.0, 2.0));
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
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
