use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_blueprint_windows() {
    let json = dsl::os_pack::json::to_json_string(&layout());
    assert!(json.contains(builder::FORMS_PLAY_WINDOW_BLUEPRINT) && json.contains(try_wizard::FORMS_PLAY_WINDOW_TRY), "layout must reference both window kinds: {json}");
}
