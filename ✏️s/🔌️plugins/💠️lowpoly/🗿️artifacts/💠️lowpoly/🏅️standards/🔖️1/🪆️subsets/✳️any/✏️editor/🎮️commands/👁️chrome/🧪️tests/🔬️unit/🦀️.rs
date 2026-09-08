
use crate::editor::lowpoly::LowpolyCommand;
use crate::editor::lowpoly::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn toggle_show_edges_emits_config_operation() {
    let mut a = app().await;
    let result = dispatch(&mut a, LowpolyCommand::ToggleShowEdges(super::toggle_show_edges::ToggleShowEdges {})).await;
    assert!(result.mutations.is_empty(), "chrome toggle is config-only");
}
