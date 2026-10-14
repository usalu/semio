extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use crate::context::{app, dispatch};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn toggle_show_edges_emits_config_operation() {
    let mut a = app().await;
    let result = dispatch(&mut a, LowpolyCommand::ToggleShowEdges(semio_s_artifact_lowpoly_lowpoly_commands::chrome::toggle_show_edges::ToggleShowEdges {})).await;
    assert!(result.mutations.is_empty(), "chrome toggle is config-only");
}
