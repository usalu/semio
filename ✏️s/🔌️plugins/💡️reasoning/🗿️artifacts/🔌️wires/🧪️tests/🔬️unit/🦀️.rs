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

/// 🩹️ A document whose composed `content` handle arrived WITHOUT its working scene still reads its
/// board. This is the exact live defect the play pane showed on 2026-09-22: the reasoning-wires
/// canvas drew nothing but its grid and the Artifact panel's RELATIONSHIPS section was empty, while
/// the same render's IDENTITIES section listed all seven topics — identities come off the persisted
/// `wires_fixture`, the board came off the child owner, and the owner had not survived the transport
/// the document arrived through (the sibling law above pins that a plain wire round trip of the
/// handle drops it). `wires_working_scene` now recovers from `wires_fixture.board`, which every
/// transport carries because it is ordinary persisted document data.
#[semio_framework_async_macros::async_test]
async fn an_unmaterialized_content_child_still_reads_the_documents_own_board() {
    let loaded = crate::schema::metabolism_wires_example_snapshot().expect("the committed metabolism example parses");
    assert_eq!(crate::schema::fixture_nodes(&wires_working_board(&loaded)).len(), 7, "the codec-decoded example materializes its child");

    // 🚚️ Exactly what a transport that runs neither the pack nor the DSL codec hands the app back:
    // the same persisted fields, and a `content` handle reduced to its `(child_id, target)` pair.
    let wire = dsl::os_pack::to_json_string(&loaded.content);
    let stripped: WiresContentChild = dsl::os_pack::from_json_str(&wire).expect("Wires child wire roundtrip");
    assert!(stripped.local_owner::<WiresWorkingScene>().is_none(), "the wire form carries no working scene — that is the premise of this law");
    let arrived = WiresSnapshot { wires_fixture: loaded.wires_fixture.clone(), content: stripped, meta: loaded.meta.clone() };

    let board = wires_working_board(&arrived);
    assert_eq!(crate::schema::fixture_nodes(&board).len(), 7, "the canvas renders one layer per identity, never an empty grid");
    assert_eq!(crate::schema::fixture_edges(&board).len(), 9, "the Artifact panel's RELATIONSHIPS section reads these");
    assert_eq!(crate::schema::fixture_nodes(&board), crate::schema::fixture_nodes(&wires_working_board(&loaded)), "the recovered board is the decoded one, node for node");
}

/// 🩹️ The recovery is for an ABSENT owner only: a document the user really did empty keeps its empty
/// board, however stale the persisted `wires_fixture.board` beside it may be.
#[semio_framework_async_macros::async_test]
async fn an_owned_empty_scene_is_never_refilled_from_the_persisted_board() {
    let loaded = crate::schema::metabolism_wires_example_snapshot().expect("the committed metabolism example parses");
    let emptied = WiresSnapshot { wires_fixture: loaded.wires_fixture.clone(), content: wires_content_child_with_owner(Vec::new(), Vec::new()), meta: loaded.meta.clone() };
    assert!(crate::schema::fixture_nodes(&wires_working_board(&emptied)).is_empty(), "an owner that exists is honoured verbatim");
}


/// 🖼️ Every board node of the curated example reaches the canvas as a drawable box and every edge as a
/// line between two node centres — the framework `Canvas2dScene` contract, not the raw board records the
/// host silently skipped (the reasoning-wires pane drew only its grid, ticket 26/09/19).
#[test]
fn the_curated_example_projects_into_drawable_canvas_layers() {
    let document = crate::schema::metabolism_wires_example_snapshot().expect("curated example parses");
    let board = crate::wires_working_board(&document);
    let layers: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_string(&dsl::os_pack::json::Value::Array(crate::schema::wires_canvas_layers(&board, &document.wires_fixture)))).expect("independent JSON oracle");
    let layers = layers.as_array().expect("layer list");
    let boxes = layers.iter().filter(|layer| matches!(layer["kind"].as_str(), Some("circle" | "rect")) && layer["width"].as_f64().is_some_and(|width| width > 0.0) && layer["height"].as_f64().is_some_and(|height| height > 0.0)).count();
    let lines = layers.iter().filter(|layer| layer["kind"] == "line" && ["x0", "y0", "x1", "y1"].iter().all(|key| layer[*key].as_f64().is_some_and(f64::is_finite))).count();
    assert_eq!(boxes, crate::schema::fixture_nodes(&board).len());
    assert!(lines >= crate::schema::fixture_edges(&board).len(), "every board edge is a line; a relationship without one adds its own");
    assert!(boxes > 0 && lines > 0, "the curated example has content to draw");
    assert!(layers.iter().all(|layer| !layer["text"].is_string()), "a board `text` string must not shadow the layer record's own `text` object");
}

/// 🧬️ The projection names exactly the snapshot's declared child slots — what the live envelope load checks
/// before a decoded document may replace the store.
#[test]
fn the_child_restore_projection_names_every_declared_child_slot() {
    let snapshot = crate::empty_wires_snapshot();
    let projection = crate::wires_child_restore_projection(&snapshot).expect("the loaded-parent child projection");
    assert_eq!(projection.len(), <crate::WiresSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());
}
