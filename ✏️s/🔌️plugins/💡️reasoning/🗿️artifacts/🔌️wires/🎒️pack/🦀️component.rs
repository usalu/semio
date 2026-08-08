//! 📦️ Wires artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::wires::MindmapWiresDocument;
use store::PackError;

/// 📦️ Encodes a `MindmapWiresDocument` to its binary pack form.
pub fn encode(document: &MindmapWiresDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `MindmapWiresDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<MindmapWiresDocument, PackError> {
    <MindmapWiresDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::dsl as wires_dsl;
    use crate::artifacts::wires::op::MindmapWiresMutation;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl_metabolism() {
        let document = wires_dsl::parse_dsl(wires_dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT).expect("parse metabolism example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_empty_document() {
        let document = crate::artifacts::wires::empty_mindmap_wires_document();
        store::test_support::assert_dsl_pack_equivalence(&document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `MindmapWiresMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`). Uses `AddNode` (not
    /// `ReplaceDocument`) deliberately — see `op`'s own tests for the known, still-open
    /// `ReplaceDocument` op-text ordering divergence on its raw `DslValue` fields.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};
        use serde_json::json;
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<MindmapWiresDocument, MindmapWiresMutation> = DocumentStore::new(create_document_envelope(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "mindmap-wires", crate::artifacts::wires::empty_mindmap_wires_document(), None));
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        store.dispatch(DocumentCommand::Apply { mutations: vec![MindmapWiresMutation::AddNode { node }], description: None }).expect("apply");
        let edit: &Edit<MindmapWiresMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<MindmapWiresDocument, MindmapWiresMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
