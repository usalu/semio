//! ⚖️ GIS terrain artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).
//!
//! 🧷️ `Gis3dTerrainOperation` derives `dsl::DslOps` directly (no foreign `CollectionOperation` in its
//! shape, unlike the map artifact), so this component is a pure pass-through over the derived codec.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::gisterrain::op::Gis3dTerrainOperation;
use protocol::OpBinary;

//#region 🔖️Codec
/// 📦️ Encodes a `Gis3dTerrainOperation` to its binary command form.
pub fn encode_op(operation: &Gis3dTerrainOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Gis3dTerrainOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Gis3dTerrainOperation, protocol::ProtocolError> {
    Gis3dTerrainOperation::decode_op(bytes)
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gisterrain::{Gis3dTerrainDocument, GIS_3D_TERRAIN_SCHEMA};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Gis3dTerrainOperation::SetExaggeration { exaggeration: 2.0 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn gis3d_terrain_set_exaggeration_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&Gis3dTerrainOperation::SetExaggeration { exaggeration: 3.0 });
    }

    #[test]
    fn gis3d_terrain_set_imported_features_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&Gis3dTerrainOperation::SetImportedFeatures { features_json: r#"{"positions":[]}"#.into() });
    }

    #[test]
    fn gis3d_terrain_set_document_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&Gis3dTerrainOperation::SetDocument { document: Gis3dTerrainDocument { exaggeration: 2.0, imported_features_json: "null".into() } });
    }

    /// 🧷️ Pins the exact pre-migration bytes for every terrain operation row. Hex copied verbatim from
    /// the pre-migration baseline dump (ticket
    /// `26/08/05/GIS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`, `🧪️wire-baseline-3d-before.txt`).
    #[test]
    fn operation_rows_keep_their_pre_migration_bytes() {
        let hex = |operation: &Gis3dTerrainOperation| encode_op(operation).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(hex(&Gis3dTerrainOperation::SetExaggeration { exaggeration: 3.0 }), "0100000100050000000000000840");
        assert_eq!(hex(&Gis3dTerrainOperation::SetImportedFeatures { features_json: r#"{"positions":[]}"#.into() }), "010101107b22706f736974696f6e73223a5b5d7d01000600");
        assert_eq!(hex(&Gis3dTerrainOperation::SetDocument { document: Gis3dTerrainDocument { exaggeration: 2.0, imported_features_json: "null".into() } }), "010201046e756c6c01000e0d0200050000000000000040010600");
    }

    #[test]
    fn gis3d_terrain_document_text_round_trips_through_store() {
        let initial = Gis3dTerrainDocument { exaggeration: 1.0, imported_features_json: String::new() };
        let envelope = store::create_document_envelope(GIS_3D_TERRAIN_SCHEMA, "gis3d-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![Gis3dTerrainOperation::SetExaggeration { exaggeration: 2.0 }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
