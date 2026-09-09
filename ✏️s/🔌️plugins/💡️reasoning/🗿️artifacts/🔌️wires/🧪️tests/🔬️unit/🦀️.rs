use super::*;

trait WiresChildOwnerOracle {
    fn expected() -> serde_json::Value;
}

struct SerdeJsonWiresChildOwnerOracle;

impl WiresChildOwnerOracle for SerdeJsonWiresChildOwnerOracle {
    fn expected() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral Wires child-owner fixture")
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_kind_uses_the_wires_fixture_schema() {
    assert_eq!(artifact_kind().schema, MINDMAP_WIRES_SCHEMA);
    assert_eq!(artifact_kind().id, "graph.wires");
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_has_empty_fixtures() {
    let snapshot = empty_wires_snapshot();
    assert_eq!(snapshot.wires_fixture.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(0));
    assert_eq!(wires_working_board(&snapshot).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(0));
}

/// 🧪️ Round-trip law: every board node/edge field survives `wires_content_snapshot_from_scene`
/// → `scene_from_wires_content_snapshot`, including fields the neutral `SemioGraphNode`/
/// `SemioGraphEdge` shape has no native slot for (`radius`/`root`/`edgeKind`/...).
#[semio_framework_async_macros::async_test]
async fn node_edge_content_round_trips_through_the_composed_child_snapshot() {
    let node = dsl::to_dsl_value(&dsl::json!({
        "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 3.0, "y": 4.0,
        "radius": 24.0, "text": "Alpha", "root": true, "handles": []
    }))
    .unwrap();
    let edge = dsl::to_dsl_value(&dsl::json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
    let content = wires_content_snapshot_from_scene(std::slice::from_ref(&node), std::slice::from_ref(&edge));
    assert_eq!(content.nodes.len(), 1);
    assert_eq!(content.nodes[0].id.value, "node-1");
    assert_eq!(content.nodes[0].label, "Alpha");
    assert_eq!(content.edges[0].source.value, "node-1");
    let (nodes, edges) = scene_from_wires_content_snapshot(&content);
    assert_eq!(nodes, vec![node]);
    assert_eq!(edges, vec![edge]);
}

#[semio_framework_async_macros::async_test]
async fn content_child_handle_is_content_addressed_and_deterministic() {
    let node = dsl::to_dsl_value(&dsl::json!({ "id": "a", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "text": "A", "handles": [] })).unwrap();
    let handle_a = wires_content_child_handle(std::slice::from_ref(&node), &[]);
    let handle_b = wires_content_child_handle(std::slice::from_ref(&node), &[]);
    assert_eq!(handle_a.child_id, handle_b.child_id, "same content must mint the same handle");
    let handle_c = wires_content_child_handle(&[], &[]);
    assert_ne!(handle_a.child_id, handle_c.child_id, "different content must mint a different handle");
}

#[semio_framework_async_macros::async_test]
async fn wires_child_restore_projection_accepts_the_exact_owned_content() {
    let snapshot = empty_wires_snapshot();
    let projection = store::ChildRestoreProjection::from_snapshot(&snapshot).expect("canonical Wires content child");
    assert_eq!(projection.len(), 1);
    assert!(projection.admits_member("content", &snapshot.content.target));
    assert_eq!(snapshot.content.child_id, snapshot.content.target.artifact_id);
}

#[semio_framework_async_macros::async_test]
async fn wires_working_scene_is_owned_by_the_exact_snapshot_child() {
    let owned = wires_content_child_with_owner(Vec::new(), Vec::new());
    let wire = dsl::os_pack::to_json_string(&owned);
    let reconstructed: WiresContentChild = dsl::os_pack::from_json_str(&wire).expect("Wires child wire roundtrip");
    let observed = serde_json::json!({
        "ownedHasScene": owned.local_owner::<WiresWorkingScene>().is_some(),
        "wireIdentityMatches": owned == reconstructed,
        "wireHasScene": reconstructed.local_owner::<WiresWorkingScene>().is_some(),
    });

    assert_eq!(observed, SerdeJsonWiresChildOwnerOracle::expected());
}
