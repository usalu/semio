//! ⚖️ Layout artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpText`/`protocol::OpBinary for LayoutMutation` are implemented directly in
//! `../📝️text/🦀️component.rs` (`serde_json`-based, no DSL mirror needed now that every variant wraps
//! a plain local payload struct — see that file's doc comment for why the pre-migration
//! `LayoutMutationDsl`/`FramePatchDsl`/`ColorPatch` mirrors were retired). This component only adds
//! the thin artifact-facing `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.
//!
//! The app's typed `LayoutCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it lives in `🎛️apps/📏️layout/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::layout::schema::mutations::text::LayoutMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `LayoutMutation` to its binary state-patch form.
pub fn encode_op(operation: &LayoutMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LayoutMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<LayoutMutation, protocol::ProtocolError> {
    LayoutMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::layout::LayoutSnapshot;
    use crate::artifacts::layout::mutations::rename_layout;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = LayoutMutation::RenameLayout(rename_layout::mutation::RenameLayout { new_name: "Renamed".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_binary_round_trips_a_store_with_applied_operations() {
        use crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA;

        let initial = crate::artifacts::layout::engine::default_document();
        let envelope = store::create_document_envelope(LAYOUT_DOCUMENT_SCHEMA, "layout-doc-binary-test", initial, None);
        let mut doc_store: store::ArtifactStore<LayoutSnapshot, LayoutMutation> = store::ArtifactStore::new(envelope);
        doc_store
            .dispatch(store::ArtifactCommand::Apply {
                mutations: vec![LayoutMutation::RenameLayout(rename_layout::mutation::RenameLayout { new_name: "Renamed".into() })],
                description: Some("rename document".into()),
            })
            .expect("apply rename");
        store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
        store::os_store::test_support::assert_live_equals_replay(&doc_store);
    }
}
//#endregion 🧪️Tests
