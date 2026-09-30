use super::*;

#[semio_framework_async_macros::async_test]
async fn a_live_patch_declares_the_narrow_scope_and_an_anonymous_one_keeps_full() {
    assert_eq!(patch_coalesce_key(Some("p1")).as_deref(), Some("widget-field:p1"), "a live patch folds under its press");
    assert_eq!(patch_coalesce_key(Some("")), None, "an empty press identity is no identity");
    assert_eq!(patch_coalesce_key(None), None, "an anonymous patch is a described edit");
    assert_ne!(
        crate::editor::generation3d::commands::node_graph_edit::slider_gesture_ui_scope(),
        semio_framework::kernel::UiDirtyScope::Full,
        "a live patch must not repaint the whole shell"
    );
}
