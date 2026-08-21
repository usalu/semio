//! ⚖️ GIS map artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).
//!
//! 🧷️ `GisMapMutation` derives `dsl::DslEnum` directly (no foreign `CollectionMutation` in its
//! shape — every variant wraps a local `dsl::DslRecord` payload declared in its own triad leaf), so
//! this component is a pure pass-through over the derived codec, matching `🏔️gisterrain`'s sibling
//! facet's identical shape.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::gismap::schema::mutations::text::GisMapMutation;
use protocol::OpBinary;

//#region 🔖️Codec
/// 📦️ Encodes a `GisMapMutation` to its binary command form.
pub async fn encode_op(operation: &GisMapMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisMapMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<GisMapMutation, protocol::ProtocolError> {
    GisMapMutation::decode_op(bytes)
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::mutations::{create_position, create_region, create_route, delete_position, reorder_positions, reorder_regions, reorder_routes, replace_position_data};
    use crate::artifacts::gismap::schema::{default_document, empty_gis_map_snapshot};
    use crate::artifacts::gismap::GIS_MAP_SCHEMA;
    use serde_json::json;

    async fn dsl_of(value: &serde_json::Value) -> dsl::DslValue {
        dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
    }

    async fn sample_feature(id: &str) -> crate::artifacts::gismap::MapFeature {
        crate::artifacts::gismap::MapFeature { id: id.into(), data: dsl_of(&json!({ "id": id, "lon": 1.0, "lat": 2.0 })) }
    }

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = GisMapMutation::CreatePosition(create_position::mutation::CreatePosition { index: 0, item: sample_feature("p1") });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_positions_op_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreatePosition(create_position::mutation::CreatePosition { index: 0, item: sample_feature("p1") }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::DeletePosition(delete_position::mutation::DeletePosition { id: "p1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderPositions(reorder_positions::mutation::ReorderPositions { id: "p1".into(), to_index: 3 }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReplacePositionData(replace_position_data::mutation::ReplacePositionData { id: "p1".into(), new_data: dsl_of(&json!({ "label": "Home" })) }));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_routes_op_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreateRoute(create_route::mutation::CreateRoute { index: 0, item: sample_feature("p1") }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderRoutes(reorder_routes::mutation::ReorderRoutes { id: "p1".into(), to_index: 1 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_regions_op_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreateRegion(create_region::mutation::CreateRegion { index: 0, item: sample_feature("p1") }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderRegions(reorder_regions::mutation::ReorderRegions { id: "p1".into(), to_index: 2 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_document_text_round_trips_through_store() {
        let initial = empty_gis_map_snapshot();
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis2d-demo", initial, None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![GisMapMutation::CreatePosition(create_position::mutation::CreatePosition { index: 0, item: sample_feature("p1") })], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_default_document_is_non_empty() {
        assert!(!default_document().positions.is_empty());
    }
}
//#endregion 🧪️Tests
