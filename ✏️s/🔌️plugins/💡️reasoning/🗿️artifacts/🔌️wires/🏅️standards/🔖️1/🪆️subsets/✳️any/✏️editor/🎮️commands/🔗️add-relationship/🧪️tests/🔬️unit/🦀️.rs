use super::*;
use crate::editor::wires::commands::add_node;
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;

/// 🔗️ `addRelationship` connects the NAMED endpoints: two freshly added nodes, related `node-2 → node-1`
/// (the reverse of creation order, so a hardcoded `node-1 → node-2` cannot pass).
#[semio_framework_async_macros::async_test]
async fn add_relationship_connects_the_named_nodes_and_selects_the_edge() {
    let mut app = new_app().await;
    for _ in 0..2 {
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    }
    dispatch(&mut app, WiresCommand::AddRelationship(AddRelationship { kind: "owns".into(), source_id: "node-2".into(), target_id: "node-1".into() })).await;
    let projection = app.snapshot().expect("snapshot");
    let board = crate::wires_working_board(&projection);
    let edges = fixture_edges(&board);
    assert_eq!(edges.len(), 1);
    assert_eq!(entity_id(&edges[0], "source"), Some("node-2"));
    assert_eq!(entity_id(&edges[0], "target"), Some("node-1"));
    assert_eq!(entity_id(&edges[0], "edgeKind"), Some("wires.owns"));
}

/// 🛑️ Missing, unknown and identical endpoints are refused by name — never a silent success and never
/// a guessed pair of nodes.
#[semio_framework_async_macros::async_test]
async fn add_relationship_refuses_missing_unknown_and_identical_endpoints_by_name() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    for (payload, code) in [
        (AddRelationship { kind: "owns".into(), source_id: String::new(), target_id: String::new() }, "wires.relationship-endpoints-missing"),
        (AddRelationship { kind: "owns".into(), source_id: "node-1".into(), target_id: String::new() }, "wires.relationship-endpoint-missing"),
        (AddRelationship { kind: "owns".into(), source_id: "node-1".into(), target_id: "node-9".into() }, "mutation.target-missing"),
        (AddRelationship { kind: "owns".into(), source_id: "node-1".into(), target_id: "node-1".into() }, "wires.relationship-self"),
    ] {
        app.dispatch_typed(WiresCommand::AddRelationship(payload), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("admitted");
        let fault = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut *app, 1).await.err().expect("refused");
        assert!(fault.message.contains(code) || fault.code.0 == code, "expected {code}, got {fault:?}");
    }
    let projection = app.snapshot().expect("snapshot");
    assert!(fixture_edges(&crate::wires_working_board(&projection)).is_empty(), "no refused invocation wrote an edge");
}
