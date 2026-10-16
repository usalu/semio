use crate::editor::playbook::modes::builder::windows::activity as activity_window;
use crate::editor::playbook::modes::builder::windows::builder as builder_window;
use crate::editor::playbook::modes::builder::windows::changes as changes_window;
use crate::editor::playbook::modes::builder::windows::files as files_window;
use crate::editor::playbook::modes::builder::windows::source as source_window;
use crate::editor::playbook::modes::builder::windows::steps as steps_window;
use crate::editor::playbook::modes::builder::*;

#[semio_framework_async_macros::async_test]
async fn the_default_tab_stack_lists_every_authored_scene_window() {
    let json = semio_framework_pack_json::to_json_string(&layout());
    for window in [
        builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER,
        steps_window::PLAYBOOK_PLAY_WINDOW_STEPS,
        changes_window::PLAYBOOK_PLAY_WINDOW_CHANGES,
        activity_window::PLAYBOOK_PLAY_WINDOW_ACTIVITY,
        source_window::PLAYBOOK_PLAY_WINDOW_SOURCE,
        files_window::PLAYBOOK_PLAY_WINDOW_FILES,
    ] {
        assert!(json.contains(window), "layout must reference {window}: {json}");
    }
}
