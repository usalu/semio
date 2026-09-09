use super::*;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data, replace_route_data};
use crate::schema::{default_document, empty_gis_map_snapshot};
use crate::GIS_MAP_SCHEMA;
use serde_json::json;

fn dsl_of(value: &serde_json::Value) -> dsl::DslValue {
    dsl::DslValue::from(value)
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
async fn gis_map_document_text_round_trips_through_store() {
    let initial = empty_gis_map_snapshot();
    let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis2d-demo", initial, None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    store.install_member_store_owners_exact(gis_map_document_store_owners());
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: sample_feature("p1") })], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    close_gis_map_candidate(store);
}

#[semio_framework_async_macros::async_test]
async fn gis_map_default_document_is_non_empty() {
    assert!(!default_document().positions.is_empty());
}

fn empty_gis_map_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> GisMapStoreInitializationAuthority {
    let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis-map-retained-load", empty_gis_map_snapshot(), None);
    GisMapStoreInitializationAuthority::new(envelope, operation, generation)
}

fn drive_gis_map_initializer(authority: &mut GisMapStoreInitializationAuthority, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::StepOutcome {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(authority, &mut context);
        if outcome.is_terminal() {
            return outcome;
        }
    }
    panic!("GIS retained initializer did not reach a bounded terminal")
}

fn close_gis_map_candidate(mut candidate: store::ArtifactStore<GisMapSnapshot, GisMapMutation>) {
    use semio_framework_plugin::ArtifactOwnedDisposer;

    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<GisMapSnapshot, GisMapMutation>::new();
    for _ in 0..100_000 {
        match disposer.close_step(&mut candidate, 1, GIS_MAP_OWNED_FIELD_BYTES).expect("GIS candidate close step") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
            }
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } | semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("fresh GIS candidate close unexpectedly blocked: {reason}"),
            semio_framework_plugin::PluginCloseStep::Complete => {
                assert!(disposer.terminal_is_empty(&candidate));
                drop(disposer);
                drop(candidate);
                return;
            }
        }
    }
    panic!("GIS candidate did not reach terminal-empty close")
}

#[test]
fn gis_map_store_initializer_publishes_next_generation_and_candidate_closes_incrementally() {
    let operation = semio_framework_job::OperationId(601);
    let generation = semio_framework_job::Generation(21);
    let mut authority = empty_gis_map_initializer(operation, generation);
    assert!(matches!(drive_gis_map_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact GIS candidate");
    assert_eq!(candidate.generation_now(), 22);
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_gis_map_candidate(candidate);
}

#[test]
fn gis_map_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
    let operation = semio_framework_job::OperationId(602);
    let generation = semio_framework_job::Generation(23);
    let mut cancelled = empty_gis_map_initializer(operation, generation);
    semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
    assert!(matches!(drive_gis_map_initializer(&mut cancelled, operation, generation), semio_framework_job::StepOutcome::Cancelled));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
    drop(cancelled);

    let mut stale = empty_gis_map_initializer(operation, generation);
    let mut outcome = drive_gis_map_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1));
    assert!(matches!(&outcome, semio_framework_job::StepOutcome::Fault(_)));
    assert!(matches!(outcome.close_step(0, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 }));
    for _ in 0..=semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES {
        if outcome.terminal_is_empty() {
            break;
        }
        match outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            semio_framework_job::JobPayloadCloseStep::Complete => {}
        }
    }
    assert!(outcome.terminal_is_empty());
    drop(outcome);
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
}

#[test]
fn gis_map_nested_value_mutation_and_all_child_handles_retire_one_owner_per_grant() {
    fn drain(mut retirement: Box<dyn store::ErasedSnapshotRetirement>) {
        for _ in 0..10_000 {
            match retirement.close_step(1, GIS_MAP_OWNED_FIELD_BYTES).expect("one nested GIS owner retires") {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return;
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned GIS retirement cannot block"),
            }
        }
        panic!("nested GIS retirement did not reach terminal")
    }

    let mut snapshot = empty_gis_map_snapshot();
    snapshot.image = Some(store::ArtifactChild::new("image-child".into(), snapshot.drawing.target.clone()));
    drain(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, snapshot));

    let mutation = GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData {
        id: "position".repeat(32),
        new_data: dsl::DslValue::Object(vec![("nested".repeat(32), dsl::DslValue::Array(vec![dsl::DslValue::String("payload".repeat(128)), dsl::DslValue::String("tail".into())]))]),
    });
    drain(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapMutationRetirementFactory, mutation));
}

#[test]
fn gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership() {
    let feature = |id: &str| MapFeature { id: id.into(), data: dsl::DslValue::Null };
    let mutations = vec![
        GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: feature("position") }),
        GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "position".into() }),
        GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "position".into(), to_index: 1 }),
        GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id: "position".into(), new_data: dsl::DslValue::Null }),
        GisMapMutation::CreateRoute(create_route::CreateRoute { index: 0, item: feature("route") }),
        GisMapMutation::DeleteRoute(delete_route::DeleteRoute { id: "route".into() }),
        GisMapMutation::ReorderRoutes(reorder_routes::ReorderRoutes { id: "route".into(), to_index: 1 }),
        GisMapMutation::ReplaceRouteData(replace_route_data::ReplaceRouteData { id: "route".into(), new_data: dsl::DslValue::Null }),
        GisMapMutation::CreateRegion(create_region::CreateRegion { index: 0, item: feature("region") }),
        GisMapMutation::DeleteRegion(delete_region::DeleteRegion { id: "region".into() }),
        GisMapMutation::ReorderRegions(reorder_regions::ReorderRegions { id: "region".into(), to_index: 1 }),
        GisMapMutation::ReplaceRegionData(replace_region_data::ReplaceRegionData { id: "region".into(), new_data: dsl::DslValue::Null }),
    ];
    for mutation in mutations {
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapMutationRetirementFactory, mutation);
        assert!(matches!(retirement.close_step(0, GIS_MAP_OWNED_FIELD_BYTES).expect("zero-grant GIS retirement"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        for _ in 0..100 {
            match retirement.close_step(1, GIS_MAP_OWNED_FIELD_BYTES).expect("one catalog owner retires") {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    break;
                }
                store::SnapshotRetirementStep::Blocked => panic!("unshared GIS mutation owner cannot block"),
            }
        }
        assert!(retirement.terminal_is_empty());
        drop(retirement);
    }
}
