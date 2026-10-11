use super::*;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data, replace_route_data, set_position_property, remove_position_property, set_route_property, remove_route_property, set_region_property, remove_region_property};
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


fn retire_owned_exact<T>(factory: &dyn store::ArtifactOwnedValueRetirementFactory<T>, value: T) -> Box<dyn store::ErasedSnapshotRetirement> {
    let birth = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: factory.retirement_birth_bytes(&value), maximum_release_bytes: 0, maximum_depth: 2 };
    match factory.retire_owned(value, birth) {
        Ok((owner, receipt)) => {
            assert!(receipt.fits(birth));
            owner
        }
        Err((error, _value)) => panic!("GIS owner birth refused: {error}"),
    }
}

fn grant_for(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

fn install_gis_map_owners(store: &mut store::ArtifactStore<GisMapSnapshot, GisMapMutation>) {
    let owners = store::funded_bounded_artifact_store_owners::<GisMapSnapshot, GisMapMutation>().expect("GIS owner catalog is fully funded");
    if let Err((error, _owners)) = store.install_document_store_owners_exact(owners) {
        panic!("GIS store refused its exact owner catalog: {error}");
    }
}

#[semio_framework_async_macros::async_test]
async fn gis_map_document_text_round_trips_through_store() {
    let initial = empty_gis_map_snapshot();
    let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis2d-demo", initial, None);
    let mut store = store::ArtifactStore::new(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    install_gis_map_owners(&mut store);
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: sample_feature("p1") })], transaction: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    close_gis_map_candidate(store);
}

fn empty_gis_map_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> GisMapStoreInitializationAuthority {
    let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis-map-retained-load", empty_gis_map_snapshot(), None);
    GisMapStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))
}

#[derive(Debug, PartialEq, Eq)]
enum GisDriveEnd {
    Complete,
    Cancelled,
    Fault,
}

fn drive_gis_map_initializer(authority: &mut GisMapStoreInitializationAuthority, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> GisDriveEnd {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        let end = match semio_framework_plugin::ArtifactStoreInitializationAuthority::step(authority, &mut context).expect("GIS initializer turn") {
            Some(semio_framework_job::JobOutcomeBorrow::Complete { .. }) => Some(GisDriveEnd::Complete),
            Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => Some(GisDriveEnd::Cancelled),
            Some(semio_framework_job::JobOutcomeBorrow::Fault { .. }) => Some(GisDriveEnd::Fault),
            _ => None,
        };
        if let Some(end) = end {
            return end;
        }
    }
    panic!("GIS retained initializer did not reach a bounded terminal")
}

fn close_gis_map_initializer(authority: &mut GisMapStoreInitializationAuthority) {
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;
    for _ in 0..100_000 {
        if authority.terminal_is_empty() {
            return;
        }
        authority.begin_close();
        let grant = grant_for(authority.retirement_demands(GIS_MAP_OWNED_FIELD_BYTES).expect("GIS initializer close quote"));
        let step = authority.close_step(grant).expect("GIS initializer close step");
        assert!(step.progress().fits(grant));
    }
    panic!("GIS initializer did not reach terminal-empty close")
}

fn close_gis_map_candidate(mut candidate: store::ArtifactStore<GisMapSnapshot, GisMapMutation>) {
    use semio_framework_plugin::ArtifactOwnedDisposer;

    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<GisMapSnapshot, GisMapMutation>();
    for _ in 0..100_000 {
        if disposer.terminal_is_empty(&candidate) {
            drop(disposer);
            drop(candidate);
            return;
        }
        let demand = disposer.retirement_demands(&candidate, 0).expect("GIS candidate close quote");
        let grant = grant_for(demand);
        match disposer.close_step(&mut candidate, grant).expect("GIS candidate close step") {
            semio_framework_plugin::PluginLifecycleStep::Progress(progress) | semio_framework_plugin::PluginLifecycleStep::Complete(progress) => assert!(progress.fits(grant)),
            semio_framework_plugin::PluginLifecycleStep::AwaitingInput { reason } | semio_framework_plugin::PluginLifecycleStep::Blocked { reason } => panic!("fresh GIS candidate close unexpectedly blocked: {reason}"),
        }
    }
    panic!("GIS candidate did not reach terminal-empty close")
}

#[test]
fn gis_map_history_edit_initializer_aliases_genesis_and_publishes_next_generation_and_candidate_closes_incrementally() {
    let operation = semio_framework_job::OperationId(601);
    let generation = semio_framework_job::Generation(21);
    let mut authority = empty_gis_map_initializer(operation, generation);
    let genesis = authority.envelope.as_ref().expect("retained GisMap envelope").vcs.genesis.facts().share_snapshot();
    assert_eq!(drive_gis_map_initializer(&mut authority, operation, generation), GisDriveEnd::Complete);
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact GIS candidate");
    assert_eq!(candidate.generation_now(), 22);
    assert!(std::sync::Arc::ptr_eq(&genesis, &candidate.snapshot_owner()));
    drop(genesis);
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
    assert_eq!(drive_gis_map_initializer(&mut cancelled, operation, generation), GisDriveEnd::Cancelled);
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
    drop(cancelled);

    let mut stale = empty_gis_map_initializer(operation, generation);
    assert_eq!(drive_gis_map_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1)), GisDriveEnd::Fault);
    close_gis_map_initializer(&mut stale);
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
}

#[test]
fn gis_map_nested_value_mutation_and_all_child_handles_retire_one_owner_per_grant() {
    fn drain(mut retirement: Box<dyn store::ErasedSnapshotRetirement>) {
        for _ in 0..10_000 {
            let grant = grant_for(retirement.next_demand(GIS_MAP_OWNED_FIELD_BYTES).expect("one nested GIS owner quotes its turn"));
            match retirement.close_step(grant).expect("one nested GIS owner retires") {
                semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant)),
                semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => {
                    assert!(progress.fits(grant));
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return;
                }
            }
        }
        panic!("nested GIS retirement did not reach terminal")
    }

    let mut snapshot = empty_gis_map_snapshot();
    snapshot.image = Some(store::ArtifactChild::new("image-child".into(), snapshot.drawing.target.clone()));
    drain(retire_owned_exact(&GisMapSnapshotRetirementFactory, snapshot));

    let mutation = GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData {
        id: "position".repeat(32),
        new_data: semio_framework_value::DslValue::Object(vec![("nested".repeat(32), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String("payload".repeat(128)), semio_framework_value::DslValue::Bytes(vec![7; 1024]), semio_framework_value::DslValue::String("tail".into())]))]),
    });
    drain(retire_owned_exact(&GisMapMutationRetirementFactory, mutation));
}

#[test]
fn gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership() {
    let feature = |id: &str| MapFeature { id: id.into(), data: semio_framework_value::DslValue::Null };
    let mutations = vec![
        GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: feature("position") }),
        GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "position".into() }),
        GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "position".into(), to_index: 1 }),
        GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id: "position".into(), new_data: semio_framework_value::DslValue::Null }),
        GisMapMutation::CreateRoute(create_route::CreateRoute { index: 0, item: feature("route") }),
        GisMapMutation::DeleteRoute(delete_route::DeleteRoute { id: "route".into() }),
        GisMapMutation::ReorderRoutes(reorder_routes::ReorderRoutes { id: "route".into(), to_index: 1 }),
        GisMapMutation::ReplaceRouteData(replace_route_data::ReplaceRouteData { id: "route".into(), new_data: semio_framework_value::DslValue::Null }),
        GisMapMutation::CreateRegion(create_region::CreateRegion { index: 0, item: feature("region") }),
        GisMapMutation::DeleteRegion(delete_region::DeleteRegion { id: "region".into() }),
        GisMapMutation::ReorderRegions(reorder_regions::ReorderRegions { id: "region".into(), to_index: 1 }),
        GisMapMutation::ReplaceRegionData(replace_region_data::ReplaceRegionData { id: "region".into(), new_data: semio_framework_value::DslValue::Null }),
        GisMapMutation::SetPositionProperty(set_position_property::SetPositionProperty { feature: "position".into(), key: "label".into(), value: semio_framework_value::DslValue::Null, before: Some("next".into()) }),
        GisMapMutation::RemovePositionProperty(remove_position_property::RemovePositionProperty { feature: "position".into(), key: "label".into() }),
        GisMapMutation::SetRouteProperty(set_route_property::SetRouteProperty { feature: "route".into(), key: "label".into(), value: semio_framework_value::DslValue::Null, before: Some("next".into()) }),
        GisMapMutation::RemoveRouteProperty(remove_route_property::RemoveRouteProperty { feature: "route".into(), key: "label".into() }),
        GisMapMutation::SetRegionProperty(set_region_property::SetRegionProperty { feature: "region".into(), key: "label".into(), value: semio_framework_value::DslValue::Null, before: Some("next".into()) }),
        GisMapMutation::RemoveRegionProperty(remove_region_property::RemoveRegionProperty { feature: "region".into(), key: "label".into() }),
    ];
    for mutation in mutations {
        let mut retirement = retire_owned_exact(&GisMapMutationRetirementFactory, mutation);
        let zero = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 8 };
        assert!(matches!(retirement.close_step(zero).expect("zero-grant GIS retirement"), semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) if progress == Default::default()));
        for _ in 0..1_000 {
            let grant = grant_for(retirement.next_demand(GIS_MAP_OWNED_FIELD_BYTES).expect("one catalog owner quotes its turn"));
            if matches!(retirement.close_step(grant).expect("one catalog owner retires"), semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(retirement.terminal_is_empty());
        drop(retirement);
    }
}

//#region 🧾️CandidateProjectionLaw
/// 🧹️ Drives one candidate store to its exact terminal-empty ownership witness.
fn close_candidate_store(store: store::ArtifactStore<GisMapSnapshot, GisMapMutation>) {
    close_gis_map_candidate(store);
}

/// 🧾️ LAW: the candidate this crate's OWN store-initialization authority hands the replacement pump
/// must project into the bounded child authority the pump demands, and no candidate is EVER exposed
/// before the initializer is terminal.
///
/// 🩺️ Why here and not beside the snapshot law: the pump
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `AwaitingMembers`) calls
/// `store::ChildRestoreProjection::from_snapshot(candidate.snapshot_ref())` and maps EVERY
/// `ChildRestoreProjectionError` onto the single opaque string `candidate parent child projection is
/// invalid` — the fault `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed`
/// reports (ticket 26/09/19). The sibling law
/// `every_gis_map_parent_snapshot_projects_its_canonical_child_handles` already pins every snapshot
/// this crate can CONSTRUCT; this one pins the object the pump actually holds — the candidate the
/// initializer builds field-by-field out of `GisMapSnapshotCloneAuthority`, whose working value
/// starts as `empty_child()` placeholders (all five handle fields empty, exactly the shape
/// `ChildRestoreProjection`'s visitor rejects as `InvalidReference`). If a candidate ever escapes
/// mid-clone, THIS is the law that names it instead of the pump's erased string.
#[semio_framework_async_macros::async_test]
async fn the_initialization_candidate_projects_its_canonical_child_handles() {
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;

    for (label, snapshot) in [("empty", empty_gis_map_snapshot()), ("default-document", default_document())] {
        let expected = snapshot.clone();
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis-map-initialization-law", snapshot, None);
        let operation = semio_framework_job::OperationId(u64::MAX - 401);
        let generation = semio_framework_job::Generation(61);
        let mut authority = GisMapStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut preview_sequence = 0;
        let mut complete = false;
        for _ in 0..200_000 {
            let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            match ArtifactStoreInitializationAuthority::step(&mut authority, &mut cx).expect("GIS initializer turn") {
                Some(semio_framework_job::JobOutcomeBorrow::Complete { .. }) => {
                    complete = true;
                    break;
                }
                Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => panic!("{label} initializer cancelled"),
                Some(semio_framework_job::JobOutcomeBorrow::Fault { .. }) => panic!("{label} initializer faulted"),
                _ => {}
            }
            if let Some(early) = ArtifactStoreInitializationAuthority::take_candidate(&mut authority) {
                let early_root = early.snapshot_root();
                let projection = format!("{:?}", store::ChildRestoreProjection::from_snapshot(&*early_root));
                drop(early_root);
                close_candidate_store(early);
                panic!("{label}: a candidate was handed over before the initializer reported Complete (it projects {projection}) — the pump would see the placeholder handles");
            }
        }
        assert!(complete, "{label} initializer converges");

        let candidate = ArtifactStoreInitializationAuthority::take_candidate(&mut authority).unwrap_or_else(|| panic!("{label} candidate handoff"));
        {
            let root = candidate.snapshot_root();
            let projection = store::ChildRestoreProjection::from_snapshot(&*root).unwrap_or_else(|error| {
                panic!("{label} candidate parent is not projectable: {error:?} (drawing {:?}/{:?}, image {:?}, value {:?}/{:?})", root.drawing.child_id, root.drawing.target, root.image, root.value.child_id, root.value.target)
            });
            assert_eq!(projection.len(), 2, "{label} candidate declares exactly the drawing and value children");
            for index in 0..projection.len() {
                let (slot, fields) = projection.get(index).expect("admitted row");
                assert_eq!(fields.child_id, fields.artifact_id, "{label} candidate slot {slot}: a composed child's id IS its target artifact id");
                assert_eq!(fields.artifact_kind, "s.stdio.semio", "{label} candidate slot {slot} kind");
            }
            assert_eq!(*root, expected, "{label} candidate is the exact field-by-field clone of the loaded snapshot (every feature list and every child handle)");
        }
        assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&authority), "{label} initializer is terminal once its candidate is handed over");
        drop(authority);
        close_candidate_store(candidate);
    }
}
//#endregion 🧾️CandidateProjectionLaw
