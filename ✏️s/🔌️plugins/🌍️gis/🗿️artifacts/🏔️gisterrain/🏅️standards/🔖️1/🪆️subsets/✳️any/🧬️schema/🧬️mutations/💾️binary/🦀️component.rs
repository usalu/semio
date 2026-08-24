//! ⚖️ GIS terrain artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).
//!
//! 🧷️ `GisTerrainMutation` derives `dsl::DslEnum` directly (no foreign `CollectionMutation` in its
//! shape, unlike the map artifact), so this component is a pure pass-through over the derived codec.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::gisterrain::schema::mutations::text::GisTerrainMutation;
use protocol::OpBinary;

//#region 🔖️Codec
/// 📦️ Encodes a `GisTerrainMutation` to its binary command form.
pub fn encode_op(operation: &GisTerrainMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisTerrainMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<GisTerrainMutation, protocol::ProtocolError> {
    GisTerrainMutation::decode_op(bytes)
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gisterrain::schema::mutations::change_exaggeration::mutation::ChangeExaggeration;
    use crate::artifacts::gisterrain::schema::mutations::change_imported_features::mutation::ChangeImportedFeatures;
    use crate::artifacts::gisterrain::{GisTerrainSnapshot, GIS_3D_TERRAIN_SCHEMA};

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.0 });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis3d_terrain_change_exaggeration_op_line_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 3.0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis3d_terrain_change_imported_features_op_line_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: r#"{"positions":[]}"#.into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis3d_terrain_document_text_round_trips_through_store() {
        let initial = GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: String::new(), ..Default::default() };
        let envelope = store::create_document_envelope(GIS_3D_TERRAIN_SCHEMA, "gis3d-demo", initial, None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.0 })], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
