//! ⚖️ Writer artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! This component only carries the artifact-facing `encode_op`/`decode_op` wrappers plus the op
//! text↔binary equivalence law and a whole-store round trip. The app's typed `WriterCommand` enum —
//! which used to share the old `📡️protocol` crate with this codec — is an EDITOR-surface concern, not
//! an artifact one: it now lives in the subset's `✏️editor/🦀️component.rs`, assembled from the
//! `🎮️commands/*` payload
//! modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::writer::op::WriterMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `WriterMutation` to its binary state-patch form.
pub async fn encode_op(operation: &WriterMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `WriterMutation` from its binary state-patch form.
pub async fn decode_op(bytes: &[u8]) -> Result<WriterMutation, protocol::ProtocolError> {
    WriterMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::{schema, WriterSnapshot};

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text: "hello".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// ✍️ Hand-built representative document — used across the artifact's own component tests.
    async fn jack_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
    }

    /// 🧬️ Reaches `jack_snapshot()` from `empty_writer_snapshot()` via the semantic vocabulary —
    /// `SetSnapshot` (whole-document replace) is banned, so what used to be one mutation is now the
    /// sequence of scalar mutations that actually differ between the two documents (`schema` is
    /// identical in both, so it gets no mutation). `EditText` mints its `document` handle from
    /// `base.id`/`base.language_id` at apply time, so it must run LAST, after `RenameWriter`/
    /// `ChangeLanguage` have already landed — otherwise its handle would target the wrong owner id.
    async fn jack_mutations() -> Vec<WriterMutation> {
        let jack = jack_snapshot();
        let text = crate::artifacts::writer::writer_text(&jack);
        vec![
            WriterMutation::RenameWriter(crate::artifacts::writer::schema::mutations::RenameWriter { new_id: jack.id }),
            WriterMutation::ChangeLanguage(crate::artifacts::writer::schema::mutations::ChangeLanguage { new_language_id: jack.language_id }),
            WriterMutation::ChangeUri(crate::artifacts::writer::schema::mutations::ChangeUri { new_uri: jack.uri }),
            WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text }),
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_document_text_round_trips_through_the_store() {
        let mut store = store::ArtifactStore::<WriterSnapshot, WriterMutation>::new(store::create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: jack_mutations(), description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot"), jack_snapshot());
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `WriterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip law.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{ArtifactId, Edit, SchemaId};

        let mut store = store::ArtifactStore::<WriterSnapshot, WriterMutation>::new(store::create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: jack_mutations(), description: None }).expect("apply");
        let edit: &Edit<WriterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<WriterSnapshot, WriterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[semio_framework_async_macros::async_test]
    async fn verify_protocol_bytes_against_encoded_spr() {
        let operation = WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text: "hello".into() });
        let bytes = encode_op(&operation).expect("encode op");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
    }
}
