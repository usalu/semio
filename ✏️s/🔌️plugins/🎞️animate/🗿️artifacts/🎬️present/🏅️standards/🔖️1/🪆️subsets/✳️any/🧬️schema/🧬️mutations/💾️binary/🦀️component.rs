//! 📡️ Animate present artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! Also hosts the `PresentEnvelope`/`PresentStore` type aliases and the VCS envelope helpers — both need
//! `PresentMutation` (from `crate::artifacts::present::op`) alongside `PresentSnapshot` (from the artifact's
//! own component file), so this is the natural home for them.
//!
//! The app's typed `PresentCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/🎬️present/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`. Its
//! WASM bridge moved to `🎛️apps/🎬️present/🦀️wasm.rs`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::present::schema::{empty_present_snapshot, PresentError};
use crate::artifacts::present::schema::mutations::text::PresentMutation;
use crate::artifacts::present::{PresentSnapshot, PRESENT_DOCUMENT_SCHEMA};
use protocol::OpBinary;
use store::{create_document_envelope, materialize_document_snapshot, ArtifactEnvelope, ArtifactStore};

/// 📦️ Encodes a `PresentMutation` to its binary state-patch form.
pub fn encode_op(operation: &PresentMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `PresentMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<PresentMutation, protocol::ProtocolError> {
    PresentMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type PresentEnvelope = ArtifactEnvelope<PresentSnapshot, PresentMutation>;
pub type PresentStore = ArtifactStore<PresentSnapshot, PresentMutation>;
//#endregion 🔖️Store

//#region 🔖️VcsEnvelope
/// 📦️ Creates an empty typed VCS envelope for a presentation deck document.
pub fn create_present_envelope(id: &str) -> PresentEnvelope {
    create_document_envelope(PRESENT_DOCUMENT_SCHEMA, id, empty_present_snapshot(), None)
}

/// 📐️ Replays every stored edit in `envelope_json` and returns the materialized deck projection.
pub fn materialize_present_projection_json(envelope_json: &str) -> Result<PresentSnapshot, PresentError> {
    let envelope: PresentEnvelope = serde_json::from_str(envelope_json)?;
    let edit_ids: Vec<String> = envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
    Ok(materialize_document_snapshot(&envelope, &edit_ids)?)
}
//#endregion 🔖️VcsEnvelope

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::schema::mutations::text::PresentMutation;
    use crate::artifacts::present::schema::mutations::{create_tile, replace_tiles};
    use store::{os_store::test_support, ArtifactCommand};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() });
        test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn envelope_helpers_round_trip() {
        let envelope = create_present_envelope("deck-1");
        let json = serde_json::to_string(&envelope).expect("serialize");
        let deck = materialize_present_projection_json(&json).expect("materialize");
        assert_eq!(deck.schema, PRESENT_DOCUMENT_SCHEMA);
        assert!(deck.tiles.is_empty());
    }

    #[test]
    fn present_deck_materializes() {
        let mut store = PresentStore::new(create_document_envelope(PRESENT_DOCUMENT_SCHEMA, "animate-present", empty_present_snapshot(), None));
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile: crate::artifacts::present::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: crate::artifacts::present::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } } })],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.snapshot().expect("projection").tiles.len(), 1);
    }

    //#region 🔖️DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = PresentStore::new(create_document_envelope(PRESENT_DOCUMENT_SCHEMA, "animate-present", crate::artifacts::present::default_present_snapshot(), None));
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile: crate::artifacts::present::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: crate::artifacts::present::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } } })],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖️DocumentTextTests
}
//#endregion 🧪️Tests
