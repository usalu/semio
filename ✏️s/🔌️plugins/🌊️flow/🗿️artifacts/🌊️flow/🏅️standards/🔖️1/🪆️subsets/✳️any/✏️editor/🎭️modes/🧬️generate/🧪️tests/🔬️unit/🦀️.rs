use super::*;

#[semio_framework_async_macros::async_test]
async fn the_named_layout_lists_all_three_generate_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    for window in [generations::FLOW_PLAY_WINDOW_GENERATIONS, form::FLOW_PLAY_WINDOW_GENERATE_FORM, preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW] {
        assert!(json.contains(window), "layout must reference {window}: {json}");
    }
}

#[semio_framework_async_macros::async_test]
async fn generation_commands_are_owned_by_the_generate_mode() {
    let mode = definition();
    assert_eq!(mode.commands.iter().map(|command| command.id.as_str()).collect::<Vec<_>>(), ["addGeneration", "removeGeneration", "selectGeneration", "renameGeneration", "updateGenerationValues"]);
    assert!(mode.commands.iter().all(|command| !command.in_palette));
}
