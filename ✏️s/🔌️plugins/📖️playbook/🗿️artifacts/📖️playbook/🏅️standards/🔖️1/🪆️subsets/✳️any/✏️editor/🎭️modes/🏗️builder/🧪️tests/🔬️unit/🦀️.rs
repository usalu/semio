use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_tab_stack_lists_every_authored_scene_window() {
    let json = protocol::json::to_json_string(&layout());
    for window in [
        builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER,
        steps_window::PLAYBOOK_PLAY_WINDOW_STEPS,
        changes_window::PLAYBOOK_PLAY_WINDOW_CHANGES,
        activity_window::PLAYBOOK_PLAY_WINDOW_ACTIVITY,
        source_window::PLAYBOOK_PLAY_WINDOW_SOURCE,
    ] {
        assert!(json.contains(window), "layout must reference {window}: {json}");
    }
}
