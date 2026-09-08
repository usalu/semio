
use super::*;
use crate::editor::fem2d::Fem2dCommand;
use crate::editor::fem2d::testkit::{dispatch, fem2d_app};

#[semio_framework_async_macros::async_test]
async fn set_result_display_is_config_only() {
    let mut app = fem2d_app();
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, Fem2dCommand::SetResultDisplay(SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 })).await;
    assert!(result.mutations.is_empty(), "setResultDisplay must not emit document operations (it's config-only)");
    assert_eq!(app.snapshot().expect("snapshot"), before);
}
