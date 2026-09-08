
use super::*;

fn scene(directed: bool) -> EquationWorkingScene {
    let mut graph = EquationGraph::default();
    graph.directed = directed;
    EquationWorkingScene { graph, geometry: EquationGeometry::default() }
}

fn owned_snapshot(directed: bool) -> EquationSnapshot {
    let scene = scene(directed);
    equation_snapshot_with_state(scene.graph, scene.geometry)
}

fn replace_scene_owner(snapshot: &mut EquationSnapshot, scene: Arc<EquationWorkingScene>) {
    snapshot.notation.set_local_owner(scene.clone());
    snapshot.results.set_local_owner(scene.clone());
    snapshot.computed.set_local_owner(scene);
}

#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
    assert_eq!(artifact_kind().schema, "computation.equation");
    assert_eq!(MATH_DOCUMENT_SCHEMA, "semio.equation/v1");
}

#[semio_framework_async_macros::async_test]
async fn default_graph_has_nodes_and_edges() {
    let graph = EquationGraph::default();
    assert!(!graph.nodes.is_empty());
    assert!(!graph.edges.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn default_geometry_has_points() {
    assert!(!EquationGeometry::default().points.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn carrier_fixture_contains_child_state_and_rejects_a_wire_only_parent() {
    let snapshot = owned_snapshot(true);
    let fixture = equation_fixture(&snapshot).expect("owned scene projects");
    assert_eq!(fixture.graph, equation_scene_owner(&snapshot).unwrap().graph);
    assert_eq!(fixture.geometry, EquationGeometry::default());

    let wire = snapshot.to_value();
    let decoded = EquationSnapshot::from_value(wire).expect("parent wire decodes");
    assert_eq!(equation_fixture(&decoded), Err(store::ArtifactChildMaterializationError::Absent));
}

#[semio_framework_async_macros::async_test]
async fn scene_owner_fixture_proves_identity_isolation_aba_wire_omission_and_bounded_close() {
    let fixture: dsl::os_pack::json::Value = dsl::os_pack::json::parse(include_str!("../../🧫️fixtures/👑️equation-scene-owner-law.json")).expect("language-neutral equation scene fixture");
    let cases = fixture["cases"].as_array().expect("fixture cases");
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(fixture["ownedSlots"], 3);
    assert_eq!(cases.len(), fixture["maximumCases"].as_u64().expect("bounded maximum") as usize);
    assert_eq!(cases.len(), 5);

    for case in cases {
        let law = case["law"].as_str().expect("law");
        let left_directed = case["leftDirected"].as_bool().expect("leftDirected");
        let right_directed = case["rightDirected"].as_bool().expect("rightDirected");
        match law {
            "tripleIdentity" => {
                let snapshot = owned_snapshot(left_directed);
                let notation = snapshot.notation.local_owner::<EquationWorkingScene>().expect("notation owner");
                let results = snapshot.results.local_owner::<EquationWorkingScene>().expect("results owner");
                let computed = snapshot.computed.local_owner::<EquationWorkingScene>().expect("computed owner");
                assert!(Arc::ptr_eq(&notation, &results) && Arc::ptr_eq(&results, &computed));
                assert_eq!(Arc::strong_count(&notation), 6);
            }
            "instanceIsolation" => {
                let left = owned_snapshot(left_directed);
                let mut right = left.clone();
                replace_scene_owner(&mut right, Arc::new(scene(right_directed)));
                assert_eq!(left.results.child_id, right.results.child_id, "hostile identity collision is deliberate");
                assert_eq!(equation_graph(&left).directed, left_directed);
                assert_eq!(equation_graph(&right).directed, right_directed);
            }
            "abaIsolation" => {
                let stale_a = owned_snapshot(left_directed);
                let mut reused_identity_b = stale_a.clone();
                replace_scene_owner(&mut reused_identity_b, Arc::new(scene(right_directed)));
                assert_eq!(stale_a.computed.child_id, reused_identity_b.computed.child_id);
                assert_eq!(equation_graph(&reused_identity_b).directed, right_directed);
                drop(reused_identity_b);
                assert_eq!(equation_graph(&stale_a).directed, left_directed);
            }
            "wireOmission" => {
                let left = owned_snapshot(left_directed);
                let mut right = left.clone();
                replace_scene_owner(&mut right, Arc::new(scene(right_directed)));
                let left_wire = left.to_value();
                let right_wire = right.to_value();
                assert_eq!(left_wire, right_wire, "local owners never alter the durable wire");
                let decoded = EquationSnapshot::from_value(left_wire).expect("first-party codec decodes snapshot");
                assert!(decoded.results.local_owner::<EquationWorkingScene>().is_none());
            }
            "boundedClose" => {
                let snapshot = owned_snapshot(left_directed);
                let retained = snapshot.results.local_owner::<EquationWorkingScene>().expect("retained owner");
                let weak = Arc::downgrade(&retained);
                assert_eq!(Arc::strong_count(&retained), fixture["ownedSlots"].as_u64().expect("owned slots") as usize + 1);
                drop(snapshot);
                assert_eq!(Arc::strong_count(&retained), 1);
                drop(retained);
                assert!(weak.upgrade().is_none());
            }
            other => panic!("unexpected equation scene law {other}"),
        }
    }
}
