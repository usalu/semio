//! 📦️ Playbook artifact — binary document surface + laws (constitutional: pack).
//!
//! `store::DocumentPack for PlaybookSpec` is implemented directly in the shared `playbook` kernel crate;
//! see `🗿️artifacts/📖️playbook/🦀️component.rs` for why. This component only adds the thin artifact-facing
//! `encode`/`decode` wrappers plus the pack↔dsl equivalence law and the command-envelope round-trip law.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::playbook::PlaybookSpec;
use store::PackError;

/// 📦️ Encodes a `PlaybookSpec` to its binary pack form.
pub fn encode(document: &PlaybookSpec) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `PlaybookSpec` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<PlaybookSpec, PackError> {
    <PlaybookSpec as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::engine::empty_playbook_projection;
    use crate::artifacts::playbook::{dsl, PLAYBOOK_DOCUMENT_SCHEMA};

    #[test]
    fn pack_round_trips_the_empty_projection() {
        let document = empty_playbook_projection();
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn facade_generator_example_pack_round_trips() {
        let document = dsl::parse_dsl(dsl::FACADE_GENERATOR_EXAMPLE_TEXT).expect("📖️facade-generator.playbook parses");
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn facade_generator_example_pack_agrees_with_dsl() {
        let document = dsl::parse_dsl(dsl::FACADE_GENERATOR_EXAMPLE_TEXT).expect("📖️facade-generator.playbook parses");
        store::test_support::assert_dsl_pack_equivalence(&document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `PlaybookOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this file's
    /// existing dsl/pack round-trip laws (same pattern as `mathematical_pack`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::playbook::op::{update_playbook_title_operation, PlaybookOperation};
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<PlaybookSpec, PlaybookOperation> = DocumentStore::new(create_document_envelope(PLAYBOOK_DOCUMENT_SCHEMA, "playbook-demo", empty_playbook_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![update_playbook_title_operation(Some("Recipe".into()))], description: None }).expect("apply");
        let edit: &Edit<PlaybookOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<PlaybookSpec, PlaybookOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
