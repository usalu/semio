
use super::*;
use crate::editor::wires::WiresCommand;
use crate::editor::wires::testkit::{dispatch, new_app};
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;

#[semio_framework_async_macros::async_test]
async fn add_node_appends_and_selects() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(AddNode { kind: "identity".into() })).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(fixture_nodes(&crate::wires_working_board(&projection)).len(), 1);
    assert!(find_board_node(&projection, "node-1").is_some());
}
