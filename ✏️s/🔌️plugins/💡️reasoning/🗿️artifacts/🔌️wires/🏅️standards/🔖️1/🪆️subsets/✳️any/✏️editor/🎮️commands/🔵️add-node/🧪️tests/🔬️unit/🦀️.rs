use super::*;
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;
use crate::schema::{board_node, fixture_nodes};

#[semio_framework_async_macros::async_test]
async fn add_node_appends_and_selects() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(AddNode { kind: "identity".into() })).await;
    let board = crate::editor::wires::unit_tests::context::board(&app);
    assert_eq!(fixture_nodes(&board).len(), 1);
    assert!(board_node(&board, "node-1").is_some());
}
