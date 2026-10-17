#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::{blueprint, preview};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_WINDOW_BLUEPRINT) && json.contains(semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW), "layout must reference both window kinds: {json}");
}
