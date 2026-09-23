use super::*;
use crate::editor::wires::commands::add_node;
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;

/// 🔗️ `addRelationship` connects `node-1` to `node-2`, so the board needs both: on the empty
/// `initial_snapshot` the `connect-nodes` diff answers `mutation.target-missing` and nothing lands.
#[semio_framework_async_macros::async_test]
async fn add_relationship_appends_edge_and_selects() {
    let mut app = new_app().await;
    for _ in 0..2 {
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    }
    dispatch(&mut app, WiresCommand::AddRelationship(AddRelationship { kind: "owns".into() })).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(fixture_edges(&crate::wires_working_board(&projection)).len(), 1);
}
