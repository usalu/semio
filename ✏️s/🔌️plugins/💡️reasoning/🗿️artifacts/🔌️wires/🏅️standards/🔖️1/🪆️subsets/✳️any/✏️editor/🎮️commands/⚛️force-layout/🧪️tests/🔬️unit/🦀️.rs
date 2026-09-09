use super::*;
use crate::editor::wires::commands::reorganize;
use crate::editor::wires::testkit::{dispatch, metabolism_app};
use crate::editor::wires::WiresCommand;

#[semio_framework_async_macros::async_test]
async fn force_layout_action_repositions_metabolism_nodes() {
    let mut app = metabolism_app().await;
    let before: Vec<(f64, f64)> = fixture_nodes(&crate::wires_working_board(&app.snapshot().expect("snapshot"))).iter().map(node_position).collect();
    dispatch(&mut app, WiresCommand::ForceLayout(ForceLayout {})).await;
    let after: Vec<(f64, f64)> = fixture_nodes(&crate::wires_working_board(&app.snapshot().expect("snapshot"))).iter().map(node_position).collect();
    assert_eq!(before.len(), after.len());
    assert_ne!(before, after, "force layout should move at least one node");
}

#[semio_framework_async_macros::async_test]
async fn reorganize_repositions_metabolism_nodes() {
    let mut app = metabolism_app().await;
    let before: Vec<(f64, f64)> = fixture_nodes(&crate::wires_working_board(&app.snapshot().expect("snapshot"))).iter().map(node_position).collect();
    dispatch(&mut app, WiresCommand::Reorganize(reorganize::Reorganize {})).await;
    let after: Vec<(f64, f64)> = fixture_nodes(&crate::wires_working_board(&app.snapshot().expect("snapshot"))).iter().map(node_position).collect();
    assert_eq!(before.len(), after.len());
    assert_ne!(before, after, "reorganize should move at least one node");
}
