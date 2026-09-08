
use super::*;
use crate::editor::wires::WiresCommand;
use crate::editor::wires::testkit::{dispatch, new_app};

#[semio_framework_async_macros::async_test]
async fn add_relationship_appends_edge_and_selects() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddRelationship(AddRelationship { kind: "owns".into() })).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(fixture_edges(&crate::wires_working_board(&projection)).len(), 1);
}
