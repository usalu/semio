//! 📦️ Playground artifact — binary document surface + laws.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::playground::PlaygroundSnapshot;
use store::PackError;

/// 📦️ Encodes a `PlaygroundSnapshot` to its binary pack form.
pub fn encode(snapshot: &PlaygroundSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(snapshot)
}

/// 📖️ Decodes a `PlaygroundSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<PlaygroundSnapshot, PackError> {
    <PlaygroundSnapshot as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playground::op::PlaygroundMutation;
    use crate::artifacts::playground::PLAYGROUND_DOCUMENT_SCHEMA;

    #[test]
    fn playground_snapshot_dsl_pack_equivalence() {
        let snapshot = crate::artifacts::playground::engine::empty_playground_snapshot();
        store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
        let bytes = encode(&snapshot);
        assert_eq!(decode(&bytes).expect("decode"), snapshot);
    }

    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<PlaygroundSnapshot, PlaygroundMutation> = DocumentStore::new(
            create_document_envelope(PLAYGROUND_DOCUMENT_SCHEMA, "playground-demo", crate::artifacts::playground::engine::empty_playground_snapshot(), None),
        );
        store
            .dispatch(DocumentCommand::Apply {
                mutations: vec![PlaygroundMutation::SetSnapshot {
                    snapshot: PlaygroundSnapshot { schema: "playground.playground".into() },
                }],
                description: None,
            })
            .expect("apply");
        let edit: &Edit<PlaygroundMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<PlaygroundSnapshot, PlaygroundMutation>(
            edit,
            &DocumentId(store.envelope().id.clone()),
            &SchemaId(store.envelope().schema.clone()),
        );
    }
}
//#endregion 🧪️Tests
