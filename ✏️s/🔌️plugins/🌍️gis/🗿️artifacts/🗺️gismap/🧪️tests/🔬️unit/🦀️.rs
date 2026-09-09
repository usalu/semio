use super::*;
use geo::BoundingRect;
use semio_framework_plugin::{ArtifactInferenceExecutionRequest, ArtifactInferenceServiceRegistry, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};

fn vector_snapshot(case: &serde_json::Value) -> GisMapSnapshot {
    let snapshot = &case["snapshot"];
    gis_map_snapshot_with_derived_children(GisMapSnapshot {
        positions: serde_json::from_value(snapshot["positions"].clone()).expect("positions vector"),
        routes: serde_json::from_value(snapshot["routes"].clone()).expect("routes vector"),
        regions: serde_json::from_value(snapshot["regions"].clone()).expect("regions vector"),
        ..Default::default()
    })
}

fn execute(
    snapshot: &GisMapSnapshot,
    budgets: &WireArtifactInferenceBudget,
    cancellation_id: &str,
    cache_mode: WireArtifactInferenceCacheMode,
) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
    let pack = <GisMapSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    gis_map_inference_service().infer(&ArtifactInferenceExecutionRequest { policy: b"gis-map-v1", budgets, cancellation_id, previous_state: None, requested_cache_mode: cache_mode, canonical_payload: &pack, dependencies: &[] })
}

#[semio_framework_async_macros::async_test]
async fn map_artifact_kind_matches_the_map_out_interchange_kind() {
    let kind = artifact_kind();
    assert_eq!(kind.id, GISMAP_DIALECT.artifact_kind);
    assert_eq!(kind.schema, GIS_MAP_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn the_map_snapshot_defaults_to_empty_feature_collections() {
    let document = GisMapSnapshot::default();
    assert!(document.positions.is_empty());
    assert!(document.routes.is_empty());
    assert!(document.regions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inference_service_exposes_whole_map_metadata() {
    let service = gis_map_inference_service();
    let metadata = service.metadata();
    assert_eq!(metadata.owner, "gis");
    assert_eq!(metadata.artifact_kind, "s.gis.gismap");
    assert_eq!(metadata.artifact_schema, "s.gis.gismap");
    assert_eq!(metadata.inference_schema, "s.gis.gismap.inference");
    let mut registry = ArtifactInferenceServiceRegistry::new();
    registry.register(service).expect("service registers");
    definition().expect("service-bearing definition builds");
}

#[cfg(feature = "component-app-assembly")]
#[semio_framework_async_macros::async_test]
async fn declaration_exposes_one_executable_whole_map_inference() {
    declaration().expect("service-bearing declaration builds");
}

#[semio_framework_async_macros::async_test]
async fn language_neutral_vectors_match_geo_bounding_rect_oracle_and_stable_payload() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/📐️infer-gismap-1/🔣️.json")).expect("language-neutral inference vectors");
    assert_eq!(vectors["subjectSchema"], "../../../🧬️schema/💡️inferences/🔣️.json");
    assert_eq!(vectors["inferenceSchema"], "s.gis.gismap.inference");
    assert_eq!(vectors["schemaVersion"], 1);
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1_000, recursion_depth: 32 };
    for case in vectors["cases"].as_array().expect("cases") {
        let snapshot = vector_snapshot(case);
        let first = execute(&snapshot, &budgets, case["id"].as_str().expect("case id"), WireArtifactInferenceCacheMode::Cold).expect("inference succeeds");
        let second = execute(&snapshot, &budgets, &format!("{}-repeat", case["id"].as_str().expect("case id")), WireArtifactInferenceCacheMode::Cold).expect("repeat succeeds");
        assert_eq!(first.canonical_payload, second.canonical_payload);
        assert_eq!(first.validity, "valid");
        assert_eq!(first.quality, "exact");
        assert!(first.complete);
        let inference =
            <standards::v1::subsets::any::schema::inferences::GisMapInference as semio_framework_os_kernel::FromValue>::from_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&first.canonical_payload).expect("canonical inference payload"))
                .expect("typed inference");
        let expected = &case["expected"];
        assert_eq!(inference.position_count as u64, expected["positionCount"].as_u64().expect("position count"));
        assert_eq!(inference.route_count as u64, expected["routeCount"].as_u64().expect("route count"));
        assert_eq!(inference.region_count as u64, expected["regionCount"].as_u64().expect("region count"));
        let oracle_points = case["oracleCoordinates"].as_array().expect("oracle coordinates").iter().map(|point| geo::Point::new(point[0].as_f64().expect("longitude"), point[1].as_f64().expect("latitude"))).collect::<geo::MultiPoint>();
        let oracle = oracle_points.bounding_rect();
        match (inference.bounds, oracle, expected["bounds"].as_object()) {
            (None, None, None) => {}
            (Some(actual), Some(oracle), Some(expected)) => {
                assert_eq!(actual.lon_min, oracle.min().x);
                assert_eq!(actual.lon_max, oracle.max().x);
                assert_eq!(actual.lat_min, oracle.min().y);
                assert_eq!(actual.lat_max, oracle.max().y);
                assert_eq!(actual.lon_min, expected["lonMin"].as_f64().expect("lon min"));
                assert_eq!(actual.lon_max, expected["lonMax"].as_f64().expect("lon max"));
                assert_eq!(actual.lat_min, expected["latMin"].as_f64().expect("lat min"));
                assert_eq!(actual.lat_max, expected["latMax"].as_f64().expect("lat max"));
            }
            state => panic!("vector, subject, and geo oracle disagree: {state:?}"),
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn malformed_snapshot_is_a_structured_execution_error() {
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000, work_units: 10, recursion_depth: 4 };
    let error = gis_map_inference_service()
        .infer(&ArtifactInferenceExecutionRequest {
            policy: &[],
            budgets: &budgets,
            cancellation_id: "malformed",
            previous_state: None,
            requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
            canonical_payload: b"not-a-gismap-pack",
            dependencies: &[],
        })
        .err()
        .expect("malformed snapshot must fail");
    assert_eq!(error.code, "gis.gismap.inference.snapshot-decode");
}

#[semio_framework_async_macros::async_test]
async fn service_enforces_work_recursion_and_cancellation_identity() {
    let snapshot = GisMapSnapshot { positions: vec![MapFeature { id: "nested".into(), data: dsl::DslValue::from(serde_json::json!({ "geometry": { "lon": 1.0, "lat": 2.0 } })) }], ..Default::default() };
    let no_work = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 32 };
    assert_eq!(execute(&snapshot, &no_work, "work", WireArtifactInferenceCacheMode::Cold).err().expect("work budget").code, "gis.gismap.inference.budget");
    let no_allocation = WireArtifactInferenceBudget { allocation_bytes: 1, work_units: 1_000, recursion_depth: 32 };
    assert_eq!(execute(&snapshot, &no_allocation, "allocation", WireArtifactInferenceCacheMode::Cold).err().expect("allocation budget").code, "gis.gismap.inference.budget");
    let no_depth = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1_000, recursion_depth: 1 };
    assert_eq!(execute(&snapshot, &no_depth, "depth", WireArtifactInferenceCacheMode::Cold).err().expect("recursion budget").code, "gis.gismap.inference.budget");
    let valid = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1_000, recursion_depth: 32 };
    assert_eq!(execute(&snapshot, &valid, "", WireArtifactInferenceCacheMode::Cold).err().expect("cancellation identity").code, "gis.gismap.inference.cancellation");
    assert_eq!(execute(&snapshot, &valid, "incremental", WireArtifactInferenceCacheMode::Incremental).err().expect("unsupported incremental cache").code, "gis.gismap.inference.cache-mode");
    assert_eq!(execute(&snapshot, &valid, "bypass", WireArtifactInferenceCacheMode::Bypass).expect("bypass succeeds").actual_cache_mode, WireArtifactInferenceCacheMode::Bypass);
}
