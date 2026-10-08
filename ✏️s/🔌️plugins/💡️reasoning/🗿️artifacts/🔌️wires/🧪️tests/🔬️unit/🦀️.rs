use super::*;

/// 🧾️ The demo's bundled board content.
fn demo_content() -> &'static SemioGraphSnapshot {
    &wires_bundled_contents().iter().find(|(id, _)| id == WIRES_DEMO_CONTENT_ID).expect("the demo board is bundled").1
}

#[semio_framework_async_macros::async_test]
async fn artifact_kind_uses_the_wires_snapshot_schema() {
    assert_eq!(artifact_kind().schema, MINDMAP_WIRES_SCHEMA);
    assert_eq!(artifact_kind().id, "graph.wires");
}

/// ⚖️ LAW: a fresh document has no identity and names the empty board child, which genesis composes.
#[semio_framework_async_macros::async_test]
async fn the_empty_snapshot_names_the_empty_board_child() {
    let snapshot = empty_wires_snapshot();
    assert_eq!(snapshot.wires_snapshot.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(0));
    assert!(snapshot.wires_snapshot.get("board").is_none() && snapshot.wires_snapshot.get("relationships").is_none(), "the parent carries no board");
    let pack = genesis_wires_child_pack(&snapshot, WIRES_CONTENT_SLOT, &snapshot.content.child_id).expect("genesis composes the empty board");
    assert_eq!(<SemioGraphSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("graph pack"), empty_wires_content());
    assert!(genesis_wires_child_pack(&snapshot, WIRES_CONTENT_SLOT, "another-child").is_none(), "genesis answers only the child the parent names");
}

/// ⚖️ LAW (§20.15 bridge): every board node and edge field survives the graph projection — native identity, kind, label,
/// position and extent; every other field as a keyed property; an edge's relationship as its `relationship` map.
#[semio_framework_async_macros::async_test]
async fn board_values_round_trip_through_the_graph_child() {
    let node = semio_framework_value::ToValue::to_value(&semio_framework_pack_json::json!({
        "handles": [], "id": "node-1", "nodeKind": "identity", "radius": 24.0, "root": true, "shape": "circle", "text": "Alpha", "x": 3.0, "y": 4.0
    }));
    let edge = semio_framework_value::ToValue::to_value(&semio_framework_pack_json::json!({
        "edgeKind": "wires.owns", "id": "edge-1", "relationship": { "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 }, "source": "node-1", "target": "node-2"
    }));
    let content = wires_content_snapshot(std::slice::from_ref(&node), std::slice::from_ref(&edge));
    assert_eq!((content.nodes[0].id.value.as_str(), content.nodes[0].label.as_str(), content.nodes[0].kind.as_str()), ("node-1", "Alpha", "identity"));
    assert_eq!(content.nodes[0].properties.iter().map(|entry| entry.key.as_str()).collect::<Vec<_>>(), ["handles", "radius", "root", "shape"]);
    assert_eq!((content.edges[0].source.value.as_str(), content.edges[0].kind.as_str()), ("node-1", "wires.owns"));
    assert_eq!(wires_board_node(&content.nodes[0]), node);
    assert_eq!(wires_board_edge(&content.edges[0]), edge);
}

/// ⚖️ LAW: the composed read derives the relationships from the child's edges and keeps the parent's identities verbatim.
#[semio_framework_async_macros::async_test]
async fn the_composed_read_derives_relationships_from_the_child_edges() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::metabolism_wires_example_snapshot().expect("the committed metabolism example parses");
    let composed = wires_composed(&document, demo_content());
    assert_eq!(schema::board_snapshot_nodes(&composed.board).len(), 7);
    assert_eq!(schema::board_snapshot_edges(&composed.board).len(), 9);
    assert_eq!(schema::wires_relationships(&composed.identity_snapshot).len(), 9, "every demo edge carries its relationship");
    assert_eq!(schema::wires_identities(&composed.identity_snapshot), schema::wires_identities(&document.wires_snapshot));
    let first = &schema::wires_relationships(&composed.identity_snapshot)[0];
    assert_eq!((first.get("edgeId").and_then(|value| value.as_str()), first.get("kind").and_then(|value| value.as_str())), (Some("edge-1"), Some("owns")));
}

/// ⚖️ LAW: the content handle is content-addressed and deterministic.
#[semio_framework_async_macros::async_test]
async fn content_handle_is_content_addressed_and_deterministic() {
    assert_eq!(wires_content_handle(demo_content()), wires_content_handle(demo_content()), "same content mints the same handle");
    assert_ne!(wires_content_handle(demo_content()).child_id, wires_content_handle(&empty_wires_content()).child_id, "different content mints a different handle");
}

#[semio_framework_async_macros::async_test]
async fn wires_child_restore_projection_accepts_the_exact_owned_content() {
    let snapshot = empty_wires_snapshot();
    let projection = store::ChildRestoreProjection::from_snapshot(&snapshot).expect("canonical Wires content child");
    assert_eq!(projection.len(), 1);
    assert!(projection.admits_member(WIRES_CONTENT_SLOT, &snapshot.content.target));
    assert_eq!(snapshot.content.child_id, snapshot.content.target.artifact_id);
}

/// 🖼️ Every board node of the curated example reaches the canvas as a drawable box and every edge as a
/// line between two node centres — the framework `Canvas2dScene` contract, not the raw board records the
/// host silently skipped (the reasoning-wires pane drew only its grid, ticket 26/09/19).
#[test]
fn the_curated_example_projects_into_drawable_canvas_layers() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::metabolism_wires_example_snapshot().expect("curated example parses");
    let composed = wires_composed(&document, demo_content());
    let layers: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::DslValue::Array(schema::wires_canvas_layers(&composed.board, &composed.identity_snapshot))))).expect("independent JSON oracle");
    let layers = layers.as_array().expect("layer list");
    let boxes = layers.iter().filter(|layer| matches!(layer["kind"].as_str(), Some("circle" | "rect")) && layer["width"].as_f64().is_some_and(|width| width > 0.0) && layer["height"].as_f64().is_some_and(|height| height > 0.0)).count();
    let lines = layers.iter().filter(|layer| layer["kind"] == "line" && ["x0", "y0", "x1", "y1"].iter().all(|key| layer[*key].as_f64().is_some_and(f64::is_finite))).count();
    assert_eq!(boxes, schema::board_snapshot_nodes(&composed.board).len());
    assert_eq!(lines, schema::board_snapshot_edges(&composed.board).len(), "every board edge is a line");
    assert!(boxes > 0 && lines > 0, "the curated example has content to draw");
    assert!(layers.iter().all(|layer| !layer["text"].is_string()), "a board `text` string must not shadow the layer record's own `text` object");
}

/// 🧬️ The projection names exactly the snapshot's declared child slots — what the live envelope load checks
/// before a decoded document may replace the store.
#[test]
fn the_child_restore_projection_names_every_declared_child_slot() {
    let snapshot = empty_wires_snapshot();
    let projection = store::ChildRestoreProjection::from_snapshot(&snapshot).expect("the loaded-parent child projection");
    assert_eq!(projection.len(), <WiresSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::child_slots().len());
}
