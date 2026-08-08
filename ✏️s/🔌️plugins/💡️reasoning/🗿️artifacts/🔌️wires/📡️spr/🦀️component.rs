//! ⚖️ Wires artifact — binary command protocol surface + laws (constitutional: spr, renamed from
//! protocol). The app-level `WiresCommand` binary command envelope (the old hand-derived enum this
//! module used to also host) is now REBUILT by `app_commands!` in `crate::apps::wires::component` — see
//! `crate::apps::wires::WiresCommand`'s doc there.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::wires::op::MindmapWiresMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `MindmapWiresMutation` to its binary command form.
pub fn encode_op(operation: &MindmapWiresMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `MindmapWiresMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<MindmapWiresMutation, protocol::ProtocolError> {
    MindmapWiresMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::MindmapWiresDocument;
    use serde_json::json;

    /// 🗄️ Local envelope/store alias for the whole-store tests below — mirrors the `pub type
    /// MindmapWiresEnvelope`/`MindmapWiresStore` the pre-split `semio_s_mindmap` crate exported,
    /// scoped here since this is the only sub-region that still needs it after the taxonomy split.
    type MindmapWiresStore = store::DocumentStore<MindmapWiresDocument, MindmapWiresMutation>;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        let operation = MindmapWiresMutation::AddNode { node };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn store_applies_node_add() {
        let mut store = MindmapWiresStore::new(store::create_document_envelope(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "mindmap-wires", crate::artifacts::wires::empty_mindmap_wires_document(), None));
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![MindmapWiresMutation::AddNode { node }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
    }

    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = MindmapWiresStore::new(store::create_document_envelope(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "mindmap-wires", crate::artifacts::wires::empty_mindmap_wires_document(), None));
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![MindmapWiresMutation::AddNode { node }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `MindmapWiresMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`). Uses `AddNode` (not
    /// `ReplaceDocument`) deliberately — see `op`'s own tests for the known, still-open
    /// `ReplaceDocument` op-text ordering divergence on its raw `DslValue` fields.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let mut store = MindmapWiresStore::new(store::create_document_envelope(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "mindmap-wires", crate::artifacts::wires::empty_mindmap_wires_document(), None));
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![MindmapWiresMutation::AddNode { node }], description: None }).expect("apply");
        let edit: &Edit<MindmapWiresMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<MindmapWiresDocument, MindmapWiresMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
