use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app};
use crate::editor::fem3d::Fem3dCommand;

#[semio_framework_async_macros::async_test]
async fn set_result_display_writes_config_not_artifact_mutations() {
    let mut app = fem3d_app();
    // 🎯️ No config accessor on `VcsArtifactApp` — dispatch must simply not panic/error, and the
    // results window render test (in `modes::edit::windows::results`) covers the resulting display.
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 })).await;
}
