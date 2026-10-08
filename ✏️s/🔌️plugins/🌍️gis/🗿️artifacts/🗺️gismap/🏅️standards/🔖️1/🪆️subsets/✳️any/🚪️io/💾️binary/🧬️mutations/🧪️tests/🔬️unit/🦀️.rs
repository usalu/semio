use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data, replace_route_data};
use crate::standards::v1::subsets::any::io::text::snapshot::{default_document};
use crate::standards::v1::subsets::any::io::text::snapshot::{empty_gis_map_snapshot};
use crate::GIS_MAP_SCHEMA;
use serde_json::json;

fn dsl_of(value: &serde_json::Value) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::from(value)
}

fn sample_feature(id: &str) -> MapFeature {
    MapFeature { id: id.into(), data: dsl_of(&json!({ "id": id, "lon": 1.0, "lat": 2.0 })) }
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: sample_feature("p1") });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn gis_map_positions_op_lines_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: sample_feature("p1") }));
    store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "p1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "p1".into(), to_index: 3 }));
    store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id: "p1".into(), new_data: dsl_of(&json!({ "label": "Home" })) }));
}

#[semio_framework_async_macros::async_test]
async fn gis_map_routes_op_lines_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreateRoute(create_route::CreateRoute { index: 0, item: sample_feature("p1") }));
    store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderRoutes(reorder_routes::ReorderRoutes { id: "p1".into(), to_index: 1 }));
}

#[semio_framework_async_macros::async_test]
async fn gis_map_regions_op_lines_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreateRegion(create_region::CreateRegion { index: 0, item: sample_feature("p1") }));
    store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderRegions(reorder_regions::ReorderRegions { id: "p1".into(), to_index: 2 }));
}

#[semio_framework_async_macros::async_test]
async fn gis_map_default_document_is_non_empty() {
    assert!(!default_document().positions.is_empty());
}

